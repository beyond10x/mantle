use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tokio::time::{Instant, timeout};

use crate::addr;
use crate::allow::{Allowlist, Destination, TargetError};
use crate::head::{self, HeadError, MAX_HEAD, ReadError};
use crate::log;

pub struct Config {
    pub allowlist: Allowlist,
    pub max_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
    pub head_timeout: Duration,
    pub drain_timeout: Duration,
}

const WRITE_TIMEOUT: Duration = Duration::from_secs(5);
const SPLICE_BUF: usize = 16 * 1024;

/// Connections still reading their request head. Kept apart from the tunnel slots
/// (`Config::max_connections`) so sockets that send nothing cannot hold the slots a
/// well-formed request needs; a head-reader holds its place for at most `head_timeout`.
pub const MAX_PENDING_HEADS: u32 = 64;

const OVER_CAPACITY_BODY: &str = "mantle-egress: connection limit reached; retry later";

/// Accepts connections until `shutdown` resolves, then waits up to `drain_timeout` for
/// open tunnels to finish.
pub async fn serve<F>(listener: TcpListener, config: Arc<Config>, shutdown: F) -> io::Result<()>
where
    F: Future<Output = ()>,
{
    serve_using(listener, config, shutdown, Arc::new(SystemNetwork)).await
}

type IoFuture<'a, T> = Pin<Box<dyn Future<Output = io::Result<T>> + Send + 'a>>;

/// The upstream IO seam leaves parsing, allowlisting, address filtering and all deadlines
/// in the production proxy. Tests supply DNS answers and observe the addresses actually dialled.
trait Network: Send + Sync {
    fn resolve(&self, name: String, port: u16) -> IoFuture<'_, Vec<SocketAddr>>;
    fn connect(&self, address: SocketAddr) -> IoFuture<'_, TcpStream>;
}

struct SystemNetwork;
impl Network for SystemNetwork {
    fn resolve(&self, name: String, port: u16) -> IoFuture<'_, Vec<SocketAddr>> {
        Box::pin(async move { Ok(tokio::net::lookup_host((name, port)).await?.collect()) })
    }
    fn connect(&self, address: SocketAddr) -> IoFuture<'_, TcpStream> {
        Box::pin(TcpStream::connect(address))
    }
}

async fn serve_using<F>(
    listener: TcpListener,
    config: Arc<Config>,
    shutdown: F,
    network: Arc<dyn Network>,
) -> io::Result<()>
where
    F: Future<Output = ()>,
{
    let heads = Arc::new(Semaphore::new(MAX_PENDING_HEADS as usize));
    let tunnels = Arc::new(Semaphore::new(config.max_connections as usize));
    let mut connections = tokio::task::JoinSet::new();
    tokio::pin!(shutdown);
    loop {
        let accepted = tokio::select! {
            () = &mut shutdown => break,
            _ = connections.join_next(), if !connections.is_empty() => continue,
            accepted = listener.accept() => accepted,
        };
        let stream = match accepted {
            Ok((stream, _)) => stream,
            Err(e) => {
                // Usually fd exhaustion; back off rather than spin.
                log::notice(&format!("accept failed: {e}"));
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }
        };
        match Arc::clone(&heads).try_acquire_owned() {
            Ok(head_permit) => {
                let config = Arc::clone(&config);
                let tunnels = Arc::clone(&tunnels);
                let network = Arc::clone(&network);
                connections.spawn(async move {
                    handle(stream, &config, head_permit, &tunnels, network.as_ref()).await;
                });
            }
            Err(_) => refuse_pending_over_capacity(&stream),
        }
    }
    drop(listener);
    log::notice("shutting down: no longer accepting connections");
    // A head-reader takes its tunnel slot before releasing its head slot, so once every
    // head slot is back no new tunnel can start.
    let drain = async {
        let _heads = heads.acquire_many(MAX_PENDING_HEADS).await;
        let _tunnels = tunnels.acquire_many(config.max_connections).await;
    };
    if timeout(config.drain_timeout, drain).await.is_err() {
        log::notice("drain timeout reached; closing open tunnels");
    }
    // A drain deadline bounds connection lifetime even when this library's runtime remains alive.
    // Dropping detached JoinHandles did not cancel their tasks or close their sockets.
    connections.shutdown().await;
    Ok(())
}

