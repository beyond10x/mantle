//! Actual CONNECT sockets with a controlled upstream IO boundary. The proxy does all policy work.
use super::*;
use anyhow::{Context, Result, bail};
use mantle_conformance::{Boundary, Reply, bytes, encoded, integer};
use serde_json::{Value, json};
use std::sync::Mutex;

struct Upstream {
    resolver: String,
    connector: String,
    echo: SocketAddr,
    names: Mutex<Vec<String>>,
    addresses: Mutex<Vec<String>>,
}
impl Network for Upstream {
    fn resolve(&self, name: String, port: u16) -> IoFuture<'_, Vec<SocketAddr>> {
        self.names.lock().unwrap().push(name);
        Box::pin(async move {
            let addresses: &[&str] = match self.resolver.as_str() {
                "public" => &["93.184.216.34"],
                "private" => &["127.0.0.1", "169.254.169.254", "::1"],
                "mixed" => &["169.254.169.254", "93.184.216.34", "127.0.0.1"],
                "empty" => &[],
                "failure" => return Err(io::Error::other("fixture DNS unavailable")),
                "timeout" => return std::future::pending().await,
                mode => return Err(io::Error::other(format!("unknown DNS mode {mode}"))),
            };
            Ok(addresses
                .iter()
                .map(|ip| SocketAddr::new(ip.parse().unwrap(), port))
                .collect())
        })
    }
    fn connect(&self, address: SocketAddr) -> IoFuture<'_, TcpStream> {
        self.addresses.lock().unwrap().push(address.to_string());
        Box::pin(async move {
            match self.connector.as_str() {
                "echo" => TcpStream::connect(self.echo).await,
                "failure" => Err(io::Error::other("fixture connect refused")),
                "timeout" => std::future::pending().await,
                mode => Err(io::Error::other(format!("unknown connect mode {mode}"))),
            }
        })
    }
}

async fn exchange(input: &Value) -> Result<Value> {
    let echo = TcpListener::bind("127.0.0.1:0").await?;
    let captured = Arc::new(Mutex::new(Vec::new()));
    let capture = Arc::clone(&captured);
    let upstream = Arc::new(Upstream {
        resolver: input["resolver"].as_str().context("resolver")?.into(),
        connector: input["connector"].as_str().context("connector")?.into(),
        echo: echo.local_addr()?,
        names: Mutex::new(Vec::new()),
        addresses: Mutex::new(Vec::new()),
    });
    let echo_task = tokio::spawn(async move {
        let (mut stream, _) = echo.accept().await?;
        let mut buffer = [0u8; 4096];
        loop {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            capture.lock().unwrap().extend_from_slice(&buffer[..n]);
            stream.write_all(&buffer[..n]).await?;
        }
        io::Result::Ok(())
    });
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let proxy_network = Arc::clone(&upstream);
    let config = Arc::new(Config {
        allowlist: Allowlist::default_list(),
        max_connections: 2,
        connect_timeout: Duration::from_millis(60),
        idle_timeout: Duration::from_millis(60),
        head_timeout: Duration::from_millis(60),
        drain_timeout: Duration::from_millis(100),
    });
    let server = tokio::spawn(serve_using(
        listener,
        config,
        async {
            let _ = stopped.await;
        },
        proxy_network,
    ));
    let mut client = TcpStream::connect(address).await?;
    client.write_all(&bytes(&input["request"])?).await?;
    if input["half_close"].as_bool().context("half_close")? {
        client.shutdown().await?;
    }
    let mut received = Vec::new();
    timeout(Duration::from_secs(5), client.read_to_end(&mut received)).await??;
    stop.send(()).ok();
    server.await??;
    echo_task.abort();
    let status = std::str::from_utf8(received.split(|b| *b == b'\n').next().unwrap_or_default())?
        .split_whitespace()
        .nth(1)
        .map(str::parse::<u16>)
        .transpose()?
        .unwrap_or(0);
    let body = received
        .windows(4)
        .position(|p| p == b"\r\n\r\n")
        .map(|p| &received[p + 4..])
        .unwrap_or_default();
    Ok(
        json!({"status":status, "body":encoded(body), "forwarded":encoded(&captured.lock().unwrap()),
        "resolved_names":*upstream.names.lock().unwrap(), "dialled":*upstream.addresses.lock().unwrap()}),
    )
}

