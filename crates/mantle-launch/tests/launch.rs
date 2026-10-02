use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle, sleep};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const LAUNCHER: &str = env!("CARGO_BIN_EXE_mantle-launch");
const SECRET: &str = "it-secret-4f9c";
const WAIT: Duration = Duration::from_secs(10);
const FIFOS: [&str; 3] = ["in", "out", "ctl"];

/// Scratch space under $HOME, never /tmp.
fn scratch(name: &str) -> PathBuf {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let dir = PathBuf::from(std::env::var_os("HOME").expect("HOME"))
        .join(".cache/claude-tmp")
        .join(format!(
            "mantle-launch-test-{name}-{}-{nanos}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::SeqCst)
        ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn session_dir(dir: &Path) -> PathBuf {
    dir.join("session")
}

/// Runs `serve` through `sh` so the secret file arrives on fd 3, as the sandbox delivers it,
/// with an environment as empty as the sandbox's.
fn serve(dir: &Path, program: &[&str]) -> Command {
    let secret_file = dir.join("secret");
    fs::write(&secret_file, format!("{SECRET}\n")).unwrap();
    let mut command = Command::new("/bin/sh");
    command
        .env_clear()
        .args(["-c", r#"f=$1; shift; exec "$@" 3<"$f""#, "sh"])
        .arg(&secret_file)
        .arg(LAUNCHER)
        .arg("serve")
        .arg("--dir")
        .arg(session_dir(dir))
        .args(["--secret-fd", "3", "--secret-env", "MANTLE_IT_TOKEN"])
        .args(["--proxy", "http://127.0.0.1:3128"])
        .arg("--cwd")
        .arg(dir.join("work"))
        .arg("--mkdir")
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

/// Collects a child's stream on a thread.
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

    fn text(&self) -> String {
        String::from_utf8_lossy(&self.bytes.lock().unwrap()).into_owned()
    }

    fn wait_for(&self, needle: &str) {
        let deadline = Instant::now() + WAIT;
        while !self.text().contains(needle) {
            assert!(
                Instant::now() < deadline,
                "never saw {needle:?}; got {:?}",
                self.text()
            );
            sleep(Duration::from_millis(20));
        }
    }

    fn finish(self) -> String {
        self.thread.join().unwrap();
        String::from_utf8_lossy(&self.bytes.lock().unwrap()).into_owned()
    }
}

struct Client {
    child: Child,
    stdin: ChildStdin,
    stdout: Collected,
    stderr: Collected,
}

fn attach(dir: &Path) -> Client {
    let mut child = Command::new(LAUNCHER)
        .args(["attach", "--no-tty", "--dir"])
        .arg(session_dir(dir))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let stdin = child.stdin.take().unwrap();
    let stdout = Collected::new(child.stdout.take().unwrap());
    let stderr = Collected::new(child.stderr.take().unwrap());
    Client {
        child,
        stdin,
        stdout,
        stderr,
    }
}

fn wait_exit(child: &mut Child, what: &str) -> ExitStatus {
    let deadline = Instant::now() + Duration::from_secs(20);
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

fn assert_cleaned_up(dir: &Path) {
    for name in FIFOS.iter().chain(&["ctl.new", "server.lock"]) {
        let path = session_dir(dir).join(name);
        assert!(!path.exists(), "{path:?} left behind");
    }
}

#[test]
fn attach_relays_replays_scrollback_and_survives_a_detach() {
    let dir = scratch("relay");
    let seen = dir.join("seen");
    let script = format!(
        r#"printf '%s|%s|%s|%s' "$MANTLE_IT_TOKEN" "$TERM" "$HTTPS_PROXY" "$NO_PROXY" > '{}'; pwd >> '{}'; printf ready; read line; printf "got:%s" "$line"; read line; printf "bye:%s" "$line""#,
        seen.display(),
        seen.display()
    );
    let mut server = serve(&dir, &["/bin/sh", "-c", &script]).spawn().unwrap();
    let server_err = Collected::new(server.stderr.take().unwrap());
    wait_for_server(&dir, &mut server);
    for name in FIFOS {
        let meta = fs::symlink_metadata(session_dir(&dir).join(name)).unwrap();
        assert!(meta.file_type().is_fifo(), "{name} is not a FIFO");
    }

    let mut first = attach(&dir);
    first.stdout.wait_for("ready");
    first.stdin.write_all(b"hello\n").unwrap();
    first.stdout.wait_for("got:hello");

    let mut rival = attach(&dir);
    let status = wait_exit(&mut rival.child, "second attach");
    drop(rival.stdin);
    assert!(!status.success());
    let err = rival.stderr.finish();
    assert!(err.contains("another terminal is attached"), "{err}");

    // Closing the PTY session kills the client; the server must carry on.
    first.child.kill().unwrap();
    first.child.wait().unwrap();
    drop(first.stdin);
    first.stdout.finish();
    sleep(Duration::from_millis(300));
    assert!(
        server.try_wait().unwrap().is_none(),
        "server died with its client"
    );

    let mut second = attach(&dir);
    second.stdout.wait_for("got:hello");
    let replay = second.stdout.text();
    assert!(
        replay.contains("ready"),
        "scrollback not replayed: {replay:?}"
    );
    second.stdin.write_all(b"done\n").unwrap();
    second.stdout.wait_for("bye:done");

    let status = wait_exit(&mut server, "server");
    let err = server_err.finish();
    assert!(status.success(), "status {status}: {err}");
    assert!(err.contains("agent exited"), "{err}");
    assert!(!err.contains(SECRET), "secret leaked to stderr");
    let client_status = wait_exit(&mut second.child, "attach");
    assert!(client_status.success(), "attach {client_status}");
    drop(second.stdin);
    let client_out = second.stdout.finish();
    assert!(!client_out.contains(SECRET), "secret reached the terminal");
    assert_eq!(second.stderr.finish(), "");

    assert_cleaned_up(&dir);
    let seen = fs::read_to_string(&seen).unwrap();
    assert_eq!(
        seen,
        format!(
            "{SECRET}|xterm-256color|http://127.0.0.1:3128|localhost,127.0.0.1{}\n",
            dir.join("work").display()
        )
    );
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn second_server_is_refused_and_sigterm_stops_the_first() {
    let dir = scratch("live");
    let mut first = serve(&dir, &["/bin/sh", "-c", "sleep 60"]).spawn().unwrap();
    let first_err = Collected::new(first.stderr.take().unwrap());
    wait_for_server(&dir, &mut first);

    let second = serve(&dir, &["/bin/sh", "-c", "sleep 1"]).output().unwrap();
    let err = String::from_utf8_lossy(&second.stderr);
    assert!(!second.status.success());
    assert!(err.contains("a session server is already running"), "{err}");
    assert!(!err.contains(SECRET), "secret leaked to stderr");
    for name in FIFOS {
        assert!(
            session_dir(&dir).join(name).exists(),
            "refused server removed {name}"
        );
    }

    let started = Instant::now();
    let killed = Command::new("kill")
        .args(["-TERM", &first.id().to_string()])
        .status()
        .unwrap();
    assert!(killed.success());
    let status = wait_exit(&mut first, "server");
    assert!(started.elapsed() < Duration::from_secs(15));
    assert!(!status.success());
    let err = first_err.finish();
    assert!(err.contains("received signal 15"), "{err}");
    assert!(err.contains("agent exited"), "{err}");
    assert!(!err.contains(SECRET), "secret leaked to stderr");
    assert_cleaned_up(&dir);
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn secret_stays_out_of_stderr_when_the_agent_cannot_start() {
    let dir = scratch("nostart");
    let missing = dir.join("no-such-agent");
    let output = serve(&dir, &[missing.to_str().unwrap(), SECRET])
        .output()
        .unwrap();
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(err.contains("cannot start"), "{err}");
    assert!(!err.contains(SECRET), "secret leaked to stderr: {err}");
    assert_cleaned_up(&dir);
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn attach_without_a_server_is_refused() {
    let dir = scratch("noserver");
    fs::create_dir_all(session_dir(&dir)).unwrap();
    let output = Command::new(LAUNCHER)
        .args(["attach", "--no-tty", "--dir"])
        .arg(session_dir(&dir))
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(!output.status.success());
    let err = String::from_utf8_lossy(&output.stderr);
    assert!(err.contains("no session server is running"), "{err}");
    assert!(output.stdout.is_empty());
    fs::remove_dir_all(dir).unwrap();
}
