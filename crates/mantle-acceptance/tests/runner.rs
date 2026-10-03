use serde_json::Value;
use std::{fs, path::Path, process::Command, sync::OnceLock, time::Duration};
fn fixture() -> &'static (tempfile::TempDir, std::path::PathBuf) {
    static FIXTURE: OnceLock<(tempfile::TempDir, std::path::PathBuf)> = OnceLock::new();
    FIXTURE.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fixture");
        let out = Command::new("rustc")
            .args([
                "--edition=2024",
                concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/cli.rs"),
                "-o",
            ])
            .arg(&path)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        (dir, path)
    })
}
struct Fixture {
    root: tempfile::TempDir,
}
impl Fixture {
    fn new() -> Self {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("config.toml"), "synthetic config").unwrap();
        fs::write(
            root.path().join("manifest.yaml"),
            "metadata:\n  name: placeholder\nagent:\n  kind: codex\n",
        )
        .unwrap();
        Self { root }
    }
    fn command(&self) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mantle-acceptance"));
        c.env("ACCEPTANCE_FIXTURE_ROOT", self.root.path()).env(
            "ACCEPTANCE_CONFIG_SHA",
            mantle_artifact::sha256(b"synthetic config"),
        );
        c
    }
    fn run(&self, mode: &str, agent: &str) -> std::process::Output {
        bounded(
            self.command()
                .env("ACCEPTANCE_FIXTURE_MODE", mode)
                .args(["run", "--real", "--mantle"])
                .arg(&fixture().1)
                .args(["--profile", "fixture", "--agent", agent, "--manifest"])
                .arg(self.root.path().join("manifest.yaml"))
                .arg("--checkpoint")
                .arg(self.path()),
        )
    }
    fn path(&self) -> std::path::PathBuf {
        self.root.path().join("checkpoint.json")
    }
    fn read(&self) -> Value {
        serde_json::from_slice(&fs::read(self.path()).unwrap()).unwrap()
    }
    fn resume(&self, flags: &[&str]) -> Command {
        let data = self.read();
        let mut c = self.command();
        c.args(["resume", "--real", "--checkpoint"])
            .arg(self.path())
            .args([
                "--expected-run-id",
                data["run_id"].as_str().unwrap(),
                "--expected-exec-id",
                "exec1",
            ])
            .args(flags);
        c
    }
}
fn bounded(c: &mut Command) -> std::process::Output {
    mantle_worker::run_bounded(c, None, Duration::from_secs(40), 1024 * 1024).unwrap()
}
#[test]
fn both_agents_use_actual_cli_pty_and_same_inventory_without_raw_reports() {
    for agent in ["codex", "claude-code"] {
        let f = Fixture::new();
        let out = f.run("", agent);
        assert_eq!(
            out.status.code(),
            Some(2),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(f.read()["results"].as_array().unwrap().len(), 10);
        let out = bounded(&mut f.resume(&[]));
        assert_eq!(out.status.code(), Some(2));
        let out = bounded(&mut f.resume(&[
            "--attest",
            "login",
            "--attest",
            "model-tool",
            "--attest",
            "visual",
        ]));
        assert_eq!(
            out.status.code(),
            Some(0),
            "{}\n{}",
            String::from_utf8_lossy(&out.stderr),
            f.read()
        );
        let text = fs::read_to_string(f.path()).unwrap();
        assert!(!text.contains("synthetic raw"));
        assert!(!text.contains("startup text"));
        let data = f.read();
        assert!(data["source_commit"].is_null());
        assert_eq!(data["session"]["exec_id"], "exec2");
        assert_eq!(
            data["results"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|r| r["origin"] == "operator")
                .count(),
            3
        );
        assert!(
            fs::read_to_string(f.root.path().join("calls"))
                .unwrap()
                .contains("destroy acceptance-")
        );
        assert_eq!(
            bounded(f.command().arg("report").arg("--checkpoint").arg(f.path()))
                .status
                .code(),
            Some(0)
        );
    }
}
#[test]
fn lost_receipt_never_authorizes_cleanup_and_name_reuse_refuses() {
    let f = Fixture::new();
    assert_eq!(f.run("lost-receipt", "codex").status.code(), Some(1));
    assert!(f.read()["session"].is_null());
    let before = fs::read(f.root.path().join("calls")).unwrap();
    let out = bounded(
        f.command()
            .args(["cleanup", "--real", "--checkpoint"])
            .arg(f.path()),
    );
    assert_eq!(out.status.code(), Some(1));
    assert_eq!(fs::read(f.root.path().join("calls")).unwrap(), before);
    let f = Fixture::new();
    assert_eq!(f.run("", "codex").status.code(), Some(2));
    let out = bounded(
        f.command()
            .env("ACCEPTANCE_FIXTURE_MODE", "reuse")
            .args(["cleanup", "--real", "--checkpoint"])
            .arg(f.path()),
    );
    assert_eq!(out.status.code(), Some(1));
    assert!(
        fs::read_to_string(f.root.path().join("session"))
            .unwrap()
            .ends_with("RUNNING")
    );
}
#[test]
fn stale_attestation_selection_binary_and_public_checkpoint_refuse() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new();
    assert_eq!(f.run("", "codex").status.code(), Some(2));
    assert_eq!(
        bounded(
            f.command()
                .args(["resume", "--real", "--checkpoint"])
                .arg(f.path())
                .args([
                    "--expected-run-id",
                    "wrong",
                    "--expected-exec-id",
                    "exec1",
                    "--attest",
                    "login"
                ])
        )
        .status
        .code(),
        Some(1)
    );
    assert_eq!(
        bounded(f.resume(&[]).env("ACCEPTANCE_CONFIG_SHA", "changed"))
            .status
            .code(),
        Some(1)
    );
    let data = f.read();
    fs::write(
        Path::new(data["run_dir"].as_str().unwrap()).join("mantle"),
        "changed",
    )
    .unwrap();
    assert_eq!(bounded(&mut f.resume(&[])).status.code(), Some(1));
    fs::set_permissions(f.path(), fs::Permissions::from_mode(0o644)).unwrap();
    assert_eq!(
        bounded(f.command().args(["report", "--checkpoint"]).arg(f.path()))
            .status
            .code(),
        Some(1)
    );
}
#[test]
fn output_flood_fails_and_manual_abort_does_not_fabricate_completion() {
    let f = Fixture::new();
    assert_eq!(f.run("", "codex").status.code(), Some(2));
    let out = bounded(
        f.resume(&[
            "--attest",
            "login",
            "--attest",
            "model-tool",
            "--attest",
            "visual",
        ])
        .env("ACCEPTANCE_FIXTURE_MODE", "flood"),
    );
    assert_eq!(out.status.code(), Some(1));
    assert!(
        f.read()["results"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["status"] == "failed")
    );
    let f = Fixture::new();
    assert_eq!(f.run("", "codex").status.code(), Some(2));
    assert_eq!(
        bounded(
            f.command()
                .args(["cleanup", "--real", "--checkpoint"])
                .arg(f.path())
        )
        .status
        .code(),
        Some(2)
    );
    assert_eq!(f.read()["results"][1]["status"], "operator-required");
}

