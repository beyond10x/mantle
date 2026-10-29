//! Adversary cases for `story:command-agent-kind`.
//!
//! Each case drives the shipped `mantle` binary offline: no worker is recorded, so a start that
//! passes manifest validation stops at "no worker recorded" and a start that is refused by the
//! manifest never gets that far.
use std::fs;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

const CONFIG: &str = "provider = 'kubevirt'\nubuntu_serial = '20260926'\n[kubevirt]\ncontext = 'not-contacted'\nnamespace = 'not-contacted'\ncpu = 4\nmemory_gib = 8\nroot_disk_gib = 20\ndata_disk_gib = 20\n";

fn fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("state")).unwrap();
    fs::write(dir.path().join("config.toml"), CONFIG).unwrap();
    dir
}

fn cli(dir: &Path, args: &[&str]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mantle"))
        .env_clear()
        .env("HOME", dir)
        .env("PATH", dir)
        .env("MANTLE_CONFIG", dir.join("config.toml"))
        .env("MANTLE_STATE_DIR", dir.join("state"))
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() >= deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("a local operation contacted or waited on an external service");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    child.wait_with_output().unwrap()
}

fn manifest(argv_yaml: &str) -> String {
    format!(
        "apiVersion: mantle.beyond10x.dev/v1alpha1\nkind: Session\nmetadata:\n  name: adv\nworkspace:\n  repositories:\n    - name: r\n      repository: https://example.com/r.git\n      ref: main\n      mount: r\nagent:\n  kind: command\n  argv: {argv_yaml}\n  cwd: /workspace/r\n"
    )
}

/// Substrate's exec admission (`validate_exec_input` in the pinned daemon, and the host driver's
/// `exec.argv-invalid`) refuses an argv with an empty element, a NUL, an element over 4096 bytes,
/// or more than 256 elements. Mantle prefixes 13 launcher elements to the declared argv. A
/// manifest Mantle accepts today therefore inserts a session, creates a workspace and clones every
/// repository before Substrate refuses the agent start, leaving a FAILED_AGENT_START session that
/// holds the name. The manifest refusal should come first, by name, before any of that.
#[test]
fn command_argv_that_substrate_must_refuse_is_refused_by_the_manifest_before_the_worker() {
    let long = format!("[\"/bin/echo\", \"{}\"]", "a".repeat(4097));
    let many = format!(
        "[\"/bin/echo\"{}]",
        ", \"x\"".repeat(256 - 13) // 1 + 243 declared + 13 launcher = 257 elements
    );
    let cases: [(&str, String); 4] = [
        (
            "empty later element",
            "[\"/bin/bash\", \"-c\", \"\"]".to_owned(),
        ),
        ("NUL in an element", "[\"/bin/bash\", \"a\\0b\"]".to_owned()),
        ("element over 4096 bytes", long),
        ("more elements than Substrate admits", many),
    ];
    let mut accepted = Vec::new();
    for (label, argv) in &cases {
        let dir = fixture();
        let path = dir.path().join("m.yaml");
        fs::write(&path, manifest(argv)).unwrap();
        let output = cli(dir.path(), &["start", "--detached", path.to_str().unwrap()]);
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        assert!(!output.status.success(), "{label}");
        if stderr.contains("no worker recorded") || !stderr.contains("agent.argv") {
            accepted.push(format!("{label}: {}", stderr.trim()));
        }
    }
    assert!(
        accepted.is_empty(),
        "manifest accepted an argv Substrate refuses at exec admission:\n{}",
        accepted.join("\n")
    );
}

/// The story's "What changes": `mantle status --json` reports the kind as `command` and the argv.
/// The JSON report is built from `app/metadata.rs`, whose session record carries the kind but no
/// argv, so the declared argv never reaches `status --json` or `list --json`.
#[test]
fn status_json_reports_the_command_kind_and_its_argv() {
    let dir = fixture();
    assert!(cli(dir.path(), &["list"]).status.success());
    let db = rusqlite::Connection::open(dir.path().join("state/state.db")).unwrap();
    db.execute_batch(
        "INSERT INTO sessions(id,name,worker,state,manifest_digest,workspace,agent_exec,requested_json,created_at,failure,agent_kind,authentication) VALUES ('shell','shell','default','STOPPED','digest','ws','exec','{\"agent_argv\":[\"/usr/bin/env\",\"adversary-marker\"]}','time',NULL,'command','none');",
    )
    .unwrap();
    for args in [
        vec!["status", "--session-id", "shell", "--json"],
        vec!["list", "--json"],
    ] {
        let output = cli(dir.path(), &args);
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        let session = &value["sessions"][0];
        assert_eq!(session["agent"], "command", "{args:?}: {value}");
        assert!(
            session.to_string().contains("adversary-marker"),
            "{args:?}: the declared argv is absent from the JSON session: {session}"
        );
    }
}
