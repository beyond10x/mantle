//! Local path references and explicit runtime selection. No provider or agent credentials live here.
use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, Write};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail, ensure};
use rustix::fs::{AtFlags, Mode, OFlags, RenameFlags, mkdirat, openat, renameat_with, unlinkat};
use serde::{Deserialize, Serialize};

const MAX_PROFILE: u64 = 16 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub name: String,
    pub config_path: PathBuf,
    pub state_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
pub struct Selection {
    pub profile: Option<String>,
    pub config_path: PathBuf,
    pub state_dir: PathBuf,
}

/// Read-only descriptor inspection for diagnostics, including legacy selections. Never creates
/// directories and refuses special files before any potentially blocking read.
pub fn inspect_regular(path: &Path, cap: u64) -> Result<File> {
    let dir = directory(path.parent().context("file needs parent")?, false, false)?;
    let file = File::from(openat(
        &dir,
        path.file_name().context("file needs name")?,
        OFlags::RDONLY | OFlags::NONBLOCK | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::empty(),
    )?);
    let meta = file.metadata()?;
    ensure!(
        meta.is_file() && meta.len() <= cap,
        "diagnostic input must be a bounded regular file"
    );
    Ok(file)
}

pub fn read_bounded(path: &Path, cap: u64) -> Result<String> {
    let mut text = String::new();
    inspect_regular(path, cap)?
        .take(cap + 1)
        .read_to_string(&mut text)?;
    ensure!(
        text.len() as u64 <= cap,
        "diagnostic input exceeds byte limit"
    );
    Ok(text)
}

pub fn validate_name(name: &str) -> Result<()> {
    ensure!(
        !name.is_empty()
            && name.len() <= 63
            && name
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_' || c == b'-')
            && name.as_bytes()[0].is_ascii_alphanumeric(),
        "profile name must start with a lowercase letter or digit and contain only lowercase letters, digits, underscores or hyphens (maximum 63 characters)"
    );
    Ok(())
}

fn validate_path(path: &Path) -> Result<()> {
    ensure!(path.is_absolute(), "profile paths must be absolute");
    ensure!(
        path.components()
            .all(|c| matches!(c, Component::RootDir | Component::Normal(_))),
        "profile paths must not contain parent components"
    );
    ensure!(
        !path
            .as_os_str()
            .as_encoded_bytes()
            .iter()
            .any(|c| c.is_ascii_control()),
        "profile paths must not contain control characters"
    );
    Ok(())
}

/// OpenSSH expands percent tokens and environment references after parsing its options.
/// Reject those forms instead of silently addressing another identity or known-hosts file.
pub fn validate_state_path(path: &Path) -> Result<()> {
    validate_path(path)?;
    let text = path.to_str().context("state path must be valid UTF-8")?;
    ensure!(
        !text.contains(['\"', '\'', '\\', '%']) && !text.contains("${"),
        "state path contains unsupported OpenSSH quote, escape or expansion syntax"
    );
    Ok(())
}

/// Walk through descriptors, refusing symlinks in every path component. A sticky system
/// temporary directory is allowed as an ancestor; the final directory must be owner-private.
fn directory(path: &Path, create: bool, private: bool) -> Result<File> {
    validate_path(path)?;
    let mut dir = File::open("/")?;
    for component in path.components() {
        let Component::Normal(name) = component else {
            continue;
        };
        if create {
            match mkdirat(&dir, name, Mode::from_raw_mode(0o700)) {
                Ok(()) | Err(rustix::io::Errno::EXIST) => {}
                Err(error) => return Err(error.into()),
            }
        }
        dir = File::from(openat(
            &dir,
            name,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        )?);
        let meta = dir.metadata()?;
        let owner = meta.uid();
        ensure!(
            owner == 0 || owner == rustix::process::geteuid().as_raw(),
            "directory is not owned by the operator or root"
        );
        ensure!(
            meta.mode() & 0o022 == 0 || (owner == 0 && meta.mode() & 0o1000 != 0),
            "directory is writable by another user"
        );
    }
    if private {
        let meta = dir.metadata()?;
        ensure!(
            meta.uid() == rustix::process::geteuid().as_raw()
                && meta.permissions().mode() & 0o077 == 0,
            "directory must be owner-private"
        );
    }
    Ok(dir)
}

pub struct Registry {
    path: PathBuf,
}

impl Registry {
    pub fn new(home: &Path) -> Self {
        Self {
            path: home.join(".config/mantle/profiles"),
        }
    }

    pub fn add(&self, profile: &Profile) -> Result<()> {
        validate_name(&profile.name)?;
        validate_path(&profile.config_path)?;
        validate_state_path(&profile.state_dir)?;
        let text = toml::to_string(profile).context("encoding profile paths")?;
        ensure!(
            text.len() as u64 <= MAX_PROFILE,
            "profile paths exceed size limit"
        );
        let dir = directory(&self.path, true, true).context("opening private profile registry")?;
        let pending = format!(".pending-{}", ulid::Ulid::new());
        let mut file = File::from(openat(
            &dir,
            &pending,
            OFlags::CREATE | OFlags::EXCL | OFlags::WRONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::from_raw_mode(0o600),
        )?);
        let result = (|| -> Result<()> {
            file.write_all(text.as_bytes())?;
            file.sync_all()?;
            renameat_with(
                &dir,
                &pending,
                &dir,
                format!("{}.toml", profile.name),
                RenameFlags::NOREPLACE,
            )
            .context("publishing profile (existing names cannot be overwritten)")?;
            dir.sync_all()?;
            Ok(())
        })();
        if result.is_err() {
            let _ = unlinkat(&dir, &pending, AtFlags::empty());
        }
        result
    }

