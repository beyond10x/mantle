use std::process::Command;

#[test]
fn json_selection_failure_is_structured_before_initialization() {
    let executable = std::env::var_os("MANTLE_METADATA_BASE_CLI")
        .or_else(|| option_env!("CARGO_BIN_EXE_mantle").map(Into::into))
        .expect("CLI executable");
    let output = Command::new(executable)
        .env_clear()
        .env("HOME", "/nonexistent/mantle-metadata-fixture")
        .args(["--profile", "missing", "list", "--json"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("\"format\":\"mantle-session-metadata/v1\""),
        "missing bounded JSON response: {stdout}"
    );
    assert!(stdout.contains("\"outcome\":\"selection-refused\""));
    assert!(output.stderr.is_empty(), "JSON mode leaked raw diagnostics");
}

#[test]
fn terminal_records_and_wal_rows_are_read_without_migration_or_raw_fields() {
    use std::fs;
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("config.toml"), "provider='kubevirt'\nubuntu_serial='20260926'\n[kubevirt]\ncontext='fixture'\nnamespace='fixture'\ncpu=2\nmemory_gib=4\nroot_disk_gib=8\ndata_disk_gib=8\n").unwrap();
    let cli = || {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mantle"));
        c.env_clear()
            .env("HOME", root.path())
            .env("PATH", root.path())
            .env("MANTLE_CONFIG", root.path().join("config.toml"))
            .env("MANTLE_STATE_DIR", root.path().join("state"));
        c
    };
    assert!(cli().arg("list").output().unwrap().status.success());
    let db = rusqlite::Connection::open(root.path().join("state/state.db")).unwrap();
    db.execute_batch("PRAGMA journal_mode=WAL; INSERT INTO sessions VALUES('ended','fixture','default','STOPPED','digest','ws','exec','secret-request','time','secret-refusal','codex','chatgpt-device');").unwrap();
    let schema: String = db
        .query_row("SELECT group_concat(sql) FROM sqlite_master", [], |r| {
            r.get(0)
        })
        .unwrap();
    for args in [
        vec!["list", "--json"],
        vec!["status", "--session-id", "ended", "--json"],
    ] {
        let output = cli().args(args).output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["sessions"][0]["id"], "ended");
        assert_eq!(value["sessions"][0]["recorded_state"], "STOPPED");
        assert!(value["sessions"][0]["observed"].is_null());
        assert!(!String::from_utf8_lossy(&output.stdout).contains("secret-"));
        assert!(output.stderr.is_empty());
    }
    assert_eq!(
        db.query_row("SELECT group_concat(sql) FROM sqlite_master", [], |r| {
            r.get::<_, String>(0)
        })
        .unwrap(),
        schema
    );
    assert_eq!(
        db.query_row("SELECT requested_json FROM sessions", [], |r| r
            .get::<_, String>(0))
            .unwrap(),
        "secret-request"
    );
    for verb in ["attach", "exec"] {
        let mut command = cli();
        command.args([verb, "fixture", "--expected-session-id", "different"]);
        if verb == "exec" {
            command.args(["--", "true"]);
        }
        assert!(!command.output().unwrap().status.success());
    }
}

