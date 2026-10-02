//! `serve`: owns a pseudo-terminal running the agent and relays it to at most one attached
//! terminal through the named pipes of the session directory.
//!
//! Client arrival is learnt by opening `out` for writing without blocking every
//! `PROBE_INTERVAL`: the open fails with ENXIO until an `attach` holds the read end.
//! Departure shows as POLLERR on that write end (no reader left) or EPIPE on a write.

use std::env;
use std::fs::{self, File, OpenOptions, Permissions};
use std::io::{self, ErrorKind, Read, Write};
use std::os::fd::AsFd;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
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
/// How long a redraw keeps the window one row off before putting it back. Node and Bun read the
/// size on SIGWINCH and emit `resize` only when it differs from the last one read, so the agent
/// must find the changed size before it is restored.
const REDRAW_HOLD: Duration = Duration::from_millis(200);

pub fn run(args: &ServeArgs) -> Result<u8> {
    for dir in &args.private_dirs {
        session::prepare_private_dir(dir, false)?;
    }
    for dir in &args.volatile_dirs {
        session::prepare_private_dir(dir, true)?;
    }
    for file in &args.check_private_files {
        session::check_private_file(file)?;
    }
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
    session::prepare_dir(&args.dir)?;
    let paths = Paths::new(&args.dir);
    let Some(lock) = session::try_lock(&paths.server_lock)? else {
        bail!(
            "a session server is already running in {}",
            args.dir.display()
        );
    };
    signals::install(&[libc::SIGTERM, libc::SIGINT])?;
    signals::ignore(libc::SIGPIPE)?;

    let result = (|| {
        if args.volatile_replay {
            require_absent_output(&paths.dir.join("last-output"))?;
        }
        let pipes = Pipes::create(&paths)?;
        serve(args, &paths, pipes, secret)
    })();
    for fifo in paths.fifos() {
        if let Err(err) = session::remove_fifo(fifo) {
            eprintln!("mantle-launch: {err:#}");
        }
    }
    fs::remove_file(&paths.server_lock).ok();
    drop(lock);
    result
}

