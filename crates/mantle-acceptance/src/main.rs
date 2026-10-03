use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
#[derive(Parser)]
#[command(
    version,
    about = "Bounded, metadata-only acceptance through an installed Mantle CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Action,
}
#[derive(Clone, ValueEnum)]
enum Agent {
    Codex,
    ClaudeCode,
}
#[derive(Clone, ValueEnum)]
enum Attestation {
    Login,
    ModelTool,
    Visual,
}
#[derive(Subcommand)]
enum Action {
    /// Create a disposable session and print an operator login/model handoff.
    Run {
        #[arg(long, required = true)]
        real: bool,
        #[arg(long)]
        mantle: PathBuf,
        #[arg(long)]
        profile: String,
        #[arg(long, value_enum)]
        agent: Agent,
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long)]
        checkpoint: PathBuf,
        #[arg(long)]
        release_manifest: Option<PathBuf>,
    },
    /// Record generation-bound operator observations and run bounded machine checks.
    Resume {
        #[arg(long, required = true)]
        real: bool,
        #[arg(long)]
        checkpoint: PathBuf,
        #[arg(long)]
        expected_run_id: String,
        #[arg(long)]
        expected_exec_id: String,
        #[arg(long, value_enum)]
        attest: Vec<Attestation>,
    },
    /// Read the private checkpoint; does not contact a worker.
    Report {
        #[arg(long)]
        checkpoint: PathBuf,
    },
    /// Explicitly destroy only the session proven by this run's creation receipt.
    Cleanup {
        #[arg(long, required = true)]
        real: bool,
        #[arg(long)]
        checkpoint: PathBuf,
    },
}
fn main() -> std::process::ExitCode {
    let result = (|| -> anyhow::Result<_> {
        mantle_worker::initialize_transport_signals()?;
        match Cli::parse().command {
            Action::Run {
                mantle,
                profile,
                agent,
                manifest,
                checkpoint,
                release_manifest,
                ..
            } => mantle_acceptance::runner::run(
                &mantle,
                &profile,
                match agent {
                    Agent::Codex => "codex",
                    Agent::ClaudeCode => "claude-code",
                },
                &manifest,
                &checkpoint,
                release_manifest.as_deref(),
            ),
            Action::Resume {
                checkpoint,
                expected_run_id,
                expected_exec_id,
                attest,
                ..
            } => mantle_acceptance::runner::resume(
                &checkpoint,
                &expected_run_id,
                &expected_exec_id,
                &attest
                    .iter()
                    .map(|a| match a {
                        Attestation::Login => "login",
                        Attestation::ModelTool => "model-tool",
                        Attestation::Visual => "visual",
                    })
                    .collect::<Vec<_>>(),
            ),
            Action::Report { checkpoint } => mantle_acceptance::runner::report(&checkpoint),
            Action::Cleanup { checkpoint, .. } => mantle_acceptance::runner::cleanup(&checkpoint),
        }
    })();
    match result {
        Ok(code) => std::process::ExitCode::from(code),
        Err(_) => {
            eprintln!(
                "Acceptance refused or failed; inspect the metadata checkpoint. No raw diagnostic output is retained."
            );
            std::process::ExitCode::FAILURE
        }
    }
}
