mod adapters {
    pub mod aws;
    pub mod kubevirt;
    pub mod ssh;
    pub mod state;
    pub mod substrate;
}
mod app {
    pub mod session;
    pub mod terminal;
    pub mod worker;
}
mod config;
mod domain {
    pub mod manifest;
    pub mod session;
}

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::adapters::state::Store;
use crate::config::{Config, state_dir};

/// Portable cloud development sessions on Substrate.
#[derive(Debug, Parser)]
#[command(
    name = "mantle",
    version,
    after_help = "MANTLE_CONFIG selects an absolute configuration file; MANTLE_STATE_DIR selects an absolute private state directory. Defaults: ~/.config/mantle/config.toml and ~/.local/state/mantle. READY denotes common confinement/toolchain readiness, not agent authentication."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Manage the EC2 worker.
    Worker {
        #[command(subcommand)]
        command: WorkerCommand,
    },
    /// Start a session from a manifest and attach to it.
    Start {
        /// The session manifest.
        #[arg(default_value = "mantle.yaml")]
        manifest: String,
        /// Start the agent without attaching the terminal.
        #[arg(long)]
        detached: bool,
    },
    /// Attach the terminal to a running session (Ctrl-] d detaches).
    Attach { name: String },
    /// List live sessions as recorded locally.
    List,
    /// Show a session: recorded intent, then what Substrate observes now.
    Status { name: String },
    /// Stop the agent and destroy the session's workspace.
    Stop { name: String },
    /// Run one command in a session's workspace under the agent's confinement, without its credential.
    Exec {
        name: String,
        #[arg(last = true, required = true)]
        argv: Vec<String>,
    },
}

#[derive(Debug, Subcommand)]
enum WorkerCommand {
    /// Create or start the worker, install Mantle's binaries, and check Substrate's facts.
    Up {
        /// Directory holding static `mantle-egress`, `mantle-launch`, and `mantle-worker` binaries.
        #[arg(long, default_value_os_t = default_worker_binaries())]
        binaries: PathBuf,
        /// Do not stop the worker after two idle hours.
        #[arg(long)]
        no_idle_stop: bool,
    },
    /// Stop the worker. The data volume and its workspaces are kept.
    Down,
    /// Open a shell on the worker host (outside any sandbox), through the provider's tunnel.
    Ssh,
    /// Show the worker and its Substrate facts.
    Status,
}

fn default_worker_binaries() -> PathBuf {
    let target = std::env::var_os("CARGO_TARGET_DIR").map_or_else(
        || {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_default()
                .join(".cache/b10x-target/mantle")
        },
        PathBuf::from,
    );
    target.join("x86_64-unknown-linux-musl/release")
}

fn main() -> Result<std::process::ExitCode> {
    mantle_worker::initialize_transport_signals()?;
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(run())
}

async fn run() -> Result<std::process::ExitCode> {
    let cli = Cli::parse();
    let config = Config::load()?;
    let store = Store::open(&state_dir()?.join("state.db"))?;
    match cli.command {
        Command::Worker { command } => match command {
            WorkerCommand::Up {
                binaries,
                no_idle_stop,
            } => {
                app::worker::up(
                    &config,
                    &store,
                    &app::worker::UpOptions {
                        worker_binaries: &binaries,
                        idle_stop: !no_idle_stop,
                    },
                )
                .await
            }
            WorkerCommand::Down => app::worker::down(&config).await,
            WorkerCommand::Status => app::worker::status(&config, &store).await,
            WorkerCommand::Ssh => app::worker::ssh(&config, &store),
        },
        Command::Start { manifest, detached } => {
            app::session::start(&config, &store, &manifest, !detached).await
        }
        Command::Attach { name } => app::session::attach(&config, &store, &name).await,
        Command::List => app::session::list(&store),
        Command::Status { name } => app::session::status(&config, &store, &name).await,
        Command::Stop { name } => app::session::stop(&config, &store, &name).await,
        Command::Exec { name, argv } => {
            return app::session::exec(&config, &store, &name, &argv)
                .await
                .map(std::process::ExitCode::from);
        }
    }?;
    Ok(std::process::ExitCode::SUCCESS)
}