/// Refuse every existing entry without following links or reading its contents. This runs
/// under the server lock before readiness is published or the agent can produce output.
fn require_absent_output(path: &Path) -> Result<()> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).context("cannot inspect last-output for volatile replay"),
        Ok(_) => {
            bail!("volatile replay requires an absent last-output; existing entry left unchanged")
        }
    }
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
        session::open_fifo(path, write, libc::O_NONBLOCK)
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
        repaint: Repaint::default(),
        nudge: Nudge::default(),
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
    if !args.volatile_replay {
        let last = paths.dir.join("last-output");
        if let Err(err) = write_private(&last, &server.scrollback.to_vec()) {
            eprintln!("mantle-launch: cannot keep the last output: {err}");
        }
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
    /// Set when `pending` dropped output the terminal never received.
    repaint: Repaint,
    /// A forced redraw waiting to put the true window size back.
    nudge: Nudge,
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

/// Whether the attached terminal lost output to the full `pending` ring. Its screen is then wrong,
/// so once it has drained what is still pending the agent is made to redraw it, as on attach.
#[derive(Debug, Default)]
struct Repaint {
    owed: bool,
}

impl Repaint {
    fn dropped(&mut self) {
        self.owed = true;
    }

    /// A new or departed terminal owes nothing; attach repaints on its own.
    fn clear(&mut self) {
        self.owed = false;
    }

    /// True once per loss, when output was dropped and the terminal has since caught up.
    fn due(&mut self, caught_up: bool) -> bool {
        let due = self.owed && caught_up;
        if due {
            self.owed = false;
        }
        due
    }
}

/// A forced redraw as two real size changes: one row off now, the true size `REDRAW_HOLD` later,
/// on a later poll turn. Each is a change the agent can observe; a change and its undo in one
/// turn is not.
#[derive(Debug, Default)]
struct Nudge {
    restore_at: Option<Instant>,
}

impl Nudge {
    /// Starts a nudge from `window` and returns the size to set now, or `None` while one is
    /// already under way (its restore will repaint as well).
    fn start(&mut self, window: Window, now: Instant) -> Option<Window> {
        if self.restore_at.is_some() {
            return None;
        }
        self.restore_at = Some(now + REDRAW_HOLD);
        let rows = if window.rows > 1 {
            window.rows - 1
        } else {
            window.rows + 1
        };
        Some(Window { rows, ..window })
    }

    /// True once, when the held size is due to be put back.
    fn due(&mut self, now: Instant) -> bool {
        let due = self.restore_at.is_some_and(|at| now >= at);
        if due {
            self.restore_at = None;
        }
        due
    }

    /// A size set from `ctl` replaces the held one, and is itself a change the agent sees.
    fn cancel(&mut self) {
        self.restore_at = None;
    }

    /// How long poll may wait without making the restore late.
    fn timeout(&self, now: Instant, max: Duration) -> Duration {
        self.restore_at
            .map_or(max, |at| at.saturating_duration_since(now).min(max))
    }
}

impl Server {
    fn run(&mut self, child: &mut Child) -> Result<ExitStatus> {
        let mut stop_deadline: Option<Instant> = None;
        let mut killed = false;
        let mut next_probe = Instant::now();
        loop {
            if sys::has_exited(child.id())? {
                // The rest of the agent's process group outlives a leader that exits, and a
                // member may ignore SIGHUP and SIGTERM. Killed while the leader is unreaped, so
                // the group id is still the agent's.
                sys::signal_group(self.pgid, libc::SIGKILL).ok();
                let status = child.wait()?;
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
        let timeout = self.nudge.timeout(Instant::now(), PROBE_INTERVAL);
        sys::poll(&mut fds, timeout).context("poll failed")?;
        let [master, input, ctl, client] = fds.map(|fd| fd.revents);

        if self.nudge.due(Instant::now()) {
            self.restore_window();
        }
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
        if self.client.is_some() && self.repaint.due(self.pending.is_empty()) {
            self.redraw();
        }
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
                    if self.client.is_some() && self.pending.push(&buf[..n]) > 0 {
                        self.repaint.dropped();
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

    /// Applies the complete `COLS ROWS` lines waiting in `ctl`, the last of each read only. At
    /// most `READS_PER_TURN` reads, so a writer that keeps the pipe full cannot starve the terminal.
    fn read_ctl(&mut self) {
        let mut buf = [0u8; 512];
        for _ in 0..READS_PER_TURN {
            match self.pipes.ctl.read(&mut buf) {
                Ok(0) => return,
                Ok(n) => {
                    if let Some(window) = self.ctl_lines.feed(&buf[..n]).pop() {
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
            self.nudge.cancel();
        }
    }

    fn probe_client(&mut self) {
        match session::open_fifo(&self.output_path, true, libc::O_NONBLOCK) {
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
        self.repaint.clear();
        self.pending.push_ring(&self.scrollback);
        self.flush_client();
        let before = self.window;
        self.read_ctl();
        if self.window == before {
            self.redraw();
        } else {
            sys::signal_group(self.pgid, libc::SIGWINCH).ok();
        }
    }

    /// Makes the agent redraw its screen at the current size: the window goes one row off now
    /// and back `REDRAW_HOLD` later (see `Nudge`), each with SIGWINCH to the group. The poll
    /// loop keeps running in between.
    fn redraw(&mut self) {
        if self.master_open
            && let Some(nudged) = self.nudge.start(self.window, Instant::now())
        {
            sys::set_window(self.master.as_fd(), nudged).ok();
        }
        sys::signal_group(self.pgid, libc::SIGWINCH).ok();
    }

    /// Ends a nudge: the true size again, which the agent sees as a second change.
    fn restore_window(&mut self) {
        if self.master_open {
            sys::set_window(self.master.as_fd(), self.window).ok();
            sys::signal_group(self.pgid, libc::SIGWINCH).ok();
        }
    }

    fn detach(&mut self) {
        self.client = None;
        self.pending.clear();
        self.repaint.clear();
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

/// Writes `bytes` to `path` as an owner-only regular file. A symlink is not followed, a file with
/// another link is refused (either would redirect the write), and an existing file is forced to
/// 0600 through its descriptor before it is truncated.
fn write_private(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
    {
        Ok(file) => file,
        Err(err) if err.raw_os_error() == Some(libc::ELOOP) => {
            return Err(io::Error::other(format!(
                "{} is a symbolic link; refusing to follow it",
                path.display()
            )));
        }
        Err(err) => return Err(err),
    };
    let meta = file.metadata()?;
    if !meta.is_file() || meta.nlink() != 1 {
        return Err(io::Error::other(format!(
            "{} is not a regular file with a single link",
            path.display()
        )));
    }
    file.set_permissions(Permissions::from_mode(0o600))?;
    file.set_len(0)?;
    file.write_all(bytes)
}

#[cfg(test)]
mod tests {
    use std::os::unix::fs::{PermissionsExt, symlink};

    use super::*;
    use crate::session::tests::scratch;

    #[test]
    fn nothing_is_repainted_while_no_output_was_dropped() {
        let mut repaint = Repaint::default();
        assert!(!repaint.due(true));
        assert!(!repaint.due(false));
    }

    #[test]
    fn dropped_output_is_repainted_once_the_terminal_has_caught_up() {
        let mut repaint = Repaint::default();
        repaint.dropped();
        assert!(
            !repaint.due(false),
            "repainted while output was still pending"
        );
        assert!(repaint.due(true), "no repaint after the terminal caught up");
        assert!(!repaint.due(true), "repainted twice for one loss");
        repaint.dropped();
        repaint.dropped();
        assert!(repaint.due(true));
        assert!(!repaint.due(true));
    }

    const WINDOW: Window = Window { cols: 80, rows: 24 };

    #[test]
    fn a_nudge_holds_a_real_size_change_before_restoring() {
        let start = Instant::now();
        let mut nudge = Nudge::default();
        assert_eq!(
            nudge.start(WINDOW, start),
            Some(Window { cols: 80, rows: 23 })
        );
        assert!(!nudge.due(start), "restored in the same turn");
        assert!(
            !nudge.due(start + REDRAW_HOLD / 2),
            "restored before the hold"
        );
        assert!(nudge.due(start + REDRAW_HOLD));
        assert!(!nudge.due(start + REDRAW_HOLD * 2), "restored twice");
    }

    #[test]
    fn a_nudge_under_way_is_not_restarted_and_can_be_cancelled() {
        let start = Instant::now();
        let mut nudge = Nudge::default();
        assert!(nudge.start(WINDOW, start).is_some());
        assert_eq!(nudge.start(WINDOW, start + REDRAW_HOLD / 2), None);
        assert!(
            nudge.due(start + REDRAW_HOLD),
            "a second start moved the restore"
        );
        assert!(nudge.start(WINDOW, start).is_some());
        nudge.cancel();
        assert!(!nudge.due(start + REDRAW_HOLD * 2));
    }

    #[test]
    fn a_one_row_window_is_nudged_up() {
        let mut nudge = Nudge::default();
        let one = Window { cols: 80, rows: 1 };
        assert_eq!(
            nudge.start(one, Instant::now()),
            Some(Window { cols: 80, rows: 2 })
        );
    }

    #[test]
    fn poll_does_not_sleep_past_a_pending_restore() {
        let start = Instant::now();
        let mut nudge = Nudge::default();
        assert_eq!(nudge.timeout(start, PROBE_INTERVAL), PROBE_INTERVAL);
        nudge.start(WINDOW, start);
        let quarter = REDRAW_HOLD / 4;
        assert_eq!(
            nudge.timeout(start + REDRAW_HOLD - quarter, Duration::from_secs(5)),
            quarter
        );
        assert_eq!(
            nudge.timeout(start + REDRAW_HOLD * 2, PROBE_INTERVAL),
            Duration::ZERO
        );
    }

    #[test]
    fn a_new_terminal_does_not_inherit_an_owed_repaint() {
        let mut repaint = Repaint::default();
        repaint.dropped();
        repaint.clear();
        assert!(!repaint.due(true));
    }

    #[test]
    fn private_output_refuses_links_and_resets_the_mode() {
        let dir = scratch("private");
        let victim = dir.join("victim");
        fs::write(&victim, "untouched").unwrap();

        let soft = dir.join("soft");
        symlink(&victim, &soft).unwrap();
        assert!(write_private(&soft, b"new").is_err());
        let hard = dir.join("hard");
        fs::hard_link(&victim, &hard).unwrap();
        assert!(write_private(&hard, b"new").is_err());
        assert_eq!(fs::read_to_string(&victim).unwrap(), "untouched");

        let existing = dir.join("existing");
        fs::write(&existing, "a much longer old content").unwrap();
        fs::set_permissions(&existing, fs::Permissions::from_mode(0o644)).unwrap();
        write_private(&existing, b"new").unwrap();
        assert_eq!(fs::read_to_string(&existing).unwrap(), "new");
        let mode = fs::metadata(&existing).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "mode {mode:o}");
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn volatile_preflight_refuses_metadata_errors_other_than_absence() {
        let dir = scratch("volatile-inspection");
        let file = dir.join("file");
        fs::write(&file, b"unchanged").unwrap();
        let invalid = file.join("last-output");
        assert_eq!(
            fs::symlink_metadata(&invalid).unwrap_err().raw_os_error(),
            Some(libc::ENOTDIR)
        );
        assert!(require_absent_output(&invalid).is_err());
        assert_eq!(fs::read(&file).unwrap(), b"unchanged");
        assert!(require_absent_output(&dir.join("absent")).is_ok());
        fs::remove_dir_all(dir).unwrap();
    }
}
