//! Adversarial cases, pass 2, for `mantle-launch serve` and `attach`: whether the redraw that
//! `serve` forces on attach, and after the attached terminal lost output, makes an agent repaint.
//!
//! The agent here is a shell that repaints when it sees its window size change, which is how
//! Node and Bun (Claude Code's runtime) decide: a SIGWINCH whose size equals the last one read
//! emits no `resize` event, so Ink-based programs do not re-render.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle, sleep};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const LAUNCHER: &str = env!("CARGO_BIN_EXE_mantle-launch");
const SECRET: &str = "adv2-secret-3a90";
const WAIT: Duration = Duration::from_secs(10);

/// An agent that prints `REDRAW <rows> <cols>` whenever a SIGWINCH finds a size different from
/// the last one it read, and `ready` once its handler is in place.
const SIZE_AWARE_AGENT: &str = r#"last=$(stty size)
trap 'now=$(stty size); if [ "$now" != "$last" ]; then echo "REDRAW $now"; last=$now; fi' WINCH
echo ready
"#;

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
        "mantle-launch-adv2-{name}-{}-{nanos}-{}",
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
fn serve(dir: &Path, script: &str) -> Child {
    let secret_file = dir.join("secret");
    fs::write(&secret_file, format!("{SECRET}\n")).unwrap();
    fs::create_dir_all(dir.join("work")).unwrap();
    let mut child = Command::new("/bin/sh")
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .args(["-c", r#"f=$1; shift; exec "$@" 3<"$f""#, "sh"])
        .arg(&secret_file)
        .arg(LAUNCHER)
        .arg("serve")
        .arg("--dir")
        .arg(session_dir(dir))
        .args(["--secret-fd", "3", "--secret-env", "MANTLE_ADV2_TOKEN"])
        .arg("--cwd")
        .arg(dir.join("work"))
        .args(["--", "/bin/sh", "-c", script])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let ctl = session_dir(dir).join("ctl");
    let deadline = Instant::now() + WAIT;
    while !ctl.exists() {
        assert!(
            child.try_wait().unwrap().is_none(),
            "server exited during start-up"
        );
        assert!(Instant::now() < deadline, "server never created {ctl:?}");
        sleep(Duration::from_millis(20));
    }
    child
}

fn attach(dir: &Path) -> Child {
    Command::new(LAUNCHER)
        .args(["attach", "--no-tty", "--dir"])
        .arg(session_dir(dir))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap()
}

fn stop(server: &mut Child) -> ExitStatus {
    Command::new("kill")
        .args(["-TERM", &server.id().to_string()])
        .status()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if let Some(status) = server.try_wait().unwrap() {
            return status;
        }
        if Instant::now() >= deadline {
            server.kill().ok();
            panic!("server did not exit");
        }
        sleep(Duration::from_millis(20));
    }
}

struct Collected {
    bytes: Arc<Mutex<Vec<u8>>>,
    _thread: JoinHandle<()>,
}

impl Collected {
    fn new(mut stream: impl Read + Send + 'static) -> Self {
        let bytes = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&bytes);
        let thread = thread::spawn(move || {
            let mut buf = [0u8; 65536];
            while let Ok(n) = stream.read(&mut buf) {
                if n == 0 {
                    break;
                }
                sink.lock().unwrap().extend_from_slice(&buf[..n]);
            }
        });
        Self {
            bytes,
            _thread: thread,
        }
    }

    fn text(&self) -> String {
        String::from_utf8_lossy(&self.bytes.lock().unwrap()).into_owned()
    }

    /// Waits until `needle` appears after byte `from`; returns where it ends.
    fn wait_for(&self, needle: &str, from: usize, within: Duration) -> Option<usize> {
        let deadline = Instant::now() + within;
        loop {
            let text = self.text();
            if let Some(at) = text.get(from..).and_then(|rest| rest.find(needle)) {
                return Some(from + at + needle.len());
            }
            if Instant::now() >= deadline {
                return None;
            }
            sleep(Duration::from_millis(20));
        }
    }
}