/// Over the head-reading budget: answer with whatever the socket buffer takes without
/// waiting, then close. Nothing is spawned, so a flood costs no task and no held fd.
fn refuse_pending_over_capacity(stream: &TcpStream) {
    let _ = stream.try_write(response(UNAVAILABLE, OVER_CAPACITY_BODY).as_bytes());
    log::emit(&log::Line {
        dest: "-",
        outcome: "refused:over-capacity",
        up: 0,
        down: 0,
        elapsed: Duration::ZERO,
    });
}

struct Refusal {
    tag: &'static str,
    status: Option<(u16, &'static str)>,
    body: String,
}

impl Refusal {
    fn new(tag: &'static str, status: (u16, &'static str), body: String) -> Self {
        Self {
            tag,
            status: Some(status),
            body,
        }
    }

    fn silent(tag: &'static str) -> Self {
        Self {
            tag,
            status: None,
            body: String::new(),
        }
    }
}

const BAD_REQUEST: (u16, &str) = (400, "Bad Request");
const FORBIDDEN: (u16, &str) = (403, "Forbidden");
const NOT_ALLOWED: (u16, &str) = (405, "Method Not Allowed");
const REQUEST_TIMEOUT: (u16, &str) = (408, "Request Timeout");
const TOO_LARGE: (u16, &str) = (431, "Request Header Fields Too Large");
const BAD_GATEWAY: (u16, &str) = (502, "Bad Gateway");
const UNAVAILABLE: (u16, &str) = (503, "Service Unavailable");
const GATEWAY_TIMEOUT: (u16, &str) = (504, "Gateway Timeout");

#[derive(Default)]
struct Tally {
    dest: Option<String>,
    up: u64,
    down: u64,
}

/// The budget a connection currently counts against. Held until the connection's task
/// ends, including while a refusal is written, so every live task holds exactly one.
struct Slot {
    head: Option<OwnedSemaphorePermit>,
    tunnel: Option<OwnedSemaphorePermit>,
}

impl Slot {
    /// Moves from the head budget to the tunnel budget, taking the tunnel slot first so
    /// the connection is never counted against neither.
    fn promote(&mut self, tunnels: &Arc<Semaphore>) -> Result<(), Refusal> {
        let permit = Arc::clone(tunnels).try_acquire_owned().map_err(|_| {
            Refusal::new("over-capacity", UNAVAILABLE, OVER_CAPACITY_BODY.to_owned())
        })?;
        self.tunnel = Some(permit);
        self.head = None;
        Ok(())
    }
}

async fn handle(
    mut client: TcpStream,
    config: &Config,
    head_permit: OwnedSemaphorePermit,
    tunnels: &Arc<Semaphore>,
    network: &dyn Network,
) {
    let started = Instant::now();
    let mut tally = Tally::default();
    let mut slot = Slot {
        head: Some(head_permit),
        tunnel: None,
    };
    let outcome = match tunnel(&mut client, config, &mut tally, &mut slot, tunnels, network).await {
        Ok(end) => format!("allowed end={end}"),
        Err(refusal) => {
            if let Some(status) = refusal.status {
                respond(&mut client, status, &refusal.body).await;
            }
            format!("refused:{}", refusal.tag)
        }
    };
    log::emit(&log::Line {
        dest: tally.dest.as_deref().unwrap_or("-"),
        outcome: &outcome,
        up: tally.up,
        down: tally.down,
        elapsed: started.elapsed(),
    });
    drop(slot);
}