    fn read(dir: &File, name: &str) -> Result<Profile> {
        validate_name(name)?;
        let mut file = File::from(
            openat(
                dir,
                format!("{name}.toml"),
                OFlags::RDONLY | OFlags::NONBLOCK | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                Mode::empty(),
            )
            .context("opening profile")?,
        );
        let meta = file.metadata()?;
        ensure!(
            meta.is_file()
                && meta.uid() == rustix::process::geteuid().as_raw()
                && meta.mode() & 0o077 == 0,
            "profile must be an owner-private regular file"
        );
        ensure!(meta.len() <= MAX_PROFILE, "profile exceeds size limit");
        let mut bytes = Vec::new();
        (&mut file).take(MAX_PROFILE + 1).read_to_end(&mut bytes)?;
        ensure!(
            bytes.len() as u64 <= MAX_PROFILE,
            "profile exceeds size limit"
        );
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| anyhow::anyhow!("profile is not valid UTF-8"))?;
        let profile: Profile = toml::from_str(text)
            .map_err(|_| anyhow::anyhow!("profile has malformed or unsupported fields"))?;
        ensure!(
            profile.name == name,
            "profile name does not match its registry entry"
        );
        validate_path(&profile.config_path)?;
        validate_state_path(&profile.state_dir)?;
        Ok(profile)
    }

    pub fn show(&self, name: &str) -> Result<Profile> {
        validate_name(name)?;
        Self::read(
            &directory(&self.path, false, true).context("opening profile registry")?,
            name,
        )
    }

    pub fn list(&self) -> Result<Vec<Profile>> {
        let dir = match directory(&self.path, false, true) {
            Ok(dir) => dir,
            Err(error)
                if error.downcast_ref::<rustix::io::Errno>() == Some(&rustix::io::Errno::NOENT) =>
            {
                return Ok(vec![]);
            }
            Err(error) => return Err(error.context("opening profile registry")),
        };
        let mut profiles = Vec::new();
        for entry in rustix::fs::Dir::read_from(&dir)? {
            let entry = entry?;
            let name = entry
                .file_name()
                .to_str()
                .context("invalid registry entry name")?;
            if name == "." || name == ".." || name.starts_with(".pending-") {
                continue;
            }
            let name = name
                .strip_suffix(".toml")
                .context("unexpected profile registry entry")?;
            ensure!(
                profiles.len() < 1024,
                "profile registry exceeds entry limit"
            );
            profiles.push(Self::read(&dir, name)?);
        }
        profiles.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(profiles)
    }
}

impl Selection {
    pub fn resolve(
        home: &Path,
        explicit: Option<&str>,
        environment: Option<&str>,
        config: Option<OsString>,
        state: Option<OsString>,
    ) -> Result<Self> {
        if let Some(name) = explicit.or(environment) {
            ensure!(
                config.is_none() && state.is_none(),
                "named profiles cannot be combined with MANTLE_CONFIG or MANTLE_STATE_DIR"
            );
            let profile = Registry::new(home).show(name)?;
            return Ok(Self {
                profile: Some(profile.name),
                config_path: profile.config_path,
                state_dir: profile.state_dir,
            });
        }
        let config_path =
            config.map_or_else(|| home.join(".config/mantle/config.toml"), PathBuf::from);
        let state_dir = state.map_or_else(|| home.join(".local/state/mantle"), PathBuf::from);
        ensure!(
            config_path.is_absolute() && state_dir.is_absolute(),
            "Mantle configuration and state paths must be absolute"
        );
        Ok(Self {
            profile: None,
            config_path,
            state_dir,
        })
    }

    pub fn prepare_state(&self) -> Result<()> {
        if self.profile.is_none() {
            return crate::config::ensure_private_dir(&self.state_dir);
        }
        let dir = directory(&self.state_dir, true, true)
            .context("opening selected private state directory")?;
        for name in [
            "state.db",
            "state.db-wal",
            "state.db-shm",
            "id_ed25519",
            "id_ed25519.pub",
            "known_hosts",
        ] {
            match openat(
                &dir,
                name,
                OFlags::RDONLY | OFlags::NONBLOCK | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                Mode::empty(),
            ) {
                Ok(fd) => {
                    ensure!(
                        File::from(fd).metadata()?.is_file(),
                        "selected state entry must be a regular file"
                    );
                }
                Err(rustix::io::Errno::NOENT) => {}
                Err(error) => return Err(error).context("opening selected state entry"),
            }
        }
        Ok(())
    }

    pub fn read_config(&self) -> Result<String> {
        if self.profile.is_none() {
            return std::fs::read_to_string(&self.config_path).context("reading configuration");
        }
        let parent = self
            .config_path
            .parent()
            .context("configuration needs a filename")?;
        let name = self
            .config_path
            .file_name()
            .context("configuration needs a filename")?;
        let dir = directory(parent, false, false).context("opening configuration directory")?;
        let file = File::from(
            openat(
                &dir,
                name,
                OFlags::RDONLY | OFlags::NONBLOCK | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                Mode::empty(),
            )
            .context("opening configuration")?,
        );
        ensure!(
            file.metadata()?.is_file(),
            "configuration must be a regular file"
        );
        let mut text = String::new();
        file.take(1024 * 1024 + 1)
            .read_to_string(&mut text)
            .context("reading configuration")?;
        if text.len() > 1024 * 1024 {
            bail!("configuration exceeds size limit");
        }
        Ok(text)
    }
}
