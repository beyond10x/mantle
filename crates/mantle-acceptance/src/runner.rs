//! Bounded installed-CLI qualification. Raw child/terminal output is never persisted.
use crate::{CreationReceipt, MAX_METADATA, Metadata, SelectionBinding, Session, encoded, now};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};
const FORMAT: &str = "mantle-acceptance/v1";
const TIMEOUT: Duration = Duration::from_secs(120);
pub const INVENTORY: [&str; 10] = [
    "create",
    "login",
    "model-tool",
    "visual",
    "status-exec",
    "detach-reconnect",
    "resize-interrupt",
    "transport-loss",
    "retain-restart",
    "destroy",
];
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub case: String,
    pub status: String,
    pub origin: String,
    pub phase: String,
    pub exec_id: Option<String>,
    pub observed_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint {
    pub format: String,
    pub run_id: String,
    pub agent: String,
    pub profile: String,
    pub selection: SelectionBinding,
    pub executable: PathBuf,
    pub executable_sha256: String,
    pub executable_version: String,
    pub source_commit: Option<String>,
    pub run_dir: PathBuf,
    pub phase: String,
    pub session: Option<Session>,
    pub results: Vec<Evidence>,
}
impl Checkpoint {
    pub fn exit_code(&self) -> u8 {
        if self.results.iter().any(|r| r.status == "failed") {
            1
        } else if self.results.len() == INVENTORY.len()
            && INVENTORY.iter().all(|name| {
                self.results
                    .iter()
                    .filter(|r| r.case == *name && r.status == "passed")
                    .count()
                    == 1
            })
        {
            0
        } else {
            2
        }
    }
    fn observe(&mut self, case: &str, status: &str, origin: &str) {
        if let Some(row) = self.results.iter_mut().find(|r| r.case == case) {
            // Later recovery may clean resources, but cannot replace measured qualification
            // failure or its original generation, phase, timestamp and evidence origin.
            if row.status == "failed" {
                return;
            }
            *row = Evidence {
                case: case.into(),
                status: status.into(),
                origin: origin.into(),
                phase: self.phase.clone(),
                exec_id: self.session.as_ref().and_then(|s| s.exec_id.clone()),
                observed_at: now(),
            };
        }
    }
}
fn private_file(path: &Path, cap: u64) -> Result<Vec<u8>> {
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(
            rustix::fs::OFlags::NOFOLLOW.bits() as i32 | rustix::fs::OFlags::NONBLOCK.bits() as i32,
        )
        .open(path)?;
    let m = file.metadata()?;
    ensure!(
        m.is_file()
            && m.len() <= cap
            && m.mode() & 0o077 == 0
            && m.uid() == rustix::process::geteuid().as_raw(),
        "private bounded regular file required"
    );
    let mut bytes = Vec::new();
    file.take(cap + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= cap, "private file exceeds bound");
    Ok(bytes)
}
fn read(path: &Path) -> Result<Checkpoint> {
    let value: Checkpoint = serde_json::from_slice(&private_file(path, MAX_METADATA as u64)?)?;
    ensure!(
        value.format == FORMAT
            && value.results.len() == INVENTORY.len()
            && INVENTORY.iter().all(|name| value
                .results
                .iter()
                .filter(|r| r.case == *name)
                .count()
                == 1),
        "unsupported checkpoint"
    );
    ensure!(
        value.agent == "codex" || value.agent == "claude-code",
        "unsupported agent"
    );
    Ok(value)
}
fn lock(path: &Path) -> Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags(
            rustix::fs::OFlags::NOFOLLOW.bits() as i32 | rustix::fs::OFlags::NONBLOCK.bits() as i32,
        )
        .open(path.with_extension("lock"))?;
    ensure!(file.metadata()?.is_file(), "checkpoint lock is not regular");
    file.try_lock()?;
    Ok(file)
}
fn save(path: &Path, checkpoint: &Checkpoint) -> Result<()> {
    let mut file = tempfile::NamedTempFile::new_in(path.parent().context("checkpoint parent")?)?;
    file.write_all(&encoded(checkpoint)?)?;
    file.as_file().sync_all()?;
    file.persist(path)?;
    File::open(path.parent().unwrap())?.sync_all()?;
    Ok(())
}
fn binary(path: &Path) -> Result<Vec<u8>> {
    ensure!(path.is_absolute(), "CLI path must be absolute");
    mantle_artifact::read_bounded(&fs::canonicalize(path)?, 128 * 1024 * 1024)
}
fn command(binary: &Path) -> Command {
    let mut command = Command::new(binary);
    command
        .env_remove("MANTLE_PROFILE")
        .env_remove("MANTLE_CONFIG")
        .env_remove("MANTLE_STATE_DIR");
    command
}
fn output(command: &mut Command) -> Result<std::process::Output> {
    invoke(command, TIMEOUT)
}
fn metadata(command: &mut Command) -> Result<Metadata> {
    let out = output(command)?;
    ensure!(out.status.success(), "CLI metadata failed");
    let data: Metadata = serde_json::from_slice(&out.stdout)?;
    ensure!(
        data.format == crate::FORMAT && matches!(data.outcome.as_str(), "recorded" | "observed"),
        "invalid metadata outcome"
    );
    Ok(data)
}
fn pinned(checkpoint: &Checkpoint) -> Result<Command> {
    ensure!(
        mantle_artifact::sha256(&binary(&checkpoint.executable)?) == checkpoint.executable_sha256,
        "selected executable changed"
    );
    let snapshot = checkpoint.run_dir.join("mantle");
    ensure!(
        mantle_artifact::sha256(&mantle_artifact::read_bounded(
            &snapshot,
            128 * 1024 * 1024
        )?) == checkpoint.executable_sha256,
        "pinned executable changed"
    );
    let mut cmd = command(&snapshot);
    cmd.env("MANTLE_CONFIG", &checkpoint.selection.config_path)
        .env("MANTLE_STATE_DIR", &checkpoint.selection.state_dir);
    Ok(cmd)
}
fn revalidate(checkpoint: &Checkpoint) -> Result<()> {
    let _ = pinned(checkpoint)?;
    let current = metadata(command(&checkpoint.run_dir.join("mantle")).args([
        "--profile",
        &checkpoint.profile,
        "list",
        "--json",
    ]))?;
    ensure!(
        current.selection.as_ref() == Some(&checkpoint.selection),
        "profile/configuration changed"
    );
    Ok(())
}
fn exact(checkpoint: &Checkpoint, live: bool) -> Result<Session> {
    let owned = checkpoint
        .session
        .as_ref()
        .context("creation ownership unresolved")?;
    let report = metadata(pinned(checkpoint)?.args([
        "status",
        "--session-id",
        &owned.id,
        "--json",
        "--timeout-secs",
        "10",
    ]))?;
    let mut expected = checkpoint.selection.clone();
    expected.profile = None;
    ensure!(
        report.selection.as_ref() == Some(&expected) && report.sessions.len() == 1,
        "selection/record mismatch"
    );
    let row = report.sessions.into_iter().next().unwrap();
    ensure!(
        row.id == owned.id
            && row.name == owned.name
            && row.agent == checkpoint.agent
            && row.workspace_id == owned.workspace_id,
        "session binding changed"
    );
    if live {
        ensure!(
            report.outcome == "observed"
                && row
                    .observed
                    .as_ref()
                    .is_some_and(|o| o.exec_state.as_deref() == Some("running")
                        && !o.refused
                        && o.workspace_state.as_deref() == Some("ready")),
            "agent not observed running"
        );
    }
    Ok(row)
}
fn mutation(checkpoint: &Checkpoint, verb: &str, extra: &[&str]) -> Result<()> {
    let session = checkpoint
        .session
        .as_ref()
        .context("creation ownership unresolved")?;
    revalidate(checkpoint)?;
    let out = output(
        pinned(checkpoint)?
            .args([verb, &session.name, "--expected-session-id", &session.id])
            .args(extra),
    )?;
    ensure!(out.status.success(), "CLI operation failed");
    Ok(())
}
fn shell_quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
}
pub fn attach_command(checkpoint: &Checkpoint) -> Result<String> {
    let session = checkpoint.session.as_ref().context("creation unresolved")?;
    Ok(format!(
        "env -u MANTLE_PROFILE MANTLE_CONFIG={} MANTLE_STATE_DIR={} {} attach {} --expected-session-id {}",
        shell_quote(
            checkpoint
                .selection
                .config_path
                .to_str()
                .context("path encoding")?
        ),
        shell_quote(
            checkpoint
                .selection
                .state_dir
                .to_str()
                .context("path encoding")?
        ),
        shell_quote(
            checkpoint
                .run_dir
                .join("mantle")
                .to_str()
                .context("path encoding")?
        ),
        shell_quote(&session.name),
        shell_quote(&session.id)
    ))
}
pub fn run(
    executable: &Path,
    profile: &str,
    agent: &str,
    manifest: &Path,
    path: &Path,
    release: Option<&Path>,
) -> Result<u8> {
    ensure!(
        path.is_absolute() && !path.try_exists()?,
        "checkpoint must be a new absolute path"
    );
    let _lock = lock(path)?;
    inventory(agent)?;
    let bytes = binary(executable)?;
    let digest = mantle_artifact::sha256(&bytes);
    let source = if let Some(path) = release {
        let bundle = mantle_artifact::verify(path)?;
        ensure!(
            bundle.artifacts[mantle_artifact::GNU]
                .get("bin/mantle")
                .is_some_and(|b| mantle_artifact::sha256(b) == digest),
            "release executable mismatch"
        );
        Some(bundle.manifest.source_commit)
    } else {
        None
    };
    let run_id = ulid::Ulid::new().to_string().to_lowercase();
    let dir = tempfile::Builder::new()
        .prefix("mantle-acceptance-")
        .tempdir_in(path.parent().context("checkpoint parent")?)?;
    let snapshot = dir.path().join("mantle");
    fs::write(&snapshot, bytes)?;
    fs::set_permissions(&snapshot, fs::Permissions::from_mode(0o700))?;
    let version = output(command(&snapshot).arg("--version"))?;
    ensure!(version.status.success(), "selected CLI version unavailable");
    let version = String::from_utf8(version.stdout)?;
    let version = version.trim();
    ensure!(
        version.starts_with("mantle ")
            && version.len() <= 80
            && version
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b" .+-".contains(&c)),
        "unsupported version output"
    );
    let data = metadata(command(&snapshot).args(["--profile", profile, "list", "--json"]))?;
    let selection = data.selection.context("selection missing")?;
    let mut checkpoint = Checkpoint {
        format: FORMAT.into(),
        run_id: run_id.clone(),
        agent: agent.into(),
        profile: profile.into(),
        selection,
        executable: executable.into(),
        executable_sha256: digest,
        executable_version: version.into(),
        source_commit: source,
        run_dir: dir.path().to_owned(),
        phase: "creating".into(),
        session: None,
        results: INVENTORY
            .iter()
            .map(|case| Evidence {
                case: (*case).into(),
                status: if matches!(*case, "login" | "model-tool" | "visual") {
                    "operator-required"
                } else {
                    "not-run"
                }
                .into(),
                origin: if matches!(*case, "login" | "model-tool" | "visual") {
                    "operator"
                } else {
                    "machine"
                }
                .into(),
                phase: "initial".into(),
                exec_id: None,
                observed_at: now(),
            })
            .collect(),
    };
    crate::write_new(path, &checkpoint)?;
    let _retained = dir.keep();
    let result = (|| -> Result<()> {
        let mut document: serde_yaml::Value = serde_yaml::from_slice(
            &mantle_artifact::read_bounded(manifest, MAX_METADATA as u64)?,
        )?;
        let name = format!("acceptance-{run_id}");
        let metadata = document
            .get_mut("metadata")
            .and_then(serde_yaml::Value::as_mapping_mut)
            .context("manifest metadata mapping required")?;
        metadata.insert("name".into(), name.clone().into());
        let agent_mapping = document
            .get_mut("agent")
            .and_then(serde_yaml::Value::as_mapping_mut)
            .context("manifest agent mapping required")?;
        agent_mapping.insert("kind".into(), agent.into());
        agent_mapping.insert(
            "auth".into(),
            if agent == "codex" {
                "chatgpt-device"
            } else {
                "claude-oauth"
            }
            .into(),
        );
        let manifest_path = checkpoint.run_dir.join("session.yaml");
        fs::write(&manifest_path, serde_yaml::to_string(&document)?)?;
        let receipt_path = checkpoint.run_dir.join("receipt.json");
        revalidate(&checkpoint)?;
        let out = output(
            pinned(&checkpoint)?
                .arg("start")
                .arg(&manifest_path)
                .arg("--detached")
                .arg("--receipt")
                .arg(&receipt_path),
        )?;
        ensure!(out.status.success(), "creation failed or unresolved");
        let receipt: CreationReceipt =
            serde_json::from_slice(&private_file(&receipt_path, MAX_METADATA as u64)?)?;
        validate_receipt(&receipt, &checkpoint.selection, &name, agent)?;
        checkpoint.session = Some(receipt.session);
        checkpoint.phase = "manual".into();
        checkpoint.observe("create", "passed", "machine");
        save(path, &checkpoint)?;
        let observed = exact(&checkpoint, true)?;
        ensure!(
            observed.exec_id == checkpoint.session.as_ref().unwrap().exec_id,
            "created exec changed"
        );
        println!(
            "Run {}. Complete login, a repository model/tool turn and visual checks in your own terminal:\n{}\nThen resume with --expected-run-id {} --expected-exec-id {} and explicit --attest flags.",
            checkpoint.run_id,
            attach_command(&checkpoint)?,
            checkpoint.run_id,
            observed.exec_id.as_deref().unwrap()
        );
        Ok(())
    })();
    if result.is_err() {
        checkpoint.observe("create", "failed", "machine");
        save(path, &checkpoint)?;
        return Ok(1);
    }
    Ok(2)
}
pub fn report(path: &Path) -> Result<u8> {
    let checkpoint = read(path)?;
    println!("{}", String::from_utf8(encoded(&checkpoint)?)?);
    Ok(checkpoint.exit_code())
}
pub fn cleanup(path: &Path) -> Result<u8> {
    let _lock = lock(path)?;
    let mut checkpoint = read(path)?;
    let result = mutation(&checkpoint, "destroy", &["--yes"]).and_then(|()| {
        ensure!(
            exact(&checkpoint, false)?.recorded_state == "STOPPED",
            "destruction unconfirmed"
        );
        Ok(())
    });
    checkpoint.phase = "cleanup".into();
    checkpoint.observe(
        "destroy",
        if result.is_ok() { "passed" } else { "failed" },
        "machine",
    );
    save(path, &checkpoint)?;
    if result.is_err() {
        return Ok(1);
    }
    Ok(checkpoint.exit_code())
}
pub fn resume(path: &Path, run: &str, exec: &str, attest: &[&str]) -> Result<u8> {
    let _lock = lock(path)?;
    let mut checkpoint = read(path)?;
    ensure!(
        checkpoint.run_id == run
            && checkpoint.phase == "manual"
            && checkpoint
                .session
                .as_ref()
                .and_then(|s| s.exec_id.as_deref())
                == Some(exec),
        "stale attestation generation"
    );
    revalidate(&checkpoint)?;
    ensure!(
        exact(&checkpoint, true)?.exec_id.as_deref() == Some(exec),
        "exec generation changed"
    );
    self::attest(&mut checkpoint, run, exec, attest)?;
    save(path, &checkpoint)?;
    if ["login", "model-tool", "visual"].iter().any(|name| {
        !checkpoint
            .results
            .iter()
            .any(|r| r.case == *name && r.status == "passed")
    }) {
        return Ok(2);
    }
    let marker = format!("/workspace/.mantle-acceptance-{}", checkpoint.run_id);
    for case in [
        "status-exec",
        "detach-reconnect",
        "resize-interrupt",
        "transport-loss",
        "retain-restart",
        "destroy",
    ] {
        checkpoint.phase = case.into();
        save(path, &checkpoint)?;
        let result = (|| -> Result<()> {
            revalidate(&checkpoint)?;
            match case {
                "status-exec" => {
                    exact(&checkpoint, true)?;
                    mutation(&checkpoint, "exec", &["--", "touch", &marker])?;
                    mutation(&checkpoint, "exec", &["--", "test", "-f", &marker])?;
                }
                "detach-reconnect" => {
                    terminal(&checkpoint, false, false)?;
                    terminal(&checkpoint, false, false)?;
                    ensure!(
                        exact(&checkpoint, true)?.exec_id.as_deref() == Some(exec),
                        "detach changed exec"
                    );
                }
                "resize-interrupt" => {
                    terminal(&checkpoint, true, false)?;
                    ensure!(
                        exact(&checkpoint, true)?.exec_id.as_deref() == Some(exec),
                        "control changed exec"
                    );
                }
                "transport-loss" => {
                    terminal(&checkpoint, false, true)?;
                    terminal(&checkpoint, false, false)?;
                    ensure!(
                        exact(&checkpoint, true)?.exec_id.as_deref() == Some(exec),
                        "transport loss changed exec"
                    );
                }
                "retain-restart" => {
                    mutation(&checkpoint, "stop", &[])?;
                    ensure!(
                        exact(&checkpoint, false)?.recorded_state == "RETAINED",
                        "retention unconfirmed"
                    );
                    mutation(&checkpoint, "restart", &[])?;
                    let row = exact(&checkpoint, true)?;
                    ensure!(
                        row.exec_id.is_some() && row.exec_id.as_deref() != Some(exec),
                        "restart did not replace exec"
                    );
                    checkpoint.session = Some(row);
                    save(path, &checkpoint)?;
                    mutation(&checkpoint, "exec", &["--", "test", "-f", &marker])?;
                }
                "destroy" => {
                    mutation(&checkpoint, "destroy", &["--yes"])?;
                    ensure!(
                        exact(&checkpoint, false)?.recorded_state == "STOPPED",
                        "destruction unconfirmed"
                    );
                }
                _ => unreachable!(),
            }
            Ok(())
        })();
        checkpoint.observe(
            case,
            if result.is_ok() { "passed" } else { "failed" },
            "machine",
        );
        save(path, &checkpoint)?;
        if result.is_err() {
            return Ok(1);
        }
    }
    checkpoint.phase = "complete".into();
    save(path, &checkpoint)?;
    Ok(checkpoint.exit_code())
}
fn terminal(checkpoint: &Checkpoint, controls: bool, loss: bool) -> Result<()> {
    use rustix::{
        fs::{Mode, OFlags},
        pty::{OpenptFlags, grantpt, openpt, ptsname, unlockpt},
        termios::{Winsize, tcsetwinsize},
    };
    let master = openpt(OpenptFlags::RDWR | OpenptFlags::NOCTTY | OpenptFlags::CLOEXEC)?;
    rustix::fs::fcntl_setfl(&master, OFlags::NONBLOCK)?;
    grantpt(&master)?;
    unlockpt(&master)?;
    let slave = File::from(rustix::fs::open(
        ptsname(&master, Vec::new())?.as_c_str(),
        OFlags::RDWR | OFlags::NOCTTY | OFlags::CLOEXEC,
        Mode::empty(),
    )?);
    let mut master = File::from(master);
    tcsetwinsize(
        &master,
        Winsize {
            ws_row: 24,
            ws_col: 80,
            ws_xpixel: 0,
            ws_ypixel: 0,
        },
    )?;
    let row = checkpoint
        .session
        .as_ref()
        .context("ownership unresolved")?;
    let mut command = pinned(checkpoint)?;
    command
        .args(["attach", &row.name, "--expected-session-id", &row.id])
        .stdin(Stdio::from(slave.try_clone()?))
        .stdout(Stdio::from(slave.try_clone()?))
        .stderr(Stdio::from(slave));
    let child =
        mantle_worker::BoundedProcess::spawn_with_stdio(&mut command, Duration::from_secs(20))?;
    let deadline = Instant::now() + Duration::from_secs(15);
    let started = Instant::now();
    let mut sent = false;
    let mut read_bytes = 0usize;
    loop {
        ensure!(Instant::now() < deadline, "PTY observation timed out");
        let mut buffer = [0; 8192];
        match master.read(&mut buffer) {
            Ok(n) => {
                read_bytes += n;
                ensure!(read_bytes <= MAX_METADATA, "PTY output bound exceeded");
            }
            Err(e)
                if matches!(e.kind(), std::io::ErrorKind::WouldBlock)
                    || e.raw_os_error() == Some(5) => {}
            Err(e) => return Err(e.into()),
        }
        if let Some(code) = child.exit_code()? {
            ensure!(sent && code == 0, "attach exited before successful detach");
            child.finish()?;
            return Ok(());
        }
        if !sent && read_bytes > 0 && started.elapsed() >= Duration::from_secs(2) {
            child.check_running()?;
            if loss {
                child.finish()?;
                return Ok(());
            }
            if controls {
                tcsetwinsize(
                    &master,
                    Winsize {
                        ws_row: 37,
                        ws_col: 111,
                        ws_xpixel: 0,
                        ws_ypixel: 0,
                    },
                )?;
                child.notify_resize()?;
                master.write_all(&[3])?;
                std::thread::sleep(Duration::from_millis(100));
            }
            master.write_all(&[0x1d, b'd'])?;
            sent = true;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// Fixed case inventory shared by both supported agents.
pub fn inventory(agent: &str) -> Result<&'static [&'static str]> {
    ensure!(
        matches!(agent, "codex" | "claude-code"),
        "unsupported agent"
    );
    Ok(&INVENTORY)
}
/// Pure generation gate used before accepting an operator assertion.
pub fn attest(checkpoint: &mut Checkpoint, run: &str, exec: &str, cases: &[&str]) -> Result<()> {
    ensure!(
        checkpoint.run_id == run
            && checkpoint.phase == "manual"
            && checkpoint
                .session
                .as_ref()
                .and_then(|s| s.exec_id.as_deref())
                == Some(exec),
        "stale attestation generation"
    );
    ensure!(
        cases
            .iter()
            .all(|case| matches!(*case, "login" | "model-tool" | "visual")),
        "unsupported attestation"
    );
    for case in cases {
        checkpoint.observe(case, "passed", "operator");
    }
    Ok(())
}
/// Receipt ownership is established only by the successful create response, never name lookup.
pub fn validate_receipt(
    receipt: &CreationReceipt,
    selection: &SelectionBinding,
    name: &str,
    agent: &str,
) -> Result<()> {
    let mut expected = selection.clone();
    expected.profile = None;
    ensure!(
        receipt.format == crate::RECEIPT_FORMAT
            && receipt.selection == expected
            && receipt.session.name == name
            && receipt.session.agent == agent
            && receipt.session.recorded_state == "RUNNING"
            && receipt.session.workspace_id.is_some()
            && receipt.session.exec_id.is_some()
            && !receipt.session.id.is_empty(),
        "creation receipt mismatch"
    );
    Ok(())
}
/// Production subprocess boundary, also exercised by native ESS without any remote resource.
pub fn invoke(command: &mut Command, timeout: Duration) -> Result<std::process::Output> {
    mantle_worker::run_bounded(command, None, timeout, MAX_METADATA)
}
