//! Adversarial cases against the egress gateway. Every case here asserts the contract the
//! crate states about itself (`lib.rs`: "refuses to dial non-public addresses"; `addr.rs`:
//! "Forms that embed an IPv4 address are judged by that address") or the acceptance of
//! `story:egress-gateway`. No case touches the external network.

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use mantle_egress::addr::refusal;
use mantle_egress::{Allowlist, Config, serve};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;

fn refused(s: &str) -> bool {
    let ip: IpAddr = s.parse().unwrap();
    refusal(ip).is_some()
}

fn permitted_but_not_public(cases: &[&str]) -> Vec<String> {
    cases
        .iter()
        .filter(|s| !refused(s))
        .map(|s| (*s).to_owned())
        .collect()
}

/// 192.0.0.0/24 is the IETF protocol-assignments block (IANA special-purpose registry,
/// "globally reachable: false"). 192.0.0.192 is a metadata endpoint in at least one cloud.
#[test]
fn ipv4_ietf_protocol_block_is_refused() {
    let leaked = permitted_but_not_public(&["192.0.0.192", "192.0.0.8", "192.0.0.1"]);
    assert!(leaked.is_empty(), "dialled as public: {leaked:?}");
}

/// Benchmarking (198.18.0.0/15) and documentation (192.0.2.0/24, 198.51.100.0/24,
/// 203.0.113.0/24) ranges are not public.
#[test]
fn ipv4_benchmarking_and_documentation_ranges_are_refused() {
    let leaked = permitted_but_not_public(&[
        "198.18.0.1",
        "198.19.255.254",
        "192.0.2.1",
        "198.51.100.1",
        "203.0.113.1",
    ]);
    assert!(leaked.is_empty(), "dialled as public: {leaked:?}");
}

/// RFC 8215 local-use NAT64 prefix 64:ff9b:1::/48. On a network whose translator uses it,
/// this address is 169.254.169.254.
#[test]
fn ipv6_local_use_nat64_prefix_embedding_metadata_is_refused() {
    let leaked = permitted_but_not_public(&["64:ff9b:1::a9fe:a9fe", "64:ff9b:1::7f00:1"]);
    assert!(leaked.is_empty(), "dialled as public: {leaked:?}");
}

/// SIIT IPv4-translated form ::ffff:0:a.b.c.d (RFC 2765) embeds an IPv4 address; addr.rs
/// promises such forms are judged by the embedded address.
#[test]
fn ipv6_siit_translated_form_embedding_metadata_is_refused() {
    let leaked = permitted_but_not_public(&["::ffff:0:a9fe:a9fe", "::ffff:0:7f00:1"]);
    assert!(leaked.is_empty(), "dialled as public: {leaked:?}");
}

/// Only 2000::/3 is global unicast. Discard-only 100::/64, documentation 2001:db8::/32,
/// benchmarking 2001:2::/48 and unallocated space outside 2000::/3 are not public.
#[test]
fn ipv6_non_global_unicast_space_is_refused() {
    let leaked = permitted_but_not_public(&[
        "100::1",
        "2001:db8::1",
        "2001:2::1",
        "4000::1",
        "e000::1",
        "::1:0:0:1",
    ]);
    assert!(leaked.is_empty(), "dialled as public: {leaked:?}");
}

async fn start(max_connections: u32) -> (SocketAddr, oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let config = Arc::new(Config {
        allowlist: Allowlist::default_list(),
        max_connections,
        connect_timeout: Duration::from_secs(5),
        idle_timeout: Duration::from_secs(5),
        head_timeout: Duration::from_secs(5),
        drain_timeout: Duration::from_secs(1),
    });
    let (tx, rx) = oneshot::channel::<()>();
    tokio::spawn(serve(listener, config, async {
        let _ = rx.await;
    }));
    (addr, tx)
}

/// One worker runs one gateway shared by every session on it (`deploy/substrate.service`
/// pins a single aperture to 127.0.0.1:3128). A slot is taken at accept, before any byte
/// of the head, so a session that opens `max_connections` sockets and sends nothing holds
/// every slot for `head_timeout`, and by reconnecting holds them indefinitely. Another
/// client's well-formed request then never reaches the allowlist check.
#[tokio::test]
async fn silent_sockets_do_not_starve_a_well_formed_request() {
    let (addr, _stop) = start(2).await;
    let mut squatters = Vec::new();
    for _ in 0..2 {
        squatters.push(TcpStream::connect(addr).await.unwrap());
    }
    tokio::time::sleep(Duration::from_millis(200)).await;

    let mut s = TcpStream::connect(addr).await.unwrap();
    s.write_all(b"CONNECT example.com:443 HTTP/1.1\r\n\r\n")
        .await
        .unwrap();
    let mut out = Vec::new();
    tokio::time::timeout(Duration::from_secs(10), s.read_to_end(&mut out))
        .await
        .expect("proxy closes the connection")
        .unwrap();
    let resp = String::from_utf8(out).unwrap();
    assert!(
        resp.starts_with("HTTP/1.1 403 Forbidden\r\n"),
        "two silent sockets decided the outcome of a third client's request: {resp}"
    );
    drop(squatters);
}
