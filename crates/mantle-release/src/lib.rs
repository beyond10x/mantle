//! Source-bound packaging and explicit installation/publication orchestration.
pub mod build;
pub mod licenses;
pub mod publish;
use anyhow::{Context, Result, ensure};
use mantle_artifact::{GNU, VerifiedBundle};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt, symlink},
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
};

pub fn command(command: &mut Command, seconds: u64, cap: usize) -> Result<Vec<u8>> {
    let out = mantle_worker::run_bounded(command, None, Duration::from_secs(seconds), cap)?;
    ensure!(
        out.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    Ok(out.stdout)
}
pub fn text(command: &mut Command) -> Result<String> {
    Ok(String::from_utf8(self::command(command, 30, 1024 * 1024)?)?
        .trim()
        .to_owned())
}
pub fn verify_versions(bundle: &VerifiedBundle) -> Result<()> {
    let temporary = tempfile::tempdir()?;
    for (target, files) in &bundle.artifacts {
        let dir = temporary.path().join(target);
        fs::create_dir(&dir)?;
        for (path, bytes) in files.iter().filter(|(path, _)| path.starts_with("bin/")) {
            let name = Path::new(path).file_name().context("binary name")?;
            let path = dir.join(name);
            write_new(&path, bytes, 0o755)?;
            let observed = text(Command::new(path).arg("--version"))?;
            ensure!(
                observed == format!("{} {}", name.to_string_lossy(), bundle.manifest.version),
                "binary version mismatch"
            );
        }
    }
    Ok(())
}
pub fn write_new(path: &Path, bytes: &[u8], mode: u32) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    file.write_all(bytes)?;
    file.set_permissions(fs::Permissions::from_mode(mode))?;
    file.sync_all()?;
    Ok(())
}
fn directory(path: &Path) -> Result<()> {
    let meta = fs::symlink_metadata(path)?;
    use std::os::unix::fs::MetadataExt;
    ensure!(
        meta.is_dir()
            && meta.uid() == rustix::process::geteuid().as_raw()
            && meta.mode() & 0o022 == 0,
        "installation directory must be owned and not writable by others"
    );
    Ok(())
}
fn create_directory(path: &Path, mode: u32) -> Result<bool> {
    match fs::DirBuilder::new().mode(mode).create(path) {
        Ok(()) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            directory(path)?;
            Ok(false)
        }
        Err(e) => Err(e.into()),
    }
}
fn expected_link(path: &Path, target: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(meta) => {
            ensure!(
                meta.file_type().is_symlink() && fs::read_link(path)? == target,
                "unmanaged destination collision"
            );
            Ok(true)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e.into()),
    }
}

