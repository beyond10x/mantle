//! The session directory: three named pipes and two lock files shared by `serve` and `attach`.

use std::fs::{self, DirBuilder, File, OpenOptions, Permissions, TryLockError};
use std::io::{self, ErrorKind};
use std::os::unix::fs::{DirBuilderExt, FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::sys;

pub struct Paths {
    pub dir: PathBuf,
    /// Terminal input, `attach` → `serve`.
    pub input: PathBuf,
    /// Agent output, `serve` → `attach`.
    pub output: PathBuf,
    /// Window sizes, `attach` → `serve`.
    pub ctl: PathBuf,
    /// `ctl` is created here and renamed into place once the server reads it, so its presence
    /// means a server is listening.
    pub ctl_staging: PathBuf,
    pub server_lock: PathBuf,
    pub client_lock: PathBuf,
}

impl Paths {
    pub fn new(dir: &Path) -> Self {
        Self {
            dir: dir.to_owned(),
            input: dir.join("in"),
            output: dir.join("out"),
            ctl: dir.join("ctl"),
            ctl_staging: dir.join("ctl.new"),
            server_lock: dir.join("server.lock"),
            client_lock: dir.join("client.lock"),
        }
    }

    pub fn fifos(&self) -> [&Path; 4] {
        [&self.input, &self.output, &self.ctl, &self.ctl_staging]
    }
}

/// Creates the session directory owner-only, or takes over the one already there. The session
/// directory lies in the shared workspace, so a symlink at that path is refused rather than
/// followed, and an existing directory is forced to 0700 through its own descriptor.
pub fn prepare_dir(dir: &Path) -> Result<()> {
    match fs::symlink_metadata(dir) {
        Ok(meta) if meta.file_type().is_symlink() => {
            bail!(
                "{} is a symbolic link; refusing to follow it",
                dir.display()
            );
        }
        Ok(meta) if !meta.is_dir() => bail!("{} exists and is not a directory", dir.display()),
        Ok(_) => {}
        Err(err) if err.kind() == ErrorKind::NotFound => {
            DirBuilder::new()
                .recursive(true)
                .mode(0o700)
                .create(dir)
                .with_context(|| format!("cannot create {}", dir.display()))?;
        }
        Err(err) => return Err(err).with_context(|| format!("cannot inspect {}", dir.display())),
    }
    // A symlink swapped in after the check above fails here with ELOOP.
    let handle = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW)
        .open(dir)
        .with_context(|| format!("cannot open {} without following a link", dir.display()))?;
    handle
        .set_permissions(Permissions::from_mode(0o700))
        .with_context(|| format!("cannot make {} owner-only", dir.display()))
}

/// True when `dir` is a directory itself, not a symlink to one.
pub fn is_real_dir(dir: &Path) -> bool {
    fs::symlink_metadata(dir).is_ok_and(|meta| meta.is_dir())
}

/// Opens the named pipe at `path` for reading, or for writing when `write`, with the extra open
/// `flags`. A symlink is not followed and anything other than a named pipe is refused.
pub fn open_fifo(path: &Path, write: bool, flags: libc::c_int) -> io::Result<File> {
    let file = OpenOptions::new()
        .read(!write)
        .write(write)
        .custom_flags(flags | libc::O_NOFOLLOW)
        .open(path)?;
    if !file.metadata()?.file_type().is_fifo() {
        return Err(io::Error::other(format!(
            "{} is not a named pipe",
            path.display()
        )));
    }
    Ok(file)
}

/// Takes an exclusive non-blocking `flock` on `path`, creating it. `None` means another open
/// file holds it. A holder may unlink the file before releasing it, so the lock only counts if
/// the path still names the locked inode afterwards. A symlink at `path` is refused.
pub fn try_lock(path: &Path) -> Result<Option<File>> {
    loop {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
            .open(path)
            .with_context(|| format!("cannot open {} without following a link", path.display()))?;
        let held = file.metadata()?;
        if !held.is_file() {
            bail!("{} exists and is not a regular file", path.display());
        }
        match file.try_lock() {
            Ok(()) => {}
            Err(TryLockError::WouldBlock) => return Ok(None),
            Err(TryLockError::Error(err)) => {
                return Err(err).with_context(|| format!("cannot lock {}", path.display()));
            }
        }
        match fs::symlink_metadata(path) {
            Ok(now) if now.dev() == held.dev() && now.ino() == held.ino() => return Ok(Some(file)),
            Ok(_) => {}
            Err(err) if err.kind() == ErrorKind::NotFound => {}
            Err(err) => {
                return Err(err).with_context(|| format!("cannot inspect {}", path.display()));
            }
        }
    }
}

