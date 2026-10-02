//! Local negative controls. These exercise a test seccomp filter, not Substrate or Codex.
#![allow(unsafe_code)]

use std::fs::{self, DirBuilder, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::os::unix::net::UnixListener;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

const LAUNCHER: &str = env!("CARGO_BIN_EXE_mantle-launch");

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let base = std::env::var_os("TMPDIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(std::env::var_os("HOME").unwrap()).join(".cache"));
        let path = base.join(format!("mantle-codex-control-{}", std::process::id()));
        DirBuilder::new().mode(0o700).create(&path).unwrap();
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
struct Process(Child);
impl Drop for Process {
    fn drop(&mut self) {
        // This PID belongs to this test. SIGTERM asks the launcher to reap its PTY group.
        unsafe {
            libc::kill(self.0.id() as i32, libc::SIGTERM);
        }
        let end = Instant::now() + Duration::from_secs(12);
        while self.0.try_wait().unwrap().is_none() && Instant::now() < end {
            sleep(Duration::from_millis(20));
        }
        self.0.kill().ok();
        self.0.wait().ok();
    }
}

/// Deliberately small local control, not a copy of Substrate's complete confinement policy.
/// Refuses socket(AF_UNIX, ...) with EPERM. Substrate permits stream socketpairs
/// for process-local IPC (including Rust's child-spawn error channel); this control
/// deliberately does not model its additional datagram-socketpair restriction.
fn deny_unix_sockets() -> std::io::Result<()> {
    let statement = |code, k| libc::sock_filter {
        code,
        jt: 0,
        jf: 0,
        k,
    };
    let jump = |k, jt, jf| libc::sock_filter {
        code: 0x15,
        jt,
        jf,
        k,
    };
    let filter = [
        statement(0x20, 0), // seccomp_data.nr
        jump(libc::SYS_socket as u32, 0, 3),
        statement(0x20, 16), // seccomp_data.args[0] (Linux little-endian targets)
        jump(libc::AF_UNIX as u32, 0, 1),
        statement(0x06, libc::SECCOMP_RET_ERRNO | libc::EPERM as u32),
        statement(0x06, libc::SECCOMP_RET_ALLOW),
    ];
    let program = libc::sock_fprog {
        len: filter.len() as u16,
        filter: filter.as_ptr().cast_mut(),
    };
    // Invoked only in the newly forked child's pre_exec; filter pointer lives through prctl.
    if unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) } != 0
        || unsafe { libc::prctl(libc::PR_SET_SECCOMP, libc::SECCOMP_MODE_FILTER, &program) } != 0
    {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

#[test]
fn local_socket_control_rejects_af_unix() {
    if std::env::var_os("MANTLE_SOCKET_CONTROL_CHILD").is_some() {
        let error = UnixListener::bind("/socket-should-be-refused-before-path-lookup").unwrap_err();
        assert_eq!(error.raw_os_error(), Some(libc::EPERM));
        return;
    }
    let mut cmd = Command::new(std::env::current_exe().unwrap());
    cmd.args([
        "--exact",
        "local_socket_control_rejects_af_unix",
        "--nocapture",
    ])
    .env("MANTLE_SOCKET_CONTROL_CHILD", "1");
    unsafe {
        cmd.pre_exec(deny_unix_sockets);
    }
    assert!(cmd.status().unwrap().success());
}

#[test]
fn fifo_launcher_relays_without_unix_sockets_or_secret_slot() {
    let scratch = Scratch::new();
    let session = scratch.0.join("session");
    let mut cmd = Command::new(LAUNCHER);
    cmd.env_clear()
        .args(["serve", "--dir"])
        .arg(&session)
        .arg("--cwd")
        .arg(&scratch.0)
        .args(["--", "/bin/cat"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit());
    unsafe {
        cmd.pre_exec(deny_unix_sockets);
    }
    let mut process = Process(cmd.spawn().unwrap());
    let deadline = Instant::now() + Duration::from_secs(5);
    while !session.join("ctl").exists() {
        assert!(process.0.try_wait().unwrap().is_none(), "launcher exited");
        assert!(Instant::now() < deadline, "launcher never became ready");
        sleep(Duration::from_millis(10));
    }
    let mut output = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(session.join("out"))
        .unwrap();
    let mut input = OpenOptions::new()
        .write(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(session.join("in"))
        .unwrap();
    input.write_all(b"cq-fifo-control\n").unwrap();
    let mut bytes = Vec::new();
    while !bytes
        .windows(b"cq-fifo-control".len())
        .any(|x| x == b"cq-fifo-control")
    {
        assert!(Instant::now() < deadline, "no relay response");
        let mut chunk = [0; 1024];
        match output.read(&mut chunk) {
            Ok(n) => bytes.extend_from_slice(&chunk[..n]),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => panic!("relay read: {e}"),
        }
        assert!(bytes.len() <= 4096, "control output exceeded bound");
        sleep(Duration::from_millis(10));
    }
    drop(process);
}
