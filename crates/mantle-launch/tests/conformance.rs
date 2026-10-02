use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::fd::{AsFd, AsRawFd, OwnedFd};
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
                    "volatile_replay":a.volatile_replay,
                    "private_dirs":a.private_dirs.iter().map(|p|encoded(p.as_os_str().as_bytes())).collect::<Vec<_>>(),
                    "volatile_dirs":a.volatile_dirs.iter().map(|p|encoded(p.as_os_str().as_bytes())).collect::<Vec<_>>(),
                    "check_private_files":a.check_private_files.iter().map(|p|encoded(p.as_os_str().as_bytes())).collect::<Vec<_>>(),
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
    let base = match std::env::var_os("MANTLE_TEST_SCRATCH") {
        Some(path) => std::path::PathBuf::from(path),
        None => std::path::PathBuf::from(std::env::var_os("HOME").context("HOME")?)
            .join(".cache/mantle-conformance"),
    };
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
            "mantle.launch.PrivatePaths" => private_paths()?,
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
            "mantle.launch.VolatileReplayLifecycle" => volatile_lifecycle()?,
            "mantle.launch.VolatileReplayExitPaths" => volatile_exit_paths()?,
            "mantle.launch.VolatileReplayPreflight" => volatile_preflight()?,
            "mantle.launch.VolatileReplayBounds" => replay_bounds(true)?,
            "mantle.launch.PersistentReplayCompatibility" => replay_bounds(false)?,
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

const OUTPUT_CANARY: &[u8] = b"launcher-output-canary-7f8433";
const OBSERVATION_LIMIT: usize = 64 * 1024;

/// One reader owns each descriptor. Polling bounds every read; no collector thread can outlive
/// its fixture, and overflow fails the observation instead of allocating unbounded memory.
struct LimitedOutput {
    file: File,
    bytes: Vec<u8>,
}
impl LimitedOutput {
    fn new(fd: impl Into<OwnedFd>) -> Self {
        Self {
            file: File::from(fd.into()),
            bytes: Vec::new(),
        }
    }
    fn read_more(&mut self, until: Instant) -> Result<bool> {
        loop {
            ensure!(Instant::now() < until, "output observation deadline");
            let mut fds = [mantle_launch::sys::pollfd(
                Some(self.file.as_fd()),
                libc::POLLIN,
            )];
            mantle_launch::sys::poll(&mut fds, Duration::from_millis(20))?;
            if fds[0].revents == 0 {
                continue;
            }
            let mut buffer = [0; 4096];
            let n = self.file.read(&mut buffer)?;
            ensure!(
                self.bytes.len() + n <= OBSERVATION_LIMIT,
                "output observation overflow"
            );
            self.bytes.extend_from_slice(&buffer[..n]);
            return Ok(n != 0);
        }
    }
    fn contains(&self, wanted: &[u8]) -> bool {
        self.bytes.windows(wanted.len()).any(|part| part == wanted)
    }
    fn until(&mut self, wanted: &[u8]) -> Result<()> {
        let deadline = Instant::now() + Duration::from_secs(5);
        while !self.contains(wanted) {
            ensure!(
                self.read_more(deadline)?,
                "output ended before fixture observation"
            );
        }
        Ok(())
    }
    fn finish(&mut self) -> Result<()> {
        let deadline = Instant::now() + Duration::from_secs(5);
        while self.read_more(deadline)? {}
        Ok(())
    }
}

