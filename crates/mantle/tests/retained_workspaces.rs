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