#[test]
fn json_status_uses_only_metadata_wire_and_preserves_record_on_unavailable_observation() {
    use serde_json::json;
    use std::{fs, time::Duration};
    let root = tempfile::tempdir().unwrap();
    let build = Command::new("rustc")
        .args([
            "--edition=2024",
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/metadata_ssh.rs"),
            "-o",
        ])
        .arg(root.path().join("ssh"))
        .output()
        .unwrap();
    assert!(build.status.success());
    fs::write(root.path().join("config.toml"),"provider='kubevirt'\nubuntu_serial='20260926'\n[kubevirt]\ncontext='fixture'\nnamespace='fixture'\ncpu=2\nmemory_gib=4\nroot_disk_gib=8\ndata_disk_gib=8\n").unwrap();
    let cli = || {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mantle"));
        c.env_clear()
            .env("HOME", root.path())
            .env("PATH", root.path())
            .env("MANTLE_CONFIG", root.path().join("config.toml"))
            .env("MANTLE_STATE_DIR", root.path().join("state"))
            .env("METADATA_FIXTURE", root.path());
        c
    };
    assert!(cli().arg("list").output().unwrap().status.success());
    fs::write(root.path().join("state/id_ed25519"), "synthetic").unwrap();
    fs::write(root.path().join("state/known_hosts"), "synthetic").unwrap();
    fs::set_permissions(
        root.path().join("state/id_ed25519"),
        std::os::unix::fs::PermissionsExt::from_mode(0o600),
    )
    .unwrap();
    let db = rusqlite::Connection::open(root.path().join("state/state.db")).unwrap();
    db.execute_batch("INSERT INTO workers VALUES('default','fixture','kubevirt/fixture/fixture',NULL); INSERT INTO sessions VALUES('running','fixture','default','RUNNING','digest','ws','exec','{}','time',NULL,'codex','chatgpt-device');").unwrap();
    for (index,result) in [json!({"snapshot":format!("sha256:{}","7".repeat(64)),"driver":"host","driver_version":"fixture","config_generation":1,"probed_at":"2026-10-03T00:00:00Z","facts":{"operation.ledger-subject-max-rows":1000,"operation.ledger-subject-max-bytes":1048576,"operation.ledger-global-max-rows":10000,"operation.ledger-global-max-bytes":10485760}}),json!({"id":"ws","kind":"workspace","labels":{},"observed_at":"2026-10-03T00:00:00Z","state":"ready"}),json!({"id":"exec","kind":"exec","workspace":"ws","state":"running","observed_at":"2026-10-03T00:00:00Z","requested":{"capability_snapshot":format!("sha256:{}","7".repeat(64)),"network":"none","profile":"workspace","require":true}})].into_iter().enumerate() {
        let body = json!({"api_version":"v1","request_id":"fixture","result":result}).to_string();
        fs::write(root.path().join(format!("response-{index}")),format!("HTTP/1.1 200 OK\r\nx-b10x-contract: {}\r\nx-b10x-contract-bundle-sha256: {}\r\ncontent-length: {}\r\nconnection: close\r\ncontent-type: application/json\r\n\r\n{}",b10x_substrate_sdk::CONTRACT,b10x_substrate_sdk::CONTRACT_SHA256,body.len(),body)).unwrap();
    }
    let out = mantle_worker::run_bounded(
        cli().args([
            "status",
            "--session-id",
            "running",
            "--json",
            "--timeout-secs",
            "2",
        ]),
        None,
        Duration::from_secs(10),
        65536,
    )
    .unwrap();
    let data: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert!(out.status.success(), "{data}");
    assert_eq!(data["sessions"][0]["observed"]["exec_state"], "running");
    for index in 0..3 {
        assert!(
            !fs::read_to_string(root.path().join(format!("request-{index}")))
                .unwrap()
                .contains("/output")
        );
    }
    fs::remove_file(root.path().join("ssh")).unwrap();
    let out = cli()
        .args([
            "status",
            "--session-id",
            "running",
            "--json",
            "--timeout-secs",
            "1",
        ])
        .output()
        .unwrap();
    assert!(!out.status.success());
    let data: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(data["outcome"], "observation-unavailable");
    assert_eq!(data["sessions"][0]["id"], "running");
    assert!(out.stderr.is_empty());
}

#[test]
fn creation_receipt_collision_refuses_before_remote_calls() {
    use std::fs;
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("config.toml"),"provider='kubevirt'\nubuntu_serial='20260926'\n[kubevirt]\ncontext='fixture'\nnamespace='fixture'\ncpu=2\nmemory_gib=4\nroot_disk_gib=8\ndata_disk_gib=8\n").unwrap();
    let receipt = root.path().join("receipt");
    fs::write(&receipt, b"existing evidence").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mantle"))
        .env_clear()
        .env("HOME", root.path())
        .env("PATH", root.path())
        .env("MANTLE_CONFIG", root.path().join("config.toml"))
        .env("MANTLE_STATE_DIR", root.path().join("state"))
        .args(["start", "missing-manifest", "--detached", "--receipt"])
        .arg(&receipt)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("receipt must be new"));
    assert_eq!(fs::read(receipt).unwrap(), b"existing evidence");
    let db = rusqlite::Connection::open(root.path().join("state/state.db")).unwrap();
    assert_eq!(
        db.query_row("SELECT COUNT(*) FROM sessions", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        0
    );
}
