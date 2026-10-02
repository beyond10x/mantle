//! `attach`: relays this process's terminal to a running `serve` through the named pipes.
//! Killing it (the PTY session closing) leaves the server and the agent running.

use std::fs::File;
use std::io::{self, ErrorKind, Read, Write};
use std::os::fd::{AsFd, BorrowedFd};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result, bail};

use crate::cli::AttachArgs;
use crate::session::{self, Paths};
use crate::{signals, sys};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const RELAY_CHUNK: usize = 16 * 1024;

pub fn run(args: &AttachArgs) -> Result<u8> {
    let paths = Paths::new(&args.dir);
    if !session::is_real_dir(&paths.dir) {
        bail!("no session server is running in {}", args.dir.display());
    }
    let Some(_client_lock) = session::try_lock(&paths.client_lock)? else {
        bail!("another terminal is attached");
    };
    let mut ctl = match session::open_fifo(&paths.ctl, true, libc::O_NONBLOCK) {
        Ok(file) => file,
        Err(err) if matches!(err.raw_os_error(), Some(libc::ENXIO | libc::ENOENT)) => {
            bail!("no session server is running in {}", args.dir.display());
        }
        Err(err) => {
            return Err(err).with_context(|| format!("cannot open {}", paths.ctl.display()));
        }
    };
    signals::install(&[libc::SIGTERM, libc::SIGINT, libc::SIGHUP, libc::SIGWINCH])?;
    signals::ignore(libc::SIGPIPE)?;

    let mut stdin = File::from(io::stdin().as_fd().try_clone_to_owned()?);
    let mut stdout = File::from(io::stdout().as_fd().try_clone_to_owned()?);
    let tty = !args.no_tty;
    let _raw = if tty {
        Some(sys::raw_mode(stdin.as_fd()).context("standard input is not a terminal")?)
    } else {
        None
    };
    if tty {
        send_window(&mut ctl, stdin.as_fd());
    }
    let (mut output, mut input) = connect(&paths)?;

    let _nonblocking =
        sys::nonblocking(&[stdin.as_fd(), stdout.as_fd(), output.as_fd(), input.as_fd()])
            .context("cannot make terminal relay nonblocking")?;
    let mut to_terminal = Pending::new();
    let mut to_agent = Pending::new();
    let mut terminal_open = true;
    let mut server_open = true;
    loop {
        if signals::take_terminate().is_some() {
            break;
        }
        if signals::take_resize() && tty {
            send_window(&mut ctl, stdin.as_fd());
        }
        if to_terminal.empty() && (!server_open || (!terminal_open && to_agent.empty())) {
            break;
        }
        let mut fds = [
            sys::pollfd(
                (terminal_open && to_agent.empty()).then(|| stdin.as_fd()),
                libc::POLLIN,
            ),
            sys::pollfd(
                (server_open && to_terminal.empty()).then(|| output.as_fd()),
                libc::POLLIN,
            ),
            sys::pollfd(
                (!to_terminal.empty()).then(|| stdout.as_fd()),
                libc::POLLOUT,
            ),
            sys::pollfd((!to_agent.empty()).then(|| input.as_fd()), libc::POLLOUT),
        ];
        sys::poll(&mut fds, Duration::from_secs(1)).context("poll failed")?;
        let [
            from_terminal,
            from_server,
            terminal_writable,
            agent_writable,
        ] = fds.map(|fd| fd.revents);
        // At most one bounded operation per descriptor before observing signals again. A
        // partial write retains its exact suffix; full queues stop polling their source.
        if terminal_writable != 0 && !to_terminal.flush(&mut stdout) {
            break;
        }
        if agent_writable != 0 && !to_agent.flush(&mut input) {
            break;
        }
        if from_server != 0 {
            server_open = to_terminal.read(&mut output);
        }
        if from_terminal != 0 {
            terminal_open = to_agent.read(&mut stdin);
        }
    }
    Ok(0)
}

/// One direction holds at most one read, with no allocation or growth while backpressured.
struct Pending {
    bytes: [u8; RELAY_CHUNK],
    start: usize,
    end: usize,
}
impl Pending {
    fn new() -> Self {
        Self {
            bytes: [0; RELAY_CHUNK],
            start: 0,
            end: 0,
        }
    }
    fn empty(&self) -> bool {
        self.start == self.end
    }
    fn read(&mut self, from: &mut File) -> bool {
        debug_assert!(self.empty());
        match from.read(&mut self.bytes) {
            Ok(0) => false,
            Ok(n) => {
                self.start = 0;
                self.end = n;
                true
            }
            Err(err) => matches!(err.kind(), ErrorKind::Interrupted | ErrorKind::WouldBlock),
        }
    }
    fn flush(&mut self, to: &mut File) -> bool {
        match to.write(&self.bytes[self.start..self.end]) {
            Ok(0) => false,
            Ok(n) => {
                self.start += n;
                true
            }
            Err(err) => matches!(err.kind(), ErrorKind::Interrupted | ErrorKind::WouldBlock),
        }
    }
}

fn send_window(ctl: &mut File, terminal: BorrowedFd<'_>) {
    if let Some(window) = sys::get_window(terminal).ok().and_then(|w| w.bounded()) {
        ctl.write_all(window.line().as_bytes()).ok();
    }
}

/// Opens `out` (which blocks until the server's next probe opens its write end) and then `in`.
/// The opens run on a helper thread so a server that vanished cannot hang this process.
fn connect(paths: &Paths) -> Result<(File, File)> {
    let output = paths.output.clone();
    let input = paths.input.clone();
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let opened = session::open_fifo(&output, false, 0).and_then(|output| {
            let input = session::open_fifo(&input, true, 0)?;
            Ok((output, input))
        });
        sender.send(opened).ok();
    });
    match receiver.recv_timeout(CONNECT_TIMEOUT) {
        Ok(opened) => opened.context("cannot open the session pipes"),
        Err(_) => bail!(
            "the session server did not answer within {}s",
            CONNECT_TIMEOUT.as_secs()
        ),
    }
}