struct ReplayFixture {
    server: Process,
    errors: LimitedOutput,
    dir: tempfile::TempDir,
}
impl ReplayFixture {
    fn new(volatile: bool) -> Result<Self> {
        let dir = scratch()?;
        let mut command = replay_command(dir.path(), volatile);
        let mut server = Process(command.spawn()?);
        let errors = LimitedOutput::new(server.0.stderr.take().context("stderr")?);
        ready(dir.path(), &mut server)?;
        Ok(Self {
            dir,
            server,
            errors,
        })
    }
    fn client(&self) -> Result<(Process, LimitedOutput)> {
        let mut client = Process(
            Command::new(LAUNCHER)
                .env_clear()
                .args(["attach", "--no-tty", "--dir"])
                .arg(self.dir.path().join("session"))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()?,
        );
        let output = LimitedOutput::new(client.0.stdout.take().context("stdout")?);
        Ok((client, output))
    }
    fn input(&self, line: &[u8]) -> Result<()> {
        let mut input = mantle_launch::session::open_fifo(
            &self.dir.path().join("session/in"),
            true,
            libc::O_NONBLOCK,
        )?;
        input.write_all(line)?;
        Ok(())
    }
    fn last_output(&self) -> Result<bool> {
        entry_present(&self.dir.path().join("session/last-output"))
    }
    fn pid(&self) -> Result<u32> {
        Ok(fs::read_to_string(self.dir.path().join("dispatched"))?.parse()?)
    }
}
fn replay_command(path: &Path, volatile: bool) -> Command {
    let mut command = serve(path);
    command.args(["--scrollback-bytes", "1024"]);
    if volatile {
        command.arg("--volatile-replay");
    }
    command
        .arg("--")
        .arg(PROBE)
        .args(["replay", "--pid-file"])
        .arg(path.join("dispatched"))
        .arg("--done-file")
        .arg(path.join("emitted"));
    command
}
fn entry_present(path: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.into()),
    }
}
fn stop_client(client: &mut Process) -> Result<()> {
    if client.0.try_wait()?.is_none() {
        client.0.kill()?;
    }
    client.0.wait()?;
    Ok(())
}
fn observed_exit(status: std::process::ExitStatus) -> i32 {
    use std::os::unix::process::ExitStatusExt;
    status
        .code()
        .unwrap_or_else(|| -status.signal().expect("Unix exit status"))
}

fn volatile_lifecycle() -> Result<Value> {
    let mut fixture = ReplayFixture::new(true)?;
    let (mut first, mut output) = fixture.client()?;
    output.until(b"ready")?;
    let pid = fixture.pid()?;
    fixture.input(b"emit\n")?;
    output.until(OUTPUT_CANARY)?;
    let relayed = output.contains(OUTPUT_CANARY);
    let last_output_during = fixture.last_output()?;
    stop_client(&mut first)?;
    output.finish()?;
    let survived_detach = fixture.server.0.try_wait()?.is_none() && fixture.pid()? == pid;
    let (mut second, mut replay) = fixture.client()?;
    replay.until(OUTPUT_CANARY)?;
    let replayed = replay.contains(OUTPUT_CANARY);
    let mut ctl = mantle_launch::session::open_fifo(
        &fixture.dir.path().join("session/ctl"),
        true,
        libc::O_NONBLOCK,
    )?;
    ctl.write_all(b"120 40\n")?;
    replay.until(b"window:120x40")?;
    let resized = replay.contains(b"window:120x40");
    fixture.input(b"exit\n")?;
    let exit = observed_exit(wait(&mut fixture.server.0)?);
    wait(&mut second.0)?;
    replay.finish()?;
    fixture.errors.finish()?;
    Ok(
        json!({"relayed":relayed,"replayed":replayed,"survived_detach":survived_detach,
        "resized":resized,"last_output_during":last_output_during,"last_output_after":fixture.last_output()?,
        "exit":exit,"canary_in_diagnostics":fixture.errors.contains(OUTPUT_CANARY)}),
    )
}

