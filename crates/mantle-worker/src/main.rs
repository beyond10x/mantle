use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Verify and install Mantle's pinned worker agent")]
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    mantle_worker::initialize_transport_signals()?;
    let installer = mantle_worker::Installer::production()?;
    let value = match cli.command {
        Operation::InstallCodex => serde_json::to_value(installer.install()?)?,
        Operation::InspectCodex => serde_json::to_value(installer.inspect()?)?,
    };
    println!("{}", serde_json::to_string(&value)?);
    Ok(())
}
