//! Bounded static documentation build. No repository credentials or network access.
use std::{collections::BTreeSet, fs, path::PathBuf};

use anyhow::{Context, Result, ensure};
use clap::{Parser, Subcommand};
use serde_json::json;

const HTML: &str = include_str!("../../../website/index.html");
const CSS: &str = include_str!("../../../website/styles.css");

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Validate links and build the exact static site plus publication provenance.
    Build {
        #[arg(long)]
        out: PathBuf,
        #[arg(long)]
        commit: String,
    },
    /// Validate the authored documentation without writing files.
    Check,
}

fn attributes<'a>(html: &'a str, name: &str) -> Vec<&'a str> {
    let prefix = format!("{name}=\"");
    html.split(&prefix)
        .skip(1)
        .filter_map(|tail| tail.split('"').next())
        .collect()
}

fn check(html: &str) -> Result<()> {
    ensure!(
        html.starts_with("<!doctype html>"),
        "missing HTML document declaration"
    );
    ensure!(
        html.contains("lang=\"en\"") && html.contains("name=\"viewport\""),
        "missing language or mobile viewport"
    );
    let ids = attributes(html, "id");
    let unique: BTreeSet<_> = ids.iter().copied().collect();
    ensure!(ids.len() == unique.len(), "duplicate anchor");
    for href in attributes(html, "href") {
        if let Some(anchor) = href.strip_prefix('#') {
            ensure!(unique.contains(anchor), "missing anchor {anchor}");
        } else {
            ensure!(
                href.starts_with("https://") || href == "/mantle/styles.css",
                "unexpected route {href}"
            );
        }
    }
    ensure!(
        !html.contains("<script"),
        "the documentation must remain script-free"
    );
    ensure!(
        html.contains("https://beyond10x.github.io/mantle/"),
        "wrong canonical base"
    );
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    check(HTML)?;
    match cli.command {
        Action::Check => println!("Mantle documentation: anchors, routes and document valid"),
        Action::Build { out, commit } => {
            ensure!(
                commit.len() == 40
                    && commit.bytes().all(|b| b.is_ascii_hexdigit())
                    && commit != "0".repeat(40),
                "commit must be a nonzero full Git revision"
            );
            fs::create_dir_all(out.join(".well-known"))?;
            fs::write(out.join("index.html"), HTML)?;
            fs::write(out.join("styles.css"), CSS)?;
            fs::write(out.join(".nojekyll"), "")?;
            let manifest = json!({"schema":"b10x-project-site/v1", "repository":"mantle", "commit":commit, "baseUrl":"/mantle/"});
            fs::write(
                out.join(".well-known/b10x-site.json"),
                serde_json::to_vec_pretty(&manifest)?,
            )
            .context("writing site provenance")?;
            println!("Mantle documentation built at {}", out.display());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_links_resolve() {
        check(HTML).expect("valid site");
    }

    #[test]
    fn broken_anchors_and_wrong_asset_bases_are_refused() {
        assert!(check(&HTML.replace("id=\"quickstart\"", "id=\"missing\"")).is_err());
        assert!(check(&HTML.replace("/mantle/styles.css", "/styles.css")).is_err());
    }
}
