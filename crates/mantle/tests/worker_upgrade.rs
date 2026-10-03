#[allow(dead_code)]
#[path = "../../mantle-worker/tests/support/upgrade.rs"]
mod support;
use mantle_worker::maintenance::Host;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::Command,
    sync::OnceLock,
    time::{Duration, Instant},
};
fn ssh_binary() -> &'static PathBuf {
    static SSH: OnceLock<(tempfile::TempDir, PathBuf)> = OnceLock::new();
    &SSH.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        let out = dir.path().join("ssh");
        let output = mantle_worker::run_bounded(
            Command::new("rustc")
                .args(["--edition=2024"])
                .arg(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/tests/support/upgrade_ssh.rs"
                ))
                .arg("-o")
                .arg(&out),
            None,
            Duration::from_secs(30),
            65536,
        )
        .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        (dir, out)
    })
    .1
}
#[test]
fn actual_old_worker_check_observes_offline_facts_without_upload_or_state_writes() {
    exercise(false);
}

#[test]
fn actual_apply_transports_verified_bundle_and_cleans_owned_stage() {
    exercise(true);
}

#[test]
fn upgrade_worker_transaction_child() {
    let Some(manifest) = std::env::var_os("UPGRADE_FIXTURE_APPLY_MANIFEST") else {
        return;
    };
    let host = support::FixtureHost {
        fs: mantle_worker::maintenance::LocalHost {
            root: std::env::var_os("UPGRADE_FIXTURE_HOST").unwrap().into(),
        },
        fault: std::cell::RefCell::new(String::new()),
        calls: std::cell::RefCell::new(Vec::new()),
    };
    let report = mantle_worker::upgrade::apply(
        &host,
        &host.fs.root.join("opt/mantle"),
        std::path::Path::new(&manifest),
        &(),
    )
    .unwrap();
    fs::write(
        std::env::var_os("UPGRADE_FIXTURE_APPLY_REPORT").unwrap(),
        serde_json::to_vec(&report).unwrap(),
    )
    .unwrap();
}

fn exercise(apply: bool) {
    let f = support::Fixture::new();
    let home = f.temp.path().join("home");
    let state = home.join("state");
    fs::create_dir_all(&state).unwrap();
    fs::set_permissions(&state, fs::Permissions::from_mode(0o700)).unwrap();
    fs::write(state.join("id_ed25519"), b"fixture private key\n").unwrap();
    fs::set_permissions(state.join("id_ed25519"), fs::Permissions::from_mode(0o600)).unwrap();
    fs::write(state.join("known_hosts"), b"fixture pinned host\n").unwrap();
    let config = home.join("config.toml");
    fs::write(&config,"provider='kubevirt'\nubuntu_serial='20260926'\n[kubevirt]\ncontext='fixture'\nnamespace='fixture'\ncpu=2\nmemory_gib=4\nroot_disk_gib=8\ndata_disk_gib=8\n").unwrap();
    let connection = rusqlite::Connection::open(state.join("state.db")).unwrap();
    connection.execute_batch("CREATE TABLE workers(name TEXT PRIMARY KEY,instance TEXT NOT NULL,region TEXT NOT NULL,data_volume TEXT); INSERT INTO workers VALUES('default','fixture','kubevirt/fixture/fixture',NULL);").unwrap();
    drop(connection);
    let before_db = fs::read(state.join("state.db")).unwrap();
    let before_bin = f.snapshot();
    fs::write(f.path("jobs"), b"").unwrap();
    for unit in mantle_worker::maintenance::UNITS {
        fs::write(
            f.path(&format!("show-{unit}")),
            f.host
                .command(
                    &["systemctl", "show", unit],
                    Instant::now() + Duration::from_secs(1),
                )
                .unwrap(),
        )
        .unwrap();
    }
    let trace = f.temp.path().join("ssh.trace");
    let mut cmd = Command::new(
        std::env::var_os("MANTLE_UPGRADE_BASE_CLI")
            .unwrap_or_else(|| env!("CARGO_BIN_EXE_mantle").into()),
    );
    cmd.args([
        "worker",
        "upgrade",
        if apply { "--apply" } else { "--check" },
        "--manifest",
    ])
    .arg(&f.manifest)
    .env("HOME", &home)
    .env("MANTLE_CONFIG", &config)
    .env("MANTLE_STATE_DIR", &state)
    .env_remove("MANTLE_PROFILE")
    .env(
        "PATH",
        format!(
            "{}:{}",
            ssh_binary().parent().unwrap().display(),
            std::env::var("PATH").unwrap()
        ),
    )
    .env("UPGRADE_FIXTURE_HOST", &f.host.fs.root)
    .env("UPGRADE_FIXTURE_TRACE", &trace)
    .env("UPGRADE_FIXTURE_TEST_EXE", std::env::current_exe().unwrap());
    let output =
        mantle_worker::run_bounded(&mut cmd, None, Duration::from_secs(30), 65536).unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        report["outcome"],
        if apply {
            "applied-restart-required"
        } else {
            "eligible"
        }
    );
    assert_eq!(report["compatibility"], "matching");
    if !apply {
        assert_eq!(report["installed_versions"][0], "mantle-egress 0.1.3");
    }
    assert_eq!(fs::read(state.join("state.db")).unwrap(), before_db);
    if !apply {
        assert_eq!(f.snapshot(), before_bin);
    }
    assert_eq!(
        fs::read(state.join("known_hosts")).unwrap(),
        b"fixture pinned host\n"
    );
    let calls = fs::read_to_string(&trace).unwrap();
    assert!(calls.contains("'--all'"));
    if apply {
        assert!(calls.contains("'upgrade-apply'"));
        assert!(calls.contains("'rm' '-r' '--' '/var/tmp/mantle-delivery.123456abcdef'"));
        assert!(!f.path("/var/tmp/mantle-delivery.123456abcdef").exists());
        assert_ne!(f.snapshot()["mantle-worker"], before_bin["mantle-worker"]);
        assert_eq!(f.snapshot()["claude"], before_bin["claude"]);
        assert_eq!(f.snapshot()["codex"], before_bin["codex"]);
        return;
    }
    assert!(
        !calls.contains("mktemp")
            && !calls.contains("tee")
            && !calls.contains("upgrade-check")
            && !calls.contains("install-codex")
    );
    fs::write(
        f.path("/sys/fs/cgroup/system.slice/substrate.service/cgroup.events"),
        b"populated 1\nfrozen 0\n",
    )
    .unwrap();
    let output =
        mantle_worker::run_bounded(&mut cmd, None, Duration::from_secs(30), 65536).unwrap();
    assert!(!output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["outcome"], "refused");
}