#[derive(Default)]
struct Egress;
impl Boundary for Egress {
    fn execute(&mut self, command: &str, input: &Value) -> Result<Reply> {
        match command {
            "mantle.egress.CheckDefaultDestination" => {
                let allowed = Allowlist::default_list().permits(&Destination {
                    host: input["host"].as_str().context("host")?.into(),
                    port: integer(&input["port"])?.try_into()?,
                });
                Ok(if allowed {
                    Reply {
                        outcome: "allowed".into(),
                        response: Some(json!({"allowed":true})),
                        ..Reply::default()
                    }
                } else {
                    Reply {
                        outcome: "denied".into(),
                        error: Some("mantle.egress.NotAllowed".into()),
                        ..Reply::default()
                    }
                })
            }
            "mantle.egress.Exchange" => {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                Ok(Reply::returned(runtime.block_on(exchange(input))?))
            }
            "mantle.egress.ClassifyAddress" => {
                let address = input["address"].as_str().context("address")?.parse()?;
                Ok(Reply::returned(json!({"refusal":addr::refusal(address)})))
            }
            "mantle.egress.CheckBudgets" | "mantle.egress.CheckDrain" => {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                Ok(Reply::returned(
                    runtime.block_on(lifetime(command.ends_with("CheckDrain")))?,
                ))
            }
            _ => bail!("unsupported egress command {command}"),
        }
    }
}

async fn header(stream: &mut TcpStream) -> Result<u16> {
    let mut head = Vec::new();
    timeout(Duration::from_secs(5), async {
        while !head.ends_with(b"\r\n\r\n") {
            let byte = stream.read_u8().await?;
            head.push(byte);
        }
        io::Result::Ok(())
    })
    .await??;
    Ok(std::str::from_utf8(&head)?
        .split_whitespace()
        .nth(1)
        .context("status")?
        .parse()?)
}

async fn lifetime(drain: bool) -> Result<Value> {
    let echo = TcpListener::bind("127.0.0.1:0").await?;
    let network = Arc::new(Upstream {
        resolver: "public".into(),
        connector: "echo".into(),
        echo: echo.local_addr()?,
        names: Mutex::new(Vec::new()),
        addresses: Mutex::new(Vec::new()),
    });
    let echo_task = tokio::spawn(async move {
        while let Ok((mut socket, _)) = echo.accept().await {
            tokio::spawn(async move {
                let (mut read, mut write) = socket.split();
                tokio::io::copy(&mut read, &mut write).await.ok();
            });
        }
    });
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let config = Arc::new(Config {
        allowlist: Allowlist::default_list(),
        max_connections: 1,
        connect_timeout: Duration::from_secs(1),
        idle_timeout: Duration::from_secs(10),
        head_timeout: Duration::from_secs(5),
        drain_timeout: Duration::from_millis(60),
    });
    let server = tokio::spawn(serve_using(
        listener,
        config,
        async {
            let _ = stopped.await;
        },
        network,
    ));
    let request = b"CONNECT github.com:443 HTTP/1.1\r\n\r\n";
    let mut first = TcpStream::connect(address).await?;
    first.write_all(request).await?;
    let first_status = header(&mut first).await?;
    if drain {
        stop.send(()).ok();
        server.await??;
        let closed = matches!(timeout(Duration::from_millis(200),first.read_u8()).await,Ok(Err(error)) if error.kind()==io::ErrorKind::UnexpectedEof);
        let stopped_accepting = TcpStream::connect(address).await.is_err();
        echo_task.abort();
        return Ok(
            json!({"first_status":first_status,"stopped_accepting":stopped_accepting,"connection_closed":closed}),
        );
    }
    let mut second = TcpStream::connect(address).await?;
    second.write_all(request).await?;
    let saturated_status = header(&mut second).await?;
    first.shutdown().await?;
    let mut sink = Vec::new();
    timeout(Duration::from_secs(5), first.read_to_end(&mut sink)).await??;
    // EOF observes the first tunnel completing. Yield its task's final permit drop.
    tokio::task::yield_now().await;
    let mut silent = Vec::new();
    for _ in 0..32 {
        silent.push(TcpStream::connect(address).await?);
    }
    let mut third = TcpStream::connect(address).await?;
    third.write_all(request).await?;
    let after_release_status = header(&mut third).await?;
    drop(third);
    drop(silent);
    stop.send(()).ok();
    server.await??;
    echo_task.abort();
    Ok(json!({"statuses":[first_status,saturated_status,after_release_status]}))
}

