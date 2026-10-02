//! In-sandbox launcher. `serve` runs the agent on a pseudo-terminal it owns, so the agent
//! outlives any terminal; `attach` relays a terminal to it through named pipes in the shared
//! session directory. No Unix-domain sockets are used: the sandbox refuses them.

use mantle_launch::{attach, cli, serve};

use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Command};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match &cli.command {
        Command::Serve(args) => serve::run(args),
        Command::Attach(args) => attach::run(args),
    };
    match result {
        Ok(code) => ExitCode::from(code),
        Err(err) => {
            eprintln!("mantle-launch: {err:#}");
            ExitCode::FAILURE
        }
    }
}
