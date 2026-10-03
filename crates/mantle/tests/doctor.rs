use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

fn run(mut command: Command) -> Output {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(15);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() > deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("doctor exceeded process deadline");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    child.wait_with_output().unwrap()
}
fn cli(home: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_mantle"));
    command
        .env_clear()
        .env("HOME", home)
        .env("PATH", "/usr/bin:/bin")
        .args(["doctor", "--json", "--timeout-secs", "1"]);
    command
}
fn report(output: Output) -> Value {
    assert_eq!(
        output.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "structured JSON report missing: {error}; stderr={}",
            String::from_utf8_lossy(&output.stderr)
        )
    });
    assert_eq!(report["healthy"], false);
    assert!(!String::from_utf8_lossy(&output.stdout).contains("PRIVATE_CANARY"));
    assert!(output.stderr.is_empty());
    report
}
#[test]
fn doctor_reports_selection_and_configuration_failures_as_json_without_mutating_state() {
    let home = tempfile::tempdir().unwrap();
    let mut command = cli(home.path());
    command.args(["--profile", "missing"]);
    let result = report(run(command));
    assert!(result["selection"].is_null());
    assert_eq!(result["checks"][0]["status"], "failed");
    assert!(!home.path().join(".config").exists());
    let mut absent_home = cli(home.path());
    absent_home.env_remove("HOME");
    assert!(report(run(absent_home))["selection"].is_null());
    use std::os::unix::ffi::OsStringExt as _;
    let mut non_utf8 = cli(home.path());
    non_utf8.env(
        "MANTLE_CONFIG",
        std::ffi::OsString::from_vec(b"/fixture/\xff".to_vec()),
    );
    assert!(report(run(non_utf8))["selection"].is_null());
    let config = home.path().join("config.toml");
    fs::write(&config, "PRIVATE_CANARY malformed").unwrap();
    let mut command = cli(home.path());
    command
        .env("MANTLE_CONFIG", &config)
        .env("MANTLE_STATE_DIR", home.path().join("state"));
    let result = report(run(command));
    assert!(!result["selection"].is_null());
    assert!(
        result["checks"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["stage"] == "configuration" && c["status"] == "failed")
    );
    assert!(!home.path().join("state").exists());
    fs::remove_file(&config).unwrap();
    assert!(
        Command::new("/usr/bin/mkfifo")
            .arg(&config)
            .status()
            .unwrap()
            .success()
    );
    let mut command = cli(home.path());
    command.env("MANTLE_CONFIG", &config);
    report(run(command));
    assert!(!home.path().join(".local").exists());
    let mut human = Command::new(env!("CARGO_BIN_EXE_mantle"));
    human
        .env_clear()
        .env("HOME", home.path())
        .env("MANTLE_CONFIG", &config)
        .args(["doctor", "--timeout-secs", "1"]);
    let human = run(human);
    assert_eq!(human.status.code(), Some(1));
    assert!(human.stderr.is_empty());
    let human = String::from_utf8(human.stdout).unwrap();
    assert!(human.contains("configuration: failed") && human.contains("provider: skipped"));
    assert!(!human.contains("PRIVATE_CANARY"));
}