/// SIGKILL removes the launcher before it can reap its PTY child. Adopt only during this case
/// and retain the child's unreaped pid until explicit group kill + waitpid completes.
struct AdoptedChild {
    pid: Option<u32>,
    previous: libc::c_int,
}
impl AdoptedChild {
    #[allow(unsafe_code)]
    fn begin() -> Result<Self> {
        let mut previous: libc::c_int = 0;
        // SAFETY: prctl writes one integer into a live pointer; this process owns this setting.
        ensure!(
            unsafe { libc::prctl(libc::PR_GET_CHILD_SUBREAPER, &mut previous) } == 0,
            "get subreaper"
        );
        // SAFETY: the process changes its own child-reaping behavior, restored on drop.
        ensure!(
            unsafe { libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1) } == 0,
            "set subreaper"
        );
        Ok(Self {
            pid: None,
            previous,
        })
    }
    #[allow(unsafe_code)]
    fn reap(&mut self) -> Result<Option<i32>> {
        let Some(pid) = self.pid else {
            return Ok(None);
        };
        mantle_launch::sys::signal_group(pid, libc::SIGKILL)?;
        let until = Instant::now() + Duration::from_secs(5);
        loop {
            let mut status = 0;
            // SAFETY: pid belongs to this fixture's now-adopted, unreaped child; status is live.
            let reaped = unsafe { libc::waitpid(pid as libc::pid_t, &mut status, libc::WNOHANG) };
            if reaped == pid as libc::pid_t {
                self.pid = None;
                // Closing the server PTY may make the fixture exit before SIGKILL reaches
                // it. Both paths must be reaped; report the status rather than assuming it.
                use std::os::unix::process::ExitStatusExt;
                return Ok(Some(observed_exit(std::process::ExitStatus::from_raw(
                    status,
                ))));
            }
            ensure!(
                reaped >= 0,
                "cannot reap adopted fixture child: {}",
                std::io::Error::last_os_error()
            );
            ensure!(Instant::now() < until, "adopted child reap deadline");
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}
impl Drop for AdoptedChild {
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        let _ = self.reap();
        // SAFETY: restoring the process-owned setting observed by begin().
        unsafe {
            libc::prctl(libc::PR_SET_CHILD_SUBREAPER, self.previous);
        }
    }
}
fn volatile_exit_paths() -> Result<Value> {
    // Both the direct regression test and the native suite run this case. Serialize changes
    // to the process-wide subreaper setting while leaving ordinary fixtures independent.
    static CRASH_CASE: Mutex<()> = Mutex::new(());
    let _crash_case = CRASH_CASE.lock().unwrap();
    let mut exits = Vec::new();
    let mut last_outputs = Vec::new();
    let mut canary_in_diagnostics = false;
    for action in ["exit", "exit23", "term"] {
        let mut fixture = ReplayFixture::new(true)?;
        let (mut client, mut output) = fixture.client()?;
        output.until(b"ready")?;
        fixture.input(b"emit\n")?;
        output.until(OUTPUT_CANARY)?;
        if action == "term" {
            terminate(&fixture.server.0);
        } else {
            fixture.input(format!("{action}\n").as_bytes())?;
        }
        exits.push(observed_exit(wait(&mut fixture.server.0)?));
        stop_client(&mut client)?;
        output.finish()?;
        fixture.errors.finish()?;
        last_outputs.push(fixture.last_output()?);
        canary_in_diagnostics |= fixture.errors.contains(OUTPUT_CANARY);
    }
    let dir = scratch()?;
    let mut command = serve(dir.path());
    command
        .args(["--volatile-replay", "--"])
        .arg(dir.path().join("missing-program"));
    let mut missing = Process(command.spawn()?);
    let mut errors = LimitedOutput::new(missing.0.stderr.take().context("stderr")?);
    exits.push(observed_exit(wait(&mut missing.0)?));
    errors.finish()?;
    last_outputs.push(entry_present(&dir.path().join("session/last-output"))?);
    canary_in_diagnostics |= errors.contains(OUTPUT_CANARY);

    let mut adopted = AdoptedChild::begin()?;
    let mut fixture = ReplayFixture::new(true)?;
    let (mut client, mut output) = fixture.client()?;
    output.until(b"ready")?;
    fixture.input(b"emit\n")?;
    output.until(OUTPUT_CANARY)?;
    adopted.pid = Some(fixture.pid()?);
    fixture.server.0.kill()?;
    // The direct child remains unreaped until wait; adoption has happened after it exits.
    exits.push(observed_exit(wait(&mut fixture.server.0)?));
    let killed_child_exit = adopted.reap()?.context("fixture child was not reaped")?;
    stop_client(&mut client)?;
    output.finish()?;
    fixture.errors.finish()?;
    last_outputs.push(fixture.last_output()?);
    canary_in_diagnostics |= fixture.errors.contains(OUTPUT_CANARY);
    Ok(
        json!({"exits":exits,"last_outputs":last_outputs,"canary_in_diagnostics":canary_in_diagnostics,
        "killed_child_reaped":adopted.pid.is_none(),"killed_child_exit":killed_child_exit}),
    )
}