/// Control for the cases below: the size-aware agent does repaint when its window really
/// changes, so a missing `REDRAW` there is serve's doing and not the agent's.
#[test]
fn a_size_aware_agent_repaints_on_a_real_resize() {
    let dir = scratch("control");
    let mut server = serve(
        &dir,
        &format!("{SIZE_AWARE_AGENT}while :; do sleep 0.05; done"),
    );
    let mut client = attach(&dir);
    let out = Collected::new(client.stdout.take().unwrap());
    let ready = out.wait_for("ready", 0, WAIT).expect("agent never ready");
    fs::write(session_dir(&dir).join("ctl"), "120 40\n").unwrap();
    let redrawn = out.wait_for("REDRAW 40 120", ready, Duration::from_secs(5));
    client.kill().ok();
    client.wait().ok();
    stop(&mut server);
    assert!(
        redrawn.is_some(),
        "the model agent did not repaint on a real resize; got {:?}",
        out.text()
    );
    fs::remove_dir_all(dir).ok();
}

/// A terminal attaching at the size the agent already has gets the scrollback and then, by
/// `Server::attach` -> `redraw`, a forced repaint. The redraw sets rows-1 and restores the size
/// back to back, so by the time the agent reads its size nothing has changed and it does not
/// repaint. Real Claude Code 2.1.287 under `serve` sent 0 bytes after the replay on 3 attaches.
#[test]
fn attaching_at_an_unchanged_size_makes_the_agent_repaint() {
    let dir = scratch("attach-redraw");
    let mut server = serve(
        &dir,
        &format!("{SIZE_AWARE_AGENT}while :; do sleep 0.05; done"),
    );
    // Let the agent install its trap before any terminal attaches.
    sleep(Duration::from_millis(500));
    let mut client = attach(&dir);
    let out = Collected::new(client.stdout.take().unwrap());
    let ready = out.wait_for("ready", 0, WAIT).expect("agent never ready");
    let redrawn = out.wait_for("REDRAW", ready, Duration::from_secs(3));
    client.kill().ok();
    client.wait().ok();
    stop(&mut server);
    assert!(
        redrawn.is_some(),
        "serve's attach redraw did not make a size-aware agent repaint; terminal got {:?}",
        out.text()
    );
    fs::remove_dir_all(dir).ok();
}

/// The correction's `Repaint`: once a slow terminal lost output to the full `pending` ring and
/// has caught up, serve forces a redraw. It uses the same back-to-back resize, so the agent
/// never repaints and the terminal keeps the screen the dropped bytes garbled.
#[test]
fn a_terminal_that_lost_output_gets_a_repaint_once_it_catches_up() {
    let dir = scratch("overflow-redraw");
    let mut server = serve(
        &dir,
        &format!(
            "{SIZE_AWARE_AGENT}sleep 1; head -c 3000000 /dev/zero | tr '\\0' x; echo; echo FLOODED; \
             while :; do sleep 0.05; done"
        ),
    );
    sleep(Duration::from_millis(500));
    // stdout is piped and not read yet: attach blocks on it and stops draining `out`, so the
    // 3 MB flood overflows serve's 256 KiB `pending` ring.
    let mut client = attach(&dir);
    let stdout = client.stdout.take().unwrap();
    sleep(Duration::from_secs(4));
    let out = Collected::new(stdout);
    let flooded = out
        .wait_for("FLOODED", 0, Duration::from_secs(15))
        .expect("the terminal never caught up with the flood");
    let redrawn = out.wait_for("REDRAW", flooded, Duration::from_secs(3));
    client.kill().ok();
    client.wait().ok();
    stop(&mut server);
    let text = out.text();
    assert!(
        redrawn.is_some(),
        "no repaint after the terminal lost output and caught up; {} bytes received, tail {:?}",
        text.len(),
        &text[text.len().saturating_sub(80)..]
    );
    fs::remove_dir_all(dir).ok();
}
