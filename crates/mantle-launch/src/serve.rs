//! `serve`: owns a pseudo-terminal running the agent and relays it to at most one attached
//! terminal through the named pipes of the session directory.
//!
//! Client arrival is learnt by opening `out` for writing without blocking every
//! `PROBE_INTERVAL`: the open fails with ENXIO until an `attach` holds the read end.
//! Departure shows as POLLERR on that write end (no reader left) or EPIPE on a write.

use std::env;
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::os::fd::AsFd;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};

use crate::cli::ServeArgs;
use crate::ctl::{Window, WindowLines};
use crate::ring::Ring;
use crate::session::{self, Paths};
use crate::{secret, signals, sys};

const INITIAL_WINDOW: Window = Window {
    cols: 200,
    rows: 50,
};
const PROBE_INTERVAL: Duration = Duration::from_millis(100);
const STOP_TIMEOUT: Duration = Duration::from_secs(10);
const FINAL_FLUSH: Duration = Duration::from_secs(1);
const MAX_TO_AGENT: usize = 64 * 1024;
const MIN_PENDING: usize = 64 * 1024;
const READ_CHUNK: usize = 16 * 1024;
const READS_PER_TURN: usize = 16;

pub fn run(args: &ServeArgs) -> Result<u8> {
    let secret = match (args.secret_fd, args.secret_env.as_deref()) {
        (Some(fd), Some(name)) => Some((name, secret::read_from_fd(fd, name)?)),
        _ => None,
    };
    for dir in &args.mkdirs {
        fs::create_dir_all(dir).with_context(|| format!("cannot create {}", dir.display()))?;
    }
    if !args.cwd.is_dir() {
        bail!(
            "working directory {} is not a directory",
            args.cwd.display()
        );
    }
    DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&args.dir)
        .with_context(|| format!("cannot create {}", args.dir.display()))?;
    let paths = Paths::new(&args.dir);
    let Some(lock) = session::try_lock(&paths.server_lock)? else {
        bail!(
            "a session server is already running in {}",
            args.dir.display()
        );
    };
    signals::install(&[libc::SIGTERM, libc::SIGINT])?;
    signals::ignore(libc::SIGPIPE)?;

    let result = Pipes::create(&paths).and_then(|pipes| serve(args, &paths, pipes, secret));
    for fifo in paths.fifos() {
        if let Err(err) = session::remove_fifo(fifo) {
            eprintln!("mantle-launch: {err:#}");
        }
    }
    fs::remove_file(&paths.server_lock).ok();
    drop(lock);
    result
}

/// The server's ends of `in` and `ctl`. It also holds a write end of each, so their read ends
/// never see end-of-file when a client leaves.
struct Pipes {
    input: File,
    _input_writer: File,
    ctl: File,
    _ctl_writer: File,
}

impl Pipes {
    fn create(paths: &Paths) -> Result<Self> {
        for fifo in paths.fifos() {
            session::remove_fifo(fifo)?;
        }
        session::make_fifo(&paths.input)?;
        let (input, input_writer) = open_both_ends(&paths.input)?;
        session::make_fifo(&paths.output)?;
        session::make_fifo(&paths.ctl_staging)?;
        let (ctl, ctl_writer) = open_both_ends(&paths.ctl_staging)?;
        fs::rename(&paths.ctl_staging, &paths.ctl)
            .with_context(|| format!("cannot create {}", paths.ctl.display()))?;
        Ok(Self {
            input,
            _input_writer: input_writer,
            ctl,
            _ctl_writer: ctl_writer,
        })
    }
}

fn open_both_ends(path: &Path) -> Result<(File, File)> {
    let open = |write: bool| {
        OpenOptions::new()
            .read(!write)
            .write(write)
            .custom_flags(libc::O_NONBLOCK)
            .open(path)
            .with_context(|| format!("cannot open {}", path.display()))
    };
    let reader = open(false)?;
    Ok((reader, open(true)?))
}

