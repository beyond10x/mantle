use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

fn cli(home: &Path, args: &[&str]) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_mantle"));
    command
        .env_clear()
        .env("HOME", home)
        .env("PATH", "/usr/bin:/bin")
        .args(args);
    command
}
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
            panic!("profile operation exceeded deadline");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    child.wait_with_output().unwrap()
}
fn success(output: Output) -> String {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
fn add(home: &Path, name: &str, config: &Path, state: &Path) -> Command {
    cli(
        home,
        &[
            "profile",
            "add",
            name,
            "--config",
            config.to_str().unwrap(),
            "--state-dir",
            state.to_str().unwrap(),
        ],
    )
}
fn config(path: &Path) {
    fs::write(path,"provider='kubevirt'\nubuntu_serial='20260926'\n[kubevirt]\ncontext='fixture'\nnamespace='fixture'\ncpu=2\nmemory_gib=4\nroot_disk_gib=8\ndata_disk_gib=8\n").unwrap();
}

#[test]
fn profile_registry_is_local_private_and_concurrent_additions_do_not_clobber() {
    let home = tempfile::tempdir().unwrap();
    assert!(success(run(cli(home.path(), &["profile", "list"]))).is_empty());
    assert!(
        !run(cli(home.path(), &["profile", "show", "missing"]))
            .status
            .success()
    );
    assert!(!home.path().join(".config").exists());
    let config = home.path().join("missing-config");
    let state = home.path().join("missing-state");
    let mut command = add(home.path(), "first", &config, &state);
    command.env("MANTLE_CONFIG", "unrelated-invalid-path");
    success(run(command));
    assert!(!config.exists() && !state.exists());
    let registry = home.path().join(".config/mantle/profiles");
    assert_eq!(
        registry.metadata().unwrap().permissions().mode() & 0o777,
        0o700
    );
    assert_eq!(
        registry
            .join("first.toml")
            .metadata()
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    let before = fs::read(registry.join("first.toml")).unwrap();
    assert!(
        !run(add(home.path(), "first", &state, &config))
            .status
            .success()
    );
    assert_eq!(fs::read(registry.join("first.toml")).unwrap(), before);
    let results = std::thread::scope(|scope| {
        let handles: Vec<_> = ["a", "b", "same", "same"]
            .into_iter()
            .map(|name| {
                let home = home.path();
                let config = &config;
                let state = &state;
                scope.spawn(move || run(add(home, name, config, state)))
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().unwrap().status.success())
            .collect::<Vec<_>>()
    });
    assert_eq!(results.iter().filter(|ok| **ok).count(), 3);
    let list = success(run(cli(home.path(), &["profile", "list"])));
    for name in ["a", "b", "first", "same"] {
        assert!(
            list.lines()
                .any(|line| line.split_whitespace().next() == Some(name)),
            "{list}"
        );
    }
    let shown = success(run(cli(home.path(), &["profile", "show", "first"])));
    assert!(shown.contains(config.to_str().unwrap()) && shown.contains(state.to_str().unwrap()));
    assert!(!state.exists());
}

#[test]
fn profile_selection_precedence_conflicts_and_database_isolation() {
    let home = tempfile::tempdir().unwrap();
    for name in ["one", "two"] {
        let path = home.path().join(format!("{name}.toml"));
        config(&path);
        success(run(add(home.path(), name, &path, &home.path().join(name))));
    }
    let mut explicit = cli(home.path(), &["--profile", "one", "list"]);
    explicit.env("MANTLE_PROFILE", "missing");
    success(run(explicit));
    use std::os::unix::ffi::OsStringExt as _;
    let mut explicit = cli(home.path(), &["--profile", "one", "list"]);
    explicit.env("MANTLE_PROFILE", std::ffi::OsString::from_vec(vec![0xff]));
    success(run(explicit));
    let mut invalid_environment = cli(home.path(), &["list"]);
    invalid_environment.env("MANTLE_PROFILE", std::ffi::OsString::from_vec(vec![0xff]));
    assert!(!run(invalid_environment).status.success());
    assert!(home.path().join("one/state.db").exists());
    assert!(!home.path().join("two").exists());
    for variable in ["MANTLE_CONFIG", "MANTLE_STATE_DIR"] {
        let mut conflict = cli(home.path(), &["--profile", "two", "list"]);
        conflict.env(variable, home.path().join("forbidden"));
        assert!(!run(conflict).status.success());
        assert!(!home.path().join("two").exists() && !home.path().join("forbidden").exists());
    }
    let mut selected = cli(home.path(), &["list"]);
    selected.env("MANTLE_PROFILE", "two");
    success(run(selected));
    for name in ["one", "two"] {
        let db = rusqlite::Connection::open(home.path().join(name).join("state.db")).unwrap();
        db.execute("INSERT INTO sessions VALUES (?1,?1,'default','RUNNING','digest','ws','ex','{}','time',NULL,'codex','chatgpt-device')",[name]).unwrap();
        let out = success(run(cli(home.path(), &["--profile", name, "list"])));
        assert!(out.contains(name));
        assert!(!out.contains(if name == "one" { "two" } else { "one" }));
    }
    let legacy = home.path().join("legacy.toml");
    config(&legacy);
    let mut command = cli(home.path(), &["list"]);
    command
        .env("MANTLE_CONFIG", &legacy)
        .env("MANTLE_STATE_DIR", home.path().join("legacy-state"));
    success(run(command));
    assert!(home.path().join("legacy-state/state.db").exists());
    fs::copy(&legacy, home.path().join(".config/mantle/config.toml")).unwrap();
    success(run(cli(home.path(), &["list"])));
    assert!(home.path().join(".local/state/mantle/state.db").exists());
}

#[test]
fn profile_registry_rejects_unsafe_names_paths_and_descriptor_types() {
    let home = tempfile::tempdir().unwrap();
    let target = home.path().join("sentinel");
    fs::write(&target, "PRIVATE_SENTINEL").unwrap();
    for name in ["../escape", "a/b", ".", "bad name", ""] {
        assert!(
            !run(add(home.path(), name, &target, &home.path().join("state")))
                .status
                .success()
        );
    }
    assert!(
        !run(add(
            home.path(),
            "relative",
            Path::new("relative"),
            &home.path().join("state")
        ))
        .status
        .success()
    );
    assert!(!home.path().join(".config").exists());
    for suffix in [
        "with\"quote",
        "with'quote",
        "with\\escape",
        "with%p",
        "with${HOME}",
    ] {
        assert!(
            !run(add(
                home.path(),
                "unsafe-state",
                &target,
                &home.path().join(suffix)
            ))
            .status
            .success()
        );
        assert!(!home.path().join(".config").exists());
        assert!(!home.path().join(suffix).exists());
    }
    success(run(add(
        home.path(),
        "valid",
        &target,
        &home.path().join("state"),
    )));
    let registry = home.path().join(".config/mantle/profiles");
    symlink(&target, registry.join("linked.toml")).unwrap();
    for args in [
        ["profile", "show", "linked"].as_slice(),
        ["profile", "list"].as_slice(),
    ] {
        let output = run(cli(home.path(), args));
        assert!(!output.status.success());
        assert!(!String::from_utf8_lossy(&output.stderr).contains("PRIVATE_SENTINEL"));
    }
    fs::remove_file(registry.join("linked.toml")).unwrap();
    for (name, contents) in [
        ("malformed", b"PRIVATE_SENTINEL".to_vec()),
        ("oversized", vec![b'x'; 65536]),
    ] {
        let path = registry.join(format!("{name}.toml"));
        fs::write(&path, contents).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
        let output = run(cli(home.path(), &["profile", "show", name]));
        assert!(!output.status.success());
        assert!(!String::from_utf8_lossy(&output.stderr).contains("PRIVATE_SENTINEL"));
    }
    let fifo = registry.join("fifo.toml");
    assert!(
        Command::new("/usr/bin/mkfifo")
            .arg(&fifo)
            .status()
            .unwrap()
            .success()
    );
    assert!(
        !run(cli(home.path(), &["profile", "show", "fifo"]))
            .status
            .success()
    );
    assert_eq!(fs::read_to_string(target).unwrap(), "PRIVATE_SENTINEL");
}

#[test]
fn profile_selection_refuses_symlinked_configuration_state_and_registry_ancestors() {
    let home = tempfile::tempdir().unwrap();
    let real_config = home.path().join("real.toml");
    config(&real_config);
    let linked_config = home.path().join("linked.toml");
    symlink(&real_config, &linked_config).unwrap();
    let state = home.path().join("state");
    success(run(add(
        home.path(),
        "linked-config",
        &linked_config,
        &state,
    )));
    assert!(
        !run(cli(home.path(), &["--profile", "linked-config", "list"]))
            .status
            .success()
    );
    assert!(!state.exists());
    let sentinel = home.path().join("sentinel");
    fs::write(&sentinel, "unchanged").unwrap();
    fs::create_dir(&state).unwrap();
    fs::set_permissions(&state, fs::Permissions::from_mode(0o700)).unwrap();
    success(run(add(home.path(), "real", &real_config, &state)));
    for name in [
        "state.db",
        "state.db-wal",
        "state.db-shm",
        "id_ed25519",
        "id_ed25519.pub",
        "known_hosts",
    ] {
        let path = state.join(name);
        symlink(&sentinel, &path).unwrap();
        assert!(
            !run(cli(home.path(), &["--profile", "real", "list"]))
                .status
                .success(),
            "{name}"
        );
        fs::remove_file(&path).unwrap();
        assert!(!state.join("state.db").exists());
        assert_eq!(fs::read_to_string(&sentinel).unwrap(), "unchanged");
    }
    let linked_state = home.path().join("linked-state");
    symlink(&state, &linked_state).unwrap();
    success(run(add(
        home.path(),
        "linked-state",
        &real_config,
        &linked_state,
    )));
    assert!(
        !run(cli(home.path(), &["--profile", "linked-state", "list"]))
            .status
            .success()
    );
    let other = tempfile::tempdir().unwrap();
    symlink(home.path().join(".config"), other.path().join(".config")).unwrap();
    assert!(
        !run(cli(other.path(), &["profile", "show", "real"]))
            .status
            .success()
    );
    assert!(
        !run(add(other.path(), "new", &real_config, &state))
            .status
            .success()
    );
    assert!(
        !home
            .path()
            .join(".config/mantle/profiles/new.toml")
            .exists()
    );
}

#[test]
fn selected_profiles_isolate_ssh_identity_known_hosts_and_private_sockets() {
    let home = tempfile::tempdir().unwrap();
    let mut compiler = Command::new("rustc");
    compiler
        .args([
            "--edition=2024",
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/ssh_fixture.rs"),
            "-o",
        ])
        .arg(home.path().join("ssh"));
    success(run(compiler));
    let mut sockets = Vec::new();
    for name in ["one", "two"] {
        let config_path = home.path().join(format!("{name}.toml"));
        config(&config_path);
        fs::write(
            &config_path,
            fs::read_to_string(&config_path)
                .unwrap()
                .replace("context='fixture'", &format!("context='{name}'")),
        )
        .unwrap();
        let state = home.path().join(if name == "one" {
            format!("{} # spaces:λ", "long".repeat(40))
        } else {
            name.into()
        });
        success(run(add(home.path(), name, &config_path, &state)));
        success(run(cli(home.path(), &["--profile", name, "list"])));
        fs::write(state.join("id_ed25519"), format!("synthetic {name} key")).unwrap();
        fs::write(state.join("known_hosts"), format!("synthetic {name} host")).unwrap();
        let db = rusqlite::Connection::open(state.join("state.db")).unwrap();
        db.execute(
            "INSERT INTO workers VALUES ('default',?1,?2,NULL)",
            [name, &format!("kubevirt/{name}/fixture")],
        )
        .unwrap();
        db.execute_batch("INSERT INTO sessions VALUES ('s','session','default','RUNNING','digest','ws','agent','{}','time',NULL,'codex','chatgpt-device')").unwrap();
        let fixture = home.path().join(name).join("profile-inspect");
        fs::create_dir_all(&fixture).unwrap();
        let mut command = cli(
            home.path(),
            &["--profile", name, "exec", "session", "--", "true"],
        );
        command
            .env("PATH", home.path())
            .env("MANTLE_SSH_FIXTURE", &fixture);
        let output = run(command);
        assert_eq!(
            output.status.code(),
            Some(1),
            "inspection fixture deliberately closes discovery"
        );
        let args = fs::read_to_string(fixture.join("argv")).unwrap();
        let args: Vec<_> = args.lines().collect();
        let key = args[args.iter().position(|a| *a == "-i").unwrap() + 1];
        assert_eq!(Path::new(key), state.join("id_ed25519"));
        assert!(
            args.contains(
                &format!(
                    "UserKnownHostsFile=\"{}\"",
                    state.join("known_hosts").display()
                )
                .as_str()
            )
        );
        assert!(args.contains(&format!("HostKeyAlias={name}").as_str()));
        assert!(
            args.iter().any(|a| a.starts_with("ProxyCommand=")
                && a.contains("fixture")
                && a.contains(name))
        );
        let socket = args[args.iter().position(|a| *a == "-L").unwrap() + 1]
            .split_once(':')
            .unwrap()
            .0;
        assert!(socket.len() < 100 && !Path::new(socket).starts_with(&state));
        assert!(!Path::new(socket).exists(), "owned tunnel cleaned up");
        sockets.push(socket.to_string());
        assert_eq!(
            fs::read_to_string(state.join("id_ed25519")).unwrap(),
            format!("synthetic {name} key")
        );
        assert_eq!(
            fs::read_to_string(state.join("known_hosts")).unwrap(),
            format!("synthetic {name} host")
        );
    }
    assert_ne!(sockets[0], sockets[1]);
}
