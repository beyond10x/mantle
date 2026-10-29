//! Bounded static documentation build. No repository credentials or network access.
//!
//! Every page is a fragment under `website/pages/`, wrapped in `website/layout.html`. The build
//! refuses a page whose anchors, routes or document shape are wrong, and a link from `README.md`
//! or `CHANGELOG.md` to a page or anchor the site does not have.
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
};

use anyhow::{Context, Result, bail, ensure};
use clap::{Parser, Subcommand};
use serde_json::json;

const LAYOUT: &str = include_str!("../../../website/layout.html");
const CSS: &str = include_str!("../../../website/styles.css");
const README: &str = include_str!("../../../README.md");
const CHANGELOG: &str = include_str!("../../../CHANGELOG.md");
const BASE: &str = "/mantle/";
const ORIGIN: &str = "https://beyond10x.github.io";

struct Page {
    slug: &'static str,
    nav: &'static str,
    title: &'static str,
    description: &'static str,
    body: &'static str,
}

/// The public pages, in navigation order. The empty slug is the start page.
const PAGES: &[Page] = &[
    Page {
        slug: "",
        nav: "Overview",
        title: "Mantle",
        description: "Mantle runs a Claude Code or Codex development session on a remote Linux worker, confined by Substrate. The laptop renders the terminal.",
        body: include_str!("../../../website/pages/index.html"),
    },
    Page {
        slug: "architecture",
        nav: "Architecture",
        title: "Architecture",
        description: "The laptop, Mantle, Substrate and the worker: what each layer owns and how a session crosses them.",
        body: include_str!("../../../website/pages/architecture.html"),
    },
    Page {
        slug: "configuration",
        nav: "Configuration",
        title: "Configuration and manifests",
        description: "The laptop configuration file and the session manifest: every field Mantle serves, its default and its bound.",
        body: include_str!("../../../website/pages/configuration.html"),
    },
    Page {
        slug: "operations",
        nav: "Operations",
        title: "Operations",
        description: "Session commands, retained workspaces, named profiles, doctor, automation metadata, acceptance runs and offline worker upgrades.",
        body: include_str!("../../../website/pages/operations.html"),
    },
    Page {
        slug: "releases",
        nav: "Releases",
        title: "Releases and verification",
        description: "Verifying and installing a Mantle release bundle, the conformance suite, and the maintainer build and publication procedure.",
        body: include_str!("../../../website/pages/releases.html"),
    },
    Page {
        slug: "security",
        nav: "Security",
        title: "Security model",
        description: "What a Mantle session can reach, where credentials go, what is recorded, and what the confinement does not protect against.",
        body: include_str!("../../../website/pages/security.html"),
    },
    Page {
        slug: "history",
        nav: "History",
        title: "Release history",
        description: "What each Mantle release changed, its compatibility notes and the evidence recorded for it.",
        body: include_str!("../../../website/pages/history.html"),
    },
];

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
    let prefix = format!(" {name}=\"");
    html.split(&prefix)
        .skip(1)
        .filter_map(|tail| tail.split('"').next())
        .collect()
}

fn route(slug: &str) -> String {
    if slug.is_empty() {
        BASE.to_owned()
    } else {
        format!("{BASE}{slug}/")
    }
}

fn render(page: &Page) -> String {
    let nav: String = PAGES
        .iter()
        .map(|other| {
            let current = if other.slug == page.slug {
                " aria-current=\"page\""
            } else {
                ""
            };
            format!(
                "<a href=\"{}\"{current}>{}</a>",
                route(other.slug),
                other.nav
            )
        })
        .collect();
    let title = if page.slug.is_empty() {
        "Mantle — remote, confined development sessions".to_owned()
    } else {
        format!("{} — Mantle", page.title)
    };
    LAYOUT
        .replace("{{title}}", &title)
        .replace("{{description}}", page.description)
        .replace("{{canonical}}", &format!("{ORIGIN}{}", route(page.slug)))
        .replace("{{nav}}", &nav)
        .replace("{{body}}", page.body)
}

