//! Adversarial cases for `mantle-launch serve` and `attach`: shutdown of the agent's process
//! group, the session directory against files planted by a process in the same workspace,
//! liveness against a stalled client and a flooded control pipe, the exit code and the
//! credential descriptor.

use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle, sleep};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const LAUNCHER: &str = env!("CARGO_BIN_EXE_mantle-launch");
const SECRET: &str = "adv-secret-71d2";
const WAIT: Duration = Duration::from_secs(10);

/// Scratch space under `MANTLE_TEST_SCRATCH`, or under $HOME; never /tmp.
fn scratch(name: &str) -> PathBuf {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let root = std::env::var_os("MANTLE_TEST_SCRATCH").map_or_else(
        || PathBuf::from(std::env::var_os("HOME").expect("HOME")).join(".cache/claude-tmp"),
        PathBuf::from,
    );
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let dir = root.join(format!(
        "mantle-launch-adv-{name}-{}-{nanos}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::SeqCst)
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn session_dir(dir: &Path) -> PathBuf {
    dir.join("session")
}

/// `serve` with the credential on fd 3, as the sandbox delivers it, and an empty environment.
fn serve(dir: &Path, program: &[&str]) -> Command {
    let secret_file = dir.join("secret");
    fs::write(&secret_file, format!("{SECRET}\n")).unwrap();
    fs::create_dir_all(dir.join("work")).unwrap();
    let mut command = Command::new("/bin/sh");
    command
        .env_clear()
        .args(["-c", r#"f=$1; shift; exec "$@" 3<"$f""#, "sh"])
        .arg(&secret_file)
        .arg(LAUNCHER)
        .arg("serve")
        .arg("--dir")
        .arg(session_dir(dir))
        .args(["--secret-fd", "3", "--secret-env", "MANTLE_ADV_TOKEN"])
        .arg("--cwd")
        .arg(dir.join("work"))
        .arg("--")
        .args(program)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    command
}

fn wait_for_server(dir: &Path, server: &mut Child) {
    let ctl = session_dir(dir).join("ctl");
    let deadline = Instant::now() + WAIT;
    while !ctl.exists() {
        assert!(
            server.try_wait().unwrap().is_none(),
            "server exited during start-up"
        );
        assert!(Instant::now() < deadline, "server never created {ctl:?}");
        sleep(Duration::from_millis(20));
    }
}

fn wait_for_file(path: &Path, within: Duration) -> Option<String> {
    let deadline = Instant::now() + within;
    loop {
        if let Ok(text) = fs::read_to_string(path)
            && !text.is_empty()
        {
            return Some(text);
        }
        if Instant::now() >= deadline {
            return None;
        }
        sleep(Duration::from_millis(20));
    }
}

fn wait_exit(child: &mut Child, what: &str) -> ExitStatus {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            return status;
        }
        if Instant::now() >= deadline {
            child.kill().ok();
            panic!("{what} did not exit");
        }
        sleep(Duration::from_millis(20));
    }
}

fn signal(pid: &str, name: &str) {
    Command::new("kill")
        .args([&format!("-{name}"), pid])
        .stderr(Stdio::null())
        .status()
        .unwrap();
}

/// True while `pid` names a process that has not yet exited (a zombie counts as exited).
fn alive(pid: &str) -> bool {
    match fs::read_to_string(format!("/proc/{pid}/stat")) {
        Ok(stat) => stat
            .rsplit_once(") ")
            .is_some_and(|(_, rest)| !rest.starts_with('Z')),
        Err(_) => false,
    }
}

struct Collected {
    bytes: Arc<Mutex<Vec<u8>>>,
    thread: JoinHandle<()>,
}

impl Collected {
    fn new(mut stream: impl Read + Send + 'static) -> Self {
        let bytes = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&bytes);
        let thread = thread::spawn(move || {
            let mut buf = [0u8; 4096];
            while let Ok(n) = stream.read(&mut buf) {
                if n == 0 {
                    break;
                }
                sink.lock().unwrap().extend_from_slice(&buf[..n]);
            }
        });
        Self { bytes, thread }
    }

    fn finish(self) -> String {
        self.thread.join().unwrap();
        String::from_utf8_lossy(&self.bytes.lock().unwrap()).into_owned()
    }
}

/// SIGTERM to `serve` must end every process of the agent's group, not only its leader. A
/// member that ignores SIGTERM and SIGHUP outlives a leader that does not, and `serve` returns
/// as soon as the leader is reaped, before its own STOP_TIMEOUT escalation to SIGKILL.
#[test]
fn sigterm_to_serve_ends_the_agents_whole_process_group() {
    let dir = scratch("group");
    let pidfile = dir.join("straggler.pid");
    let script = format!(
        r#"(trap '' TERM HUP; exec sleep 300) & echo $! > '{}'; wait"#,
        pidfile.display()
    );
    let mut server = serve(&dir, &["/bin/sh", "-c", &script]).spawn().unwrap();
    let err = Collected::new(server.stderr.take().unwrap());
    wait_for_server(&dir, &mut server);
    let pid = wait_for_file(&pidfile, WAIT)
        .expect("agent never wrote its child's pid")
        .trim()
        .to_owned();
    assert!(alive(&pid), "straggler {pid} not running before the stop");

    signal(&server.id().to_string(), "TERM");
    let status = wait_exit(&mut server, "server");
    sleep(Duration::from_millis(200));
    let survived = alive(&pid);
    if survived {
        signal(&pid, "KILL");
    }
    let err = err.finish();
    assert!(
        !survived,
        "serve exited ({status}) while pid {pid} of the agent's process group still runs; serve said: {err}"
    );
    fs::remove_dir_all(dir).ok();
}

/// `last-output` is written beside the pipes when the agent ends. A symlink planted at that
/// name by a process sharing `/workspace` must not redirect the write.
#[test]
fn last_output_does_not_follow_a_planted_symlink() {
    let dir = scratch("symlink");
    let session = session_dir(&dir);
    fs::create_dir_all(&session).unwrap();
    fs::set_permissions(&session, fs::Permissions::from_mode(0o700)).unwrap();
    let victim = dir.join("victim");
    fs::write(&victim, "untouched").unwrap();
    symlink(&victim, session.join("last-output")).unwrap();

    let output = serve(&dir, &["/bin/sh", "-c", "printf agent-marker-5e1"])
        .output()
        .unwrap();
    let err = String::from_utf8_lossy(&output.stderr);
    let content = fs::read_to_string(&victim).unwrap();
    assert_eq!(
        content, "untouched",
        "serve wrote the agent's last output through a symlink to {victim:?}; serve said: {err}"
    );
    fs::remove_dir_all(dir).ok();
}

/// `last-output` holds the agent's last screenful and is meant to be owner-only (0600) whatever
/// was at that path before.
#[test]
fn last_output_is_owner_only_even_when_the_file_already_exists() {
    let dir = scratch("lastmode");
    let session = session_dir(&dir);
    fs::create_dir_all(&session).unwrap();
    fs::set_permissions(&session, fs::Permissions::from_mode(0o700)).unwrap();
    let last = session.join("last-output");
    fs::write(&last, "old").unwrap();
    fs::set_permissions(&last, fs::Permissions::from_mode(0o644)).unwrap();

    let output = serve(&dir, &["/bin/sh", "-c", "printf agent-marker-9b3"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let text = fs::read_to_string(&last).unwrap();
    assert!(text.contains("agent-marker-9b3"), "{text:?}");
    let mode = fs::metadata(&last).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o600, "last-output left at mode {mode:o}");
    fs::remove_dir_all(dir).ok();
}

/// `--dir` is created 0700; a directory already at that path keeps whatever mode it had.
#[test]
fn session_dir_is_owner_only_even_when_it_already_exists() {
    let dir = scratch("dirmode");
    let session = session_dir(&dir);
    fs::create_dir_all(&session).unwrap();
    fs::set_permissions(&session, fs::Permissions::from_mode(0o755)).unwrap();

    let output = serve(&dir, &["/bin/sh", "-c", "true"]).output().unwrap();
    assert!(output.status.success(), "{output:?}");
    let mode = fs::metadata(&session).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o700, "session directory left at mode {mode:o}");
    fs::remove_dir_all(dir).ok();
}

/// A symlink planted at `--dir` must not move the pipes, locks and `last-output` elsewhere.
#[test]
fn a_symlinked_session_dir_is_not_followed() {
    let dir = scratch("dirlink");
    let elsewhere = dir.join("elsewhere");
    fs::create_dir_all(&elsewhere).unwrap();
    symlink(&elsewhere, session_dir(&dir)).unwrap();

    let output = serve(&dir, &["/bin/sh", "-c", "printf agent-marker-c04"])
        .output()
        .unwrap();
    let written: Vec<String> = fs::read_dir(&elsewhere)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert!(
        written.is_empty(),
        "serve ({}) followed the symlinked --dir and wrote {written:?} into {elsewhere:?}",
        output.status
    );
    fs::remove_dir_all(dir).ok();
}

/// An attached terminal that stops reading (a stalled channel) must not stall the agent: its
/// output keeps being drained into the bounded buffers.
#[test]
fn a_client_that_never_reads_does_not_block_the_agent() {
    let dir = scratch("stalled");
    let done = dir.join("done");
    let script = format!(
        "sleep 1; yes adversary-line | head -c 3000000; echo ok > '{}'; sleep 30",
        done.display()
    );
    let mut server = serve(&dir, &["/bin/sh", "-c", &script]).spawn().unwrap();
    let server_err = Collected::new(server.stderr.take().unwrap());
    wait_for_server(&dir, &mut server);
    // stdout is piped and never read: attach blocks on it, and stops reading `out`.
    let mut client = Command::new(LAUNCHER)
        .args(["attach", "--no-tty", "--dir"])
        .arg(session_dir(&dir))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let finished = wait_for_file(&done, Duration::from_secs(20)).is_some();
    client.kill().ok();
    client.wait().ok();
    signal(&server.id().to_string(), "TERM");
    wait_exit(&mut server, "server");
    let err = server_err.finish();
    assert!(
        finished,
        "the agent never finished writing while a client did not read; serve said: {err}"
    );
    fs::remove_dir_all(dir).ok();
}

/// A writer that keeps `ctl` full must not starve the agent's terminal: `read_ctl` loops until
/// the pipe is empty, so while it never empties no agent output is drained and the agent blocks.
#[test]
fn a_flooded_ctl_pipe_does_not_stall_the_agent() {
    let dir = scratch("ctlflood");
    let done = dir.join("done");
    let script = format!(
        "sleep 1; yes adversary-line | head -c 1000000; echo ok > '{}'; sleep 30",
        done.display()
    );
    let mut server = serve(&dir, &["/bin/sh", "-c", &script]).spawn().unwrap();
    let server_err = Collected::new(server.stderr.take().unwrap());
    wait_for_server(&dir, &mut server);

    let stop = Arc::new(AtomicBool::new(false));
    let flood = {
        let stop = Arc::clone(&stop);
        let mut ctl = fs::OpenOptions::new()
            .write(true)
            .open(session_dir(&dir).join("ctl"))
            .unwrap();
        thread::spawn(move || {
            let chunk = "80 24\n".repeat(10_000);
            while !stop.load(Ordering::SeqCst) {
                if ctl.write_all(chunk.as_bytes()).is_err() {
                    break;
                }
            }
        })
    };
    let finished = wait_for_file(&done, Duration::from_secs(8)).is_some();
    stop.store(true, Ordering::SeqCst);
    flood.join().unwrap();
    signal(&server.id().to_string(), "TERM");
    wait_exit(&mut server, "server");
    let err = server_err.finish();
    assert!(
        finished,
        "the agent made no progress for 8s while ctl was flooded; serve said: {err}"
    );
    fs::remove_dir_all(dir).ok();
}

/// `serve` exits with the agent's code, and 128+N when the agent dies of signal N.
#[test]
fn exit_code_is_the_agents() {
    let dir = scratch("exit");
    let status = serve(&dir, &["/bin/sh", "-c", "exit 7"]).status().unwrap();
    assert_eq!(status.code(), Some(7));
    let status = serve(&dir, &["/bin/sh", "-c", "kill -USR1 $$"])
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(128 + 10));
    fs::remove_dir_all(dir).ok();
}

/// The credential descriptor is closed before the agent starts; the value reaches the agent
/// only as the named variable, and never `last-output` unless the agent itself prints it.
#[test]
fn the_secret_descriptor_does_not_reach_the_agent() {
    let dir = scratch("fd3");
    let seen = dir.join("seen");
    let script = format!(
        r#"if [ -e /proc/self/fd/3 ]; then r=open; else r=closed; fi; printf '%s|%s' "$r" "${{#MANTLE_ADV_TOKEN}}" > '{}'"#,
        seen.display()
    );
    let output = serve(&dir, &["/bin/sh", "-c", &script]).output().unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        fs::read_to_string(&seen).unwrap(),
        format!("closed|{}", SECRET.len())
    );
    let last = fs::read(session_dir(&dir).join("last-output")).unwrap_or_default();
    assert!(!String::from_utf8_lossy(&last).contains(SECRET));
    assert!(!String::from_utf8_lossy(&output.stderr).contains(SECRET));
    fs::remove_dir_all(dir).ok();
}
