//! Bounded, unauthenticated Codex qualification. Raw terminal bytes are never printed or retained.
//! `confined` creates and destroys only its own Substrate workspace; no aperture or secret slot.
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::net::UnixListener;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, ensure};
use b10x_substrate_sdk::{Client, ExecutionPolicy, ReadOnlyRoot};
use clap::{Args, Parser, Subcommand};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tokio::io::AsyncReadExt;

const O_NONBLOCK: i32 = 0o4000; // Linux open(2); this probe targets Linux workers.
const BYTE_LIMIT: usize = 256 * 1024;
const SCRATCH_LIMIT: u64 = 64 * 1024 * 1024;
const BINARY_LIMIT: u64 = 512 * 1024 * 1024;
const VERSION: &str = "codex-cli 0.153.4";
const HOST_SENTINEL: &str = "/opt/mantle-codex-qualification-host-only.txt";

#[derive(Parser)]
#[command(
    about = "Bounded unauthenticated Codex probe; output is observations, not qualification approval"
)]
struct Cli {
    #[command(subcommand)]
    mode: Mode,
}
#[derive(Subcommand)]
enum Mode {
    /// Run where invoked. This mode alone never establishes Substrate confinement.
    Local(Local),
    /// Invoke the same helper in a new, no-egress/no-secret Substrate workspace.
    Confined(Confined),
    /// Internal negative controls; no authentication or program output is read.
    Controls {
        #[arg(long)]
        scratch_parent: PathBuf,
    },
}
#[derive(Args)]
struct Local {
    #[arg(long)]
    codex: PathBuf,
    #[arg(long)]
    sha256: String,
    #[arg(long)]
    launcher: PathBuf,
    /// Existing, non-symlink, non-group/world-writable directory; a private child is created.
    #[arg(long)]
    scratch_parent: PathBuf,
    /// Total real-TUI observation time, including detach, same-size attach, resize and slow read.
    #[arg(long, default_value_t = 12, value_parser = clap::value_parser!(u64).range(6..=30))]
    seconds: u64,
    /// Exercise the installed worker toolchain via Codex's documented sandbox command.
    #[arg(long)]
    tool_controls: bool,
    /// Explicit experiment: use Substrate's outer confinement with Codex approvals requested.
    #[arg(long, requires = "tool_controls")]
    outer_profile_control: bool,
}
#[derive(Args)]
struct Confined {
    #[arg(long)]
    socket: PathBuf,
    /// Probe-owned root on the worker holding codex and codex_qualification (this executable).
    #[arg(long)]
    worker_root: PathBuf,
    #[arg(long)]
    sha256: String,
    #[arg(long)]
    outer_profile_control: bool,
}