fn volatile_preflight() -> Result<Value> {
    use std::os::unix::fs::{MetadataExt, symlink};
    let mut refused = Vec::new();
    let mut dispatched = Vec::new();
    let mut ready = Vec::new();
    let mut preserved = Vec::new();
    let mut locks_left = Vec::new();
    let mut canary_in_diagnostics = false;
    for kind in ["regular", "link", "dangling", "directory"] {
        let dir = scratch()?;
        let session = dir.path().join("session");
        fs::create_dir(&session)?;
        let last = session.join("last-output");
        let target = dir.path().join("target");
        match kind {
            "regular" => fs::write(&last, OUTPUT_CANARY)?,
            "link" => {
                fs::write(&target, OUTPUT_CANARY)?;
                symlink(&target, &last)?;
            }
            "dangling" => symlink(&target, &last)?,
            _ => {
                fs::create_dir(&last)?;
                fs::write(last.join("sentinel"), OUTPUT_CANARY)?;
            }
        }
        let before = fs::symlink_metadata(&last)?;
        let target_before = fs::symlink_metadata(&target).ok();
        let mut creations = creation_watch(&session)?;
        let mut child = Process(replay_command(dir.path(), true).spawn()?);
        let mut errors = LimitedOutput::new(child.0.stderr.take().context("stderr")?);
        let status = wait(&mut child.0)?;
        errors.finish()?;
        refused.push(!status.success());
        dispatched.push(entry_present(&dir.path().join("dispatched"))?);
        ready.push(created_entry(&mut creations, b"ctl")? || entry_present(&session.join("ctl"))?);
        locks_left.push(entry_present(&session.join("server.lock"))?);
        let after = fs::symlink_metadata(&last)?;
        let unchanged = before.ino() == after.ino()
            && before.dev() == after.dev()
            && before.mode() == after.mode()
            && match kind {
                "regular" => fs::read(&last)? == OUTPUT_CANARY,
                "link" => {
                    let target_after = fs::symlink_metadata(&target)?;
                    let original = target_before.as_ref().context("original target")?;
                    fs::read_link(&last)? == target
                        && fs::read(&target)? == OUTPUT_CANARY
                        && original.ino() == target_after.ino()
                        && original.mode() == target_after.mode()
                }
                "dangling" => fs::read_link(&last)? == target && !entry_present(&target)?,
                _ => fs::read(last.join("sentinel"))? == OUTPUT_CANARY,
            };
        preserved.push(unchanged);
        canary_in_diagnostics |= errors.contains(OUTPUT_CANARY);
    }
    Ok(
        json!({"refused":refused,"dispatched":dispatched,"ready":ready,"preserved":preserved,
        "locks_left":locks_left,"canary_in_diagnostics":canary_in_diagnostics}),
    )
}

#[allow(unsafe_code)]
fn creation_watch(path: &Path) -> Result<File> {
    use std::os::fd::FromRawFd;
    let name = std::ffi::CString::new(path.as_os_str().as_bytes())?;
    // SAFETY: returns a new descriptor owned below; the C path is terminated and live.
    let fd = unsafe { libc::inotify_init1(libc::IN_NONBLOCK | libc::IN_CLOEXEC) };
    ensure!(
        fd >= 0,
        "inotify initialization: {}",
        std::io::Error::last_os_error()
    );
    // SAFETY: inotify_init1 succeeded and no other owner exists.
    let file = unsafe { File::from_raw_fd(fd) };
    // SAFETY: both descriptor and name remain live for the call.
    ensure!(
        unsafe { libc::inotify_add_watch(fd, name.as_ptr(), libc::IN_CREATE | libc::IN_MOVED_TO) }
            >= 0,
        "inotify watch: {}",
        std::io::Error::last_os_error()
    );
    Ok(file)
}
fn created_entry(watch: &mut File, wanted: &[u8]) -> Result<bool> {
    let mut found = false;
    let mut total = 0;
    loop {
        let mut bytes = [0; 4096];
        let n = match watch.read(&mut bytes) {
            Ok(n) => n,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => return Ok(found),
            Err(error) => return Err(error.into()),
        };
        ensure!(n > 0, "inotify ended");
        total += n;
        ensure!(total <= OBSERVATION_LIMIT, "inotify observation overflow");
        let mut offset = 0;
        while offset < n {
            ensure!(offset + 16 <= n, "truncated inotify event");
            let mask = u32::from_ne_bytes(bytes[offset + 4..offset + 8].try_into()?);
            ensure!(mask & libc::IN_Q_OVERFLOW == 0, "inotify queue overflow");
            let length = u32::from_ne_bytes(bytes[offset + 12..offset + 16].try_into()?) as usize;
            ensure!(offset + 16 + length <= n, "truncated inotify name");
            let name = &bytes[offset + 16..offset + 16 + length];
            found |= name.split(|b| *b == 0).next() == Some(wanted);
            offset += 16 + length;
        }
    }
}

