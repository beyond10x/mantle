//! Explicit upgrade transport. Read-only checking uses ordinary tools on older workers.
use super::worker;
use crate::{
    adapters::{ssh::Ssh, state::Store},
    config::{Config, RuntimeContext},
    profile::Selection,
};
use anyhow::{Context, Result, ensure};
use mantle_worker::maintenance::{self, Host, Metadata};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    path::Path,
    time::{Duration, Instant},
};
fn quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
fn remote(ssh: &Ssh, args: &[&str], input: Option<&[u8]>, timeout: Duration) -> Result<Vec<u8>> {
    let command = format!(
        "sudo -n env LC_ALL=C {}",
        args.iter().map(|a| quote(a)).collect::<Vec<_>>().join(" ")
    );
    let out = ssh.bounded(&command, input, timeout)?;
    ensure!(out.status.success(), "bounded upgrade observation failed");
    Ok(out.stdout)
}
pub struct RemoteHost<'a> {
    ssh: &'a Ssh,
    entries: RefCell<BTreeMap<String, Vec<String>>>,
}
impl<'a> RemoteHost<'a> {
    fn new(ssh: &'a Ssh) -> Self {
        Self {
            ssh,
            entries: RefCell::new(BTreeMap::new()),
        }
    }
}
impl Host for RemoteHost<'_> {
    fn command(&self, args: &[&str], deadline: Instant) -> Result<Vec<u8>> {
        remote(self.ssh, args, None, maintenance::remaining(deadline)?)
    }
    fn read(&self, path: &str, cap: usize, deadline: Instant) -> Result<Vec<u8>> {
        ensure!(
            self.metadata(path, deadline)?.kind == "file",
            "remote observation requires a regular file"
        );
        let bytes = self.command(
            &["head", "-c", &(cap + 1).to_string(), "--", path],
            deadline,
        )?;
        ensure!(bytes.len() <= cap, "remote observation exceeds bound");
        Ok(bytes)
    }
    fn metadata(&self, path: &str, deadline: Instant) -> Result<Metadata> {
        let bytes = self.command(
            &["stat", "--printf=%F|%u|%g|%a|%h|%s", "--", path],
            deadline,
        )?;
        let text = String::from_utf8(bytes)?;
        let fields: Vec<_> = text.split('|').collect();
        ensure!(fields.len() == 6, "invalid remote metadata");
        Ok(Metadata {
            kind: match fields[0] {
                "regular file" | "regular empty file" => "file",
                "directory" => "directory",
                "symbolic link" => "link",
                _ => "other",
            }
            .into(),
            uid: fields[1].parse()?,
            gid: fields[2].parse()?,
            mode: u32::from_str_radix(fields[3], 8)?,
            links: fields[4].parse()?,
            size: fields[5].parse()?,
        })
    }
    fn entries(&self, path: &str, deadline: Instant) -> Result<Vec<String>> {
        if let Some(names) = self.entries.borrow().get(path) {
            return Ok(names.clone());
        }
        ensure!(
            self.metadata(path, deadline)?.kind == "directory",
            "remote listing requires a real directory"
        );
        let bytes = self.command(
            &[
                "find",
                "-P",
                path,
                "-mindepth",
                "1",
                "-maxdepth",
                "1",
                "-printf",
                "%f\\0",
            ],
            deadline,
        )?;
        let mut names = Vec::new();
        for name in bytes.split(|b| *b == 0).filter(|n| !n.is_empty()) {
            ensure!(names.len() < 4096, "remote directory exceeds bound");
            let name = String::from_utf8(name.to_vec())?;
            ensure!(
                !name.contains('/') && name != "." && name != "..",
                "invalid remote entry"
            );
            names.push(name);
        }
        self.entries.borrow_mut().insert(path.into(), names.clone());
        Ok(names)
    }
    fn link(&self, path: &str, deadline: Instant) -> Result<String> {
        Ok(
            String::from_utf8(self.command(&["readlink", "--", path], deadline)?)?
                .trim_end_matches('\n')
                .into(),
        )
    }
}
/// A root-owned random transport stage, outside the installed namespace. Drop is bounded cleanup.
pub(super) struct Delivery<'a> {
    ssh: &'a Ssh,
    path: String,
    active: bool,
}
impl<'a> Delivery<'a> {
    pub(super) fn create(ssh: &'a Ssh) -> Result<Self> {
        let path = String::from_utf8(remote(
            ssh,
            &["mktemp", "-d", "/var/tmp/mantle-delivery.XXXXXXXXXXXX"],
            None,
            Duration::from_secs(15),
        )?)?
        .trim()
        .to_owned();
        ensure!(
            path.strip_prefix("/var/tmp/mantle-delivery.")
                .is_some_and(|s| s.len() == 12 && s.bytes().all(|b| b.is_ascii_alphanumeric())),
            "unrecognized owned transport stage"
        );
        let stage = Self {
            ssh,
            path,
            active: true,
        };
        let host = RemoteHost::new(ssh);
        let m = host.metadata(&stage.path, Instant::now() + Duration::from_secs(15))?;
        ensure!(
            m.kind == "directory" && m.uid == 0 && m.mode == 0o700,
            "untrusted transport stage"
        );
        Ok(stage)
    }
    pub(super) fn put(&self, name: &str, bytes: &[u8], executable: bool) -> Result<()> {
        ensure!(
            !name.is_empty()
                && name
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"-._".contains(&b)),
            "invalid delivery name"
        );
        ensure!(
            bytes.len() <= mantle_worker::MAX_HELPER_BYTES,
            "delivery file exceeds transport bound"
        );
        let path = format!("{}/{name}", self.path);
        self.ssh.checked_bounded(
            &format!("sudo -n tee -- {} >/dev/null", quote(&path)),
            Some(bytes),
            Duration::from_secs(90),
        )?;
        let digest = String::from_utf8(remote(
            self.ssh,
            &["sha256sum", "--", &path],
            None,
            Duration::from_secs(15),
        )?)?;
        ensure!(
            digest.split_whitespace().next() == Some(mantle_artifact::sha256(bytes).as_str()),
            "transport checksum mismatch"
        );
        remote(
            self.ssh,
            &[
                "chmod",
                if executable { "0755" } else { "0600" },
                "--",
                &path,
            ],
            None,
            Duration::from_secs(15),
        )?;
        Ok(())
    }
    pub(super) fn reconcile(&self) -> Result<Vec<String>> {
        let path = format!("{}/mantle-worker", self.path);
        let bytes = remote(
            self.ssh,
            &[&path, "reconcile-helpers", "--source", &self.path],
            None,
            Duration::from_secs(180),
        )?;
        Ok(serde_json::from_slice(&bytes)?)
    }
    pub(super) fn install_codex(&self) -> Result<Vec<u8>> {
        remote(
            self.ssh,
            &[&format!("{}/mantle-worker", self.path), "install-codex"],
            None,
            Duration::from_secs(210),
        )
    }
    pub(super) fn close(mut self) -> Result<()> {
        remote(
            self.ssh,
            &["rm", "-r", "--", &self.path],
            None,
            Duration::from_secs(15),
        )
        .with_context(|| format!("cleaning owned transport stage {}", self.path))?;
        self.active = false;
        Ok(())
    }
    fn apply(&self) -> Result<maintenance::Assessment> {
        let helper = format!("{}/mantle-worker", self.path);
        let manifest = format!("{}/manifest.json", self.path);
        let bytes = remote(
            self.ssh,
            &[&helper, "upgrade-apply", "--manifest", &manifest],
            None,
            Duration::from_secs(210),
        )?;
        Ok(serde_json::from_slice(&bytes)?)
    }
}
impl Drop for Delivery<'_> {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let _ = remote(
            self.ssh,
            &["rm", "-r", "--", &self.path],
            None,
            Duration::from_secs(15),
        );
    }
}