#[test]
fn doctor_bounds_local_state_failures_and_suppresses_external_probes() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let root = tempfile::tempdir().unwrap();
    let config = root.path().join("config.toml");
    fs::write(&config,"provider='kubevirt'\nubuntu_serial='20260926'\n[kubevirt]\ncontext='fixture'\nnamespace='fixture'\ncpu=2\nmemory_gib=4\nroot_disk_gib=8\ndata_disk_gib=8\n").unwrap();
    let state = root.path().join("state");
    let invoke = || {
        let mut command = cli(root.path());
        command
            .env("PATH", root.path())
            .env("MANTLE_CONFIG", &config)
            .env("MANTLE_STATE_DIR", &state);
        report(run(command))
    };
    assert!(
        invoke()["checks"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["stage"] == "state" && c["status"] == "failed")
    );
    assert!(!state.exists());
    fs::create_dir(&state).unwrap();
    fs::set_permissions(&state, fs::Permissions::from_mode(0o700)).unwrap();
    let path = state.join("state.db");
    for shape in ["fifo", "symlink", "oversized", "malformed", "placement"] {
        match shape {
            "fifo" => {
                assert!(
                    Command::new("/usr/bin/mkfifo")
                        .arg(&path)
                        .status()
                        .unwrap()
                        .success()
                );
            }
            "symlink" => symlink(&config, &path).unwrap(),
            "oversized" => fs::File::create(&path)
                .unwrap()
                .set_len(65 * 1024 * 1024)
                .unwrap(),
            "malformed" => fs::write(&path, "PRIVATE_CANARY not a database").unwrap(),
            _ => {
                let db = rusqlite::Connection::open(&path).unwrap();
                db.execute_batch("CREATE TABLE workers(name TEXT,instance TEXT,region TEXT,data_volume TEXT);INSERT INTO workers VALUES('default','fixture','wrong',NULL)").unwrap();
            }
        }
        let result = invoke();
        let stage = if shape == "placement" {
            "placement"
        } else {
            "state"
        };
        assert!(
            result["checks"]
                .as_array()
                .unwrap()
                .iter()
                .any(|c| c["stage"] == stage && c["status"] == "failed"),
            "{shape}: {result}"
        );
        assert!(
            result["checks"]
                .as_array()
                .unwrap()
                .iter()
                .any(|c| c["stage"] == "provider" && c["status"] == "skipped")
        );
        fs::remove_file(&path).unwrap();
    }
}