#[test]
fn pty_timeout_retires_descendants_and_interruption_retains_unfinished_evidence() {
    let f = Fixture::new();
    assert_eq!(f.run("", "codex").status.code(), Some(2));
    let out = bounded(
        f.resume(&[
            "--attest",
            "login",
            "--attest",
            "model-tool",
            "--attest",
            "visual",
        ])
        .env("ACCEPTANCE_FIXTURE_MODE", "pty-timeout"),
    );
    assert_eq!(out.status.code(), Some(1));
    let pid = fs::read_to_string(f.root.path().join("descendant-pid")).unwrap();
    if let Ok(stat) = fs::read_to_string(format!("/proc/{pid}/stat")) {
        assert_eq!(stat.split_whitespace().nth(2), Some("Z"));
    }
    assert!(
        f.read()["results"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["case"] == "detach-reconnect" && r["status"] == "failed")
    );
    let f = Fixture::new();
    assert_eq!(f.run("", "codex").status.code(), Some(2));
    let mut child = f
        .resume(&[
            "--attest",
            "login",
            "--attest",
            "model-tool",
            "--attest",
            "visual",
        ])
        .env("ACCEPTANCE_FIXTURE_MODE", "pty-timeout")
        .spawn()
        .unwrap();
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    while !f.root.path().join("descendant-pid").exists() {
        assert!(std::time::Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(10));
    }
    rustix::process::kill_process(
        rustix::process::Pid::from_raw(child.id() as i32).unwrap(),
        rustix::process::Signal::TERM,
    )
    .unwrap();
    assert!(!child.wait().unwrap().success());
    let data = f.read();
    assert_ne!(data["phase"], "complete");
    assert!(
        data["results"]
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["status"] != "passed")
    );
}

#[test]
fn unverified_release_and_replaced_selected_executable_refuse_before_mutation() {
    let f = Fixture::new();
    let out = bounded(
        f.command()
            .args(["run", "--real", "--mantle"])
            .arg(&fixture().1)
            .args(["--profile", "fixture", "--agent", "codex", "--manifest"])
            .arg(f.root.path().join("manifest.yaml"))
            .arg("--checkpoint")
            .arg(f.path())
            .arg("--release-manifest")
            .arg(f.root.path().join("missing-release.json")),
    );
    assert_eq!(out.status.code(), Some(1));
    assert!(!f.path().exists());
    assert!(!f.root.path().join("calls").exists());
    let selected = f.root.path().join("selected-cli");
    fs::copy(&fixture().1, &selected).unwrap();
    let out = bounded(
        f.command()
            .args(["run", "--real", "--mantle"])
            .arg(&selected)
            .args(["--profile", "fixture", "--agent", "codex", "--manifest"])
            .arg(f.root.path().join("manifest.yaml"))
            .arg("--checkpoint")
            .arg(f.path()),
    );
    assert_eq!(out.status.code(), Some(2));
    let before = fs::read(f.root.path().join("calls")).unwrap();
    fs::write(selected, b"changed executable").unwrap();
    assert_eq!(bounded(&mut f.resume(&[])).status.code(), Some(1));
    assert_eq!(fs::read(f.root.path().join("calls")).unwrap(), before);
}