pub fn run(selection: Selection, manifest: &Path, apply: bool) -> Result<std::process::ExitCode> {
    let bundle = mantle_artifact::verify(manifest)?;
    let config = RuntimeContext {
        config: Config::parse(&crate::profile::read_bounded(
            &selection.config_path,
            1024 * 1024,
        )?)?,
        selection,
    };
    let store = Store::open_readonly(&config.selection.state_dir.join("state.db"))?;
    let record = store
        .worker(worker::WORKER)?
        .context("no recorded worker; upgrade never provisions one")?;
    ensure!(
        !record.instance.is_empty()
            && record.instance.len() <= 255
            && !record.instance.starts_with('-')
            && record
                .instance
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b)),
        "invalid recorded worker identity"
    );
    let ssh = worker::ssh_readonly_for(&config, &record)?;
    let host = RemoteHost::new(&ssh);
    let report = if !apply {
        mantle_worker::upgrade::remote_check(&host, &bundle)
    } else {
        let assessment = mantle_worker::upgrade::remote_check(&host, &bundle);
        if !assessment.applicable {
            assessment
        } else {
            // Freeze and hash every original byte before creating a remote stage. A path reread
            // cannot substitute a different, valid bundle after the operator's verification.
            let parent = manifest.parent().context("manifest parent")?;
            let mut frozen = BTreeMap::new();
            let bytes = mantle_artifact::read_bounded(manifest, mantle_artifact::MAX_MANIFEST)?;
            ensure!(
                mantle_artifact::sha256(&bytes) == bundle.manifest_sha256,
                "manifest changed before transport"
            );
            frozen.insert("manifest.json".to_owned(), bytes);
            for artifact in &bundle.manifest.artifacts {
                let bytes = mantle_artifact::read_bounded(
                    &parent.join(&artifact.name),
                    mantle_worker::MAX_HELPER_BYTES as u64,
                )?;
                ensure!(
                    mantle_artifact::sha256(&bytes) == artifact.sha256,
                    "archive changed before transport"
                );
                frozen.insert(artifact.name.clone(), bytes);
            }
            let stage = Delivery::create(&ssh)?;
            stage.put(
                "mantle-worker",
                &bundle.artifacts[mantle_artifact::MUSL]["bin/mantle-worker"],
                true,
            )?;
            let version = String::from_utf8(remote(
                &ssh,
                &[&format!("{}/mantle-worker", stage.path), "--version"],
                None,
                Duration::from_secs(15),
            )?)?;
            ensure!(
                version.trim() == format!("mantle-worker {}", bundle.manifest.version),
                "temporary helper version mismatch"
            );
            for (name, bytes) in frozen {
                stage.put(&name, &bytes, false)?;
            }
            let report = stage.apply()?;
            stage.close()?;
            report
        }
    };
    println!("{}", serde_json::to_string_pretty(&report)?);
    let accepted = if apply {
        matches!(
            report.outcome.as_str(),
            "current" | "applied-restart-required"
        )
    } else {
        report.applicable
    };
    Ok(if accepted {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::FAILURE
    })
}