#[test]
fn ess_egress_conformance() -> Result<()> {
    mantle_conformance::run::<Egress>("mantle-egress", 4)
}

#[tokio::test]
async fn adversary_codex_dns_failures_never_dial_and_public_control_relays() -> Result<()> {
    for host in ["auth.openai.com", "chatgpt.com"] {
        for resolver in ["private", "empty", "failure", "timeout", "mixed"] {
            let request = format!("CONNECT {host}:443 HTTP/1.1\r\n\r\nDNS-CONTROL");
            let observed = exchange(&json!({
                "request": encoded(request.as_bytes()), "resolver": resolver,
                "connector": "echo", "half_close": true
            }))
            .await?;
            assert_eq!(observed["resolved_names"], json!([format!("{host}.")]));
            if resolver == "mixed" {
                assert_eq!(observed["status"], 200, "{host} {resolver}");
                assert_eq!(observed["dialled"], json!(["93.184.216.34:443"]));
                assert_eq!(bytes(&observed["body"])?, b"DNS-CONTROL");
                assert_eq!(bytes(&observed["forwarded"])?, b"DNS-CONTROL");
            } else {
                assert_eq!(observed["status"], 502, "{host} {resolver}");
                assert_eq!(observed["dialled"], json!([]), "{host} {resolver}");
                assert_eq!(bytes(&observed["forwarded"])?, b"");
                let body = bytes(&observed["body"])?;
                assert!(!body.windows(11).any(|part| part == b"DNS-CONTROL"));
            }
        }
    }
    Ok(())
}

#[tokio::test]
async fn adversary_codex_connect_authority_owns_policy_and_opaque_payload() -> Result<()> {
    let payload = b"\0\xffCONNECT api.openai.com:443 HTTP/1.1\r\nHost: auth.openai.com\r\n\r\nTAIL";
    for host in ["auth.openai.com", "chatgpt.com"] {
        // An allowed Host header cannot admit another CONNECT authority, and malformed
        // authorities cannot be repaired by percent decoding or stripping extra dots.
        for (authority, status) in [
            (format!("{host}.attacker.test:443"), 403),
            (format!("{host}:444"), 403),
            (format!("{host}..:443"), 400),
            (format!("{host}%00.attacker.test:443"), 400),
            (format!("user@{host}:443"), 400),
            ("api.openai.com:443".into(), 403),
        ] {
            let mut request =
                format!("CONNECT {authority} HTTP/1.1\r\nHost: {host}:443\r\n\r\n").into_bytes();
            request.extend_from_slice(payload);
            let observed = exchange(&json!({
                "request": encoded(&request), "resolver": "public",
                "connector": "echo", "half_close": true
            }))
            .await?;
            assert_eq!(observed["status"], status, "{authority}");
            assert_eq!(observed["resolved_names"], json!([]), "{authority}");
            assert_eq!(observed["dialled"], json!([]), "{authority}");
            assert_eq!(bytes(&observed["forwarded"])?, b"");
        }
        // Conversely the CONNECT authority, including existing normalization rules,
        // owns the tunnel despite conflicting headers and HTTP-shaped binary data.
        let mut request = format!(
            "CONNECT {}.:00443 HTTP/1.1\r\nHost: attacker.test:80\r\n\r\n",
            host.to_ascii_uppercase()
        )
        .into_bytes();
        request.extend_from_slice(payload);
        let observed = exchange(&json!({
            "request": encoded(&request), "resolver": "mixed",
            "connector": "echo", "half_close": true
        }))
        .await?;
        assert_eq!(observed["status"], 200, "{host}");
        assert_eq!(observed["resolved_names"], json!([format!("{host}.")]));
        assert_eq!(observed["dialled"], json!(["93.184.216.34:443"]));
        assert_eq!(bytes(&observed["forwarded"])?, payload);
        assert_eq!(bytes(&observed["body"])?, payload);
    }
    Ok(())
}
