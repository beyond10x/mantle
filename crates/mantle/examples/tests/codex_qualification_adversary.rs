use super::*;

fn private_parent() -> tempfile::TempDir {
    match std::env::var_os("MANTLE_TEST_SCRATCH") {
        Some(parent) => tempfile::tempdir_in(parent).unwrap(),
        None => tempfile::tempdir().unwrap(),
    }
}

// This child process isolates the potentially blocking open from the test runner.
// The input contains no credentials and the parent always kills/reaps a stuck child.
#[test]
fn non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer() {
    const CHILD: &str = "MANTLE_QUALIFICATION_FIFO_ATTACK";
    if let Some(path) = std::env::var_os(CHILD) {
        assert!(verify_digest(Path::new(&path), &"0".repeat(64)).is_err());
        return;
    }
    let parent = private_parent();
    let scratch = Scratch::new(parent.path()).unwrap();
    let fifo = scratch.0.join("candidate-codex");
    assert!(
        Command::new("/usr/bin/mkfifo")
            .arg(&fifo)
            .status()
            .unwrap()
            .success()
    );
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "adversary::non_regular_binary_input_is_refused_without_waiting_for_a_fifo_writer",
            "--nocapture",
        ])
        .env(CHILD, &fifo)
        .stdin(Stdio::null())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break Some(status);
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            break None;
        }
        sleep(Duration::from_millis(10));
    };
    scratch.cleanup().unwrap();
    assert!(
        status.is_some_and(|value| value.success()),
        "non-regular binary path blocked beyond the deadline instead of being refused"
    );
}

#[test]
fn binary_symlink_is_validated_against_its_current_opened_object() {
    const CHILD: &str = "MANTLE_QUALIFICATION_SYMLINK_ATTACK";
    const ABC_SHA256: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    if let Some(path) = std::env::var_os(CHILD) {
        let error = verify_digest(Path::new(&path), ABC_SHA256).unwrap_err();
        assert_eq!(error.to_string(), "binary is not a regular file");
        return;
    }
    let parent = private_parent();
    let scratch = Scratch::new(parent.path()).unwrap();
    let target = scratch.0.join("target");
    let selected = scratch.0.join("selected-codex");
    fs::write(&target, b"abc").unwrap();
    std::os::unix::fs::symlink(&target, &selected).unwrap();
    assert_eq!(digest(&selected).unwrap(), ABC_SHA256);
    let replacement = scratch.0.join("fifo-replacement");
    assert!(
        Command::new("/usr/bin/mkfifo")
            .arg(&replacement)
            .status()
            .unwrap()
            .success()
    );
    fs::rename(&replacement, &target).unwrap();
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "adversary::binary_symlink_is_validated_against_its_current_opened_object",
            "--nocapture",
        ])
        .env(CHILD, &selected)
        .stdin(Stdio::null())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(2);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break Some(status);
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            break None;
        }
        sleep(Duration::from_millis(10));
    };
    scratch.cleanup().unwrap();
    assert!(
        status.is_some_and(|value| value.success()),
        "replacement FIFO reached through a binary symlink was not promptly refused by object type"
    );
}