async fn tunnel(
    client: &mut TcpStream,
    config: &Config,
    tally: &mut Tally,
    slot: &mut Slot,
    tunnels: &Arc<Semaphore>,
    network: &dyn Network,
) -> Result<&'static str, Refusal> {
    let raw = head::read_head(client, config.head_timeout)
        .await
        .map_err(|e| match e {
            ReadError::Closed => Refusal::silent("client-closed"),
            ReadError::Io => Refusal::silent("client-error"),
            ReadError::Timeout => Refusal::new(
                "head-timeout",
                REQUEST_TIMEOUT,
                format!(
                    "mantle-egress: request head not received within {} s",
                    config.head_timeout.as_secs()
                ),
            ),
            ReadError::TooLarge => Refusal::new(
                "head-too-large",
                TOO_LARGE,
                format!("mantle-egress: request head exceeds {MAX_HEAD} bytes"),
            ),
        })?;

    let dest = head::parse_head(&raw.head).map_err(|e| match e {
        HeadError::Malformed(why) => Refusal::new(
            "malformed",
            BAD_REQUEST,
            format!("mantle-egress: malformed request: {why}"),
        ),
        HeadError::Method(m) => Refusal::new(
            "method",
            NOT_ALLOWED,
            format!("mantle-egress: method {m} is refused; only CONNECT is allowed"),
        ),
        HeadError::Target(TargetError::Malformed(why)) => Refusal::new(
            "malformed",
            BAD_REQUEST,
            format!("mantle-egress: malformed CONNECT target: {why}"),
        ),
        HeadError::Target(TargetError::IpLiteral(ip)) => {
            tally.dest = Some(ip.clone());
            Refusal::new(
                "ip-literal",
                FORBIDDEN,
                format!(
                    "mantle-egress: {ip} is an IP literal; only allowed host names are reachable"
                ),
            )
        }
    })?;
    tally.dest = Some(dest.to_string());

    if !config.allowlist.permits(&dest) {
        return Err(Refusal::new(
            "not-allowed",
            FORBIDDEN,
            format!("mantle-egress: {dest} is not an allowed destination"),
        ));
    }
    slot.promote(tunnels)?;

    let mut upstream = dial(&dest, config, network).await?;
    let _ = client.set_nodelay(true);
    let _ = upstream.set_nodelay(true);

    write_all_timed(client, b"HTTP/1.1 200 Connection Established\r\n\r\n")
        .await
        .map_err(|()| Refusal::silent("client-error"))?;
    if !raw.rest.is_empty() {
        write_all_timed(&mut upstream, &raw.rest)
            .await
            .map_err(|()| Refusal::silent("upstream-error"))?;
        tally.up += raw.rest.len() as u64;
    }
    Ok(splice(client, &mut upstream, config.idle_timeout, tally).await)
}

/// Resolves the name here rather than trusting the client, and dials only public
/// addresses: an allowed name rebound to 169.254.169.254 or loopback is refused.
async fn dial(
    dest: &Destination,
    config: &Config,
    network: &dyn Network,
) -> Result<TcpStream, Refusal> {
    let resolve_failed = || {
        Refusal::new(
            "dns-failed",
            BAD_GATEWAY,
            format!("mantle-egress: could not resolve {}", dest.host),
        )
    };
    let resolved: Vec<SocketAddr> = match timeout(
        config.connect_timeout,
        network.resolve(dest.resolver_name(), dest.port),
    )
    .await
    {
        Ok(Ok(addrs)) => addrs,
        Ok(Err(_)) | Err(_) => return Err(resolve_failed()),
    };
    if resolved.is_empty() {
        return Err(resolve_failed());
    }
    let permitted: Vec<SocketAddr> = resolved
        .into_iter()
        .filter(|a| addr::refusal(a.ip()).is_none())
        .collect();
    if permitted.is_empty() {
        return Err(Refusal::new(
            "non-public-address",
            BAD_GATEWAY,
            format!(
                "mantle-egress: {} resolves only to non-public addresses",
                dest.host
            ),
        ));
    }

    let attempt = async {
        for a in &permitted {
            if let Ok(s) = network.connect(*a).await {
                return Some(s);
            }
        }
        None
    };
    match timeout(config.connect_timeout, attempt).await {
        Ok(Some(s)) => Ok(s),
        Ok(None) => Err(Refusal::new(
            "connect-failed",
            BAD_GATEWAY,
            format!("mantle-egress: could not connect to {dest}"),
        )),
        Err(_) => Err(Refusal::new(
            "connect-timeout",
            GATEWAY_TIMEOUT,
            format!("mantle-egress: connecting to {dest} timed out"),
        )),
    }
}

