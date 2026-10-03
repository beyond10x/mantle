use anyhow::{Result, ensure};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about = "Inspect/install pinned agents and explicitly upgrade offline Mantle helpers"
)]
struct Cli {
    #[command(subcommand)]
    command: Operation,
}
#[derive(Subcommand)]
enum Operation {
    /// Install the pinned Codex candidate without restarting any service.
    InstallCodex,
    /// Verify the currently published Codex generation and report its identity.
    InspectCodex,
    /// Read-only assessment; no service action or installation occurs.
    UpgradeCheck {
        #[arg(long)]
        manifest: PathBuf,
    },
    /// Apply or recover a verified offline bundle under the host installation lock.
    UpgradeApply {
        #[arg(long)]
        manifest: PathBuf,
    },
    /// Reconcile fresh/legacy helpers under the same lock; managed bundles refuse.
    ReconcileHelpers {
        #[arg(long)]
        source: PathBuf,
    },
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    mantle_worker::initialize_transport_signals()?;
    let host = mantle_worker::maintenance::LocalHost { root: "/".into() };
    let root = std::path::Path::new("/opt/mantle");
    let value = match cli.command {
        Operation::InstallCodex => {
            serde_json::to_value(mantle_worker::Installer::production()?.install()?)?
        }
        Operation::InspectCodex => {
            serde_json::to_value(mantle_worker::Installer::production()?.inspect()?)?
        }
        Operation::UpgradeCheck { manifest } => {
            serde_json::to_value(mantle_worker::upgrade::check(&host, root, &manifest)?)?
        }
        Operation::UpgradeApply { manifest } => {
            ensure!(
                nix::unistd::Uid::effective().is_root(),
                "offline upgrade requires root"
            );
            serde_json::to_value(mantle_worker::upgrade::apply(&host, root, &manifest, &())?)?
        }
        Operation::ReconcileHelpers { source } => {
            ensure!(
                nix::unistd::Uid::effective().is_root(),
                "helper reconciliation requires root"
            );
            serde_json::to_value(mantle_worker::upgrade::reconcile_helpers(
                &host,
                root,
                &source,
                &(),
            )?)?
        }
    };
    println!("{}", serde_json::to_string(&value)?);
    Ok(())
}
