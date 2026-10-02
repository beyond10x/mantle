use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::{
    ffi::{OsStrExt, OsStringExt},
    fs::{FileTypeExt, PermissionsExt},
    process::CommandExt,
};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail, ensure};
use clap::Parser;
use mantle_conformance::{Boundary, Reply, bytes, encoded, integer};
use mantle_launch::{cli, ctl, ring, secret};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const LAUNCHER: &str = env!("CARGO_BIN_EXE_mantle-launch");
const PROBE: &str = env!("CARGO_BIN_EXE_mantle-launch-probe");
const CREDENTIAL: &[u8] = b"fixture-credential\n";

fn arguments(input: &Value) -> Result<Vec<OsString>> {
    input
        .as_array()
        .context("argv array")?
        .iter()
        .map(|v| Ok(OsString::from_vec(bytes(v)?)))
        .collect()
}
fn parse_args(input: &Value) -> Result<Value> {
    let mut argv = vec![OsString::from("mantle-launch")];
    argv.extend(arguments(&input["argv"])?);
    Ok(match cli::Cli::try_parse_from(argv) {
        Ok(parsed) => match parsed.command {
            cli::Command::Serve(a) => {
                json!({"accepted":true,"diagnostic":null,"attach":null,"serve": {
                    "dir":encoded(a.dir.as_os_str().as_bytes()),"cwd":encoded(a.cwd.as_os_str().as_bytes()),
                    "secret_fd":a.secret_fd,"secret_env":a.secret_env,"proxy":a.proxy,"scrollback_bytes":a.scrollback_bytes,
                    "mkdirs":a.mkdirs.iter().map(|p|encoded(p.as_os_str().as_bytes())).collect::<Vec<_>>(),
                    "command":a.command.iter().map(|v|encoded(v.as_bytes())).collect::<Vec<_>>()
                }})
            }
            cli::Command::Attach(a) => {
                json!({"accepted":true,"diagnostic":null,"serve":null,"attach":{"dir":encoded(a.dir.as_os_str().as_bytes()),"no_tty":a.no_tty}})
            }
        },
        Err(e) => json!({"accepted":false,"diagnostic":e.to_string(),"serve":null,"attach":null}),
    })
}
fn scratch() -> Result<tempfile::TempDir> {
    let base = std::path::PathBuf::from(std::env::var_os("HOME").context("HOME")?)
        .join(".cache/mantle-conformance");
    fs::create_dir_all(&base)?;
    Ok(tempfile::Builder::new()
        .prefix("launch-")
        .tempdir_in(base)?)
}
fn serve(dir: &Path) -> Command {
    let mut command = Command::new(LAUNCHER);
    command
        .env_clear()
        .args(["serve", "--dir"])
        .arg(dir.join("session"))
        .arg("--cwd")
        .arg(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    command
}
#[allow(unsafe_code)]
fn credential(command: &mut Command, file: &File) {
    let fd = file.as_raw_fd();
    command.args([
        "--secret-fd",
        "3",
        "--secret-env",
        "MANTLE_CONFORMANCE_SECRET",
    ]);
    // SAFETY: after fork only async-signal-safe descriptor operations run. The parent keeps the
    // source file alive through spawn; fd 3 belongs to the launcher credential contract.
    unsafe {
        command.pre_exec(move || {
            if libc::dup2(fd, 3) == -1 || libc::fcntl(3, libc::F_SETFD, 0) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            if fd != 3 {
                libc::close(fd);
            }
            Ok(())
        });
    }
}
#[allow(unsafe_code)]
fn terminate(child: &Child) {
    // SAFETY: an owned, unreaped child cannot have had its pid reused.
    unsafe {
        libc::kill(child.id() as libc::pid_t, libc::SIGTERM);
    }
}
struct Process(Child);
impl Drop for Process {
    fn drop(&mut self) {
        if self.0.try_wait().ok().flatten().is_none() {
            terminate(&self.0);
            let until = Instant::now() + Duration::from_secs(3);
            while Instant::now() < until && self.0.try_wait().ok().flatten().is_none() {
                std::thread::sleep(Duration::from_millis(10));
            }
            self.0.kill().ok();
            self.0.wait().ok();
        }
    }
}
fn wait(child: &mut Child) -> Result<std::process::ExitStatus> {
    let until = Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        ensure!(Instant::now() < until, "child did not exit within watchdog");
        std::thread::sleep(Duration::from_millis(10));
    }
}
struct Collected {
    bytes: Arc<Mutex<Vec<u8>>>,
    thread: Option<std::thread::JoinHandle<()>>,
}
impl Collected {
    fn finish(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().expect("output collector");
        }
    }
}
fn collect(mut read: impl Read + Send + 'static) -> Collected {
    let bytes = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&bytes);
    let thread = std::thread::spawn(move || {
        let mut buf = [0; 4096];
        while let Ok(n) = read.read(&mut buf) {
            if n == 0 {
                break;
            }
            sink.lock().unwrap().extend_from_slice(&buf[..n]);
        }
    });
    Collected {
        bytes,
        thread: Some(thread),
    }
}
fn text(collected: &Collected) -> String {
    String::from_utf8_lossy(&collected.bytes.lock().unwrap()).into_owned()
}
fn await_text(collected: &Collected, wanted: &str) -> Result<()> {
    let until = Instant::now() + Duration::from_secs(10);
    while !text(collected).contains(wanted) {
        ensure!(
            Instant::now() < until,
            "missing child observation {wanted}; read {}",
            text(collected)
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}
fn attach(dir: &Path) -> Result<(Process, Collected)> {
    let mut child = Command::new(LAUNCHER)
        .env_clear()
        .args(["attach", "--no-tty", "--dir"])
        .arg(dir.join("session"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let output = collect(child.stdout.take().context("stdout")?);
    Ok((Process(child), output))
}
fn emit(input: &Value) -> Result<Value> {
    let dir = scratch()?;
    let mut command = serve(dir.path());
    command
        .arg("--")
        .arg(PROBE)
        .arg("emit")
        .arg("--exit")
        .arg(integer(&input["exit"])?.to_string())
        .arg("--")
        .args(arguments(&input["argv"])?);
    let mut process = Process(command.spawn()?);
    let mut stderr = collect(process.0.stderr.take().context("stderr")?);
    let status = wait(&mut process.0)?;
    stderr.finish();
    let output = fs::read(dir.path().join("session/last-output"))?;
    Ok(
        json!({"exit":status.code(),"output":encoded(&output),"credential_in_diagnostic":text(&stderr).contains("fixture-credential")}),
    )
}
fn lifecycle() -> Result<Value> {
    let dir = scratch()?;
    let path = dir.path();
    fs::write(path.join("credential"), CREDENTIAL)?;
    let file = File::open(path.join("credential"))?;
    let mut command = serve(path);
    credential(&mut command, &file);
    command.arg("--").arg(PROBE).arg("agent");
    let mut server = Process(command.spawn()?);
    drop(file);
    let mut server_errors = collect(server.0.stderr.take().context("stderr")?);
    let until = Instant::now() + Duration::from_secs(10);
    while !path.join("session/ctl").exists() {
        ensure!(
            server.0.try_wait()?.is_none(),
            "server exited before ready: {}",
            text(&server_errors)
        );
        ensure!(Instant::now() < until, "server not ready");
        std::thread::sleep(Duration::from_millis(10));
    }
    let (mut first, first_output) = attach(path)?;
    await_text(&first_output, "ready")?;
    await_text(
        &first_output,
        &format!(
            "credential-sha256:{:x}",
            Sha256::digest(b"fixture-credential")
        ),
    )?;
    let initial = text(&first_output);
    first
        .0
        .stdin
        .as_mut()
        .context("stdin")?
        .write_all(b"hello\n")?;
    await_text(&first_output, "got:hello")?;
    let relayed = text(&first_output).contains("got:hello");
    let (mut competitor, _) = attach(path)?;
    let duplicate_attach_refused = !wait(&mut competitor.0)?.success();
    let mut rival = serve(path);
    rival.arg("--").arg(PROBE).arg("agent");
    let mut rival = Process(rival.spawn()?);
    let duplicate_server_refused = !wait(&mut rival.0)?.success();
    let fifo_permissions = ["in", "out", "ctl"]
        .iter()
        .map(|name| -> Result<u32> {
            let metadata = fs::symlink_metadata(path.join("session").join(name))?;
            ensure!(metadata.file_type().is_fifo(), "not a FIFO");
            Ok(metadata.permissions().mode() & 0o777)
        })
        .collect::<Result<Vec<_>>>()?;
    first.0.kill()?;
    first.0.wait()?;
    drop(first);
    let survived_detach = server.0.try_wait()?.is_none();
    let (mut second, second_output) = attach(path)?;
    await_text(&second_output, "got:hello")?;
    let replayed =
        text(&second_output).contains("ready") && text(&second_output).contains("got:hello");
    let mut control =
        mantle_launch::session::open_fifo(&path.join("session/ctl"), true, libc::O_NONBLOCK)?;
    control.write_all(b"120 40\n")?;
    await_text(&second_output, "window:120x40")?;
    let resized = text(&second_output).contains("window:120x40");
    second
        .0
        .stdin
        .as_mut()
        .context("stdin")?
        .write_all(b"exit\n")?;
    let server_exit = wait(&mut server.0)?.code();
    server_errors.finish();
    wait(&mut second.0)?;
    let cleaned = ["in", "out", "ctl", "ctl.new", "server.lock"]
        .iter()
        .all(|name| !path.join("session").join(name).exists());
    let credential_fingerprint = initial
        .lines()
        .find_map(|line| line.trim().strip_prefix("credential-sha256:"))
        .map(|hex| {
            hex.as_bytes()
                .chunks_exact(2)
                .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("digest text"), 16))
                .collect::<std::result::Result<Vec<_>, _>>()
        })
        .transpose()?;
    Ok(
        json!({"relayed":relayed,"replayed":replayed,"survived_detach":survived_detach,
        "duplicate_attach_refused":duplicate_attach_refused,"duplicate_server_refused":duplicate_server_refused,
        "fifo_permissions":fifo_permissions,"resized":resized,"server_exit":server_exit,"cleaned":cleaned,
        "secret_descriptor_closed":initial.contains("descriptor3-open:false"),"credential_fingerprint":credential_fingerprint,
        "credential_in_diagnostic":text(&server_errors).contains("fixture-credential")}),
    )
}

#[derive(Default)]
struct Launcher;
impl Boundary for Launcher {
    fn execute(&mut self, command: &str, input: &Value) -> Result<Reply> {
        Ok(Reply::returned(match command {
            "mantle.launch.ParseArgs" => parse_args(input)?,
            "mantle.launch.ReadSecret" => {
                match secret::read_secret(bytes(&input["bytes"])?.as_slice()) {
                    Ok(value) => {
                        json!({"accepted":true,"digest":format!("{:x}",Sha256::digest(value.as_bytes())),"diagnostic":null})
                    }
                    Err(e) => json!({"accepted":false,"digest":null,"diagnostic":e.to_string()}),
                }
            }
            "mantle.launch.ParseWindow" => {
                let mut lines = ctl::WindowLines::default();
                let mut windows = Vec::new();
                for chunk in input["chunks"].as_array().context("chunks")? {
                    windows.extend(
                        lines
                            .feed(&bytes(chunk)?)
                            .iter()
                            .map(|w| json!({"cols":w.cols,"rows":w.rows})),
                    );
                }
                json!({"windows":windows})
            }
            "mantle.launch.ReplayRing" => {
                let mut ring = ring::Ring::new(integer(&input["capacity"])?.try_into()?);
                for chunk in input["chunks"].as_array().context("chunks")? {
                    ring.push(&bytes(chunk)?);
                }
                json!({"bytes":encoded(&ring.to_vec())})
            }
            "mantle.launch.Emit" => emit(input)?,
            "mantle.launch.Lifecycle" => lifecycle()?,
            "mantle.launch.Signals" => signals()?,
            "mantle.launch.SlowReader" => slow_reader()?,
            "mantle.launch.FileSafety" => file_safety()?,
            "mantle.launch.ValidateScrollback" => {
                let result = cli::Cli::try_parse_from([
                    "mantle-launch",
                    "serve",
                    "--dir",
                    "/session",
                    "--cwd",
                    "/workspace",
                    "--scrollback-bytes",
                    &integer(&input["bytes"])?.to_string(),
                    "--",
                    "/agent",
                ]);
                return Ok(if result.is_ok() {
                    Reply {
                        outcome: "accepted".into(),
                        response: Some(json!({"valid":true})),
                        ..Reply::default()
                    }
                } else {
                    Reply {
                        outcome: "refused".into(),
                        error: Some("mantle.launch.InvalidScrollback".into()),
                        ..Reply::default()
                    }
                });
            }
            _ => bail!("unsupported launcher command {command}"),
        }))
    }
}
#[test]
fn ess_launch_conformance() -> Result<()> {
    mantle_conformance::run::<Launcher>("mantle-launch", 8)
}

fn ready(path: &Path, server: &mut Process) -> Result<()> {
    let until = Instant::now() + Duration::from_secs(10);
    while !path.join("session/ctl").exists() {
        ensure!(server.0.try_wait()?.is_none(), "server exited before ready");
        ensure!(Instant::now() < until, "readiness watchdog");
        std::thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}
fn signals() -> Result<Value> {
    let dir = scratch()?;
    let mut cmd = serve(dir.path());
    cmd.arg("--").arg(PROBE).args(["agent", "--child"]);
    let mut server = Process(cmd.spawn()?);
    ready(dir.path(), &mut server)?;
    let (mut client, mut output) = attach(dir.path())?;
    await_text(&output, "child:")?;
    terminate(&server.0);
    let status = wait(&mut server.0)?;
    wait(&mut client.0)?;
    output.finish();
    let observed =
        String::from_utf8_lossy(&fs::read(dir.path().join("session/last-output"))?).into_owned();
    Ok(
        json!({"exit":status.code(),"signal_forwarded":observed.contains("signal:terminate"),"descendant_exited":observed.contains("descendant-exited:true")}),
    )
}
fn slow_reader() -> Result<Value> {
    let dir = scratch()?;
    let mut cmd = serve(dir.path());
    cmd.arg("--").arg(PROBE).arg("flood");
    let mut server = Process(cmd.spawn()?);
    ready(dir.path(), &mut server)?;
    let mut client = Process(
        Command::new(LAUNCHER)
            .args(["attach", "--no-tty", "--dir"])
            .arg(dir.path().join("session"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?,
    );
    // Consume only the readiness line, then deliberately leave stdout blocked.
    let mut byte = [0];
    let mut initial = Vec::new();
    while !initial.ends_with(b"ready\r\n") {
        client
            .0
            .stdout
            .as_mut()
            .context("stdout")?
            .read_exact(&mut byte)?;
        initial.push(byte[0]);
    }
    client
        .0
        .stdin
        .as_mut()
        .context("stdin")?
        .write_all(b"go\n")?;
    let status = wait(&mut server.0)?;
    let output = fs::read(dir.path().join("session/last-output"))?;
    Ok(
        json!({"exit":status.code(),"retained_bytes":output.len(),"tail_is_output":output.iter().all(|b|*b==b'x')}),
    )
}
fn file_safety() -> Result<Value> {
    use mantle_launch::session as s;
    use std::os::unix::fs::symlink;
    let dir = scratch()?;
    let p = dir.path();
    fs::create_dir(p.join("target"))?;
    fs::set_permissions(p.join("target"), fs::Permissions::from_mode(0o755))?;
    symlink(p.join("target"), p.join("directory-link"))?;
    let directory_link_refused = s::prepare_dir(&p.join("directory-link")).is_err();
    fs::write(p.join("file"), b"unchanged")?;
    symlink(p.join("file"), p.join("file-link"))?;
    let lock_link_refused = s::try_lock(&p.join("file-link")).is_err();
    let fifo_link_refused = s::open_fifo(&p.join("file-link"), false, libc::O_NONBLOCK).is_err();
    let regular_fifo_refused = s::open_fifo(&p.join("file"), false, libc::O_NONBLOCK).is_err();
    let remove_regular_refused = s::remove_fifo(&p.join("file")).is_err();
    s::prepare_dir(&p.join("private"))?;
    let lock = s::try_lock(&p.join("private/lock"))?.context("lock")?;
    let duplicate_refused = s::try_lock(&p.join("private/lock"))?.is_none();
    drop(lock);
    let reacquired = s::try_lock(&p.join("private/lock"))?.is_some();
    Ok(
        json!({"directory_link_refused":directory_link_refused,"lock_link_refused":lock_link_refused,
    "fifo_link_refused":fifo_link_refused,"regular_fifo_refused":regular_fifo_refused,"remove_regular_refused":remove_regular_refused,
    "target_mode":fs::metadata(p.join("target"))?.permissions().mode()&0o777,"file_bytes":encoded(&fs::read(p.join("file"))?),
    "directory_mode":fs::metadata(p.join("private"))?.permissions().mode()&0o777,"duplicate_refused":duplicate_refused,"reacquired":reacquired}),
    )
}
