use serde_json::{Value, json};
use std::{fs, path::Path, process::Command, time::Duration};
fn bounded(command: &mut Command) -> std::process::Output {
    mantle_worker::run_bounded(command, None, Duration::from_secs(20), 65536).unwrap()
}
fn response(root: &Path, index: usize, mut result: Value) {
    if result.get("absent").is_some() {
        result["observed_at"] = json!("2026-10-03T00:00:00Z");
    }
    let body = json!({"api_version":"v1","request_id":"fixture","result":result}).to_string();
    fs::write(root.join(format!("response-{index}")),format!("HTTP/1.1 200 OK\r\nx-b10x-contract: {}\r\nx-b10x-contract-bundle-sha256: {}\r\ncontent-length: {}\r\nconnection: close\r\ncontent-type: application/json\r\n\r\n{}",b10x_substrate_sdk::CONTRACT,b10x_substrate_sdk::CONTRACT_SHA256,body.len(),body)).unwrap();
}
fn machine() -> Value {
    json!({"snapshot":format!("sha256:{}","7".repeat(64)),"driver":"host","driver_version":"fixture","config_generation":1,"probed_at":"2026-10-03T00:00:00Z","facts":{"operation.ledger-subject-max-rows":1000,"operation.ledger-subject-max-bytes":1048576,"operation.ledger-global-max-rows":10000,"operation.ledger-global-max-bytes":10485760}})
}
fn workspace() -> Value {
    json!({"id":"ws","kind":"workspace","labels":{},"observed_at":"2026-10-03T00:00:00Z","state":"ready"})
}

#[test]
fn actual_cli_stop_retains_files_and_destroy_requires_explicit_confirmation() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let compiled = bounded(
        Command::new("rustc")
            .args([
                "--edition=2024",
                concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/tests/support/lifecycle_ssh.rs"
                ),
                "-o",
            ])
            .arg(root.join("ssh")),
    );
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let config = root.join("config.toml");
    fs::write(&config,"provider='kubevirt'\nubuntu_serial='20260926'\n[kubevirt]\ncontext='fixture'\nnamespace='fixture'\ncpu=2\nmemory_gib=4\nroot_disk_gib=8\ndata_disk_gib=8\n").unwrap();
    let state = root.join("state");
    let executable = std::env::var_os("MANTLE_LIFECYCLE_BASE_CLI")
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_mantle").into());
    let cli = || {
        let mut c = Command::new(&executable);
        c.env_clear()
            .env("HOME", root)
            .env("PATH", root)
            .env("MANTLE_CONFIG", &config)
            .env("MANTLE_STATE_DIR", &state);
        c
    };
    assert!(bounded(cli().arg("list")).status.success());
    fs::write(state.join("id_ed25519"), b"synthetic fixture key").unwrap();
    let db = rusqlite::Connection::open(state.join("state.db")).unwrap();
    db.execute_batch("INSERT INTO workers VALUES('default','fixture','kubevirt/fixture/fixture',NULL); INSERT INTO sessions VALUES('session-current','fixture','default','RUNNING','digest','ws','agent','{}','time',NULL,'codex','chatgpt-device');").unwrap();
    let ws = root.join("workspace");
    fs::create_dir_all(ws.join(".mantle/home/.codex")).unwrap();
    fs::write(ws.join("marker"), b"user edits").unwrap();
    fs::write(
        ws.join(".mantle/home/.codex/auth.json"),
        b"synthetic private bytes",
    )
    .unwrap();
    let stop = root.join("stop");
    fs::create_dir(&stop).unwrap();
    response(&stop, 0, machine());
    response(
        &stop,
        1,
        json!({"id":"agent","kind":"exec","workspace":"ws","state":"exited","observed_at":"2026-10-03T00:00:00Z","requested":{"capability_snapshot":format!("sha256:{}","7".repeat(64)),"network":"none","profile":"workspace","require":true},"exit":{"code":0}}),
    );
    response(&stop, 2, json!({"id":"agent","kind":"exec","absent":true}));
    response(&stop, 3, workspace());
    // The baseline's old stop really reaches workspace deletion; no synthetic CLI switch.
    response(
        &stop,
        4,
        json!({"id":"ws","kind":"workspace","absent":true}),
    );
    let output = bounded(
        cli()
            .args(["stop", "fixture"])
            .env("MANTLE_LIFECYCLE_FIXTURE", &stop)
            .env("MANTLE_LIFECYCLE_WORKSPACE", &ws),
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        ws.join("marker").exists(),
        "stop deleted the retained user workspace"
    );
    assert_eq!(
        fs::read(ws.join(".mantle/home/.codex/auth.json")).unwrap(),
        b"synthetic private bytes"
    );
    assert_eq!(
        db.query_row(
            "SELECT state FROM sessions WHERE id='session-current'",
            [],
            |r| r.get::<_, String>(0)
        )
        .unwrap(),
        "RETAINED"
    );
    for args in [
        vec!["destroy", "fixture"],
        vec![
            "destroy",
            "fixture",
            "--yes",
            "--expected-session-id",
            "prior-session",
        ],
        vec!["stop", "fixture", "--expected-session-id", "prior-session"],
        vec![
            "restart",
            "fixture",
            "--expected-session-id",
            "prior-session",
        ],
        vec!["attach", "fixture"],
    ] {
        let output = bounded(cli().args(args));
        assert!(!output.status.success());
        assert!(ws.join("marker").exists());
    }
    let destroy = root.join("destroy");
    fs::create_dir(&destroy).unwrap();
    response(&destroy, 0, machine());
    response(&destroy, 1, workspace());
    response(
        &destroy,
        2,
        json!({"id":"ws","kind":"workspace","absent":true}),
    );
    let output = bounded(
        cli()
            .args([
                "destroy",
                "fixture",
                "--yes",
                "--expected-session-id",
                "session-current",
            ])
            .env("MANTLE_LIFECYCLE_FIXTURE", &destroy)
            .env("MANTLE_LIFECYCLE_WORKSPACE", &ws),
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!ws.exists());
    assert!(
        bounded(cli().args([
            "destroy",
            "fixture",
            "--yes",
            "--expected-session-id",
            "session-current"
        ]))
        .status
        .success()
    );
}