fn serve(
    args: &ServeArgs,
    paths: &Paths,
    pipes: Pipes,
    secret: Option<(&str, String)>,
) -> Result<u8> {
    let (master, slave) = sys::openpty(INITIAL_WINDOW).context("cannot open a pseudo-terminal")?;
    let mut child = spawn_agent(args, slave, secret)?;
    let scrollback = usize::try_from(args.scrollback_bytes).unwrap_or(usize::MAX);
    let mut server = Server {
        master: File::from(master),
        master_open: true,
        pipes,
        output_path: paths.output.clone(),
        client: None,
        scrollback: Ring::new(scrollback),
        pending: Ring::new(scrollback.max(MIN_PENDING)),
        to_agent: Vec::new(),
        window: INITIAL_WINDOW,
        ctl_lines: WindowLines::default(),
        pgid: child.id(),
        probe_error_reported: false,
    };
    let status = match server.run(&mut child) {
        Ok(status) => status,
        Err(err) => {
            sys::signal_group(server.pgid, libc::SIGKILL).ok();
            child.wait().ok();
            return Err(err);
        }
    };
    eprintln!("mantle-launch: agent exited ({status})");
    // The terminal is gone with the agent, so its last screenful is the only account of why it
    // ended. Kept beside the pipes, owner-only; a failure to write it does not change the exit.
    let last = paths.dir.join("last-output");
    if let Err(err) = write_private(&last, &server.scrollback.to_vec()) {
        eprintln!("mantle-launch: cannot keep the last output: {err}");
    }
    Ok(exit_code(status))
}

fn spawn_agent(
    args: &ServeArgs,
    slave: std::os::fd::OwnedFd,
    secret: Option<(&str, String)>,
) -> Result<Child> {
    let (program, rest) = args.command.split_first().context("no agent program")?;
    let mut command = Command::new(program);
    command
        .args(rest)
        .current_dir(&args.cwd)
        .stdin(Stdio::from(slave.try_clone()?))
        .stdout(Stdio::from(slave.try_clone()?))
        .stderr(Stdio::from(slave));
    if env::var_os("TERM").is_none() {
        command.env("TERM", "xterm-256color");
    }
    if let Some(url) = &args.proxy {
        for name in ["HTTPS_PROXY", "HTTP_PROXY", "https_proxy", "http_proxy"] {
            command.env(name, url);
        }
        for name in ["NO_PROXY", "no_proxy"] {
            command.env(name, "localhost,127.0.0.1");
        }
    }
    // Only the agent's environment receives the credential.
    if let Some((name, value)) = &secret {
        command.env(name, value);
    }
    sys::controlling_terminal_on_stdin(&mut command);
    // Dropping `command` on return closes the parent's copies of the slave.
    command
        .spawn()
        .with_context(|| format!("cannot start {}", Path::new(program).display()))
}

struct Server {
    master: File,
    /// False once every holder of the slave has closed it (reads return EIO).
    master_open: bool,
    pipes: Pipes,
    output_path: PathBuf,
    /// Write end of `out` while a terminal is attached.
    client: Option<File>,
    scrollback: Ring,
    /// Output not yet written to the attached terminal; the oldest is dropped when it is full.
    pending: Ring,
    to_agent: Vec<u8>,
    window: Window,
    ctl_lines: WindowLines,
    pgid: u32,
    probe_error_reported: bool,
}

enum Flush {
    Done,
    Blocked,
    Gone,
}