#[cfg(test)]
#[path = "conformance.rs"]
mod conformance;

/// Copies both ways until both sides have closed, either errors, or nothing has moved
/// in either direction for `idle`.
async fn splice(
    client: &mut TcpStream,
    upstream: &mut TcpStream,
    idle: Duration,
    tally: &mut Tally,
) -> &'static str {
    let (mut cr, mut cw) = client.split();
    let (mut ur, mut uw) = upstream.split();
    let mut up_buf = vec![0u8; SPLICE_BUF];
    let mut down_buf = vec![0u8; SPLICE_BUF];
    let mut up_open = true;
    let mut down_open = true;
    let deadline = tokio::time::sleep(idle);
    tokio::pin!(deadline);
    loop {
        tokio::select! {
            r = cr.read(&mut up_buf), if up_open => match r {
                Ok(0) => {
                    up_open = false;
                    let _ = uw.shutdown().await;
                }
                Ok(n) => match timeout(idle, uw.write_all(&up_buf[..n])).await {
                    Ok(Ok(())) => tally.up += n as u64,
                    Ok(Err(_)) => return "upstream-error",
                    Err(_) => return "idle",
                },
                Err(_) => return "client-error",
            },
            r = ur.read(&mut down_buf), if down_open => match r {
                Ok(0) => {
                    down_open = false;
                    let _ = cw.shutdown().await;
                }
                Ok(n) => match timeout(idle, cw.write_all(&down_buf[..n])).await {
                    Ok(Ok(())) => tally.down += n as u64,
                    Ok(Err(_)) => return "client-error",
                    Err(_) => return "idle",
                },
                Err(_) => return "upstream-error",
            },
            () = &mut deadline => return "idle",
        }
        if !up_open && !down_open {
            return "eof";
        }
        deadline.as_mut().reset(Instant::now() + idle);
    }
}

async fn write_all_timed(stream: &mut TcpStream, bytes: &[u8]) -> Result<(), ()> {
    match timeout(WRITE_TIMEOUT, stream.write_all(bytes)).await {
        Ok(Ok(())) => Ok(()),
        _ => Err(()),
    }
}

fn response(status: (u16, &str), body: &str) -> String {
    let (code, reason) = status;
    let mut extra = "";
    if code == 405 {
        extra = "Allow: CONNECT\r\n";
    }
    format!(
        "HTTP/1.1 {code} {reason}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\n{extra}Connection: close\r\n\r\n{body}\n",
        body.len() + 1
    )
}

/// Writes the refusal, then drains briefly before closing: closing a socket with unread
/// input sends a reset, which can destroy the response before the client reads it.
async fn respond(stream: &mut TcpStream, status: (u16, &str), body: &str) {
    if write_all_timed(stream, response(status, body).as_bytes())
        .await
        .is_err()
    {
        return;
    }
    let _ = stream.shutdown().await;
    let mut sink = [0u8; 4096];
    let drain = async {
        let mut total = 0usize;
        while let Ok(n) = stream.read(&mut sink).await {
            if n == 0 || total > 64 * 1024 {
                break;
            }
            total += n;
        }
    };
    let _ = timeout(Duration::from_secs(1), drain).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_shape() {
        let body = "mantle-egress: example.com:443 is not an allowed destination";
        let r = response(FORBIDDEN, body);
        assert!(r.starts_with("HTTP/1.1 403 Forbidden\r\n"));
        assert!(r.contains(&format!("Content-Length: {}\r\n", body.len() + 1)));
        assert!(r.ends_with(&format!("\r\n\r\n{body}\n")));
        assert!(response(NOT_ALLOWED, "x").contains("Allow: CONNECT\r\n"));
    }
}
