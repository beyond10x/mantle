use crate::{command, text, write_new};
use anyhow::{Context, Result, ensure};
use mantle_artifact::{Artifact, GNU, MUSL, Manifest, Payload, sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{Cursor, Read},
    path::{Component, Path, PathBuf},
    process::Command,
    time::Duration,
};
fn git(source: &Path, args: &[&str]) -> Result<String> {
    text(
        Command::new("git")
            .arg("--no-replace-objects")
            .arg("-C")
            .arg(source)
            .args(args),
    )
}

struct SourceTree {
    entries: BTreeMap<PathBuf, (String, String)>,
    blobs: BTreeMap<String, Vec<u8>>,
}

// Git archive attributes are not source identity: export-subst can rewrite bytes and
// export-ignore can omit files without dirtying the checkout. Read the raw committed
// tree and blobs, without replacement refs, as the independent export oracle.
fn source_tree(source: &Path, revision: &str) -> Result<SourceTree> {
    let tree = command(
        Command::new("git")
            .arg("--no-replace-objects")
            .arg("-C")
            .arg(source)
            .args(["ls-tree", "--full-tree", "-r", "-t", "-z", revision]),
        30,
        4 * 1024 * 1024,
    )?;
    let mut entries = BTreeMap::new();
    let mut requested = BTreeSet::new();
    for entry in tree.split(|b| *b == 0).filter(|entry| !entry.is_empty()) {
        let tab = entry
            .iter()
            .position(|b| *b == b'\t')
            .context("invalid Git tree entry")?;
        let metadata: Vec<_> = std::str::from_utf8(&entry[..tab])?.split(' ').collect();
        ensure!(metadata.len() == 3, "invalid Git tree metadata");
        let (mode, kind, oid) = (metadata[0], metadata[1], metadata[2]);
        ensure!(
            matches!(
                (mode, kind),
                ("040000", "tree") | ("100644" | "100755", "blob")
            ),
            "source export contains unsupported links or objects"
        );
        ensure!(
            oid.len() == 40 && oid.bytes().all(|b| b.is_ascii_hexdigit()),
            "invalid Git object identity"
        );
        use std::os::unix::ffi::OsStrExt;
        let path = PathBuf::from(std::ffi::OsStr::from_bytes(&entry[tab + 1..]));
        ensure!(
            !path.as_os_str().is_empty()
                && path.components().all(|c| matches!(c, Component::Normal(_))),
            "unsafe Git source path"
        );
        ensure!(
            entries
                .insert(path, (mode.to_owned(), oid.to_owned()))
                .is_none(),
            "duplicate Git source path"
        );
        if kind == "blob" {
            requested.insert(oid.to_owned());
        }
    }
    let input = requested
        .iter()
        .map(|oid| format!("{oid}\n"))
        .collect::<String>();
    let output = mantle_worker::run_bounded(
        Command::new("git")
            .arg("--no-replace-objects")
            .arg("-C")
            .arg(source)
            .args(["cat-file", "--batch"]),
        Some(input.as_bytes()),
        Duration::from_secs(30),
        64 * 1024 * 1024,
    )?;
    ensure!(
        output.status.success(),
        "reading committed Git blobs failed"
    );
    let mut remaining = output.stdout.as_slice();
    let mut blobs = BTreeMap::new();
    for oid in requested {
        let newline = remaining
            .iter()
            .position(|b| *b == b'\n')
            .context("missing Git blob header")?;
        let header: Vec<_> = std::str::from_utf8(&remaining[..newline])?
            .split(' ')
            .collect();
        ensure!(
            header.len() == 3 && header[0] == oid && header[1] == "blob",
            "Git blob identity mismatch"
        );
        let size: usize = header[2].parse()?;
        remaining = &remaining[newline + 1..];
        ensure!(
            size < remaining.len() && remaining[size] == b'\n',
            "truncated Git blob"
        );
        blobs.insert(oid, remaining[..size].to_vec());
        remaining = &remaining[size + 1..];
    }
    ensure!(remaining.is_empty(), "unexpected Git blob output");
    Ok(SourceTree { entries, blobs })
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
    let expected = source_tree(source, &source_commit)?;
    fs::create_dir(export)?;
    let bytes = command(
        Command::new("git")
            .arg("--no-replace-objects")
            .arg("-C")
            .arg(source)
            .args(["archive", "--format=tar", &source_commit]),
        30,
        64 * 1024 * 1024,
    )?;
    let mut source_tar = tar::Archive::new(Cursor::new(bytes));
    let mut provenance = false;
    let mut seen = BTreeSet::new();
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
        let path = entry.path()?.into_owned();
        let (mode, oid) = expected
            .entries
            .get(&path)
            .context("unexpected source export member")?;
        ensure!(seen.insert(path), "duplicate source export member");
        ensure!(
            entry.header().entry_type().is_dir() == (mode == "040000"),
            "source export type mismatch"
        );
        if mode != "040000" {
            let blob = &expected.blobs[oid];
            ensure!(
                entry.size() == blob.len() as u64,
                "source export size differs from committed blob"
            );
            let mut observed = Vec::new();
            entry.read_to_end(&mut observed)?;
            ensure!(
                observed == *blob,
                "source export bytes differ from committed blob"
            );
            ensure!(
                entry.header().mode()? & 0o111 == if mode == "100755" { 0o111 } else { 0 },
                "source export mode differs from committed blob"
            );
            // The entry has been consumed for verification; write the verified bytes ourselves.
            let path = export.join(entry.path()?);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            write_new(&path, blob, if mode == "100755" { 0o755 } else { 0o644 })?;
        } else {
            ensure!(entry.unpack_in(export)?, "source export escaped root");
        }
    }
    ensure!(provenance, "Git archive has no source identity");
    ensure!(
        seen.len() == expected.entries.len(),
        "source export omitted committed entries"
    );
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
        (GNU, &["mantle", "mantle-release", "mantle-acceptance"][..]),
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