/// Removes a named pipe left behind; refuses to touch anything that is not one.
pub fn remove_fifo(path: &Path) -> Result<()> {
    let meta = match fs::symlink_metadata(path) {
        Ok(meta) => meta,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(()),
        Err(err) => return Err(err).with_context(|| format!("cannot inspect {}", path.display())),
    };
    if !meta.file_type().is_fifo() {
        bail!("{} exists and is not a named pipe", path.display());
    }
    fs::remove_file(path).with_context(|| format!("cannot remove {}", path.display()))
}

pub fn make_fifo(path: &Path) -> Result<()> {
    sys::mkfifo(path, 0o600).with_context(|| format!("cannot create {}", path.display()))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn scratch(name: &str) -> PathBuf {
        let root = std::env::var_os("MANTLE_TEST_SCRATCH").map_or_else(
            || PathBuf::from(std::env::var_os("HOME").expect("HOME")).join(".cache/claude-tmp"),
            PathBuf::from,
        );
        let dir = root.join(format!("mantle-launch-unit-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn a_held_lock_is_refused_and_freed_on_drop() {
        let dir = scratch("lock");
        let path = dir.join("server.lock");
        let first = try_lock(&path).unwrap().expect("first lock");
        assert!(try_lock(&path).unwrap().is_none());
        drop(first);
        assert!(try_lock(&path).unwrap().is_some());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_lock_on_an_unlinked_file_does_not_count() {
        let dir = scratch("unlinked");
        let path = dir.join("server.lock");
        let first = try_lock(&path).unwrap().expect("first lock");
        fs::remove_file(&path).unwrap();
        let second = try_lock(&path).unwrap().expect("lock on the new file");
        assert_eq!(
            second.metadata().unwrap().ino(),
            fs::metadata(&path).unwrap().ino()
        );
        drop(first);
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_symlinked_lock_is_refused_and_its_target_left_alone() {
        let dir = scratch("locklink");
        let target = dir.join("elsewhere");
        let path = dir.join("server.lock");
        std::os::unix::fs::symlink(&target, &path).unwrap();
        assert!(try_lock(&path).is_err(), "a symlinked lock was followed");
        assert!(!target.exists(), "the lock created its symlink's target");
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn open_fifo_refuses_a_symlink_and_a_file_that_is_not_a_pipe() {
        let dir = scratch("openfifo");
        let real = dir.join("real");
        make_fifo(&real).unwrap();
        let link = dir.join("link");
        std::os::unix::fs::symlink(&real, &link).unwrap();
        assert!(open_fifo(&link, false, libc::O_NONBLOCK).is_err());
        let file = dir.join("file");
        fs::write(&file, b"x").unwrap();
        assert!(open_fifo(&file, true, libc::O_NONBLOCK).is_err());
        assert!(open_fifo(&real, false, libc::O_NONBLOCK).is_ok());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn stale_fifos_are_removed_and_other_files_left_alone() {
        let dir = scratch("fifo");
        let fifo = dir.join("in");
        remove_fifo(&fifo).unwrap();
        make_fifo(&fifo).unwrap();
        assert!(fs::symlink_metadata(&fifo).unwrap().file_type().is_fifo());
        remove_fifo(&fifo).unwrap();
        assert!(!fifo.exists());

        let file = dir.join("out");
        fs::write(&file, b"x").unwrap();
        let msg = format!("{:#}", remove_fifo(&file).unwrap_err());
        assert!(msg.contains("is not a named pipe"), "{msg}");
        assert!(file.exists());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn fifos_are_private() {
        use std::os::unix::fs::PermissionsExt;
        let dir = scratch("mode");
        let fifo = dir.join("ctl");
        make_fifo(&fifo).unwrap();
        let mode = fs::metadata(&fifo).unwrap().permissions().mode();
        assert_eq!(mode & 0o077, 0, "mode {mode:o}");
        fs::remove_dir_all(dir).unwrap();
    }
}