fn generation_inventory(
    root: &Path,
    files: &std::collections::BTreeMap<String, Vec<u8>>,
) -> Result<()> {
    let mut expected = std::collections::BTreeMap::new();
    for path in files.keys().map(Path::new) {
        expected.insert(path.to_path_buf(), false);
        for parent in path
            .ancestors()
            .skip(1)
            .filter(|p| !p.as_os_str().is_empty())
        {
            expected.insert(parent.to_path_buf(), true);
        }
    }
    let mut pending = vec![PathBuf::new()];
    while let Some(parent) = pending.pop() {
        for entry in fs::read_dir(root.join(&parent))? {
            let entry = entry?;
            let relative = parent.join(entry.file_name());
            let is_dir = expected
                .remove(&relative)
                .context("existing generation inventory was modified")?;
            let kind = entry.file_type()?;
            ensure!(
                if is_dir {
                    kind.is_dir()
                } else {
                    kind.is_file()
                },
                "existing generation entry type was modified"
            );
            if is_dir {
                directory(&entry.path())?;
                pending.push(relative);
            }
        }
    }
    ensure!(
        expected.is_empty(),
        "existing generation inventory is incomplete"
    );
    Ok(())
}
/// Single GNU generation activation. Worker replacement belongs to the offline worker upgrader.
pub fn install(bundle: &VerifiedBundle, prefix: &Path, target: &str) -> Result<PathBuf> {
    ensure!(
        target == GNU,
        "worker activation requires the explicit offline worker upgrade path"
    );
    ensure!(prefix.is_absolute(), "installation prefix must be absolute");
    create_directory(prefix, 0o755)?;
    ensure!(
        prefix.canonicalize()? == prefix,
        "installation prefix cannot contain links or noncanonical components"
    );
    let bin = prefix.join("bin");
    create_directory(&bin, 0o755)?;
    // Refuse every destination before creating managed links or activating any binary.
    for name in ["mantle", "mantle-release", "mantle-acceptance"] {
        expected_link(
            &bin.join(name),
            &PathBuf::from(format!("../.mantle/current/bin/{name}")),
        )?;
    }
    let state = prefix.join(".mantle");
    let fresh = create_directory(&state, 0o700)?;
    if fresh {
        write_new(&state.join("owner"), b"mantle-release-install/1\n", 0o600)?;
    }
    ensure!(
        mantle_artifact::read_bounded(&state.join("owner"), 128)? == b"mantle-release-install/1\n",
        "unmanaged installer state"
    );
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags((rustix::fs::OFlags::NOFOLLOW | rustix::fs::OFlags::NONBLOCK).bits() as i32)
        .open(state.join("lock"))?;
    ensure!(lock.metadata()?.is_file(), "invalid install lock");
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match rustix::fs::flock(&lock, rustix::fs::FlockOperation::NonBlockingLockExclusive) {
            Ok(()) => break,
            Err(e) if e == rustix::io::Errno::WOULDBLOCK => {
                ensure!(Instant::now() < deadline, "installation lock timeout");
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(e) => return Err(e.into()),
        }
    }
    let current = state.join("current");
    if current.symlink_metadata().is_ok() {
        let link = fs::read_link(&current).context("unmanaged active installation")?;
        let parts: Vec<_> = link.components().collect();
        ensure!(
            parts.len() == 2
                && parts[0].as_os_str() == "generations"
                && parts[1]
                    .as_os_str()
                    .to_str()
                    .is_some_and(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit())),
            "unmanaged active generation"
        );
        directory(&state.join(&link))?;
    }
    let generations = state.join("generations");
    create_directory(&generations, 0o700)?;
    let destination = generations.join(&bundle.manifest_sha256);
    let files = bundle.artifacts.get(GNU).context("GNU artifact missing")?;
    if destination.symlink_metadata().is_ok() {
        directory(&destination)?;
        generation_inventory(&destination, files)?;
        for (path, bytes) in files {
            ensure!(
                fs::symlink_metadata(destination.join(path))?
                    .permissions()
                    .mode()
                    & 0o7777
                    == if path.starts_with("bin/") {
                        0o755
                    } else {
                        0o644
                    },
                "existing generation mode was modified"
            );
            ensure!(
                mantle_artifact::read_bounded(
                    &destination.join(path),
                    mantle_artifact::MAX_PAYLOAD
                )? == *bytes,
                "existing generation was modified"
            );
        }
    } else {
        let stage = tempfile::Builder::new()
            .prefix(".stage-")
            .tempdir_in(&generations)?;
        fs::create_dir(stage.path().join("bin"))?;
        for (path, bytes) in files {
            write_new(
                &stage.path().join(path),
                bytes,
                if path.starts_with("bin/") {
                    0o755
                } else {
                    0o644
                },
            )?;
        }
        File::open(stage.path().join("bin"))?.sync_all()?;
        File::open(stage.path())?.sync_all()?;
        fs::rename(stage.path(), &destination)?;
        File::open(&generations)?.sync_all()?;
    }
    // Under the lock, re-check links in case another installer activated while we waited.
    for name in ["mantle", "mantle-release", "mantle-acceptance"] {
        let target = PathBuf::from(format!("../.mantle/current/bin/{name}"));
        if !expected_link(&bin.join(name), &target)? {
            symlink(target, bin.join(name))?;
        }
    }
    File::open(&bin)?.sync_all()?;
    let activation = tempfile::Builder::new()
        .prefix(".activation-")
        .tempdir_in(&state)?;
    symlink(
        PathBuf::from("generations").join(&bundle.manifest_sha256),
        activation.path().join("next"),
    )?;
    fs::rename(activation.path().join("next"), &current)?;
    File::open(&state)?.sync_all()?;
    Ok(destination)
}
