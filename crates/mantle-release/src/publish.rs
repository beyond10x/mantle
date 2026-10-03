//! Operator-only bot publication. CI never receives publication credentials.
use anyhow::{Context, Result, ensure};
use serde_json::Value;
use std::{path::Path, process::Command, time::Duration};
fn bot(source: &Path, policy: &Path) -> Command {
    let mut command = Command::new("b10x-gates");
    command
        .args(["gh", "--repo"])
        .arg(source)
        .arg("--policy")
        .arg(policy)
        .args(["--repository", "beyond10x/mantle", "--"]);
    command
}
fn api(source: &Path, policy: &Path, path: &str) -> Result<(u16, Value)> {
    let output = mantle_worker::run_bounded(
        bot(source, policy).args(["api", "--include", "--method", "GET", path]),
        None,
        Duration::from_secs(120),
        4 * 1024 * 1024,
    )?;
    let bytes = String::from_utf8(output.stdout)?;
    let (headers, body) = bytes
        .split_once("\r\n\r\n")
        .or_else(|| bytes.split_once("\n\n"))
        .context("bot API response lacks status headers")?;
    let status = headers
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .context("bot API status")?
        .parse()?;
    let body = serde_json::from_str(body)?;
    ensure!(
        status >= 400 || output.status.success(),
        "bot API process failed despite a success response"
    );
    Ok((status, body))
}
pub fn publish(manifest: &Path, source: &Path, tag: &str, policy: &Path) -> Result<()> {
    let bundle = mantle_artifact::verify(manifest)?;
    ensure!(
        tag == bundle.manifest.version || tag == format!("v{}", bundle.manifest.version),
        "tag differs from version"
    );
    crate::build::source_identity(source, &bundle.manifest.source_commit)?;
    let local = crate::text(
        Command::new("git")
            .arg("-C")
            .arg(source)
            .args(["rev-parse", &format!("refs/tags/{tag}^{{commit}}")]),
    )?;
    ensure!(
        local == bundle.manifest.source_commit,
        "local tag differs from manifest source"
    );
    verify_remote(source, policy, tag, &bundle.manifest.source_commit)?;
    let endpoint = format!("repos/beyond10x/mantle/releases/tags/{tag}");
    let (status, _) = api(source, policy, &endpoint)?;
    ensure!(
        status == 404,
        "release exists or absence cannot be proven; refusing immutable publication"
    );
    crate::verify_versions(&bundle)?;
    let parent = manifest.parent().context("manifest parent")?;
    let sums = mantle_artifact::read_bounded(&parent.join("SHA256SUMS"), 4096)?;
    let expected_sums = bundle
        .manifest
        .artifacts
        .iter()
        .map(|a| format!("{}  {}\n", a.sha256, a.name))
        .collect::<String>()
        + &format!("{}  manifest.json\n", bundle.manifest_sha256);
    ensure!(
        sums == expected_sums.as_bytes(),
        "SHA256SUMS differs from verified bundle"
    );
    // Freeze all exact bytes in private storage before giving paths to gh: no verification/upload race.
    let stage = tempfile::tempdir()?;
    let mut assets = Vec::new();
    for artifact in &bundle.manifest.artifacts {
        let bytes = mantle_artifact::read_bounded(
            &parent.join(&artifact.name),
            mantle_artifact::MAX_ARCHIVE,
        )?;
        ensure!(
            mantle_artifact::sha256(&bytes) == artifact.sha256,
            "archive changed before publication"
        );
        let path = stage.path().join(&artifact.name);
        crate::write_new(&path, &bytes, 0o600)?;
        assets.push((path, artifact.sha256.clone()));
    }
    let bytes = mantle_artifact::read_bounded(manifest, mantle_artifact::MAX_MANIFEST)?;
    ensure!(
        mantle_artifact::sha256(&bytes) == bundle.manifest_sha256,
        "manifest changed before publication"
    );
    for (name, bytes) in [("manifest.json", bytes), ("SHA256SUMS", sums)] {
        let path = stage.path().join(name);
        crate::write_new(&path, &bytes, 0o600)?;
        assets.push((path, mantle_artifact::sha256(&bytes)));
    }
    let notes = stage.path().join("notes.md");
    crate::write_new(&notes,format!("Mantle {} from {}. Linux x86_64 GNU CLI ({}) and static musl workers. Verify manifest.json and SHA256SUMS before installation.\n",bundle.manifest.version,bundle.manifest.source_commit,bundle.manifest.gnu_runtime).as_bytes(),0o600)?;
    let mut create = bot(source, policy);
    create
        .args([
            "release",
            "create",
            tag,
            "--repo",
            "beyond10x/mantle",
            "--verify-tag",
            "--title",
        ])
        .arg(format!("Mantle {}", bundle.manifest.version))
        .arg("--notes-file")
        .arg(notes);
    for (path, _) in &assets {
        create.arg(path);
    }
    crate::command(&mut create,300,1024*1024).context("bot publication failed; a partial release may exist and must be inspected, never overwritten")?;
    let (status, release) = api(source, policy, &endpoint)?;
    ensure!(
        status == 200 && release["tag_name"] == tag,
        "published release cannot be verified"
    );
    let observed = release["assets"].as_array().context("published assets")?;
    ensure!(
        observed.len() == assets.len(),
        "published asset inventory differs"
    );
    for (path, digest) in assets {
        let name = path
            .file_name()
            .context("asset name")?
            .to_str()
            .context("asset UTF-8")?;
        ensure!(
            observed
                .iter()
                .filter(
                    |asset| asset["name"] == name && asset["digest"] == format!("sha256:{digest}")
                )
                .count()
                == 1,
            "published asset digest not verified"
        );
    }
    verify_remote(source, policy, tag, &bundle.manifest.source_commit)?;
    println!("published and verified {tag}");
    Ok(())
}

fn verify_remote(source: &Path, policy: &Path, tag: &str, expected: &str) -> Result<()> {
    let (status, reference) = api(
        source,
        policy,
        &format!("repos/beyond10x/mantle/git/ref/tags/{tag}"),
    )?;
    ensure!(status == 200, "remote tag cannot be verified");
    let mut object = reference["object"].clone();
    let mut depth = 0;
    while object["type"] == "tag" {
        ensure!(depth < 8, "remote annotated tag nesting exceeds bound");
        depth += 1;
        let sha = object["sha"].as_str().context("remote tag SHA")?;
        ensure!(
            sha.len() == 40 && sha.bytes().all(|b| b.is_ascii_hexdigit()),
            "invalid remote tag SHA"
        );
        let (status, annotation) = api(
            source,
            policy,
            &format!("repos/beyond10x/mantle/git/tags/{sha}"),
        )?;
        ensure!(status == 200, "remote tag annotation unavailable");
        object = annotation["object"].clone();
    }
    ensure!(
        object["type"] == "commit" && object["sha"] == expected,
        "remote tag differs from manifest source"
    );
    Ok(())
}
