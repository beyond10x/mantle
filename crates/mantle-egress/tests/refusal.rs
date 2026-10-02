use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use mantle_egress::{Allowlist, Config, serve};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;

async fn start(allow: &[&str]) -> (SocketAddr, oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let config = Arc::new(Config {
        allowlist: Allowlist::from_specs(allow).unwrap(),
        max_connections: 8,
        connect_timeout: Duration::from_secs(5),
        idle_timeout: Duration::from_secs(5),
        head_timeout: Duration::from_secs(2),
        drain_timeout: Duration::from_secs(1),
    });
    let (tx, rx) = oneshot::channel::<()>();
    tokio::spawn(serve(listener, config, async {
        let _ = rx.await;
    }));
    (addr, tx)
}

async fn exchange(addr: SocketAddr, request: &[u8]) -> String {
    let mut s = TcpStream::connect(addr).await.unwrap();
    s.write_all(request).await.unwrap();
    let mut out = Vec::new();
    tokio::time::timeout(Duration::from_secs(10), s.read_to_end(&mut out))
        .await
        .expect("proxy closes the connection")
        .unwrap();
    String::from_utf8(out).unwrap()
}

#[tokio::test]
async fn refuses_unlisted_destination_with_403() {
    let (addr, _stop) = start(mantle_egress::DEFAULT_ALLOW).await;
    let resp = exchange(
        addr,
        b"CONNECT example.com:443 HTTP/1.1\r\nHost: example.com:443\r\n\r\n",
    )
    .await;
    assert!(resp.starts_with("HTTP/1.1 403 Forbidden\r\n"), "{resp}");
    assert!(resp.ends_with("mantle-egress: example.com:443 is not an allowed destination\n"));

    let resp = exchange(addr, b"CONNECT github.com:80 HTTP/1.1\r\n\r\n").await;
    assert!(resp.ends_with("mantle-egress: github.com:80 is not an allowed destination\n"));

    let resp = exchange(addr, b"CONNECT 169.254.169.254:80 HTTP/1.1\r\n\r\n").await;
    assert!(resp.starts_with("HTTP/1.1 403 Forbidden\r\n"), "{resp}");
}

#[tokio::test]
async fn refuses_non_connect() {
    let (addr, _stop) = start(mantle_egress::DEFAULT_ALLOW).await;
    let resp = exchange(
        addr,
        b"GET http://github.com/ HTTP/1.1\r\nHost: github.com\r\n\r\n",
    )
    .await;
    assert!(
        resp.starts_with("HTTP/1.1 405 Method Not Allowed\r\n"),
        "{resp}"
    );
    assert!(resp.ends_with("mantle-egress: method GET is refused; only CONNECT is allowed\n"));

    let resp = exchange(addr, b"garbage\r\n\r\n").await;
    assert!(resp.starts_with("HTTP/1.1 400 Bad Request\r\n"), "{resp}");
}

#[tokio::test]
async fn allowed_name_resolving_to_loopback_is_not_dialled() {
    let upstream = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = upstream.local_addr().unwrap().port();
    let spec = format!("localhost:{port}");
    let (addr, _stop) = start(&[spec.as_str()]).await;
    let req = format!("CONNECT localhost:{port} HTTP/1.1\r\n\r\n");
    let resp = exchange(addr, req.as_bytes()).await;
    assert!(resp.starts_with("HTTP/1.1 502 Bad Gateway\r\n"), "{resp}");
    assert!(resp.ends_with("mantle-egress: localhost resolves only to non-public addresses\n"));
    let accepted = tokio::time::timeout(Duration::from_millis(200), upstream.accept()).await;
    assert!(accepted.is_err(), "proxy must not connect to loopback");
}