/// The anchors of every rendered page, keyed by route.
fn anchors(rendered: &[(String, String)]) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let mut all = BTreeMap::new();
    for (path, html) in rendered {
        let ids = attributes(html, "id");
        let unique: BTreeSet<String> = ids.iter().map(|id| (*id).to_owned()).collect();
        ensure!(ids.len() == unique.len(), "{path}: duplicate anchor");
        all.insert(path.clone(), unique);
    }
    Ok(all)
}

/// Resolves a site-internal link from `page` and refuses one the site does not have.
fn resolve(page: &str, href: &str, anchors: &BTreeMap<String, BTreeSet<String>>) -> Result<()> {
    let (path, fragment) = match href.split_once('#') {
        Some((path, fragment)) => (path, Some(fragment)),
        None => (href, None),
    };
    let target = if path.is_empty() { page } else { path };
    let Some(ids) = anchors.get(target) else {
        bail!("{page}: link to missing page {href}");
    };
    if let Some(fragment) = fragment {
        ensure!(
            ids.contains(fragment),
            "{page}: link to missing anchor {href}"
        );
    }
    Ok(())
}

fn check_site(rendered: &[(String, String)]) -> Result<()> {
    let anchors = anchors(rendered)?;
    for (path, html) in rendered {
        ensure!(
            html.starts_with("<!doctype html>"),
            "{path}: missing HTML document declaration"
        );
        ensure!(
            html.contains("lang=\"en\"") && html.contains("name=\"viewport\""),
            "{path}: missing language or mobile viewport"
        );
        ensure!(
            !html.contains("{{"),
            "{path}: unreplaced layout placeholder"
        );
        ensure!(
            !html.contains("<script"),
            "{path}: the documentation must remain script-free"
        );
        ensure!(
            html.contains(&format!("<link rel=\"canonical\" href=\"{ORIGIN}{path}\">")),
            "{path}: wrong canonical address"
        );
        ensure!(
            html.matches("<h1").count() == 1,
            "{path}: a page has exactly one h1"
        );
        for href in attributes(html, "href") {
            if href.starts_with("https://") || href == "/mantle/styles.css" {
                continue;
            }
            ensure!(
                href.starts_with('#') || href.starts_with(BASE),
                "{path}: unexpected route {href}"
            );
            resolve(path, href, &anchors)?;
        }
    }
    Ok(())
}

/// Markdown link targets: the text between `](` and the next `)`.
fn markdown_links(text: &str) -> Vec<&str> {
    text.split("](")
        .skip(1)
        .filter_map(|tail| tail.split(')').next())
        .collect()
}

/// A link from repository Markdown into the published site must name a page and anchor it has.
fn check_markdown(
    name: &str,
    text: &str,
    anchors: &BTreeMap<String, BTreeSet<String>>,
) -> Result<()> {
    let site = format!("{ORIGIN}{BASE}");
    for href in markdown_links(text) {
        if let Some(rest) = href.strip_prefix(&site) {
            resolve(name, &format!("{BASE}{rest}"), anchors)
                .with_context(|| format!("{name} links to {href}"))?;
        }
    }
    Ok(())
}

