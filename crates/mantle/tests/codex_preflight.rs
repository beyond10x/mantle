use std::fs;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[test]
fn codex_refuses_capture_before_claude_credentials_or_session_insertion() {
    let dir = tempfile::tempdir().unwrap();
    let config = dir.path().join("config.toml");
    fs::write(&config, "provider = 'kubevirt'\nubuntu_serial = '20260926'\n[kubevirt]\ncontext = 'not-contacted'\nnamespace = 'not-contacted'\ncpu = 4\nmemory_gib = 8\nroot_disk_gib = 20\ndata_disk_gib = 20\n").unwrap();
    let manifest = dir.path().join("session.yaml");
    fs::write(&manifest, include_str!("../../../examples/codex.yaml")).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_mantle"))
        .env_clear()
        .env("MANTLE_CONFIG", config)
        .env("MANTLE_STATE_DIR", dir.path().join("state"))
        .env("HOME", dir.path())
        .args(["start", "--detached"])
        .arg(manifest)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let until = Instant::now() + Duration::from_secs(5);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() >= until {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("preflight contacted or waited on an external service");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let output = child.wait_with_output().unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!output.status.success());
    assert!(
        stderr.contains("Codex requires supported non-recording terminal capture"),
        "{stderr}"
    );
    assert!(!stderr.contains("[claude]"), "{stderr}");
    let db = rusqlite::Connection::open(dir.path().join("state/state.db")).unwrap();
    let rows: i64 = db
        .query_row("SELECT count(*) FROM sessions", [], |row| row.get(0))
        .unwrap();
    assert_eq!(rows, 0);
    // A stored Codex session cannot fall back to the recorded Claude attach route, even
    // after the manifest has changed and no worker/Claude configuration exists.
    db.execute_batch("INSERT INTO sessions(id,name,worker,state,manifest_digest,workspace,agent_exec,requested_json,created_at,failure,agent_kind,authentication) VALUES ('stored','stored-codex','missing','RUNNING','digest','ws','exec','{}','time',NULL,'codex','chatgpt-device');").unwrap();
    fs::write(
        dir.path().join("session.yaml"),
        "agent: {kind: claude-code}\n",
    )
    .unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_mantle"))
        .env_clear()
        .env("MANTLE_CONFIG", dir.path().join("config.toml"))
        .env("MANTLE_STATE_DIR", dir.path().join("state"))
        .env("HOME", dir.path())
        .args(["attach", "stored-codex"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let until = Instant::now() + Duration::from_secs(5);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() >= until {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("stored attach contacted an external service");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let output = child.wait_with_output().unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("Codex requires supported non-recording terminal capture")
    );
    let observed: (i64, String, String) = db
        .query_row(
            "SELECT count(*), workspace, agent_exec FROM sessions",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .unwrap();
    assert_eq!(observed, (1, "ws".into(), "exec".into()));
}

fn adversary_fixture() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir(dir.path().join("state")).unwrap();
    fs::write(
        dir.path().join("config.toml"),
        "provider = 'kubevirt'\nubuntu_serial = '20260926'\n[kubevirt]\ncontext = 'not-contacted'\nnamespace = 'not-contacted'\ncpu = 4\nmemory_gib = 8\nroot_disk_gib = 20\ndata_disk_gib = 20\n",
    )
    .unwrap();
    dir
}

fn adversary_cli(dir: &std::path::Path, args: &[&str]) -> std::process::Output {
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
            panic!("local identity operation exceeded its deadline");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    child.wait_with_output().unwrap()
}

const ADVERSARY_LEGACY: &str = "
CREATE TABLE sessions (
 id TEXT PRIMARY KEY, name TEXT NOT NULL, worker TEXT NOT NULL, state TEXT NOT NULL,
 manifest_digest TEXT NOT NULL, workspace TEXT, agent_exec TEXT, requested_json TEXT NOT NULL,
 created_at TEXT NOT NULL, failure TEXT);
CREATE UNIQUE INDEX sessions_live_name ON sessions(name) WHERE state <> 'STOPPED';
CREATE INDEX operator_manifest_index ON sessions(manifest_digest);
INSERT INTO sessions VALUES ('old','reused','w','STOPPED','digest-old',NULL,NULL,'{\"old\":true}','old-time','retained failure');
INSERT INTO sessions VALUES ('live','reused','w','RUNNING','digest-live','ws','exec','{\"cpu\":3}','new-time',NULL);
CREATE TABLE sources (session_id TEXT NOT NULL, name TEXT NOT NULL, repository TEXT NOT NULL,
 declared_ref TEXT NOT NULL, commit_id TEXT NOT NULL, mount TEXT NOT NULL, PRIMARY KEY(session_id,mount));
INSERT INTO sources VALUES ('old','old-source','https://example.com/old.git','old-ref','old-commit','old-mount');
";

#[test]
fn adversary_legacy_migration_preserves_indexes_and_stopped_sources_then_refuses_corruption() {
    let dir = adversary_fixture();
    let db = rusqlite::Connection::open(dir.path().join("state/state.db")).unwrap();
    db.execute_batch(ADVERSARY_LEGACY).unwrap();
    for _ in 0..3 {
        let output = adversary_cli(dir.path(), &["list"]);
        assert!(output.status.success(), "{:?}", output);
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(text.contains("claude-code") && text.contains("claude-oauth"));
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
                ("live".into(), "claude-code".into(), "claude-oauth".into()),
                ("old".into(), "claude-code".into(), "claude-oauth".into())
            ]
        );
        let preserved: (String, String, String) = db
            .query_row(
                "SELECT s.requested_json,s.failure,r.commit_id FROM sessions s JOIN sources r ON r.session_id=s.id WHERE s.id='old'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap();
        assert_eq!(
            preserved,
            (
                "{\"old\":true}".into(),
                "retained failure".into(),
                "old-commit".into()
            )
        );
        let indexes: i64 = db.query_row("SELECT count(*) FROM sqlite_master WHERE type='index' AND name IN ('operator_manifest_index','sessions_live_name')", [], |r| r.get(0)).unwrap();
        assert_eq!(indexes, 2);
    }
    db.execute(
        "UPDATE sessions SET agent_kind='codex',authentication='claude-oauth' WHERE id='old'",
        [],
    )
    .unwrap();
    let output = adversary_cli(dir.path(), &["list"]);
    assert!(!output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("invalid session identity")
    );
    assert!(
        output.stdout.is_empty(),
        "corrupt stopped identity was silently skipped"
    );
}

#[test]
fn adversary_partial_identity_schema_refusal_rolls_back_initialization() {
    let dir = adversary_fixture();
    let db = rusqlite::Connection::open(dir.path().join("state/state.db")).unwrap();
    db.execute_batch(ADVERSARY_LEGACY).unwrap();
    db.execute_batch(
        "ALTER TABLE sessions ADD COLUMN agent_kind TEXT NOT NULL DEFAULT 'claude-code';",
    )
    .unwrap();
    let schema = || {
        db.prepare("SELECT sql FROM sqlite_master ORDER BY type,name")
            .unwrap()
            .query_map([], |r| r.get::<_, Option<String>>(0))
            .unwrap()
            .collect::<rusqlite::Result<Vec<_>>>()
            .unwrap()
    };
    let before = schema();
    for _ in 0..2 {
        let output = adversary_cli(dir.path(), &["list"]);
        assert!(!output.status.success());
        assert!(
            String::from_utf8(output.stderr)
                .unwrap()
                .contains("incomplete session identity schema")
        );
        assert_eq!(
            schema(),
            before,
            "refused initialization committed partial DDL"
        );
    }
    let rows: i64 = db
        .query_row("SELECT count(*) FROM sessions", [], |r| r.get(0))
        .unwrap();
    assert_eq!(rows, 2);
}

#[test]
fn adversary_codex_never_invokes_configured_claude_credential_command() {
    use std::io::Write;
    let dir = adversary_fixture();
    let marker = dir.path().join("credential-command-ran");
    writeln!(
        fs::OpenOptions::new()
            .append(true)
            .open(dir.path().join("config.toml"))
            .unwrap(),
        "\n[claude]\ntoken_command = [\"/usr/bin/touch\", {:?}]",
        marker.to_str().unwrap()
    )
    .unwrap();
    let manifest = dir.path().join("session.yaml");
    let codex = include_str!("../../../examples/codex.yaml");
    fs::write(&manifest, codex).unwrap();
    let output = adversary_cli(
        dir.path(),
        &["start", "--detached", manifest.to_str().unwrap()],
    );
    assert!(!output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("Codex requires supported non-recording terminal capture")
    );
    assert!(!marker.exists());
    fs::write(
        &manifest,
        codex
            .replace("kind: codex", "kind: claude-code")
            .replace("auth: chatgpt-device", "auth: claude-oauth"),
    )
    .unwrap();
    let output = adversary_cli(
        dir.path(),
        &["start", "--detached", manifest.to_str().unwrap()],
    );
    assert!(!output.status.success(), "empty credential must refuse");
    assert!(
        marker.exists(),
        "positive control never invoked Claude credential source"
    );
    let db = rusqlite::Connection::open(dir.path().join("state/state.db")).unwrap();
    let rows: i64 = db
        .query_row("SELECT count(*) FROM sessions", [], |r| r.get(0))
        .unwrap();
    assert_eq!(rows, 0);
}