#[test]
fn doctor_observes_real_cli_wire_stages_and_preserves_current_wal_state() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let root = tempfile::tempdir().unwrap();
    let mut compiler = Command::new("rustc");
    compiler
        .args([
            "--edition=2024",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/support/doctor_fixture.rs"
            ),
            "-o",
        ])
        .arg(root.path().join("fixture"));
    assert!(run(compiler).status.success());
    for program in ["ssh", "kubectl", "aws", "credential"] {
        symlink(root.path().join("fixture"), root.path().join(program)).unwrap();
    }
    let config = root.path().join("config.toml");
    fs::write(&config,"provider='kubevirt'\nubuntu_serial='20260926'\n[claude]\ntoken_command=['credential']\n[kubevirt]\ncontext='fixture'\nnamespace='fixture'\ncpu=2\nmemory_gib=4\nroot_disk_gib=8\ndata_disk_gib=8\n").unwrap();
    let state = root.path().join("state");
    let mut init = Command::new(env!("CARGO_BIN_EXE_mantle"));
    init.env_clear()
        .env("HOME", root.path())
        .env("MANTLE_CONFIG", &config)
        .env("MANTLE_STATE_DIR", &state)
        .arg("list");
    assert!(run(init).status.success());
    for file in ["id_ed25519", "known_hosts"] {
        fs::write(state.join(file), "private fixture marker").unwrap();
        fs::set_permissions(state.join(file), fs::Permissions::from_mode(0o600)).unwrap();
    }
    let db = rusqlite::Connection::open(state.join("state.db")).unwrap();
    db.execute_batch("PRAGMA journal_mode=WAL; INSERT INTO workers VALUES ('default','worker-fixture','kubevirt/fixture/fixture',NULL)").unwrap();
    let schema: String = db
        .query_row("SELECT group_concat(sql) FROM sqlite_master", [], |row| {
            row.get(0)
        })
        .unwrap();
    fs::write(root.path().join("provider.json"),r#"{"metadata":{"uid":"worker-fixture"},"status":{"ready":true,"printableStatus":"Running"}}"#).unwrap();
    let versions = format!(
        "mantle-worker {0}\nmantle-launch {0}\nmantle-egress {0}\n",
        env!("CARGO_PKG_VERSION")
    );
    let cases = [
        ("", "", 0),
        ("provider", "provider", 1),
        ("ssh", "ssh", 1),
        ("service", "service", 1),
        ("disk-version", "compatibility", 1),
        ("live-version", "compatibility", 1),
        ("contract", "compatibility", 1),
        ("facts", "facts", 1),
        ("timeout", "provider", 1),
        ("tunnel-timeout", "compatibility", 1),
        ("discovery-timeout", "compatibility", 1),
        ("tunnel-exit", "compatibility", 1),
    ];
    for (failure, stage, expected) in cases {
        if root.path().join("descendant").exists() {
            fs::remove_file(root.path().join("descendant")).unwrap();
        }
        fs::write(root.path().join("failure"), failure).unwrap();
        fs::write(root.path().join("calls"), "").unwrap();
        fs::write(
            root.path().join("versions"),
            if failure == "disk-version" {
                "mantle-worker 0.0.0\n"
            } else {
                &versions
            },
        )
        .unwrap();
        let facts = if failure == "facts" {
            serde_json::json!({"operation.ledger-subject-max-rows":1000,"operation.ledger-subject-max-bytes":1048576,"operation.ledger-global-max-rows":10000,"operation.ledger-global-max-bytes":10485760})
        } else {
            serde_json::json!({"exec.argv-only":true,"exec.no-egress":true,"operation.ledger-subject-max-rows":1000,"operation.ledger-subject-max-bytes":1048576,"operation.ledger-global-max-rows":10000,"operation.ledger-global-max-bytes":10485760,"exec.cgroup-limits":{"processes":true,"memory":true,"cpu":false},"exec.cgroup-kill":true,"sessions.pty":true,"exec.egress-apertures":[{"name":"egress","destination":"127.0.0.1:3128","max_bytes":null}]})
        };
        let body=serde_json::json!({"api_version":"v1","request_id":"fixture","result":{"snapshot":format!("sha256:{}","7".repeat(64)),"driver":"host","driver_version":if failure=="live-version" {"0.0.0"} else {"0.7.10"},"config_generation":1,"probed_at":"2026-10-03T00:00:00Z","facts":facts}}).to_string();
        let contract = if failure == "contract" {
            "incompatible"
        } else {
            b10x_substrate_sdk::CONTRACT
        };
        fs::write(root.path().join("discovery.http"),format!("HTTP/1.1 200 OK\r\nx-b10x-contract: {contract}\r\nx-b10x-contract-bundle-sha256: {}\r\ncontent-length: {}\r\ncontent-type: application/json\r\nconnection: close\r\n\r\n{body}",b10x_substrate_sdk::CONTRACT_SHA256,body.len())).unwrap();
        let mut command = cli(root.path());
        command
            .env("PATH", root.path())
            .env("MANTLE_CONFIG", &config)
            .env("MANTLE_STATE_DIR", &state)
            .env("MANTLE_DOCTOR_FIXTURE", root.path());
        let output = run(command);
        assert_eq!(
            output.status.code(),
            Some(expected),
            "case {failure}: {} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty());
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(report["healthy"], expected == 0, "case {failure}: {report}");
        if expected != 0 {
            let checks = report["checks"].as_array().unwrap();
            let index = checks.iter().position(|c| c["stage"] == stage).unwrap();
            assert_eq!(
                checks[index]["status"], "failed",
                "case {failure}: {report}"
            );
            assert!(checks[index + 1..].iter().all(|c| c["status"] == "skipped"));
        }
        assert!(!String::from_utf8_lossy(&output.stdout).contains("PRIVATE_CANARY"));
        assert!(!root.path().join("credential-read").exists());
        assert_eq!(
            db.query_row("SELECT group_concat(sql) FROM sqlite_master", [], |r| {
                r.get::<_, String>(0)
            })
            .unwrap(),
            schema
        );
        assert_eq!(
            db.query_row(
                "SELECT COUNT(*) FROM workers WHERE instance='worker-fixture'",
                [],
                |r| r.get::<_, i64>(0)
            )
            .unwrap(),
            1
        );
        assert_eq!(
            db.query_row("SELECT COUNT(*) FROM sessions", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            0
        );
        for file in ["id_ed25519", "known_hosts"] {
            assert_eq!(
                fs::read_to_string(state.join(file)).unwrap(),
                "private fixture marker"
            );
        }
        if failure.ends_with("timeout") || failure == "tunnel-exit" {
            let pid = fs::read_to_string(root.path().join("descendant")).unwrap();
            let deadline = Instant::now() + Duration::from_secs(3);
            while fs::read_to_string(format!("/proc/{pid}/stat"))
                .is_ok_and(|s| !s.split(')').nth(1).unwrap_or("").starts_with(" Z"))
            {
                assert!(Instant::now() < deadline, "descendant survived timeout");
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

#[test]
fn adversary_doctor_interruption_retires_tunnel_and_removes_owned_socket_directory() {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir().unwrap();
    let mut compiler = Command::new("rustc");
    compiler
        .args([
            "--edition=2024",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/support/doctor_fixture.rs"
            ),
            "-o",
        ])
        .arg(root.path().join("fixture"));
    assert!(run(compiler).status.success());
    for program in ["ssh", "kubectl"] {
        symlink(root.path().join("fixture"), root.path().join(program)).unwrap();
    }
    let config = root.path().join("config.toml");
    fs::write(&config, "provider='kubevirt'\nubuntu_serial='20260926'\n[kubevirt]\ncontext='fixture'\nnamespace='fixture'\ncpu=2\nmemory_gib=4\nroot_disk_gib=8\ndata_disk_gib=8\n").unwrap();
    let state = root.path().join("state");
    fs::create_dir(&state).unwrap();
    let db = rusqlite::Connection::open(state.join("state.db")).unwrap();
    db.execute_batch("CREATE TABLE workers(name TEXT PRIMARY KEY,instance TEXT,region TEXT,data_volume TEXT); INSERT INTO workers VALUES('default','worker-fixture','kubevirt/fixture/fixture',NULL)").unwrap();
    for name in ["id_ed25519", "known_hosts"] {
        fs::write(state.join(name), "synthetic fixture").unwrap();
    }
    fs::write(root.path().join("provider.json"), r#"{"metadata":{"uid":"worker-fixture"},"status":{"ready":true,"printableStatus":"Running"}}"#).unwrap();
    fs::write(
        root.path().join("versions"),
        format!(
            "mantle-worker {0}\nmantle-launch {0}\nmantle-egress {0}\n",
            env!("CARGO_PKG_VERSION")
        ),
    )
    .unwrap();
    fs::write(root.path().join("failure"), "discovery-timeout").unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_mantle"));
    command
        .env_clear()
        .env("HOME", root.path())
        .env("PATH", root.path())
        .env("MANTLE_CONFIG", &config)
        .env("MANTLE_STATE_DIR", &state)
        .env("MANTLE_DOCTOR_FIXTURE", root.path())
        .args(["doctor", "--json", "--timeout-secs", "10"])
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut doctor = command.spawn().unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    let socket = loop {
        if let Ok(args) = fs::read_to_string(root.path().join("ssh-argv")) {
            let args: Vec<_> = args.lines().collect();
            if let Some(index) = args.iter().position(|a| *a == "-L") {
                let socket = std::path::PathBuf::from(args[index + 1].split_once(':').unwrap().0);
                if socket.exists() && root.path().join("descendant").exists() {
                    break socket;
                }
            }
        }
        if Instant::now() >= deadline {
            let _ = doctor.kill();
            let _ = doctor.wait();
            panic!("doctor did not reach the controlled live tunnel");
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    let descendant = fs::read_to_string(root.path().join("descendant")).unwrap();
    assert!(
        Command::new("/usr/bin/kill")
            .args(["-TERM", &doctor.id().to_string()])
            .status()
            .unwrap()
            .success()
    );
    let deadline = Instant::now() + Duration::from_secs(3);
    let status = loop {
        if let Some(status) = doctor.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = doctor.kill();
            let _ = doctor.wait();
            panic!("interrupted doctor did not terminate within cleanup bound");
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    assert!(!status.success());
    while fs::read_to_string(format!("/proc/{descendant}/stat"))
        .is_ok_and(|s| !s.split(')').nth(1).unwrap_or("").starts_with(" Z"))
    {
        assert!(
            Instant::now() < deadline,
            "tunnel descendant survived interruption"
        );
        std::thread::sleep(Duration::from_millis(5));
    }
    let directory = socket.parent().unwrap();
    let leaked = directory.exists();
    // Remove only this fixture's observed owned allocation after measuring the postcondition.
    if leaked {
        assert_eq!(directory.parent(), Some(Path::new("/tmp")));
        assert!(
            directory
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("mantle-")
        );
        fs::remove_dir_all(directory).unwrap();
    }
    assert!(
        !leaked,
        "SIGTERM left the owned diagnostic socket directory behind after process cleanup"
    );
}