impl Server {
    fn run(&mut self, child: &mut Child) -> Result<ExitStatus> {
        let mut stop_deadline: Option<Instant> = None;
        let mut killed = false;
        let mut next_probe = Instant::now();
        loop {
            if let Some(status) = child.try_wait()? {
                self.finish();
                return Ok(status);
            }
            if let Some(signal) = signals::take_terminate()
                && stop_deadline.is_none()
            {
                eprintln!("mantle-launch: received signal {signal}; stopping the agent");
                sys::signal_group(self.pgid, libc::SIGHUP).ok();
                sys::signal_group(self.pgid, libc::SIGTERM).ok();
                stop_deadline = Some(Instant::now() + STOP_TIMEOUT);
            }
            if let Some(deadline) = stop_deadline
                && !killed
                && Instant::now() >= deadline
            {
                eprintln!(
                    "mantle-launch: agent still running after {}s; killing it",
                    STOP_TIMEOUT.as_secs()
                );
                sys::signal_group(self.pgid, libc::SIGKILL).ok();
                killed = true;
            }
            if self.client.is_none() && Instant::now() >= next_probe {
                self.probe_client();
                next_probe = Instant::now() + PROBE_INTERVAL;
            }
            self.turn()?;
        }
    }

    /// One poll(2) round over the terminal, the pipes and the attached client.
    fn turn(&mut self) -> Result<()> {
        let master_events = if self.master_open {
            libc::POLLIN
                | if self.to_agent.is_empty() {
                    0
                } else {
                    libc::POLLOUT
                }
        } else {
            0
        };
        let input_room = self.master_open && self.to_agent.len() < MAX_TO_AGENT;
        let client_events = if self.pending.is_empty() {
            0
        } else {
            libc::POLLOUT
        };
        let mut fds = [
            sys::pollfd(self.master_open.then(|| self.master.as_fd()), master_events),
            sys::pollfd(input_room.then(|| self.pipes.input.as_fd()), libc::POLLIN),
            sys::pollfd(Some(self.pipes.ctl.as_fd()), libc::POLLIN),
            sys::pollfd(self.client.as_ref().map(AsFd::as_fd), client_events),
        ];
        sys::poll(&mut fds, PROBE_INTERVAL).context("poll failed")?;
        let [master, input, ctl, client] = fds.map(|fd| fd.revents);

        if master & (libc::POLLIN | libc::POLLHUP | libc::POLLERR) != 0 {
            self.read_agent();
        }
        if master & libc::POLLOUT != 0 {
            self.write_agent();
        }
        if input & libc::POLLIN != 0 {
            self.read_client_input();
        }
        if ctl & libc::POLLIN != 0 {
            self.read_ctl();
        }
        if client & (libc::POLLERR | libc::POLLHUP | libc::POLLNVAL) != 0 {
            self.detach();
        }
        self.flush_client();
        Ok(())
    }

    fn read_agent(&mut self) {
        let mut buf = [0u8; READ_CHUNK];
        for _ in 0..READS_PER_TURN {
            match self.master.read(&mut buf) {
                Ok(0) => {
                    self.close_master();
                    return;
                }
                Ok(n) => {
                    self.scrollback.push(&buf[..n]);
                    if self.client.is_some() {
                        self.pending.push(&buf[..n]);
                    }
                }
                Err(err) if err.kind() == ErrorKind::WouldBlock => return,
                Err(err) if err.kind() == ErrorKind::Interrupted => {}
                // EIO: every holder of the terminal's other side has closed it.
                Err(_) => {
                    self.close_master();
                    return;
                }
            }
        }
    }

    fn close_master(&mut self) {
        self.master_open = false;
        self.to_agent.clear();
    }

    fn write_agent(&mut self) {
        while self.master_open && !self.to_agent.is_empty() {
            match self.master.write(&self.to_agent) {
                Ok(0) => return,
                Ok(n) => {
                    self.to_agent.drain(..n);
                }
                Err(err) if err.kind() == ErrorKind::WouldBlock => return,
                Err(err) if err.kind() == ErrorKind::Interrupted => {}
                Err(_) => {
                    self.close_master();
                    return;
                }
            }
        }
    }

    fn read_client_input(&mut self) {
        let room = MAX_TO_AGENT.saturating_sub(self.to_agent.len());
        let mut buf = [0u8; READ_CHUNK];
        let take = room.min(buf.len());
        if let Ok(n) = self.pipes.input.read(&mut buf[..take]) {
            self.to_agent.extend_from_slice(&buf[..n]);
        }
        self.write_agent();
    }