struct AdversaryLifecycle {
    root: tempfile::TempDir,
    db: rusqlite::Connection,
}
impl AdversaryLifecycle {
    fn new(state: &str) -> Self {
        let root = tempfile::tempdir().unwrap();
        let compiled = bounded(
            Command::new("rustc")
                .args([
                    "--edition=2024",
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/support/lifecycle_ssh.rs"
                    ),
                    "-o",
                ])
                .arg(root.path().join("ssh")),
        );
        assert!(
            compiled.status.success(),
            "{}",
            String::from_utf8_lossy(&compiled.stderr)
        );
        fs::write(root.path().join("config.toml"), "provider='kubevirt'\nubuntu_serial='20260926'\n[kubevirt]\ncontext='fixture'\nnamespace='fixture'\ncpu=2\nmemory_gib=4\nroot_disk_gib=8\ndata_disk_gib=8\n").unwrap();
        let cli = |root: &Path| {
            let mut command = Command::new(env!("CARGO_BIN_EXE_mantle"));
            command
                .env_clear()
                .env("HOME", root)
                .env("PATH", root)
                .env("MANTLE_CONFIG", root.join("config.toml"))
                .env("MANTLE_STATE_DIR", root.join("state"));
            command
        };
        assert!(bounded(cli(root.path()).arg("list")).status.success());
        fs::write(
            root.path().join("state/id_ed25519"),
            b"synthetic fixture key",
        )
        .unwrap();
        let db = rusqlite::Connection::open(root.path().join("state/state.db")).unwrap();
        db.execute_batch(
            "INSERT INTO workers VALUES('default','fixture','kubevirt/fixture/fixture',NULL);",
        )
        .unwrap();
        db.execute("INSERT INTO sessions VALUES('session-current','fixture','default',?1,'digest','ws',?2,'{}','time',NULL,'codex','chatgpt-device')", rusqlite::params![state, if state == "RUNNING" {Some("agent")} else {None}]).unwrap();
        // A version-1 launch context with synthetic argv/env values for exact wire comparison.
        let context = json!({"version":1,"worker":{"name":"default","instance":"fixture","region":"kubevirt/fixture/fixture","data_volume":null},"cpu_time_secs":86400,"output_bytes":1048576,
            "request":{"argv":["/opt/mantle/bin/mantle-launch","serve","--dir","/workspace/.mantle/agent","--cwd","/workspace/repo","--","/opt/mantle/bin/codex"],"environment":{"HOME":"/workspace/.mantle/home","CODEX_HOME":"/workspace/.mantle/home/.codex","SAVED_VALUE":"unchanged"},"aperture":null,"root":"/opt/mantle","secret_slot":null,"secret_fd":null,"timeout_secs":3600,"memory_bytes":1073741824,"processes":64,"lease_secs":3600}});
        db.execute("INSERT INTO session_lifecycle VALUES('session-current',1,'idle','prior-operation',?1,NULL,NULL)",[context.to_string()]).unwrap();
        fs::create_dir_all(root.path().join("workspace/.mantle/home/.codex")).unwrap();
        fs::write(root.path().join("workspace/marker"), b"user edits").unwrap();
        fs::write(
            root.path().join("workspace/.mantle/home/.codex/auth.json"),
            b"synthetic private bytes",
        )
        .unwrap();
        Self { root, db }
    }
    fn plane(&self, name: &str) -> std::path::PathBuf {
        let path = self.root.path().join(name);
        fs::create_dir(&path).unwrap();
        response(&path, 0, machine());
        path
    }
    fn cli(&self, plane: &Path, action: &str) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mantle"));
        c.env_clear()
            .env("HOME", self.root.path())
            .env("PATH", self.root.path())
            .env("MANTLE_CONFIG", self.root.path().join("config.toml"))
            .env("MANTLE_STATE_DIR", self.root.path().join("state"))
            .env("MANTLE_LIFECYCLE_FIXTURE", plane)
            .env(
                "MANTLE_LIFECYCLE_WORKSPACE",
                self.root.path().join("workspace"),
            )
            .args([
                action,
                "fixture",
                "--expected-session-id",
                "session-current",
            ]);
        c
    }
    fn observed(&self) -> (String, Option<String>) {
        self.db
            .query_row(
                "SELECT state,agent_exec FROM sessions WHERE id='session-current'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap()
    }
    fn assert_files(&self) {
        assert_eq!(
            fs::read(self.root.path().join("workspace/marker")).unwrap(),
            b"user edits"
        );
        assert_eq!(
            fs::read(
                self.root
                    .path()
                    .join("workspace/.mantle/home/.codex/auth.json")
            )
            .unwrap(),
            b"synthetic private bytes"
        );
    }
}
fn observed_exec(id: &str, state: &str) -> Value {
    let mut value = json!({"id":id,"kind":"exec","workspace":"ws","state":state,"observed_at":"2026-10-03T00:00:00Z","requested":{"capability_snapshot":format!("sha256:{}","7".repeat(64)),"network":"none","profile":"workspace","require":true}});
    if state == "exited" {
        value["exit"] = json!({"code":0});
    }
    value
}
fn restart_responses(path: &Path) {
    response(path, 1, workspace());
    for index in 2..=4 {
        response(path, index, observed_exec("agent-new", "running"));
    }
}

