//! A `command` session: the declared argv is the session's process and no agent credential is read.
use std::fs;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

const CONFIG: &str = "provider = 'kubevirt'\nubuntu_serial = '20260926'\n[kubevirt]\ncontext = 'not-contacted'\nnamespace = 'not-contacted'\ncpu = 4\nmemory_gib = 8\nroot_disk_gib = 20\ndata_disk_gib = 20\n";

const LEGACY: &str = "
CREATE TABLE sessions (
 id TEXT PRIMARY KEY, name TEXT NOT NULL, worker TEXT NOT NULL, state TEXT NOT NULL,
 manifest_digest TEXT NOT NULL, workspace TEXT, agent_exec TEXT, requested_json TEXT NOT NULL,
 created_at TEXT NOT NULL, failure TEXT);
CREATE UNIQUE INDEX sessions_live_name ON sessions(name) WHERE state <> 'STOPPED';
INSERT INTO sessions VALUES ('legacy','legacy','w','RUNNING','digest','ws','exec','{}','time',NULL);
";

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
        .env("MANTLE_CONFIG", dir.join("config.toml"))
        .env("MANTLE_STATE_DIR", dir.join("state"))
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
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

#[test]
fn command_start_reaches_the_worker_boundary_without_reading_a_claude_credential() {
    let dir = fixture();
    let manifest = dir.path().join("shell.yaml");
    fs::write(&manifest, include_str!("../../../examples/shell.yaml")).unwrap();
    let output = cli(
        dir.path(),
        &["start", "--detached", manifest.to_str().unwrap()],
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success());
    assert!(
        stderr.contains("no worker recorded; run `mantle worker up` first"),
        "{stderr}"
    );
    assert!(!stderr.contains("[claude]"), "{stderr}");
    let db = rusqlite::Connection::open(dir.path().join("state/state.db")).unwrap();
    let rows: i64 = db
        .query_row("SELECT count(*) FROM sessions", [], |row| row.get(0))
        .unwrap();
    assert_eq!(rows, 0);
}

#[test]
fn a_command_row_and_an_older_row_keep_their_kinds_across_reopen() {
    let dir = fixture();
    let db = rusqlite::Connection::open(dir.path().join("state/state.db")).unwrap();
    db.execute_batch(LEGACY).unwrap();
    let migrated = cli(dir.path(), &["list"]);
    assert!(migrated.status.success(), "{migrated:?}");
    db.execute_batch(
        "INSERT INTO sessions(id,name,worker,state,manifest_digest,workspace,agent_exec,requested_json,created_at,failure,agent_kind,authentication) VALUES ('shell','shell','w','RUNNING','digest','ws2','exec2','{\"agent_argv\":[\"/bin/bash\",\"-l\"]}','time',NULL,'command','none');",
    )
    .unwrap();
    for _ in 0..2 {
        let output = cli(dir.path(), &["list"]);
        assert!(output.status.success(), "{output:?}");
        let text = String::from_utf8(output.stdout).unwrap();
        let line = |name: &str| {
            text.lines()
                .find(|line| line.starts_with(name))
                .unwrap_or_else(|| panic!("{name} listed: {text}"))
                .split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        };
        assert_eq!(line("legacy")[2..4], ["claude-code", "claude-oauth"]);
        assert_eq!(line("shell")[2..4], ["command", "none"]);
        let identities: Vec<(String, String, String)> = db
            .prepare("SELECT id,agent_kind,authentication FROM sessions ORDER BY id")
            .unwrap()
            .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
            .unwrap()
            .collect::<rusqlite::Result<_>>()
            .unwrap();
        assert_eq!(
            identities,
            vec![
                ("legacy".into(), "claude-code".into(), "claude-oauth".into()),
                ("shell".into(), "command".into(), "none".into()),
            ]
        );
    }
}

#[test]
fn a_command_row_with_a_credential_or_an_agent_row_without_one_is_refused() {
    for (kind, auth) in [
        ("command", "claude-oauth"),
        ("command", "chatgpt-device"),
        ("claude-code", "none"),
        ("codex", "none"),
    ] {
        let dir = fixture();
        assert!(cli(dir.path(), &["list"]).status.success());
        let db = rusqlite::Connection::open(dir.path().join("state/state.db")).unwrap();
        db.execute(
            "INSERT INTO sessions(id,name,worker,state,manifest_digest,workspace,agent_exec,requested_json,created_at,failure,agent_kind,authentication) VALUES ('bad','bad','w','RUNNING','digest','ws','exec','{}','time',NULL,?1,?2)",
            [kind, auth],
        )
        .unwrap();
        let output = cli(dir.path(), &["list"]);
        assert!(!output.status.success(), "{kind}/{auth} accepted");
        assert!(
            String::from_utf8(output.stderr)
                .unwrap()
                .contains("invalid session identity"),
            "{kind}/{auth}"
        );
        assert!(output.stdout.is_empty(), "{kind}/{auth}");
    }
}