    /// Applies every complete `COLS ROWS` line waiting in `ctl`.
    fn read_ctl(&mut self) {
        let mut buf = [0u8; 512];
        loop {
            match self.pipes.ctl.read(&mut buf) {
                Ok(0) => return,
                Ok(n) => {
                    for window in self.ctl_lines.feed(&buf[..n]) {
                        self.resize(window);
                    }
                }
                Err(err) if err.kind() == ErrorKind::Interrupted => {}
                Err(_) => return,
            }
        }
    }

    /// TIOCSWINSZ with a changed size makes the kernel send SIGWINCH to the foreground group.
    fn resize(&mut self, window: Window) {
        if sys::set_window(self.master.as_fd(), window).is_ok() {
            self.window = window;
        }
    }

    fn probe_client(&mut self) {
        match OpenOptions::new()
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(&self.output_path)
        {
            Ok(file) => self.attach(file),
            Err(err) if err.raw_os_error() == Some(libc::ENXIO) => {}
            Err(err) => {
                if !self.probe_error_reported {
                    eprintln!(
                        "mantle-launch: cannot open {}: {err}",
                        self.output_path.display()
                    );
                    self.probe_error_reported = true;
                }
            }
        }
    }

    /// Repaints the new terminal: the scrollback first, then a resize so the agent redraws.
    fn attach(&mut self, file: File) {
        self.client = Some(file);
        self.pending.clear();
        self.pending.push_ring(&self.scrollback);
        self.flush_client();
        let before = self.window;
        self.read_ctl();
        if self.window == before && self.master_open {
            let rows = if before.rows > 1 {
                before.rows - 1
            } else {
                before.rows + 1
            };
            sys::set_window(self.master.as_fd(), Window { rows, ..before }).ok();
            sys::set_window(self.master.as_fd(), before).ok();
        }
        sys::signal_group(self.pgid, libc::SIGWINCH).ok();
    }

    fn detach(&mut self) {
        self.client = None;
        self.pending.clear();
    }

    fn flush_client(&mut self) {
        match self.try_flush() {
            Flush::Done | Flush::Blocked => {}
            Flush::Gone => self.detach(),
        }
    }

    fn try_flush(&mut self) -> Flush {
        let Some(client) = self.client.as_mut() else {
            return Flush::Done;
        };
        while !self.pending.is_empty() {
            match client.write(self.pending.front()) {
                Ok(0) => return Flush::Blocked,
                Ok(n) => self.pending.consume(n),
                Err(err) if err.kind() == ErrorKind::WouldBlock => return Flush::Blocked,
                Err(err) if err.kind() == ErrorKind::Interrupted => {}
                // EPIPE: the terminal left.
                Err(_) => return Flush::Gone,
            }
        }
        Flush::Done
    }

    /// The agent has exited: pass its last output to the terminal, bounded in time.
    fn finish(&mut self) {
        if self.master_open {
            self.read_agent();
        }
        let deadline = Instant::now() + FINAL_FLUSH;
        while Instant::now() < deadline {
            match self.try_flush() {
                Flush::Done => return,
                Flush::Gone => {
                    self.detach();
                    return;
                }
                Flush::Blocked => {
                    let fd = self.client.as_ref().map(AsFd::as_fd);
                    let mut fds = [sys::pollfd(fd, libc::POLLOUT)];
                    if sys::poll(&mut fds, Duration::from_millis(50)).is_err() {
                        return;
                    }
                }
            }
        }
    }
}

fn exit_code(status: ExitStatus) -> u8 {
    match (status.code(), status.signal()) {
        (Some(code), _) => u8::try_from(code).unwrap_or(1),
        (None, Some(signal)) => u8::try_from(128 + signal).unwrap_or(1),
        (None, None) => 1,
    }
}

fn write_private(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write as _;
    use std::os::unix::fs::OpenOptionsExt as _;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;
    file.write_all(bytes)
}