fn replay_bounds(volatile: bool) -> Result<Value> {
    let mut fixture = ReplayFixture::new(volatile)?;
    let (mut first, mut initial) = fixture.client()?;
    initial.until(b"ready")?;
    stop_client(&mut first)?;
    initial.finish()?;
    fixture.input(b"flood\n")?;
    let until = Instant::now() + Duration::from_secs(5);
    while !entry_present(&fixture.dir.path().join("emitted"))? {
        ensure!(Instant::now() < until, "flood completion deadline");
        ensure!(
            fixture.server.0.try_wait()?.is_none(),
            "launcher ended during flood"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    // The marker says the child's final write completed. Give the server two 100ms poll
    // intervals to drain the remaining PTY bytes before measuring a new replay attachment.
    std::thread::sleep(Duration::from_millis(250));
    let (mut second, mut output) = fixture.client()?;
    output.until(OUTPUT_CANARY)?;
    stop_client(&mut second)?;
    output.finish()?;
    let mut expected = vec![b'x'; 1024 - OUTPUT_CANARY.len()];
    expected.extend_from_slice(OUTPUT_CANARY);
    if !volatile {
        terminate(&fixture.server.0);
        let exit = observed_exit(wait(&mut fixture.server.0)?);
        fixture.errors.finish()?;
        let path = fixture.dir.path().join("session/last-output");
        let bytes = fs::read(&path)?;
        return Ok(
            json!({"exit":exit,"retained_bytes":bytes.len(),"tail_matches":bytes == expected,
            "mode":fs::metadata(path)?.permissions().mode() & 0o777}),
        );
    }
    // Keep an attach stdout unread and flood again. The server must still handle termination.
    let (mut blocked, _unread) = fixture.client()?;
    fixture.input(b"flood\n")?;
    std::thread::sleep(Duration::from_millis(250));
    let started = Instant::now();
    terminate(&fixture.server.0);
    let deadline = started + Duration::from_secs(14);
    let exit = loop {
        if let Some(status) = fixture.server.0.try_wait()? {
            break observed_exit(status);
        }
        ensure!(
            Instant::now() < deadline,
            "server exceeded termination budget"
        );
        std::thread::sleep(Duration::from_millis(10));
    };
    let stopped_within_budget = started.elapsed() < Duration::from_secs(14);
    stop_client(&mut blocked)?;
    fixture.errors.finish()?;
    Ok(
        json!({"replayed_bytes":output.bytes.len(),"tail_matches":output.bytes == expected,
        "exit":exit,"last_output_after":fixture.last_output()?,"stopped_within_budget":stopped_within_budget,
        "canary_in_diagnostics":fixture.errors.contains(OUTPUT_CANARY)}),
    )
}

#[test]
fn volatile_runtime_does_not_persist_output_on_exit() -> Result<()> {
    let observed = volatile_exit_paths()?;
    assert_eq!(observed["exits"], json!([0, 23, 0, 1, -9]));
    assert_eq!(
        observed["last_outputs"],
        json!([false, false, false, false, false])
    );
    assert_eq!(observed["canary_in_diagnostics"], false);
    assert_eq!(observed["killed_child_reaped"], true);
    Ok(())
}

#[test]
fn adversary_volatile_preflight_preserves_fifo_and_socket_without_readiness() -> Result<()> {
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::net::UnixListener;
    for socket in [false, true] {
        let dir = scratch()?;
        let session = dir.path().join("session");
        fs::create_dir(&session)?;
        let last = session.join("last-output");
        let _listener = if socket {
            Some(UnixListener::bind(&last)?)
        } else {
            mantle_launch::session::make_fifo(&last)?;
            None
        };
        let before = fs::symlink_metadata(&last)?;
        let mut creations = creation_watch(&session)?;
        let started = Instant::now();
        let mut child = Process(replay_command(dir.path(), true).spawn()?);
        let mut errors = LimitedOutput::new(child.0.stderr.take().context("stderr")?);
        let status = wait(&mut child.0)?;
        errors.finish()?;
        assert_eq!(status.code(), Some(1));
        assert!(started.elapsed() < Duration::from_secs(2));
        assert!(!entry_present(&dir.path().join("dispatched"))?);
        assert!(!created_entry(&mut creations, b"ctl")?);
        assert!(!entry_present(&session.join("server.lock"))?);
        let after = fs::symlink_metadata(&last)?;
        assert_eq!(
            (after.dev(), after.ino(), after.mode()),
            (before.dev(), before.ino(), before.mode())
        );
        assert!(errors.contains(b"existing entry left unchanged"));
    }
    Ok(())
}

#[test]
fn adversary_persistent_tail_refuses_volatile_reuse_then_clean_retry_runs() -> Result<()> {
    let mut fixture = ReplayFixture::new(false)?;
    let (mut client, mut output) = fixture.client()?;
    output.until(b"ready")?;
    fixture.input(b"emit\n")?;
    output.until(OUTPUT_CANARY)?;
    fixture.input(b"exit\n")?;
    assert_eq!(wait(&mut fixture.server.0)?.code(), Some(0));
    stop_client(&mut client)?;
    output.finish()?;
    fixture.errors.finish()?;
    let last = fixture.dir.path().join("session/last-output");
    let persisted = fs::read(&last)?;
    assert!(
        persisted
            .windows(OUTPUT_CANARY.len())
            .any(|part| part == OUTPUT_CANARY)
    );
    fs::remove_file(fixture.dir.path().join("dispatched"))?;
    let mut creations = creation_watch(&fixture.dir.path().join("session"))?;
    let mut refused = Process(replay_command(fixture.dir.path(), true).spawn()?);
    let mut errors = LimitedOutput::new(refused.0.stderr.take().context("stderr")?);
    assert_eq!(wait(&mut refused.0)?.code(), Some(1));
    errors.finish()?;
    assert_eq!(fs::read(&last)?, persisted);
    assert!(!errors.contains(OUTPUT_CANARY));
    assert!(!entry_present(&fixture.dir.path().join("dispatched"))?);
    assert!(!created_entry(&mut creations, b"ctl")?);
    // Explicitly remove this test's own prior transcript; the launcher must never do so itself.
    fs::remove_file(&last)?;
    fixture.server = Process(replay_command(fixture.dir.path(), true).spawn()?);
    fixture.errors = LimitedOutput::new(fixture.server.0.stderr.take().context("stderr")?);
    ready(fixture.dir.path(), &mut fixture.server)?;
    let (mut client, mut output) = fixture.client()?;
    output.until(b"ready")?;
    fixture.input(b"emit\n")?;
    output.until(OUTPUT_CANARY)?;
    assert!(!fixture.last_output()?);
    fixture.input(b"exit\n")?;
    assert_eq!(wait(&mut fixture.server.0)?.code(), Some(0));
    stop_client(&mut client)?;
    output.finish()?;
    fixture.errors.finish()?;
    assert!(!fixture.last_output()?);
    assert!(!fixture.errors.contains(OUTPUT_CANARY));
    Ok(())
}

#[test]
#[allow(unsafe_code)]
fn adversary_sigint_while_detached_never_persists_replay() -> Result<()> {
    let mut fixture = ReplayFixture::new(true)?;
    let (mut client, mut output) = fixture.client()?;
    output.until(b"ready")?;
    fixture.input(b"emit\n")?;
    output.until(OUTPUT_CANARY)?;
    stop_client(&mut client)?;
    output.finish()?;
    assert!(fixture.server.0.try_wait()?.is_none());
    // SAFETY: this exact server Child is owned and unreaped, so its PID cannot be reused.
    assert_eq!(
        unsafe { libc::kill(fixture.server.0.id() as libc::pid_t, libc::SIGINT) },
        0
    );
    assert_eq!(wait(&mut fixture.server.0)?.code(), Some(0));
    fixture.errors.finish()?;
    assert!(!fixture.last_output()?);
    assert!(!fixture.errors.contains(OUTPUT_CANARY));
    assert!(!entry_present(
        &fixture.dir.path().join("session/server.lock")
    )?);
    Ok(())
}

fn private_paths() -> Result<Value> {
    use std::os::unix::fs::{MetadataExt, symlink};
    let dir = scratch()?;
    let root = dir.path();
    let shared = root.join("shared");
    fs::create_dir(&shared)?;
    fs::set_permissions(&shared, fs::Permissions::from_mode(0o755))?;
    let home = shared.join("private");
    fs::create_dir(&home)?;
    fs::set_permissions(&home, fs::Permissions::from_mode(0o755))?;
    let auth = home.join("auth.json");
    fs::write(&auth, b"synthetic-auth-canary")?;
    fs::set_permissions(&auth, fs::Permissions::from_mode(0o600))?;
    let before = fs::metadata(&auth)?;
    let mut initialized = true;
    for _ in 0..2 {
        let mut command = serve(root);
        command
            .arg("--private-dir")
            .arg(&home)
            .arg("--check-private-file")
            .arg(&auth)
            .arg("--")
            .arg(PROBE)
            .arg("mark")
            .arg("--path")
            .arg(root.join("dispatched"));
        let mut process = Process(command.spawn()?);
        let mut errors = LimitedOutput::new(process.0.stderr.take().context("stderr")?);
        initialized &= wait(&mut process.0)?.success();
        errors.finish()?;
        ensure!(
            !errors.contains(b"synthetic-auth-canary"),
            "file contents reached diagnostics"
        );
    }
    let after = fs::metadata(&auth)?;
    let mut refused_before_dispatch = Vec::new();
    let mut targets_preserved = true;
    for kind in [
        "parent-link",
        "final-link",
        "traversal",
        "not-directory",
        "disk-volatile",
        "file-link",
        "file-hardlink",
        "file-directory",
        "file-mode",
    ] {
        let case = scratch()?;
        let base = case.path();
        fs::create_dir(base.join("session"))?;
        let mut watch = creation_watch(&base.join("session"))?;
        let target = base.join("target");
        fs::create_dir(&target)?;
        fs::set_permissions(&target, fs::Permissions::from_mode(0o755))?;
        let candidate = base.join("candidate");
        let sentinel = target.join("sentinel");
        fs::write(&sentinel, b"synthetic-auth-canary")?;
        fs::set_permissions(&sentinel, fs::Permissions::from_mode(0o600))?;
        let original = fs::metadata(&sentinel)?;
        let mut command = serve(base);
        match kind {
            "parent-link" => {
                symlink(&target, &candidate)?;
                command.arg("--private-dir").arg(candidate.join("child"));
            }
            "final-link" => {
                symlink(&target, &candidate)?;
                command.arg("--private-dir").arg(&candidate);
            }
            "traversal" => {
                command.arg("--private-dir").arg(target.join("../escaped"));
            }
            "not-directory" => {
                fs::write(&candidate, b"unchanged")?;
                command.arg("--private-dir").arg(&candidate);
            }
            "disk-volatile" => {
                command.arg("--volatile-dir").arg(&target);
            }
            "file-link" => {
                symlink(&sentinel, &candidate)?;
                command.arg("--check-private-file").arg(&candidate);
            }
            "file-hardlink" => {
                fs::hard_link(&sentinel, &candidate)?;
                command.arg("--check-private-file").arg(&candidate);
            }
            "file-directory" => {
                command.arg("--check-private-file").arg(&target);
            }
            "file-mode" => {
                fs::write(&candidate, b"unchanged")?;
                fs::set_permissions(&candidate, fs::Permissions::from_mode(0o644))?;
                command.arg("--check-private-file").arg(&candidate);
            }
            _ => unreachable!(),
        }
        command
            .arg("--")
            .arg(PROBE)
            .arg("mark")
            .arg("--path")
            .arg(base.join("dispatched"));
        let mut process = Process(command.spawn()?);
        let mut errors = LimitedOutput::new(process.0.stderr.take().context("stderr")?);
        let status = wait(&mut process.0)?;
        errors.finish()?;
        refused_before_dispatch.push(
            !status.success()
                && !entry_present(&base.join("dispatched"))?
                && !created_entry(&mut watch, b"ctl")?,
        );
        let observed = fs::metadata(&sentinel)?;
        targets_preserved &= fs::read(&sentinel)? == b"synthetic-auth-canary"
            && original.ino() == observed.ino()
            && original.mode() == observed.mode()
            && fs::metadata(&target)?.mode() & 0o777 == 0o755;
    }
    Ok(
        json!({"initialized":initialized,"repeated_inode":before.ino()==after.ino(),
        "auth_bytes_preserved":fs::read(&auth)? == b"synthetic-auth-canary",
        "shared_mode_unchanged":fs::metadata(&shared)?.mode() & 0o777 == 0o755,
        "target_mode":fs::metadata(&home)?.mode() & 0o777,
        "refused_before_dispatch":refused_before_dispatch,"targets_preserved":targets_preserved}),
    )
}

#[test]
fn private_path_initialization_preserves_auth_and_refuses_unsafe_entries() -> Result<()> {
    let observed = private_paths()?;
    assert_eq!(observed["initialized"], true);
    assert_eq!(observed["target_mode"], 448);
    assert_eq!(observed["refused_before_dispatch"], json!(vec![true; 9]));
    assert_eq!(observed["targets_preserved"], true);
    Ok(())
}

#[test]
fn adversary_private_file_special_entries_refuse_without_opening_or_dispatch() -> Result<()> {
    use std::os::unix::fs::{MetadataExt, symlink};
    use std::os::unix::net::UnixListener;
    for kind in ["fifo", "socket", "dangling-parent", "mode-special"] {
        let dir = scratch()?;
        let root = dir.path();
        fs::create_dir(root.join("session"))?;
        let mut watch = creation_watch(&root.join("session"))?;
        let candidate = root.join("auth.json");
        let socket = match kind {
            "fifo" => {
                mantle_launch::session::make_fifo(&candidate)?;
                None
            }
            "socket" => Some(UnixListener::bind(&candidate)?),
            "dangling-parent" => {
                symlink(root.join("missing"), &candidate)?;
                None
            }
            "mode-special" => {
                fs::write(&candidate, b"adversary-private-file-canary")?;
                fs::set_permissions(&candidate, fs::Permissions::from_mode(0o4600))?;
                None
            }
            _ => unreachable!(),
        };
        let original = fs::symlink_metadata(&candidate)?;
        let path = if kind == "dangling-parent" {
            candidate.join("auth.json")
        } else {
            candidate.clone()
        };
        let started = Instant::now();
        let mut command = serve(root);
        command
            .arg("--check-private-file")
            .arg(path)
            .arg("--")
            .arg(PROBE)
            .arg("mark")
            .arg("--path")
            .arg(root.join("dispatched"));
        let mut process = Process(command.spawn()?);
        let mut errors = LimitedOutput::new(process.0.stderr.take().context("stderr")?);
        assert_eq!(wait(&mut process.0)?.code(), Some(1), "{kind}");
        errors.finish()?;
        assert!(started.elapsed() < Duration::from_secs(2), "{kind} blocked");
        assert!(!entry_present(&root.join("dispatched"))?);
        assert!(!created_entry(&mut watch, b"ctl")?);
        assert!(!errors.contains(b"adversary-private-file-canary"));
        let observed = fs::symlink_metadata(&candidate)?;
        assert_eq!(
            (observed.dev(), observed.ino(), observed.mode()),
            (original.dev(), original.ino(), original.mode())
        );
        drop(socket);
    }
    Ok(())
}

#[test]
fn adversary_private_paths_accept_tmpfs_and_preserve_auth_metadata() -> Result<()> {
    use std::os::unix::fs::MetadataExt;
    let parent =
        std::env::var_os("MANTLE_ADVERSARY_RUNTIME").unwrap_or_else(|| OsString::from("/dev/shm"));
    let runtime = tempfile::tempdir_in(parent)?;
    let dir = scratch()?;
    let root = dir.path();
    let home = root.join("private-home");
    fs::create_dir(&home)?;
    let auth = home.join("auth.json");
    fs::write(&auth, b"adversary-metadata-only-canary")?;
    fs::set_permissions(&auth, fs::Permissions::from_mode(0o600))?;
    let old = fs::metadata(&auth)?;
    for _ in 0..2 {
        let mut command = serve(root);
        command
            .arg("--private-dir")
            .arg(&home)
            .arg("--volatile-dir")
            .arg(runtime.path().join("sqlite"))
            .arg("--check-private-file")
            .arg(&auth)
            .arg("--check-private-file")
            .arg(home.join("absent/config.toml"))
            .arg("--")
            .arg(PROBE)
            .arg("mark")
            .arg("--path")
            .arg(root.join("dispatched"));
        let mut process = Process(command.spawn()?);
        let mut errors = LimitedOutput::new(process.0.stderr.take().context("stderr")?);
        assert_eq!(wait(&mut process.0)?.code(), Some(0));
        errors.finish()?;
        assert_eq!(fs::read(root.join("dispatched"))?, b"dispatched");
        assert!(!errors.contains(b"adversary-metadata-only-canary"));
        let now = fs::metadata(&auth)?;
        assert_eq!(
            (
                now.dev(),
                now.ino(),
                now.mode(),
                now.atime(),
                now.atime_nsec(),
                now.mtime(),
                now.mtime_nsec()
            ),
            (
                old.dev(),
                old.ino(),
                old.mode(),
                old.atime(),
                old.atime_nsec(),
                old.mtime(),
                old.mtime_nsec()
            )
        );
        assert_eq!(fs::metadata(&home)?.mode() & 0o7777, 0o700);
        assert_eq!(
            fs::metadata(runtime.path().join("sqlite"))?.mode() & 0o7777,
            0o700
        );
        assert!(!home.join("absent").exists());
    }
    assert_eq!(fs::read(auth)?, b"adversary-metadata-only-canary");
    Ok(())
}