fn site() -> Result<Vec<(String, String)>> {
    let rendered: Vec<(String, String)> = PAGES
        .iter()
        .map(|page| (route(page.slug), render(page)))
        .collect();
    check_site(&rendered)?;
    let anchors = anchors(&rendered)?;
    check_markdown("README.md", README, &anchors)?;
    check_markdown("CHANGELOG.md", CHANGELOG, &anchors)?;
    Ok(rendered)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let rendered = site()?;
    match cli.command {
        Action::Check => println!(
            "Mantle documentation: {} pages; anchors, routes and Markdown site links valid",
            rendered.len()
        ),
        Action::Build { out, commit } => {
            ensure!(
                commit.len() == 40
                    && commit.bytes().all(|b| b.is_ascii_hexdigit())
                    && commit != "0".repeat(40),
                "commit must be a nonzero full Git revision"
            );
            fs::create_dir_all(out.join(".well-known"))?;
            for (path, html) in &rendered {
                let directory = out.join(path.trim_start_matches(BASE));
                fs::create_dir_all(&directory)?;
                fs::write(directory.join("index.html"), html)
                    .with_context(|| format!("writing {path}"))?;
            }
            fs::write(out.join("styles.css"), CSS)?;
            fs::write(out.join(".nojekyll"), "")?;
            let manifest = json!({"schema":"b10x-project-site/v1", "repository":"mantle", "commit":commit, "baseUrl":"/mantle/"});
            fs::write(
                out.join(".well-known/b10x-site.json"),
                serde_json::to_vec_pretty(&manifest)?,
            )
            .context("writing site provenance")?;
            println!(
                "Mantle documentation built at {}: {} pages",
                out.display(),
                rendered.len()
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rendered() -> Vec<(String, String)> {
        PAGES
            .iter()
            .map(|page| (route(page.slug), render(page)))
            .collect()
    }

    fn with(path: &str, from: &str, to: &str) -> Vec<(String, String)> {
        rendered()
            .into_iter()
            .map(|(route, html)| {
                let html = if route == path {
                    html.replacen(from, to, 1)
                } else {
                    html
                };
                (route, html)
            })
            .collect()
    }

    #[test]
    fn authored_links_resolve() {
        site().expect("valid site");
    }

    #[test]
    fn every_page_has_its_own_route() {
        let routes: BTreeSet<String> = PAGES.iter().map(|page| route(page.slug)).collect();
        assert_eq!(routes.len(), PAGES.len());
    }

    #[test]
    fn broken_anchors_routes_and_asset_bases_are_refused() {
        assert!(check_site(&with("/mantle/", "id=\"install\"", "id=\"missing\"")).is_err());
        assert!(
            check_site(&with(
                "/mantle/",
                "href=\"/mantle/architecture/\"",
                "href=\"/mantle/absent/\""
            ))
            .is_err()
        );
        assert!(
            check_site(&with(
                "/mantle/operations/",
                "id=\"offline-upgrades\"",
                "id=\"elsewhere\""
            ))
            .is_err()
        );
        assert!(check_site(&with("/mantle/", "/mantle/styles.css", "/styles.css")).is_err());
        assert!(check_site(&with("/mantle/", "</main>", "<script></script></main>")).is_err());
    }

    #[test]
    fn markdown_links_into_the_site_are_checked() {
        let anchors = anchors(&rendered()).expect("anchors");
        assert!(
            check_markdown(
                "x.md",
                "[a](https://beyond10x.github.io/mantle/operations/#doctor)",
                &anchors
            )
            .is_ok()
        );
        assert!(
            check_markdown(
                "x.md",
                "[a](https://beyond10x.github.io/mantle/operations/#absent)",
                &anchors
            )
            .is_err()
        );
        assert!(
            check_markdown(
                "x.md",
                "[a](https://beyond10x.github.io/mantle/absent/)",
                &anchors
            )
            .is_err()
        );
    }

    /// Relative Markdown links, and links into this repository's `main` tree from either the
    /// Markdown or the site, must name a file that exists in the checkout.
    #[test]
    fn repository_file_links_exist() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut links: Vec<(String, String)> = Vec::new();
        for (name, text) in [("README.md", README), ("CHANGELOG.md", CHANGELOG)] {
            for href in markdown_links(text) {
                links.push((name.to_owned(), href.to_owned()));
            }
        }
        for (path, html) in rendered() {
            for href in attributes(&html, "href") {
                links.push((path.clone(), href.to_owned()));
            }
        }
        let mut checked = 0;
        for (name, href) in links {
            let target = if let Some(rest) = ["blob", "tree"].iter().find_map(|kind| {
                href.strip_prefix(&format!("https://github.com/beyond10x/mantle/{kind}/main/"))
            }) {
                rest.to_owned()
            } else if href.contains("://") || href.starts_with('#') || href.starts_with('/') {
                continue;
            } else {
                href.clone()
            };
            let file = target.split('#').next().unwrap_or(&target);
            assert!(root.join(file).exists(), "{name} links to missing {href}");
            checked += 1;
        }
        assert!(checked > 0, "no repository links were checked");
    }
}
