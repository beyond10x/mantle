//! Offline, crash-inspectable helper replacement. Lock order: host, then agent-specific lock.
use crate::{
    maintenance::{self, Assessment, Host},
    read_regular, regular_file, trusted_ancestors,
};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::Write,
    os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt, symlink},
    path::Path,
    process::Command,
    time::{Duration, Instant},
};

pub const HELPERS: [&str; 3] = ["mantle-egress", "mantle-launch", "mantle-worker"];
const JOURNAL: &str = "upgrade-pending.json";
const MARKER: &str = "managed-bundle.json";
const DEADLINE: Duration = Duration::from_secs(120);
pub trait Hooks {
    fn boundary(&self, _name: &str) -> Result<()> {
        Ok(())
    }
}
impl Hooks for () {}

pub struct HostLock {
    _file: File,
}
impl HostLock {
    pub fn acquire(root: &Path) -> Result<Self> {
        crate::create_trusted_dir(root)?;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_NONBLOCK)
            .open(root.join("install.lock"))?;
        crate::validate_regular(&file)?;
        ensure!(
            file.metadata()?.mode() & 0o777 == 0o600,
            "host lock must be private"
        );
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            match file.try_lock() {
                Ok(()) => return Ok(Self { _file: file }),
                Err(std::fs::TryLockError::WouldBlock) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(20))
                }
                _ => anyhow::bail!("host installation lock unavailable"),
            }
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Identity {
    dev: u64,
    ino: u64,
    uid: u32,
    gid: u32,
    mode: u32,
}
fn identity(path: &Path) -> Result<Identity> {
    trusted_ancestors(path)?;
    let m = fs::symlink_metadata(path)?;
    ensure!(m.is_dir(), "bundle directory must be real");
    Ok(Identity {
        dev: m.dev(),
        ino: m.ino(),
        uid: m.uid(),
        gid: m.gid(),
        mode: m.mode() & 0o7777,
    })
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Entry {
    sha256: String,
    mode: u32,
    uid: u32,
    gid: u32,
    link: Option<String>,
}
type Inventory = BTreeMap<String, Entry>;
fn inventory(path: &Path) -> Result<Inventory> {
    identity(path)?;
    let mut result = BTreeMap::new();
    for entry in fs::read_dir(path)? {
        ensure!(result.len() < 5, "unsupported helper directory inventory");
        let entry = entry?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("unsupported helper filename"))?;
        ensure!(
            HELPERS.contains(&name.as_str()) || name == "claude" || name == "codex",
            "unsupported helper directory entry"
        );
        let m = fs::symlink_metadata(entry.path())?;
        ensure!(
            m.uid() == nix::unistd::Uid::effective().as_raw() && m.mode() & 0o022 == 0
                || (name == "codex"
                    && m.file_type().is_symlink()
                    && m.uid() == nix::unistd::Uid::effective().as_raw()),
            "unsafe helper ownership or mode"
        );
        let (sha256, link) = if name == "codex" {
            ensure!(
                m.file_type().is_symlink()
                    && fs::read_link(entry.path())? == Path::new("../agents/codex/current/codex"),
                "unsupported Codex layout"
            );
            (String::new(), Some("../agents/codex/current/codex".into()))
        } else {
            ensure!(
                m.is_file() && m.nlink() == 1 && m.mode() & 0o111 != 0,
                "unsupported helper file"
            );
            (
                mantle_artifact::sha256(&read_regular(&entry.path(), crate::MAX_BINARY)?),
                None,
            )
        };
        result.insert(
            name,
            Entry {
                sha256,
                link,
                mode: m.mode() & 0o7777,
                uid: m.uid(),
                gid: m.gid(),
            },
        );
    }
    ensure!(
        HELPERS.iter().all(|name| result.contains_key(*name)),
        "incomplete existing helper directory"
    );
    Ok(result)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal {
    format: String,
    stage: String,
    status: String,
    manifest_sha256: String,
    source_commit: String,
    version: String,
    previous: Identity,
    next: Identity,
    previous_files: Inventory,
    next_files: Inventory,
}
fn sync_dir(path: &Path) -> Result<()> {
    File::open(path)?.sync_all()?;
    Ok(())
}
fn json_write(root: &Path, name: &str, value: &impl Serialize) -> Result<()> {
    let mut file = tempfile::NamedTempFile::new_in(root)?;
    file.as_file()
        .set_permissions(fs::Permissions::from_mode(0o600))?;
    file.write_all(&serde_json::to_vec(value)?)?;
    file.as_file().sync_all()?;
    file.persist(root.join(name)).map_err(|e| e.error)?;
    sync_dir(root)
}
fn read_journal(root: &Path) -> Result<Journal> {
    parse_journal(&read_regular(&root.join(JOURNAL), 65536)?)
}
fn parse_journal(bytes: &[u8]) -> Result<Journal> {
    let j: Journal = serde_json::from_slice(bytes)?;
    ensure!(
        j.format == "mantle-offline-upgrade/1"
            && j.stage.starts_with(".upgrade-")
            && j.stage
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-')
            && j.stage.len() < 100,
        "invalid upgrade journal"
    );
    ensure!(
        j.previous.dev == j.next.dev && j.previous.ino != j.next.ino,
        "invalid journal directory identities"
    );
    Ok(j)
}
fn pending(j: &Journal) -> bool {
    !matches!(j.status.as_str(), "applied" | "rolled-back" | "not-applied")
}
/// Called only while holding HostLock. A crashed transaction retains its inventory ownership
/// until recovery completes, even though its original process released the advisory lock.
pub(crate) fn ensure_agent_install_permitted(root: &Path) -> Result<()> {
    match root.join(JOURNAL).symlink_metadata() {
        Ok(_) => ensure!(
            !pending(&read_journal(root)?),
            "unfinished helper transaction requires recovery before agent installation"
        ),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    Ok(())
}
fn versions(path: &Path, version: &str, deadline: Instant) -> Result<()> {
    for name in HELPERS {
        let result = crate::run_bounded(
            Command::new(path.join(name)).arg("--version"),
            None,
            maintenance::remaining(deadline)?,
            4096,
        )?;
        ensure!(
            result.status.success()
                && String::from_utf8(result.stdout)?.trim() == format!("{name} {version}"),
            "installed helper version verification failed"
        );
    }
    Ok(())
}
fn exchange(root: &Path, stage: &Path) -> Result<()> {
    rustix::fs::renameat_with(
        rustix::fs::CWD,
        root.join("bin"),
        rustix::fs::CWD,
        stage.join("bin"),
        rustix::fs::RenameFlags::EXCHANGE,
    )?;
    sync_dir(root)?;
    sync_dir(stage)
}
fn outcome(mut assessment: Assessment, state: &str, detail: &str) -> Assessment {
    assessment.outcome = state.into();
    assessment.diagnostic = detail.into();
    assessment
}
fn verify_side(path: &Path, id: &Identity, files: &Inventory) -> Result<()> {
    ensure!(
        &identity(path)? == id && &inventory(path)? == files,
        "journal directory identity or inventory mismatch"
    );
    Ok(())
}
/// A completed transaction no longer owns the active directory's entire lifetime. The Codex
/// installer may add its stable link later under the host lock; all recorded entries stay exact.
fn verify_completed_side(path: &Path, id: &Identity, files: &Inventory) -> Result<()> {
    let mut observed = inventory(path)?;
    if !files.contains_key("codex") {
        observed.remove("codex");
    }
    ensure!(
        &identity(path)? == id && &observed == files,
        "completed bundle identity or inventory changed; no historical rollback attempted"
    );
    Ok(())
}
fn rollback(
    root: &Path,
    j: &mut Journal,
    assessment: Assessment,
    hooks: &impl Hooks,
) -> Result<Assessment> {
    let stage = root.join(&j.stage);
    let restore = (|| {
        verify_side(&stage.join("bin"), &j.previous, &j.previous_files)?;
        ensure!(
            identity(&root.join("bin"))? == j.next,
            "active directory identity changed"
        );
        hooks.boundary("before-rollback")?;
        exchange(root, &stage)?;
        hooks.boundary("after-rollback")?;
        verify_side(&root.join("bin"), &j.previous, &j.previous_files)?;
        Ok::<_, anyhow::Error>(())
    })();
    if restore.is_ok() {
        j.status = "rolled-back".into();
        json_write(root, JOURNAL, j)?;
        Ok(outcome(
            assessment,
            "rolled-back",
            "Exact prior helper directory was restored and verified; maintenance remains in effect.",
        ))
    } else {
        j.status = "recovery-required".into();
        json_write(root, JOURNAL, j)?;
        Ok(outcome(
            assessment,
            "recovery-required",
            "Restoration could not be verified. Inspect the retained journal and both directory identities; no rollback is claimed.",
        ))
    }
}
fn recover_locked(
    host: &impl Host,
    root: &Path,
    j: &mut Journal,
    hooks: &impl Hooks,
    deadline: Instant,
) -> Result<Assessment> {
    let mut assessment = maintenance::assess(host, deadline);
    assessment.candidate_source = j.source_commit.clone();
    assessment.candidate_manifest = j.manifest_sha256.clone();
    ensure!(
        assessment.applicable,
        "recovery requires verified offline maintenance"
    );
    let stage = root.join(&j.stage);
    if !pending(j) {
        // Never run postcheck/rollback against a terminal journal: a supported writer may have
        // changed the agent inventory since this transaction relinquished ownership.
        if j.status == "applied" {
            verify_completed_side(&root.join("bin"), &j.next, &j.next_files)?;
            verify_side(&stage.join("bin"), &j.previous, &j.previous_files)?;
        } else {
            verify_completed_side(&root.join("bin"), &j.previous, &j.previous_files)?;
            verify_side(&stage.join("bin"), &j.next, &j.next_files)?;
        }
        return Ok(outcome(
            assessment,
            if j.status == "applied" {
                "current"
            } else {
                &j.status
            },
            "Completed transaction verified without replay; later supported Codex installation is preserved.",
        ));
    }
    let active = identity(&root.join("bin"))?;
    let other = identity(&stage.join("bin"))?;
    if active == j.previous && other == j.next {
        verify_side(&root.join("bin"), &j.previous, &j.previous_files)?;
        j.status = if j.status == "rolled-back" {
            "rolled-back"
        } else {
            "not-applied"
        }
        .into();
        json_write(root, JOURNAL, j)?;
        return Ok(outcome(
            assessment,
            &j.status,
            "Prior helper directory is verified active; no claim is made about earlier exchanges.",
        ));
    }
    ensure!(
        active == j.next && other == j.previous,
        "unrecognized transaction directory orientation; inspect retained journal"
    );
    let postcheck = (|| {
        verify_side(&root.join("bin"), &j.next, &j.next_files)?;
        verify_side(&stage.join("bin"), &j.previous, &j.previous_files)?;
        hooks.boundary("postcheck")?;
        versions(&root.join("bin"), &j.version, deadline)?;
        Ok::<_, anyhow::Error>(())
    })();
    if postcheck.is_err() {
        return rollback(root, j, assessment, hooks);
    }
    json_write(root, MARKER, j)?;
    hooks.boundary("after-marker")?;
    j.status = "applied".into();
    json_write(root, JOURNAL, j)?;
    Ok(outcome(
        assessment,
        "applied-restart-required",
        "Verified bundle activated. Services remain masked and inactive; administrator restart is required.",
    ))
}
pub fn recover(host: &impl Host, root: &Path, hooks: &impl Hooks) -> Result<Assessment> {
    let _lock = HostLock::acquire(root)?;
    let mut journal = read_journal(root)?;
    recover_locked(host, root, &mut journal, hooks, Instant::now() + DEADLINE)
}
pub fn check(host: &impl Host, root: &Path, manifest: &Path) -> Result<Assessment> {
    let bundle = mantle_artifact::verify(manifest)?;
    check_bundle(host, root, &bundle)
}
fn check_bundle(
    host: &impl Host,
    root: &Path,
    bundle: &mantle_artifact::VerifiedBundle,
) -> Result<Assessment> {
    let mut a = candidate_assessment(host, bundle);
    if root.join(JOURNAL).symlink_metadata().is_ok() && pending(&read_journal(root)?) {
        a.diagnostic = "Retained transaction requires recovery under verified offline maintenance before a current result can be established.".into();
        return Ok(a);
    }
    if root.join(MARKER).symlink_metadata().is_ok() {
        let installed: Journal = serde_json::from_slice(&read_regular(&root.join(MARKER), 65536)?)?;
        if installed.manifest_sha256 == bundle.manifest_sha256
            && verify_completed_side(&root.join("bin"), &installed.next, &installed.next_files)
                .is_ok()
            && a.applicable
        {
            a.outcome = "current".into();
        }
    }
    Ok(a)
}
pub fn candidate_assessment(
    host: &impl Host,
    bundle: &mantle_artifact::VerifiedBundle,
) -> Assessment {
    let deadline = Instant::now() + DEADLINE;
    let mut a = maintenance::assess(host, deadline);
    a.candidate_source = bundle.manifest.source_commit.clone();
    a.candidate_manifest = bundle.manifest_sha256.clone();
    for name in HELPERS {
        let path = format!("/opt/mantle/bin/{name}");
        let version = host
            .command(&[&path, "--version"], deadline)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .map(|s| s.trim().to_owned())
            .filter(|s| s.starts_with(&format!("{name} ")) && s.len() < 128 && !s.contains('\n'))
            .unwrap_or_else(|| format!("{name} unknown"));
        a.installed_versions.push(version);
    }
    if bundle.manifest.substrate_version != maintenance::SUBSTRATE_VERSION
        || bundle.manifest.substrate_revision != maintenance::SUBSTRATE_REVISION
    {
        a.applicable = false;
        a.compatibility = "incompatible".into();
        a.outcome = "incompatible".into();
        return a;
    }
    a
}
/// Existing workers need only ordinary read-only utilities, never an uploaded checker.
pub fn remote_check(host: &impl Host, bundle: &mantle_artifact::VerifiedBundle) -> Assessment {
    let mut a = candidate_assessment(host, bundle);
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut requires_recovery = false;
    let current = (|| -> Result<bool> {
        let root_entries = host.entries("/opt/mantle", deadline)?;
        if root_entries.iter().any(|p| p == JOURNAL) {
            let path = format!("/opt/mantle/{JOURNAL}");
            let m = host.metadata(&path, deadline)?;
            ensure!(
                m.kind == "file" && m.uid == host.owner() && m.mode == 0o600 && m.links == 1,
                "unsafe upgrade journal"
            );
            let journal = parse_journal(&host.read(&path, 65536, deadline)?)?;
            if pending(&journal) {
                requires_recovery = true;
                return Ok(false);
            }
        }
        if !root_entries.iter().any(|p| p == MARKER) {
            return Ok(false);
        }
        let m = host.metadata("/opt/mantle/managed-bundle.json", deadline)?;
        ensure!(
            m.kind == "file" && m.uid == host.owner() && m.mode == 0o600 && m.links == 1,
            "unsafe managed marker"
        );
        let installed: Journal = serde_json::from_slice(&host.read(
            "/opt/mantle/managed-bundle.json",
            65536,
            deadline,
        )?)?;
        if installed.manifest_sha256 != bundle.manifest_sha256 {
            return Ok(false);
        }
        let names = host.entries("/opt/mantle/bin", deadline)?;
        ensure!(
            installed.next_files.keys().all(|n| names.contains(n))
                && names
                    .iter()
                    .all(|n| installed.next_files.contains_key(n) || n == "codex"),
            "managed inventory changed"
        );
        if !installed.next_files.contains_key("codex") && names.iter().any(|n| n == "codex") {
            let path = "/opt/mantle/bin/codex";
            let m = host.metadata(path, deadline)?;
            ensure!(
                m.kind == "link"
                    && m.uid == host.owner()
                    && host.link(path, deadline)? == "../agents/codex/current/codex",
                "unsupported later Codex entry"
            );
        }
        for (name, entry) in installed.next_files {
            ensure!(
                HELPERS.contains(&name.as_str()) || name == "claude" || name == "codex",
                "unsupported managed name"
            );
            let path = format!("/opt/mantle/bin/{name}");
            let m = host.metadata(&path, deadline)?;
            ensure!(
                m.uid == entry.uid && m.gid == entry.gid && m.mode == entry.mode,
                "managed metadata changed"
            );
            if let Some(link) = entry.link {
                ensure!(
                    m.kind == "link" && host.link(&path, deadline)? == link,
                    "managed link changed"
                );
            } else {
                ensure!(m.kind == "file" && m.links == 1, "managed file changed");
                let digest =
                    String::from_utf8(host.command(&["sha256sum", "--", &path], deadline)?)?;
                ensure!(
                    digest.split_whitespace().next() == Some(entry.sha256.as_str()),
                    "managed bytes changed"
                );
                if HELPERS.contains(&name.as_str()) {
                    ensure!(
                        entry.sha256
                            == mantle_artifact::sha256(
                                &bundle.artifacts[mantle_artifact::MUSL][&format!("bin/{name}")]
                            ),
                        "candidate bytes differ"
                    );
                }
            }
        }
        Ok(true)
    })();
    match current {
        Ok(true) if a.applicable => a.outcome = "current".into(),
        Err(_) => {
            a.applicable = false;
            a.outcome = "unknown".into();
            a.diagnostic = "Installed bundle metadata or inventory could not be verified.".into();
        }
        _ => {}
    }
    if requires_recovery {
        a.diagnostic = "Retained transaction requires recovery under verified offline maintenance before a current result can be established.".into();
    }
    a
}
pub fn apply(
    host: &impl Host,
    root: &Path,
    manifest: &Path,
    hooks: &impl Hooks,
) -> Result<Assessment> {
    let bundle = mantle_artifact::verify(manifest)?;
    let assessment = check_bundle(host, root, &bundle)?;
    if !assessment.applicable {
        return Ok(assessment);
    }
    trusted_ancestors(root)?;
    let _lock = HostLock::acquire(root)?;
    let deadline = Instant::now() + DEADLINE;
    if root.join(JOURNAL).symlink_metadata().is_ok() {
        let mut prior = read_journal(root)?;
        if pending(&prior) {
            let recovered = recover_locked(host, root, &mut prior, hooks, deadline)?;
            if prior.manifest_sha256 != bundle.manifest_sha256
                && recovered.outcome == "applied-restart-required"
            {
                return Ok(outcome(
                    recovered,
                    "recovered-other-bundle",
                    "Recovered the earlier journal's bundle, not the requested candidate. Review its source and run apply again for the intended candidate.",
                ));
            }
            return Ok(recovered);
        }
        let current = recover_locked(host, root, &mut prior, hooks, deadline)?;
        if current.outcome == "recovery-required" {
            return Ok(current);
        }
        if prior.manifest_sha256 == bundle.manifest_sha256 && prior.status == "applied" {
            return Ok(outcome(
                current,
                "current",
                "Exact selected source and payload inventory are already installed.",
            ));
        }
        fs::rename(
            root.join(JOURNAL),
            root.join(&prior.stage).join("journal.json"),
        )?;
        sync_dir(root)?;
    }
    let previous = identity(&root.join("bin"))?;
    let previous_files = inventory(&root.join("bin"))?;
    let staging = tempfile::Builder::new()
        .prefix(".upgrade-")
        .tempdir_in(root)?;
    let next = staging.path().join("bin");
    fs::create_dir(&next)?;
    fs::set_permissions(&next, fs::Permissions::from_mode(previous.mode))?;
    nix::unistd::fchown(
        File::open(&next)?,
        Some(nix::unistd::Uid::from_raw(previous.uid)),
        Some(nix::unistd::Gid::from_raw(previous.gid)),
    )?;
    for (name, entry) in &previous_files {
        if HELPERS.contains(&name.as_str()) {
            continue;
        }
        if let Some(link) = &entry.link {
            symlink(link, next.join(name))?;
            nix::unistd::fchownat(
                File::open(&next)?,
                name.as_str(),
                Some(nix::unistd::Uid::from_raw(entry.uid)),
                Some(nix::unistd::Gid::from_raw(entry.gid)),
                nix::fcntl::AtFlags::AT_SYMLINK_NOFOLLOW,
            )?;
        } else {
            let bytes = read_regular(&root.join("bin").join(name), crate::MAX_BINARY)?;
            write_file(&next.join(name), &bytes, entry.mode, entry.uid, entry.gid)?;
        }
    }
    for name in HELPERS {
        write_file(
            &next.join(name),
            &bundle.artifacts[mantle_artifact::MUSL][&format!("bin/{name}")],
            0o755,
            previous.uid,
            previous.gid,
        )?;
    }
    versions(&next, &bundle.manifest.version, deadline)?;
    let mut journal = Journal {
        format: "mantle-offline-upgrade/1".into(),
        stage: staging
            .path()
            .file_name()
            .context("stage name")?
            .to_str()
            .context("stage encoding")?
            .into(),
        status: "prepared".into(),
        manifest_sha256: bundle.manifest_sha256,
        source_commit: bundle.manifest.source_commit,
        version: bundle.manifest.version,
        previous,
        next: identity(&next)?,
        previous_files,
        next_files: inventory(&next)?,
    };
    sync_dir(&next)?;
    sync_dir(staging.path())?;
    hooks.boundary("before-journal")?;
    let stage = staging.keep();
    json_write(root, JOURNAL, &journal)?;
    hooks.boundary("after-journal")?;
    let assessment = maintenance::assess(host, deadline);
    ensure!(
        assessment.applicable,
        "offline prerequisites changed before exchange"
    );
    verify_side(
        &root.join("bin"),
        &journal.previous,
        &journal.previous_files,
    )?;
    verify_side(&stage.join("bin"), &journal.next, &journal.next_files)?;
    hooks.boundary("before-exchange")?;
    exchange(root, &stage)?;
    if hooks.boundary("after-exchange").is_err() {
        return rollback(root, &mut journal, assessment, hooks);
    }
    journal.status = "exchanged".into();
    json_write(root, JOURNAL, &journal)?;
    recover_locked(host, root, &mut journal, hooks, deadline)
}
fn write_file(path: &Path, bytes: &[u8], mode: u32, uid: u32, gid: u32) -> Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    f.write_all(bytes)?;
    nix::unistd::fchown(
        &f,
        Some(nix::unistd::Uid::from_raw(uid)),
        Some(nix::unistd::Gid::from_raw(gid)),
    )?;
    f.set_permissions(fs::Permissions::from_mode(mode))?;
    f.sync_all()?;
    Ok(())
}
/// Fresh/legacy provisioning shares the same host lock, and checks the marker only after locking.
pub fn reconcile_helpers(
    host: &impl Host,
    root: &Path,
    source: &Path,
    hooks: &impl Hooks,
) -> Result<Vec<String>> {
    let mut inputs = BTreeMap::new();
    for name in HELPERS {
        inputs.insert(name, crate::read_delivery_binary(&source.join(name))?);
    }
    hooks.boundary("before-host-lock")?;
    let _lock = HostLock::acquire(root)?;
    ensure!(
        !root.join(MARKER).try_exists()? && !root.join(JOURNAL).try_exists()?,
        "managed helper bundles require explicit offline upgrade"
    );
    crate::create_trusted_dir(&root.join("bin"))?;
    let deadline = Instant::now() + DEADLINE;
    let mut deferred = Vec::new();
    let properties = "--property=ActiveState";
    for name in HELPERS {
        let destination = root.join("bin").join(name);
        let bytes = &inputs[name];
        if read_regular(&destination, crate::MAX_HELPER_BYTES as u64).is_ok_and(|old| old == *bytes)
        {
            continue;
        }
        let unit = if name == "mantle-egress" {
            "mantle-egress.service"
        } else {
            "substrate.service"
        };
        let observed = host.command(
            &["systemctl", "show", "--no-pager", properties, unit],
            deadline,
        )?;
        ensure!(
            observed == b"ActiveState=inactive\n" || observed == b"ActiveState=active\n",
            "unknown provisioning service state"
        );
        if observed == b"ActiveState=active\n" && destination.symlink_metadata().is_ok() {
            deferred.push(name.into());
            continue;
        }
        if destination.symlink_metadata().is_ok() {
            regular_file(&destination)?;
        }
        let mut temporary = tempfile::NamedTempFile::new_in(root.join("bin"))?;
        temporary.write_all(bytes)?;
        temporary
            .as_file()
            .set_permissions(fs::Permissions::from_mode(0o755))?;
        temporary.as_file().sync_all()?;
        temporary.persist(destination).map_err(|e| e.error)?;
        sync_dir(&root.join("bin"))?;
    }
    Ok(deferred)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_helper_transaction_excludes_codex_writer_before_fetch_or_agent_paths() {
        let root = tempfile::tempdir().unwrap();
        let previous = Identity {
            dev: 1,
            ino: 1,
            uid: nix::unistd::Uid::effective().as_raw(),
            gid: nix::unistd::Gid::effective().as_raw(),
            mode: 0o755,
        };
        let mut journal = Journal {
            format: "mantle-offline-upgrade/1".into(),
            stage: ".upgrade-fixture".into(),
            status: "prepared".into(),
            manifest_sha256: "1".repeat(64),
            source_commit: "2".repeat(40),
            version: "0.1.4".into(),
            next: Identity {
                ino: 2,
                ..previous.clone()
            },
            previous,
            previous_files: Inventory::new(),
            next_files: Inventory::new(),
        };
        let installer = crate::Installer {
            root: root.path().into(),
            candidate: crate::candidate(),
            version_output: String::new(),
        };
        for status in ["prepared", "exchanged", "recovery-required"] {
            journal.status = status.into();
            json_write(root.path(), JOURNAL, &journal).unwrap();
            let before = fs::read(root.path().join(JOURNAL)).unwrap();
            let error = installer
                .install_with("x86_64", |_, _| {
                    panic!("unfinished helper transaction must exclude fetching")
                })
                .unwrap_err();
            assert!(
                error.to_string().contains("unfinished helper transaction"),
                "{error:#}"
            );
            assert!(!root.path().join("agents").exists());
            assert_eq!(fs::read(root.path().join(JOURNAL)).unwrap(), before);
        }
    }
}
