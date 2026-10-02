use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use clap::Parser;
use mantle_egress::allow::parse_allow_entry;
use mantle_egress::{Allowlist, Config, Destination, log, serve};
use tokio::net::TcpListener;
use tokio::signal::unix::{SignalKind, signal};

const HEAD_TIMEOUT: Duration = Duration::from_secs(10);
const DRAIN_TIMEOUT: Duration = Duration::from_secs(10);

/// Loopback CONNECT proxy admitting only allowlisted destinations.
#[derive(Debug, Parser)]
#[command(name = "mantle-egress", version)]
struct Cli {
    /// Address to listen on.
    #[arg(long, default_value = "127.0.0.1:3128")]
    listen: SocketAddr,

    /// Allowed destination as host:port; repeatable. Replaces the default list.
    #[arg(long = "allow", value_name = "HOST:PORT", value_parser = parse_allow_entry)]
    allow: Vec<Destination>,

    /// Concurrent tunnels, counted once a request head is parsed and allowed; further
    /// requests get 503. Connections still sending their head have a separate budget.
    #[arg(long, default_value_t = 256, value_parser = clap::value_parser!(u32).range(1..=65_535))]
    max_connections: u32,

    /// Upper bound for name resolution and for connecting upstream, each.
    #[arg(long, default_value_t = 10, value_parser = clap::value_parser!(u64).range(1..=300))]
    connect_timeout_secs: u64,

    /// Tunnels with no traffic in either direction for this long are closed.
    #[arg(long, default_value_t = 600, value_parser = clap::value_parser!(u64).range(1..=86_400))]
    idle_timeout_secs: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let allowlist = if cli.allow.is_empty() {
        Allowlist::default_list()
    } else {
        Allowlist::new(cli.allow)
    };
    let listener = TcpListener::bind(cli.listen)
        .await
        .with_context(|| format!("binding {}", cli.listen))?;
    let allowed: Vec<String> = allowlist
        .entries()
        .iter()
        .map(ToString::to_string)
        .collect();
    log::notice(&format!(
        "listening on {} allow={}",
        listener.local_addr()?,
        allowed.join(",")
    ));

    let config = Arc::new(Config {
        allowlist,
        max_connections: cli.max_connections,
        connect_timeout: Duration::from_secs(cli.connect_timeout_secs),
        idle_timeout: Duration::from_secs(cli.idle_timeout_secs),
        head_timeout: HEAD_TIMEOUT,
        drain_timeout: DRAIN_TIMEOUT,
    });

    let mut term = signal(SignalKind::terminate()).context("installing SIGTERM handler")?;
    let mut int = signal(SignalKind::interrupt()).context("installing SIGINT handler")?;
    let shutdown = async move {
        tokio::select! {
            _ = term.recv() => {}
            _ = int.recv() => {}
        }
    };
    serve(listener, config, shutdown).await?;
    Ok(())
}