struct Scratch(PathBuf);
impl Scratch {
    fn new(parent: &Path) -> Result<Self> {
        ensure!(parent.is_absolute(), "scratch parent must be absolute");
        let meta = fs::symlink_metadata(parent).context("inspect scratch parent")?;
        ensure!(
            meta.is_dir() && !meta.file_type().is_symlink(),
            "scratch parent must be a real directory"
        );
        ensure!(
            meta.permissions().mode() & 0o022 == 0,
            "scratch parent is writable by other users"
        );
        ensure!(
            parent.canonicalize()? == parent,
            "scratch parent must have no symlink components"
        );
        let path = parent.join(format!("codex-probe-{}", ulid::Ulid::new()));
        DirBuilder::new().mode(0o700).create(&path)?;
        for name in ["home", "home/.codex", "work", "tmp", "cargo", "cache"] {
            DirBuilder::new().mode(0o700).create(path.join(name))?;
        }
        Ok(Self(path))
    }
    fn cleanup(self) -> Result<()> {
        fs::remove_dir_all(&self.0).context("remove only probe-owned private scratch")
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn digest(path: &Path) -> Result<String> {
    // A path precheck is racy, and opening a FIFO normally waits for a writer before
    // we can inspect it. Open nonblocking, then validate the object actually opened.
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(O_NONBLOCK)
        .open(path)
        .context("open pinned binary")?;
    let metadata = file.metadata()?;
    ensure!(metadata.is_file(), "binary is not a regular file");
    ensure!(
        metadata.len() <= BINARY_LIMIT,
        "binary exceeds probe input bound"
    );
    digest_reader(file, BINARY_LIMIT)
}

fn digest_reader(reader: impl Read, limit: u64) -> Result<String> {
    // Metadata is only a snapshot: a regular file can grow after the size check.
    // Read at most one byte beyond the limit to distinguish exact size from overflow.
    let mut reader = reader.take(limit + 1);
    let mut total = 0u64;
    let mut hash = Sha256::new();
    let mut buf = [0; 64 * 1024];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        total += n as u64;
        ensure!(total <= limit, "binary exceeds probe input bound");
        hash.update(&buf[..n]);
    }
    Ok(format!("{:x}", hash.finalize()))
}
fn verify_digest(path: &Path, expected: &str) -> Result<()> {
    ensure!(
        expected.len() == 64 && expected.bytes().all(|b| b.is_ascii_hexdigit()),
        "sha256 must be 64 hexadecimal digits"
    );
    ensure!(
        digest(path)? == expected.to_ascii_lowercase(),
        "binary SHA256 mismatch; binary was not executed"
    );
    Ok(())
}

fn isolated(command: &mut Command, root: &Path) {
    command
        .env_clear()
        .env("HOME", root.join("home"))
        .env("XDG_CONFIG_HOME", root.join("home/.config"))
        .env("XDG_CACHE_HOME", root.join("cache"))
        .env("TMPDIR", root.join("tmp"))
        .env("CARGO_HOME", root.join("cargo"))
        .env("RUSTUP_HOME", "/opt/mantle/rustup")
        .env(
            "PATH",
            "/opt/mantle/bin:/opt/mantle/cargo/bin:/usr/bin:/bin",
        )
        .env("LANG", "C.UTF-8")
        .env("TERM", "xterm-256color")
        .current_dir(root.join("work"));
}

async fn version(path: &Path, root: &Path) -> Result<()> {
    let mut base = Command::new(path);
    isolated(&mut base, root);
    base.arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let mut command = tokio::process::Command::from(base);
    command.kill_on_drop(true);
    let mut child = command.spawn().context("start version check")?;
    let mut output = child.stdout.take().context("version stdout")?.take(4097);
    tokio::time::timeout(Duration::from_secs(10), async {
        let mut bytes = Vec::new();
        output.read_to_end(&mut bytes).await?;
        ensure!(bytes.len() <= 4096, "version output exceeded bound");
        ensure!(child.wait().await?.success(), "version command failed");
        ensure!(
            String::from_utf8_lossy(&bytes).trim() == VERSION,
            "version mismatch; TUI was not executed"
        );
        Ok(())
    })
    .await
    .context("version deadline exceeded")?
}

/// A partial parser retaining at most 8 KiB in memory, with only fixed booleans escaping it.
#[derive(Default)]
struct Screen {
    total: usize,
    tail: Vec<u8>,
    welcome: bool,
    login: bool,
    socket_error: bool,
    terminal_query: bool,
}
impl Screen {
    fn feed(&mut self, bytes: &[u8]) -> Result<bool> {
        self.total += bytes.len();
        ensure!(self.total <= BYTE_LIMIT, "terminal output exceeded bound");
        self.tail.extend_from_slice(bytes);
        let text = String::from_utf8_lossy(&self.tail);
        self.welcome |= text.contains("Welcome to Codex") || text.contains("OpenAI Codex");
        self.login |=
            text.contains("Sign in") || text.contains("sign in") || text.contains("Sign In");
        self.socket_error |= text.contains("Operation not permitted")
            || text.contains("Permission denied")
            || text.contains("socket") && text.contains("failed");
        let query = self.tail.windows(4).any(|w| w == b"\x1b[6n");
        self.terminal_query |= query;
        if query {
            self.tail.clear();
        }
        if self.tail.len() > 8192 {
            self.tail.drain(..self.tail.len() - 8192);
        }
        Ok(query)
    }
    fn observation(&self) -> Value {
        json!({"bytes":self.total,"welcome_marker":self.welcome,"login_marker":self.login,
            "permission_or_socket_error_marker":self.socket_error,"cursor_query":self.terminal_query})
    }
}

struct Server(Child);
impl Server {
    fn stop(&mut self) -> Result<()> {
        if self.0.try_wait()?.is_some() {
            return Ok(());
        }
        // Signal only the launcher we created. It owns and reaps the separate PTY process group.
        let status = Command::new("/bin/kill")
            .args(["-TERM", &self.0.id().to_string()])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        ensure!(status.success(), "could not signal probe launcher");
        let deadline = Instant::now() + Duration::from_secs(12);
        while self.0.try_wait()?.is_none() {
            ensure!(
                Instant::now() < deadline,
                "launcher did not reap child within cleanup deadline"
            );
            sleep(Duration::from_millis(20));
        }
        Ok(())
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

fn open_fifo(path: &Path, write: bool) -> Result<File> {
    let mut options = OpenOptions::new();
    options.read(!write).write(write).custom_flags(O_NONBLOCK);
    Ok(options.open(path)?)
}

/// A sampled budget, not a filesystem quota. Never follows a child-created symlink.
fn check_scratch_budget(root: &Path) -> Result<()> {
    let mut directories = vec![root.to_owned()];
    let mut entries = 0usize;
    let mut bytes = 0u64;
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let metadata = fs::symlink_metadata(entry.path())?;
            entries += 1;
            ensure!(entries <= 4096, "probe scratch entry budget exceeded");
            bytes = bytes.saturating_add(metadata.len());
            ensure!(bytes <= SCRATCH_LIMIT, "probe scratch byte budget exceeded");
            if metadata.is_dir() {
                directories.push(entry.path());
            }
        }
    }
    Ok(())
}
fn read_screen(output: &mut File, input: &mut File, screen: &mut Screen) -> Result<()> {
    let mut chunk = [0; 4096];
    match output.read(&mut chunk) {
        Ok(0) => {}
        Ok(n) => {
            if screen.feed(&chunk[..n])? {
                // Cursor-position response only. No login selection, code entry or model prompt.
                input.write_all(b"\x1b[1;1R")?;
            }
        }
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
        Err(e) => return Err(e.into()),
    }
    Ok(())
}
fn tui(args: &Local, root: &Path) -> Result<Value> {
    let session = root.join("session");
    let mut command = Command::new(&args.launcher);
    isolated(&mut command, root);
    command
        .arg("serve")
        .arg("--dir")
        .arg(&session)
        .arg("--cwd")
        .arg(root.join("work"))
        .args([
            "--scrollback-bytes",
            "8192",
            "--proxy",
            "http://127.0.0.1:9",
            "--",
        ])
        .arg(&args.codex)
        .args([
            "--sandbox",
            "workspace-write",
            "--ask-for-approval",
            "on-request",
            "-c",
            "check_for_update_on_startup=false",
            "-c",
            "cli_auth_credentials_store=\"file\"",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut server = Server(command.spawn().context("start launcher")?);
    let ready_deadline = Instant::now() + Duration::from_secs(5);
    while !session.join("ctl").exists() {
        ensure!(
            server.0.try_wait()?.is_none(),
            "launcher exited before FIFO readiness"
        );
        ensure!(
            Instant::now() < ready_deadline,
            "launcher readiness deadline exceeded"
        );
        sleep(Duration::from_millis(10));
    }
    let mut output = open_fifo(&session.join("out"), false)?;
    let mut input = open_fifo(&session.join("in"), true)?;
    let mut ctl = open_fifo(&session.join("ctl"), true)?;
    ctl.write_all(b"100 30\n")?;
    let phases = ["initial", "same_size_reattach", "resize", "slow_reader"];
    let mut observations = Vec::new();
    let mut alive = true;
    for (index, phase) in phases.iter().enumerate() {
        if index == 1 {
            drop(output);
            sleep(Duration::from_millis(350));
            output = open_fifo(&session.join("out"), false)?;
            ctl.write_all(b"100 30\n")?;
        } else if index == 2 {
            ctl.write_all(b"80 24\n")?;
        } else if index == 3 {
            sleep(Duration::from_millis(500));
        }
        let mut screen = Screen::default();
        let until = Instant::now() + Duration::from_millis(args.seconds * 250);
        let mut next_storage_check = Instant::now();
        while Instant::now() < until {
            if Instant::now() >= next_storage_check {
                check_scratch_budget(root)?;
                next_storage_check = Instant::now() + Duration::from_millis(100);
            }
            read_screen(&mut output, &mut input, &mut screen)?;
            if server.0.try_wait()?.is_some() {
                alive = false;
                break;
            }
            sleep(Duration::from_millis(10));
        }
        observations
            .push(json!({"phase":phase,"screen":screen.observation(),"launcher_alive":alive}));
        if !alive {
            break;
        }
    }
    server.stop()?;
    // The launcher can retain last-output. It stays under the private root and is deleted without reading it.
    Ok(
        json!({"phases":observations,"cleanup":"launcher reaped", "usable_screen":"requires human verification",
        "slow_reader":"500ms reader pause; no forced overflow claim", "real_model_turn":"not-run: no authentication"}),
    )
}

fn controls(root: &Path) -> Value {
    let socket = root.join("negative.sock");
    let unix_socket = match UnixListener::bind(&socket) {
        Ok(listener) => {
            drop(listener);
            let _ = fs::remove_file(&socket);
            json!({"outcome":"permitted"})
        }
        Err(e) => json!({"outcome":"refused", "errno":e.raw_os_error()}),
    };
    let host_read = match File::open(HOST_SENTINEL) {
        Ok(_) => json!({"outcome":"permitted"}),
        Err(e) => json!({"outcome":"refused", "errno":e.raw_os_error()}),
    };
    let address: SocketAddr = "1.1.1.1:443".parse().expect("constant socket address");
    let direct_tcp = match TcpStream::connect_timeout(&address, Duration::from_millis(500)) {
        Ok(_) => json!({"outcome":"permitted"}),
        Err(e) => {
            json!({"outcome":"refused_or_unreachable", "errno":e.raw_os_error(), "kind":format!("{:?}",e.kind())})
        }
    };
    json!({"unix_socket":unix_socket,"host_sentinel_open":host_read,"direct_tcp":direct_tcp})
}

async fn tool_control(codex: &Path, root: &Path, profile: &str, argv: &[&str]) -> Result<Value> {
    let mut base = Command::new(codex);
    isolated(&mut base, root);
    base.args([
        "sandbox",
        "-c",
        &format!("sandbox_mode=\"{profile}\""),
        "-c",
        "approval_policy=\"on-request\"",
        "--",
    ])
    .args(argv)
    .process_group(0)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());
    let mut command = tokio::process::Command::from(base);
    command.kill_on_drop(true);
    let mut child = command.spawn()?;
    let pid = child.id().context("tool control PID")?;
    let mut stdout = child.stdout.take().context("tool stdout")?.take(8193);
    let mut stderr = child.stderr.take().context("tool stderr")?.take(8193);
    let mut out = Vec::new();
    let mut err = Vec::new();
    let result = tokio::time::timeout(Duration::from_secs(20), async {
        let (a, b) = tokio::join!(stdout.read_to_end(&mut out), stderr.read_to_end(&mut err));
        a?;
        b?;
        child.wait().await
    })
    .await;
    // A tool may fork. Reap the complete probe-owned process group even on output/timeout refusal.
    let _ = Command::new("/bin/kill")
        .args(["-KILL", "--", &format!("-{pid}")])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let status = match result {
        Ok(status) => Some(status?),
        Err(_) => {
            child.wait().await?;
            None
        }
    };
    let error_text = String::from_utf8_lossy(&err);
    let child_observation = serde_json::from_slice::<Value>(&out).ok();
    Ok(
        json!({"argv":argv,"requested_codex_sandbox":profile,"requested_approval_policy":"on-request", "approval_prompt_observed":false,
        "child_observation":child_observation,"exit":status.and_then(|s|s.code()),"timed_out":status.is_none(),
        "output_bound_exceeded":out.len()>8192 || err.len()>8192,
        "stdout_bytes":out.len(),"stderr_bytes":err.len(),
        "operation_not_permitted":error_text.contains("Operation not permitted"),
        "permission_denied":error_text.contains("Permission denied"),
        "namespace_error":error_text.contains("namespace"),
        "missing_file":error_text.contains("No such file or directory"),
        "scope":"direct Codex sandbox CLI control; not a model-issued tool call"}),
    )
}

async fn tool_controls(codex: &Path, root: &Path, outer_profile: bool) -> Result<Value> {
    let work = root.join("work");
    fs::write(
        work.join("Cargo.toml"),
        "[package]\nname = \"cq03-probe\"\nversion = \"0.0.0\"\nedition = \"2021\"\n[lib]\npath = \"lib.rs\"\n",
    )?;
    fs::write(
        work.join("lib.rs"),
        "pub fn qualification_probe() -> u32 { 42 }\n",
    )?;
    let shell = tool_control(
        codex,
        root,
        "workspace-write",
        &["/bin/sh", "-n", "/dev/null"],
    )
    .await?;
    check_scratch_budget(root)?;
    let cargo = tool_control(
        codex,
        root,
        "workspace-write",
        &[
            "/opt/mantle/cargo/bin/cargo",
            "build",
            "--offline",
            "--quiet",
            "-j",
            "1",
        ],
    )
    .await?;
    check_scratch_budget(root)?;
    let mut result = json!({"shell":shell,"cargo":cargo,"cargo_artifact_exists":work.join("target/debug/libcq03_probe.rlib").is_file()});
    if outer_profile {
        let shell = tool_control(
            codex,
            root,
            "danger-full-access",
            &["/bin/sh", "-n", "/dev/null"],
        )
        .await?;
        let cargo = tool_control(
            codex,
            root,
            "danger-full-access",
            &[
                "/opt/mantle/cargo/bin/cargo",
                "build",
                "--offline",
                "--quiet",
                "-j",
                "1",
            ],
        )
        .await?;
        check_scratch_budget(root)?;
        let executable = std::env::current_exe()?;
        let controls = tool_control(
            codex,
            root,
            "danger-full-access",
            &[
                &executable.to_string_lossy(),
                "controls",
                "--scratch-parent",
                &root.to_string_lossy(),
            ],
        )
        .await?;
        result["outer_confinement_only_candidate"] = json!({"shell":shell,"cargo":cargo,"controls":controls,
            "cargo_artifact_exists":work.join("target/debug/libcq03_probe.rlib").is_file(),
            "decision":"explicit qualification experiment; interactive approval behavior not verified"});
    }
    Ok(result)
}

async fn local(args: Local) -> Result<Value> {
    ensure!(
        args.codex.is_absolute() && args.launcher.is_absolute(),
        "executable paths must be absolute"
    );
    verify_digest(&args.codex, &args.sha256)?;
    let scratch = Scratch::new(&args.scratch_parent)?;
    version(&args.codex, &scratch.0).await?;
    let controls = controls(&scratch.0);
    if args.outer_profile_control {
        ensure!(
            controls["unix_socket"]["errno"] == json!(13)
                && controls["direct_tcp"]["errno"] == json!(101)
                && controls["host_sentinel_open"]["errno"] == json!(2),
            "outer-profile experiment requires the expected denied socket, network and host-path controls; use the confined driver"
        );
    }
    let tools = if args.tool_controls {
        tool_controls(&args.codex, &scratch.0, args.outer_profile_control).await?
    } else {
        json!("not requested")
    };
    let screen = tui(&args, &scratch.0);
    scratch.cleanup()?;
    let screen = screen?;
    Ok(
        json!({"format":"mantle.codex-qualification/1","timestamp":chrono::Utc::now().to_rfc3339(),
        "version":VERSION,"sha256":args.sha256.to_ascii_lowercase(),"placement":"where invoked; no independent confinement claim",
        "controls":controls,"tools":tools,"tui":screen,"credentials":"fresh private HOME; no login requested; no inherited environment",
        "auth_refresh":"not-run", "raw_terminal_retained":false,"scratch_cleanup":"removed"}),
    )
}

async fn confined(args: Confined) -> Result<Value> {
    ensure!(
        args.worker_root.is_absolute(),
        "worker root must be absolute"
    );
    let client = Client::builder()
        .unix_socket(&args.socket)
        .connect()
        .await?;
    let facts = serde_json::to_value(&client.machine().facts)?;
    let workspace = client
        .workspace()
        .empty()
        .label("mantle-probe", "codex-qualification")
        .create()
        .await?;
    let id = workspace.id().to_owned();
    // Emit the owned workspace id before dispatch so interrupted runs can be recovered precisely.
    eprintln!("probe-owned workspace: {id}");
    let result = async {
        let policy = ExecutionPolicy::builder()
            .timeout(Duration::from_secs(150))
            .cpu_time(Duration::from_secs(90))
            .memory_bytes(1024 << 20)
            .processes(128)
            .output_bytes(64 * 1024)
            .build()?;
        let helper = args.worker_root.join("codex_qualification");
        let codex = args.worker_root.join("codex");
        let mut command = workspace
            .command(helper.to_string_lossy())
            .args([
                "local",
                "--codex",
                &codex.to_string_lossy(),
                "--sha256",
                &args.sha256,
                "--launcher",
                "/opt/mantle/bin/mantle-launch",
                "--scratch-parent",
                "/workspace",
                "--tool-controls",
            ])
            .read_only_root(ReadOnlyRoot {
                host_path: args.worker_root.to_string_lossy().into_owned(),
                mount: args.worker_root.to_string_lossy().into_owned(),
            })
            .read_only_root(ReadOnlyRoot {
                host_path: "/opt/mantle".into(),
                mount: "/opt/mantle".into(),
            })
            .policy(policy);
        if args.outer_profile_control {
            command = command.arg("--outer-profile-control");
        }
        let output = command.run().await?;
        // Only typed JSON from our helper is accepted; raw child output never reaches the report.
        let helper_result = serde_json::from_slice::<Value>(&output.stdout).ok();
        Ok::<_, anyhow::Error>(
            json!({"machine_facts":facts,"workspace":id,"exec":output.exec,
            "stdout_truncated":output.stdout_truncated,"stderr_bytes":output.stderr.len(),
            "helper":helper_result,"requested_apertures":[],"requested_secret_slots":[]}),
        )
    }
    .await;
    let cleanup = workspace.destroy().await;
    ensure!(
        cleanup.is_ok(),
        "probe workspace cleanup failed; recover recorded id {id}"
    );
    let mut observation = result?;
    observation["workspace_cleanup"] = json!("destroyed");
    Ok(observation)
}

#[tokio::main]
async fn main() {
    let result = match Cli::parse().mode {
        Mode::Local(args) => local(args).await,
        Mode::Confined(args) => confined(args).await,
        Mode::Controls { scratch_parent } => (|| {
            let scratch = Scratch::new(&scratch_parent)?;
            let observation = controls(&scratch.0);
            scratch.cleanup()?;
            Ok(observation)
        })(),
    };
    match result {
        Ok(value) => println!(
            "{}",
            serde_json::to_string_pretty(&value).expect("serialize observation")
        ),
        Err(error) => {
            eprintln!("qualification probe: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
#[path = "tests/codex_qualification_adversary.rs"]
mod adversary;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cq01_rejects_wrong_digest_before_executing_a_binary() {
        let path = std::env::current_exe().unwrap();
        assert!(verify_digest(&path, &"0".repeat(64)).is_err());
        assert!(verify_digest(&path, &digest(&path).unwrap()).is_ok());
    }
    #[test]
    fn digest_format_is_strict() {
        for bad in ["", "not-a-digest", &"z".repeat(64)] {
            assert!(verify_digest(Path::new("/not-opened"), bad).is_err());
        }
    }
    #[test]
    fn digest_refuses_other_non_regular_descriptors() {
        assert!(digest(Path::new("/dev/zero")).is_err());
        assert!(digest(Path::new("/")).is_err());
    }
    #[test]
    fn digest_bounds_actual_input_even_when_it_exceeds_the_size_snapshot() {
        let mut input = io::Cursor::new(vec![7; 32 * 1024]);
        assert!(digest_reader(&mut input, 8).is_err());
        assert_eq!(
            input.position(),
            9,
            "read more than one overflow detection byte"
        );
        assert_eq!(
            digest_reader(&b"12345678"[..], 8).unwrap(),
            format!("{:x}", Sha256::digest(b"12345678"))
        );
    }
    #[test]
    fn terminal_observation_is_bounded_and_handles_split_queries() {
        let mut screen = Screen::default();
        assert!(!screen.feed(b"Welcome to Codex\x1b[").unwrap());
        assert!(screen.feed(b"6n").unwrap());
        assert!(screen.welcome && screen.terminal_query);
        screen.feed(&vec![b'x'; BYTE_LIMIT - 24]).unwrap();
        assert!(screen.feed(&[b'x'; 100]).is_err());
        assert!(screen.tail.len() <= 8192);
    }
    #[test]
    fn fixed_observation_never_exports_terminal_payload() {
        let mut screen = Screen::default();
        screen.feed(b"Sign in SECRET-CANARY-NEVER-REPORT").unwrap();
        let output = screen.observation().to_string();
        assert!(screen.login);
        assert!(!output.contains("SECRET-CANARY"));
    }
    #[test]
    fn unsafe_scratch_parents_are_refused_without_creating_children() {
        assert!(Scratch::new(Path::new("relative")).is_err());
        assert!(Scratch::new(Path::new("/tmp")).is_err());
        assert!(Scratch::new(Path::new("/proc/self")).is_err());
    }

    #[test]
    fn scratch_budget_refuses_large_files_and_does_not_follow_links() {
        let parent = std::env::var_os("TMPDIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from(std::env::var_os("HOME").expect("HOME")).join(".cache")
            });
        let scratch = Scratch::new(&parent).unwrap();
        std::os::unix::fs::symlink("/", scratch.0.join("host-root-link")).unwrap();
        check_scratch_budget(&scratch.0).unwrap();
        let large = scratch.0.join("large");
        File::create(&large)
            .unwrap()
            .set_len(SCRATCH_LIMIT + 1)
            .unwrap();
        assert!(check_scratch_budget(&scratch.0).is_err());
        scratch.cleanup().unwrap();
    }
}
