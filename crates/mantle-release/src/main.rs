use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
#[derive(Parser)]
#[command(
    version,
    about = "Build, verify and install source-bound Mantle release bundles"
)]
struct Args {
    #[command(subcommand)]
    command: Action,
}
#[derive(Subcommand)]
enum Action {
    Build {
        #[arg(long)]
        source: PathBuf,
        #[arg(long)]
        revision: String,
        #[arg(long)]
        output: PathBuf,
        #[arg(long)]
        work: PathBuf,
    },
    Verify {
        #[arg(long)]
        manifest: PathBuf,
    },
    Install {
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long)]
        prefix: PathBuf,
        #[arg(long,default_value=mantle_artifact::GNU)]
        target: String,
    },
    Publish {
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long)]
        source: PathBuf,
        #[arg(long)]
        tag: String,
        #[arg(long)]
        policy: PathBuf,
    },
    Notices {
        #[arg(long, default_value = ".")]
        source: PathBuf,
        #[arg(long)]
        check: bool,
    },
}
fn main() -> Result<()> {
    match Args::parse().command {
        Action::Build {
            source,
            revision,
            output,
            work,
        } => mantle_release::build::build(&source, &revision, &output, &work),
        Action::Verify { manifest } => {
            let bundle = mantle_artifact::verify(&manifest)?;
            mantle_release::verify_versions(&bundle)?;
            println!(
                "verified {} {}",
                bundle.manifest.version, bundle.manifest.source_commit
            );
            Ok(())
        }
        Action::Install {
            manifest,
            prefix,
            target,
        } => {
            let bundle = mantle_artifact::verify(&manifest)?;
            mantle_release::verify_versions(&bundle)?;
            let path = mantle_release::install(&bundle, &prefix, &target)?;
            println!("installed {}", path.display());
            Ok(())
        }
        Action::Publish {
            manifest,
            source,
            tag,
            policy,
        } => mantle_release::publish::publish(&manifest, &source, &tag, &policy),
        Action::Notices { source, check } => mantle_release::licenses::generate(&source, check),
    }
}
