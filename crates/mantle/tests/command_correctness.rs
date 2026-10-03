use base64::Engine as _;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

fn bounded(mut command: Command) -> Output {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() > deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("fixture exceeded its process deadline");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    child.wait_with_output().unwrap()
}

fn response(root: &Path, index: usize, result: Value) {
    let body = json!({"api_version":"v1", "request_id":"fixture", "result":result}).to_string();
    fs::write(root.join(format!("response-{index}")), format!(
        "HTTP/1.1 200 OK\r\nx-b10x-contract: {}\r\nx-b10x-contract-bundle-sha256: {}\r\ncontent-length: {}\r\nconnection: close\r\ncontent-type: application/json\r\n\r\n{}",
        b10x_substrate_sdk::CONTRACT, b10x_substrate_sdk::CONTRACT_SHA256, body.len(), body
    )).unwrap();
}

#[test]
fn actual_cli_preserves_remote_exit_status_and_output() {
    let root = tempfile::tempdir().unwrap();
    let mut compiler = Command::new("rustc");
    compiler
        .arg("--edition=2024")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/support/ssh_fixture.rs"
        ))
        .arg("-o")
        .arg(root.path().join("ssh"));
    let compiled = bounded(compiler);
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let config = root.path().join("config.toml");
    fs::write(&config, "provider='kubevirt'\nubuntu_serial='20260926'\n[kubevirt]\ncontext='fixture'\nnamespace='fixture'\ncpu=2\nmemory_gib=4\nroot_disk_gib=8\ndata_disk_gib=8\n").unwrap();
    let state = root.path().join("state");
    let mut initial = Command::new(env!("CARGO_BIN_EXE_mantle"));
    initial
        .env_clear()
        .env("HOME", root.path())
        .env("MANTLE_CONFIG", &config)
        .env("MANTLE_STATE_DIR", &state)
        .arg("list");
    assert!(bounded(initial).status.success());
    fs::write(state.join("id_ed25519"), "synthetic key never parsed").unwrap();
    let db = rusqlite::Connection::open(state.join("state.db")).unwrap();
    db.execute_batch("INSERT INTO workers VALUES ('default','fixture','kubevirt/fixture/fixture',NULL); INSERT INTO sessions VALUES ('s','fixture','default','RUNNING','digest','ws','agent','{}','time',NULL,'codex','chatgpt-device');").unwrap();
    let cases = [
        ("zero", "exited", json!({"code":0}), 0),
        ("empty-streams", "exited", json!({"code":0}), 0),
        ("one", "exited", json!({"code":1}), 1),
        ("forty-two", "exited", json!({"code":42}), 42),
        ("max", "exited", json!({"code":255}), 255),
        ("interrupt", "exited", json!({"signal":"INT"}), 130),
        ("terminate", "exited", json!({"signal":"TERM"}), 143),
        ("kill", "exited", json!({"signal":"KILL"}), 137),
        ("missing", "exited", Value::Null, 1),
        ("empty", "exited", json!({}), 1),
        (
            "contradictory",
            "exited",
            json!({"code":0,"signal":"INT"}),
            1,
        ),
        ("unknown", "unknown", json!({"code":0}), 1),
        ("expired", "expired", json!({"code":0}), 1),
        ("cancelled", "cancelled", json!({"code":0}), 1),
        ("running", "running", json!({"code":0}), 1),
        ("accepted", "accepted", json!({"code":0}), 1),
        ("refused", "exited", json!({"code":0}), 1),
        ("invalid-code", "exited", json!({"code":256}), 1),
        ("negative-code", "exited", json!({"code":-1}), 1),
        (
            "invalid-signal",
            "exited",
            json!({"signal":"SIGUNKNOWN"}),
            1,
        ),
    ];
    let stdout = b"out\0\xff\n";
    let stderr = b"err\0\xfe\n";
    let mut failures = Vec::new();
    for (name, status, exit, expected) in cases {
        let stdout = if name == "empty-streams" {
            b"".as_slice()
        } else {
            stdout.as_slice()
        };
        let stderr = if name == "empty-streams" {
            b"".as_slice()
        } else {
            stderr.as_slice()
        };
        let case = root.path().join(name);
        fs::create_dir(&case).unwrap();
        response(
            &case,
            0,
            json!({"snapshot":format!("sha256:{}","7".repeat(64)),"driver":"host","driver_version":"fixture","config_generation":1,"probed_at":"2026-10-03T00:00:00Z","facts":{"operation.ledger-subject-max-rows":1000,"operation.ledger-subject-max-bytes":1048576,"operation.ledger-global-max-rows":10000,"operation.ledger-global-max-bytes":10485760}}),
        );
        response(
            &case,
            1,
            json!({"id":"ws","kind":"workspace","labels":{},"observed_at":"2026-10-03T00:00:00Z","state":"ready"}),
        );
        response(
            &case,
            2,
            json!({"id":"ex","kind":"exec","workspace":"ws","state":status,"observed_at":"2026-10-03T00:00:00Z","requested":{"capability_snapshot":format!("sha256:{}","7".repeat(64)),"network":"none","profile":"workspace","require":true},"applied":null,"exit":exit,"refusal":if name=="refused" {json!({"class":"failed","code":"fixture.refused","message":"fixture refusal"})} else {Value::Null}}),
        );
        for (index, stream, bytes) in [(3, "stdout", stdout), (4, "stderr", stderr)] {
            response(
                &case,
                index,
                json!({"exec":"ex","stream":stream,"offset":0,"returned_bytes":bytes.len(),"next_offset":bytes.len(),"eof":true,"truncated":false,"content":{"encoding":"base64","data":base64::engine::general_purpose::STANDARD.encode(bytes)},"observed_at":"2026-10-03T00:00:00Z"}),
            );
        }
        let mut cli = Command::new(env!("CARGO_BIN_EXE_mantle"));
        cli.env_clear()
            .env("HOME", root.path())
            .env("PATH", root.path())
            .env("MANTLE_CONFIG", &config)
            .env("MANTLE_STATE_DIR", &state)
            .env("MANTLE_SSH_FIXTURE", &case)
            .args(["exec", "fixture", "--", "/usr/bin/printf", "literal $HOME"]);
        let output = bounded(cli);
        if output.status.code() != Some(expected) {
            failures.push(format!(
                "{name}: wanted {expected}, got {:?}: {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        if !matches!(name, "invalid-code" | "negative-code" | "invalid-signal") {
            assert_eq!(
                output.stdout,
                stdout,
                "{name}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(
                output.stderr.starts_with(stderr),
                "{name}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        let request = fs::read_to_string(case.join("request-2")).unwrap();
        let input: Value = serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(
            input["input"]["argv"],
            json!(["/usr/bin/printf", "literal $HOME"])
        );
        assert!(input["input"].get("secret_slots").is_none());
    }
    let transport = root.path().join("transport-failure");
    fs::create_dir(&transport).unwrap();
    let mut cli = Command::new(env!("CARGO_BIN_EXE_mantle"));
    cli.env_clear()
        .env("HOME", root.path())
        .env("PATH", root.path())
        .env("MANTLE_CONFIG", &config)
        .env("MANTLE_STATE_DIR", &state)
        .env("MANTLE_SSH_FIXTURE", &transport)
        .args(["exec", "fixture", "--", "/usr/bin/true"]);
    let output = bounded(cli);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("connecting to Substrate"));
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