#[test]
fn adversary_cli_recovers_recorded_restart_without_second_admission() {
    let f = AdversaryLifecycle::new("RETAINED");
    let first = f.plane("first");
    restart_responses(&first);
    // A malformed later observation fails readiness after the start response was persisted.
    response(&first, 4, json!({}));
    let failed = bounded(&mut f.cli(&first, "restart"));
    assert!(
        !failed.status.success(),
        "{}",
        String::from_utf8_lossy(&failed.stdout)
    );
    assert_eq!(
        f.observed(),
        ("RESTARTING".into(), Some("agent-new".into()))
    );
    let operation: String =
        f.db.query_row(
            "SELECT operation_id FROM session_lifecycle WHERE session_id='session-current'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let request = fs::read_to_string(first.join("request-2")).unwrap();
    assert!(request.starts_with("POST /v1/execs "));
    assert!(
        request.contains(&operation),
        "start did not use persisted operation ID"
    );
    assert!(request.contains("SAVED_VALUE"));
    let retry = f.plane("retry");
    response(&retry, 1, workspace());
    response(
        &retry,
        2,
        json!({"operation":operation,"operation_kind":"exec.start","request_hash":"1".repeat(64),"state":"terminal","accepted_at":"2026-10-03T00:00:00Z","terminal_at":"2026-10-03T00:00:00Z","capability_snapshot":format!("sha256:{}","7".repeat(64)),"actor":"fixture","principal":null,"resource":"agent-new","outcome":{"kind":"success","result":observed_exec("agent-new","running")}}),
    );
    response(&retry, 3, observed_exec("agent-new", "running"));
    response(&retry, 4, observed_exec("agent-new", "running"));
    let recovered = bounded(&mut f.cli(&retry, "restart"));
    assert!(
        recovered.status.success(),
        "{}",
        String::from_utf8_lossy(&recovered.stderr)
    );
    assert_eq!(f.observed(), ("RUNNING".into(), Some("agent-new".into())));
    for index in 0..=4 {
        let request = fs::read_to_string(retry.join(format!("request-{index}"))).unwrap();
        assert!(request.starts_with("GET "), "retry resubmitted: {request}");
    }
    assert!(
        fs::read_to_string(retry.join("request-2"))
            .unwrap()
            .contains(&format!("/v1/ops/{operation} "))
    );
    f.assert_files();
}

#[test]
fn adversary_delayed_stop_cannot_signal_after_another_stop_and_restart() {
    let f = AdversaryLifecycle::new("RUNNING");
    let delayed = f.plane("delayed");
    response(&delayed, 1, observed_exec("agent", "running"));
    fs::write(delayed.join("pause-1"), b"").unwrap();
    let mut stale_command = f.cli(&delayed, "stop");
    std::thread::scope(|scope| {
        let stale = scope.spawn(move || bounded(&mut stale_command));
        let deadline = std::time::Instant::now() + Duration::from_secs(8);
        while !delayed.join("request-1").exists() {
            assert!(
                std::time::Instant::now() < deadline,
                "older stop never reached exec observation"
            );
            std::thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(f.observed().0, "RETAINING");
        let winner = f.plane("winner");
        response(&winner, 1, observed_exec("agent", "exited"));
        response(
            &winner,
            2,
            json!({"id":"agent","kind":"exec","absent":true}),
        );
        response(&winner, 3, workspace());
        let stopped = bounded(&mut f.cli(&winner, "stop"));
        assert!(
            stopped.status.success(),
            "{}",
            String::from_utf8_lossy(&stopped.stderr)
        );
        assert_eq!(f.observed(), ("RETAINED".into(), None));
        let restarted = f.plane("restarted");
        restart_responses(&restarted);
        let output = bounded(&mut f.cli(&restarted, "restart"));
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(f.observed(), ("RUNNING".into(), Some("agent-new".into())));
        fs::write(delayed.join("release-1"), b"").unwrap();
        let stale = stale.join().unwrap();
        assert!(!stale.status.success(), "old stop was reported successful");
        assert!(String::from_utf8_lossy(&stale.stderr).contains("stale lifecycle attempt"));
        assert!(
            !delayed.join("request-2").exists(),
            "stale caller made another remote call"
        );
        assert_eq!(f.observed(), ("RUNNING".into(), Some("agent-new".into())));
        f.assert_files();
    });
}
