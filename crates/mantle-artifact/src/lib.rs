//! Strict, bounded release verification. This crate never starts a process or activates files.
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{File, OpenOptions},
    io::{Cursor, Read},
    os::unix::fs::OpenOptionsExt,
    path::Path,
};

pub const GNU: &str = "x86_64-unknown-linux-gnu";
pub const MUSL: &str = "x86_64-unknown-linux-musl";
pub const MAX_ARCHIVE: u64 = 512 * 1024 * 1024;
pub const MAX_PAYLOAD: u64 = 256 * 1024 * 1024;
pub const MAX_MANIFEST: u64 = 64 * 1024;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Payload {
    pub path: String,
    pub sha256: String,
    pub size_bytes: u64,
    pub mode: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub name: String,
    pub target: String,
    pub sha256: String,
    pub size_bytes: u64,
    pub payloads: Vec<Payload>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub format: String,
    pub version: String,
    pub source_commit: String,
    pub substrate_revision: String,
    pub substrate_version: String,
    pub rustc: String,
    pub gnu_runtime: String,
    pub musl_runtime: String,
    pub artifacts: Vec<Artifact>,
}
#[derive(Debug)]
pub struct VerifiedBundle {
    pub manifest: Manifest,
    pub manifest_sha256: String,
    pub artifacts: BTreeMap<String, BTreeMap<String, Vec<u8>>>,
}
pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub fn inventory(target: &str) -> Result<BTreeMap<String, u32>> {
    let names: &[&str] = match target {
        GNU => &["mantle", "mantle-release", "mantle-acceptance"],
        MUSL => &["mantle-egress", "mantle-launch", "mantle-worker"],
        _ => anyhow::bail!("unsupported target"),
    };
    let mut result = BTreeMap::new();
    for name in names {
        result.insert(format!("bin/{name}"), 0o755);
    }
    for name in [
        "LICENSE",
        "THIRD_PARTY_LICENSES.html",
        "RUST_RUNTIME_LICENSE.html",
    ] {
        result.insert(name.into(), 0o644);
    }
    if target == MUSL {
        result.insert("MUSL_COPYRIGHT".into(), 0o644);
    }
    Ok(result)
}
impl Manifest {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        ensure!(bytes.len() as u64 <= MAX_MANIFEST, "manifest exceeds bound");
        let value: Self = serde_json::from_slice(bytes).context("strict release manifest")?;
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.format == "mantle-release/1",
            "unsupported manifest format"
        );
        let version = semver::Version::parse(&self.version)?;
        ensure!(
            version.build.is_empty(),
            "release build metadata is unsupported"
        );
        ensure!(
            hex(&self.source_commit, 40) && hex(&self.substrate_revision, 40),
            "invalid source revision"
        );
        semver::Version::parse(&self.substrate_version)?;
        runtime_version(
            self.gnu_runtime
                .strip_prefix("glibc ")
                .context("GNU libc provenance")?,
        )?;
        ensure!(
            self.rustc.starts_with("rustc ")
                && self.rustc.len() < 4096
                && self.gnu_runtime.starts_with("glibc ")
                && self.gnu_runtime.len() < 128
                && self.musl_runtime == "1.2.5",
            "unsupported runtime provenance"
        );
        ensure!(
            self.artifacts.len() == 2,
            "exactly two supported artifacts required"
        );
        let mut targets = BTreeSet::new();
        for artifact in &self.artifacts {
            let expected = inventory(&artifact.target)?;
            ensure!(targets.insert(&artifact.target), "duplicate target");
            ensure!(
                artifact.name == format!("mantle-{}-{}.tar", self.version, artifact.target),
                "invalid archive name"
            );
            ensure!(
                hex(&artifact.sha256, 64)
                    && artifact.size_bytes > 0
                    && artifact.size_bytes <= MAX_ARCHIVE,
                "invalid archive digest or size"
            );
            ensure!(
                artifact.payloads.len() == expected.len(),
                "incorrect payload inventory"
            );
            let mut seen = BTreeSet::new();
            let mut total = 0_u64;
            for payload in &artifact.payloads {
                ensure!(seen.insert(&payload.path), "duplicate payload");
                ensure!(
                    expected.get(&payload.path) == Some(&payload.mode),
                    "payload path or mode outside whitelist"
                );
                ensure!(
                    hex(&payload.sha256, 64)
                        && payload.size_bytes > 0
                        && payload.size_bytes <= MAX_PAYLOAD,
                    "invalid payload digest or size"
                );
                total = total
                    .checked_add(payload.size_bytes)
                    .context("payload size overflow")?;
            }
            ensure!(total <= MAX_ARCHIVE, "expanded archive exceeds bound");
        }
        Ok(())
    }
}
/// Open every ancestor without following links. A FIFO cannot block the bounded reader.
pub fn regular(path: &Path) -> Result<File> {
    let absolute = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut current = OpenOptions::new()
        .read(true)
        .custom_flags(rustix::fs::OFlags::DIRECTORY.bits() as i32)
        .open("/")?;
    let parts: Vec<_> = absolute.components().collect();
    for (index, part) in parts.iter().enumerate().skip(1) {
        let std::path::Component::Normal(name) = part else {
            anyhow::bail!("noncanonical path")
        };
        let last = index == parts.len() - 1;
        let flags = rustix::fs::OFlags::RDONLY
            | rustix::fs::OFlags::NOFOLLOW
            | rustix::fs::OFlags::NONBLOCK
            | rustix::fs::OFlags::CLOEXEC;
        let fd = rustix::fs::openat(
            &current,
            *name,
            if last {
                flags
            } else {
                flags | rustix::fs::OFlags::DIRECTORY
            },
            rustix::fs::Mode::empty(),
        )?;
        current = File::from(fd);
    }
    ensure!(current.metadata()?.is_file(), "expected regular file");
    Ok(current)
}
pub fn read_bounded(path: &Path, maximum: u64) -> Result<Vec<u8>> {
    let file = regular(path)?;
    ensure!(file.metadata()?.len() <= maximum, "file exceeds size bound");
    let mut bytes = Vec::new();
    file.take(maximum + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= maximum, "file grew beyond bound");
    Ok(bytes)
}
pub fn verify(manifest_path: &Path) -> Result<VerifiedBundle> {
    let bytes = read_bounded(manifest_path, MAX_MANIFEST)?;
    let manifest = Manifest::parse(&bytes)?;
    let parent = manifest_path.parent().context("manifest parent")?;
    let mut artifacts = BTreeMap::new();
    for artifact in &manifest.artifacts {
        let bytes = read_bounded(&parent.join(&artifact.name), MAX_ARCHIVE)
            .context("reading release archive")?;
        artifacts.insert(artifact.target.clone(), verify_archive(artifact, &bytes)?);
    }
    for bytes in artifacts[GNU]
        .iter()
        .filter(|(path, _)| path.starts_with("bin/"))
        .map(|(_, bytes)| bytes)
    {
        let baseline = runtime_version(
            manifest
                .gnu_runtime
                .strip_prefix("glibc ")
                .context("GNU libc provenance")?,
        )?;
        let elf = goblin::elf::Elf::parse(bytes)?;
        if let Some(needs) = &elf.verneed {
            for file in needs.iter() {
                for version in file.iter() {
                    if let Some(version) = elf
                        .dynstrtab
                        .get_at(version.vna_name)
                        .and_then(|v| v.strip_prefix("GLIBC_"))
                    {
                        ensure!(
                            runtime_version(version)? <= baseline,
                            "GNU payload requires a newer libc than the manifest runtime"
                        );
                    }
                }
            }
        }
    }
    Ok(VerifiedBundle {
        manifest,
        manifest_sha256: sha256(&bytes),
        artifacts,
    })
}
fn runtime_version(version: &str) -> Result<Vec<u32>> {
    let parts = version
        .split('.')
        .map(str::parse)
        .collect::<std::result::Result<Vec<u32>, _>>()?;
    ensure!((2..=3).contains(&parts.len()), "invalid libc version");
    let mut parts = parts;
    parts.resize(3, 0);
    Ok(parts)
}
pub fn verify_archive(artifact: &Artifact, bytes: &[u8]) -> Result<BTreeMap<String, Vec<u8>>> {
    ensure!(artifact.size_bytes <= MAX_ARCHIVE, "archive exceeds bound");
    ensure!(
        bytes.len() as u64 == artifact.size_bytes && sha256(bytes) == artifact.sha256,
        "archive checksum or size mismatch"
    );
    let expected = inventory(&artifact.target)?;
    ensure!(
        artifact.payloads.len() == expected.len(),
        "incorrect archive inventory"
    );
    let mut archive = tar::Archive::new(Cursor::new(bytes));
    let mut result = BTreeMap::new();
    let mut encoded = 1024_u64;
    for entry in archive.entries()?.raw(true) {
        let entry = entry?;
        ensure!(
            entry.header().entry_type().is_file(),
            "archive links and special members are forbidden"
        );
        let path = std::str::from_utf8(&entry.path_bytes())?.to_owned();
        let payload = artifact
            .payloads
            .iter()
            .find(|p| p.path == path)
            .context("archive member outside manifest")?;
        ensure!(
            expected.get(&path) == Some(&payload.mode) && !result.contains_key(&path),
            "archive path outside whitelist or duplicate"
        );
        ensure!(
            entry.size() == payload.size_bytes && entry.size() <= MAX_PAYLOAD,
            "archive member exceeds declared size"
        );
        ensure!(
            entry.header().mode()? == payload.mode
                && entry.header().uid()? == 0
                && entry.header().gid()? == 0
                && entry.header().mtime()? == 0,
            "noncanonical archive metadata"
        );
        encoded = encoded
            .checked_add(512 + entry.size().div_ceil(512) * 512)
            .context("archive size overflow")?;
        let mut data = Vec::new();
        entry.take(MAX_PAYLOAD + 1).read_to_end(&mut data)?;
        ensure!(
            data.len() as u64 == payload.size_bytes && sha256(&data) == payload.sha256,
            "payload checksum mismatch"
        );
        if path.starts_with("bin/") {
            verify_elf(&data, &artifact.target)?;
        }
        result.insert(path, data);
    }
    ensure!(result.len() == expected.len(), "missing archive members");
    ensure!(
        encoded == bytes.len() as u64 && bytes[bytes.len() - 1024..].iter().all(|b| *b == 0),
        "noncanonical archive trailer"
    );
    Ok(result)
}
pub fn verify_elf(bytes: &[u8], target: &str) -> Result<()> {
    inventory(target)?;
    let elf = goblin::elf::Elf::parse(bytes).context("invalid ELF executable")?;
    ensure!(
        elf.is_64
            && elf.little_endian
            && elf.header.e_machine == goblin::elf::header::EM_X86_64
            && elf.entry != 0
            && matches!(
                elf.header.e_type,
                goblin::elf::header::ET_EXEC | goblin::elf::header::ET_DYN
            ),
        "unsupported ELF executable"
    );
    if target == MUSL {
        ensure!(
            elf.interpreter.is_none() && elf.libraries.is_empty(),
            "worker executable is not static"
        );
    }
    Ok(())
}
