use crate::{command, text, write_new};
use anyhow::{Context, Result, ensure};
use mantle_artifact::{Artifact, GNU, MUSL, Manifest, Payload, sha256};
use std::{collections::BTreeMap, fs, io::Cursor, path::Path, process::Command, time::Duration};
fn git(source: &Path, args: &[&str]) -> Result<String> {
    text(Command::new("git").arg("-C").arg(source).args(args))
}
pub fn source_identity(source: &Path, revision: &str) -> Result<String> {
    ensure!(
        revision.len() == 40 && revision.bytes().all(|b| b.is_ascii_hexdigit()),
        "exact 40-character source revision required"
    );
    let head = git(source, &["rev-parse", "HEAD"])?;
    ensure!(
        head == revision,
        "requested source revision differs from checkout"
    );
    ensure!(
        git(
            source,
            &["status", "--porcelain=v1", "--untracked-files=all"]
        )?
        .is_empty(),
        "dirty source checkout"
    );
    let flags = git(source, &["ls-files", "-v"])?;
    ensure!(
        flags.lines().all(|line| line.starts_with("H ")),
        "hidden or unsupported source index state"
    );
    Ok(head)
}
pub fn archive(
    files: &BTreeMap<String, Vec<u8>>,
    target: &str,
    version: &str,
) -> Result<(Artifact, Vec<u8>)> {
    let inventory = mantle_artifact::inventory(target)?;
    ensure!(
        files.keys().eq(inventory.keys()),
        "package inventory mismatch"
    );
    let mut builder = tar::Builder::new(Vec::new());
    let mut payloads = Vec::new();
    for (path, bytes) in files {
        let mode = inventory[path];
        let mut header = tar::Header::new_ustar();
        header.set_size(bytes.len() as u64);
        header.set_mode(mode);
        header.set_uid(0);
        header.set_gid(0);
        header.set_mtime(0);
        header.set_cksum();
        builder.append_data(&mut header, path, bytes.as_slice())?;
        payloads.push(Payload {
            path: path.clone(),
            sha256: sha256(bytes),
            size_bytes: bytes.len() as u64,
            mode,
        });
    }
    let bytes = builder.into_inner()?;
    Ok((
        Artifact {
            name: format!("mantle-{version}-{target}.tar"),
            target: target.into(),
            sha256: sha256(&bytes),
            size_bytes: bytes.len() as u64,
            payloads,
        },
        bytes,
    ))
}
pub fn export_source(source: &Path, revision: &str, export: &Path) -> Result<()> {
    let source_commit = source_identity(source, revision)?;
    fs::create_dir(export)?;
    let bytes = command(
        Command::new("git")
            .arg("-C")
            .arg(source)
            .args(["archive", "--format=tar", &source_commit]),
        30,
        64 * 1024 * 1024,
    )?;
    let mut source_tar = tar::Archive::new(Cursor::new(bytes));
    let mut provenance = false;
    for entry in source_tar.entries()? {
        let mut entry = entry?;
        if entry.header().entry_type().is_pax_global_extensions() {
            ensure!(
                !provenance && entry.size() <= 4096,
                "invalid Git archive provenance header"
            );
            let extensions = entry.pax_extensions()?.context("Git archive provenance")?;
            let mut count = 0;
            for extension in extensions {
                let extension = extension?;
                ensure!(
                    extension.key()? == "comment" && extension.value()? == source_commit,
                    "Git archive source identity mismatch"
                );
                count += 1;
            }
            ensure!(count == 1, "Git archive must carry one source identity");
            provenance = true;
            continue;
        }
        ensure!(provenance, "Git archive has no source identity");
        ensure!(
            entry.header().entry_type().is_dir() || entry.header().entry_type().is_file(),
            "source export contains unsupported links"
        );
        ensure!(entry.unpack_in(export)?, "source export escaped root");
    }
    ensure!(provenance, "Git archive has no source identity");
    Ok(())
}
pub fn build(source: &Path, revision: &str, output: &Path, work: &Path) -> Result<()> {
    let source = source.canonicalize()?;
    let source_commit = source_identity(&source, revision)?;
    ensure!(
        !output.exists(),
        "output already exists; immutable bundles cannot be overwritten"
    );
    ensure!(
        work.is_absolute() && !work.exists(),
        "build work path must be a new absolute directory"
    );
    fs::create_dir(work)?;
    let export = work.join("source");
    let target = work.join("target");
    export_source(&source, revision, &export)?;
    let workspace: toml::Value = toml::from_str(&fs::read_to_string(export.join("Cargo.toml"))?)?;
    let version = workspace["workspace"]["package"]["version"]
        .as_str()
        .context("workspace version")?
        .to_owned();
    let app: toml::Value = toml::from_str(&fs::read_to_string(
        export.join("crates/mantle/Cargo.toml"),
    )?)?;
    let substrate_revision = app["dependencies"]["b10x-substrate-sdk"]["rev"]
        .as_str()
        .context("Substrate revision")?
        .to_owned();
    let worker = fs::read_to_string(export.join("crates/mantle/src/app/worker.rs"))?;
    let substrate_version = worker
        .lines()
        .find_map(|l| {
            l.strip_prefix("pub const SUBSTRATE_VERSION: &str = \"")
                .and_then(|l| l.strip_suffix("\";"))
        })
        .context("Substrate version")?
        .to_owned();
    let rustc = text(Command::new("rustc").arg("-Vv"))?;
    ensure!(
        rustc.lines().any(|l| l == format!("host: {GNU}")),
        "release build requires Linux x86_64 GNU host"
    );
    let gnu_runtime = text(Command::new("getconf").arg("GNU_LIBC_VERSION"))?;
    let (rust_notice, musl_notice) = crate::licenses::runtime_notices()?;
    // Notice generation covers the locked workspace graph, including packages that are not linked
    // into a particular binary. Fetch it explicitly so a fresh CI machine can check notices offline.
    let fetched = command(
        Command::new("cargo")
            .current_dir(&export)
            .args(["fetch", "--locked"]),
        600,
        8 * 1024 * 1024,
    )?;
    fs::write(work.join("fetch.log"), fetched)?;
    // A private exact export and fresh target exclude caller-supplied stale same-version binaries.
    for (triple, packages) in [
        (GNU, &["mantle", "mantle-release"][..]),
        (
            MUSL,
            &["mantle-worker", "mantle-launch", "mantle-egress"][..],
        ),
    ] {
        let mut cargo = Command::new("cargo");
        cargo
            .current_dir(&export)
            .args(["build", "--locked", "--release", "--target", triple]);
        for package in packages {
            cargo.args(["-p", package]);
        }
        cargo
            .env("CARGO_TARGET_DIR", &target)
            .env("SOURCE_DATE_EPOCH", "0");
        let result = mantle_worker::run_bounded(
            &mut cargo,
            None,
            Duration::from_secs(3600),
            16 * 1024 * 1024,
        )?;
        let mut log = result.stdout;
        log.extend_from_slice(&result.stderr);
        fs::write(work.join(format!("build-{triple}.log")), &log)?;
        ensure!(
            result.status.success(),
            "target build failed; see retained build log"
        );
    }
    crate::licenses::generate(&export, true)?;
    let mut manifest = Manifest {
        format: "mantle-release/1".into(),
        version: version.clone(),
        source_commit,
        substrate_revision,
        substrate_version,
        rustc,
        gnu_runtime,
        musl_runtime: "1.2.5".into(),
        artifacts: Vec::new(),
    };
    let parent = output.parent().context("output parent")?;
    let stage = tempfile::Builder::new()
        .prefix(".mantle-bundle-")
        .tempdir_in(parent)?;
    let mut sums = String::new();
    for triple in [GNU, MUSL] {
        let mut files = BTreeMap::new();
        for path in mantle_artifact::inventory(triple)?.keys() {
            let bytes = match path.as_str() {
                "RUST_RUNTIME_LICENSE.html" => rust_notice.clone(),
                "MUSL_COPYRIGHT" => musl_notice.clone(),
                "LICENSE" | "THIRD_PARTY_LICENSES.html" => fs::read(export.join(path))?,
                _ => {
                    let name = Path::new(path).file_name().context("binary path")?;
                    fs::read(target.join(triple).join("release").join(name))?
                }
            };
            files.insert(path.clone(), bytes);
        }
        let (artifact, bytes) = archive(&files, triple, &version)?;
        write_new(&stage.path().join(&artifact.name), &bytes, 0o644)?;
        sums.push_str(&format!("{}  {}\n", artifact.sha256, artifact.name));
        manifest.artifacts.push(artifact);
    }
    let bytes = serde_json::to_vec_pretty(&manifest)?;
    write_new(&stage.path().join("manifest.json"), &bytes, 0o644)?;
    sums.push_str(&format!("{}  manifest.json\n", sha256(&bytes)));
    write_new(&stage.path().join("SHA256SUMS"), sums.as_bytes(), 0o644)?;
    let verified = mantle_artifact::verify(&stage.path().join("manifest.json"))?;
    crate::verify_versions(&verified)?;
    rustix::fs::renameat_with(
        rustix::fs::CWD,
        stage.path(),
        rustix::fs::CWD,
        output,
        rustix::fs::RenameFlags::NOREPLACE,
    )?;
    println!(
        "built and verified {} from {} ({}); source and compiler logs retained at {}",
        version,
        revision,
        manifest.gnu_runtime,
        work.display()
    );
    Ok(())
}
