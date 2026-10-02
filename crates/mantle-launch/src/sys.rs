//! Thin wrappers over the libc calls std does not offer. Every `unsafe` block of the
//! launcher outside `secret.rs` and `signals.rs` lives here.

use std::ffi::CString;
use std::io;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use crate::ctl::Window;

fn check(ret: libc::c_int) -> io::Result<libc::c_int> {
    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(ret)
    }
}

fn winsize(window: Window) -> libc::winsize {
    libc::winsize {
        ws_row: window.rows,
        ws_col: window.cols,
        ws_xpixel: 0,
        ws_ypixel: 0,
    }
}

/// Opens a pseudo-terminal pair of the given size. Both ends are close-on-exec; the master is
/// non-blocking. Returns `(master, slave)`.
#[allow(unsafe_code)]
pub fn openpty(window: Window) -> io::Result<(OwnedFd, OwnedFd)> {
    let mut master: libc::c_int = -1;
    let mut slave: libc::c_int = -1;
    let size = winsize(window);
    // SAFETY: both out-pointers refer to live locals; openpty(3) permits a null name and a null
    // termios; the winsize pointer refers to a live local for the duration of the call.
    check(unsafe {
        libc::openpty(
            &mut master,
            &mut slave,
            std::ptr::null_mut(),
            std::ptr::null(),
            &size,
        )
    })?;
    // SAFETY: openpty succeeded, so both descriptors are open and nothing else owns them.
    let (master, slave) = unsafe { (OwnedFd::from_raw_fd(master), OwnedFd::from_raw_fd(slave)) };
    add_descriptor_flags(master.as_fd(), libc::FD_CLOEXEC)?;
    add_descriptor_flags(slave.as_fd(), libc::FD_CLOEXEC)?;
    add_status_flags(master.as_fd(), libc::O_NONBLOCK)?;
    Ok((master, slave))
}

#[allow(unsafe_code)]
fn add_descriptor_flags(fd: BorrowedFd<'_>, flags: libc::c_int) -> io::Result<()> {
    // SAFETY: F_GETFD/F_SETFD on a descriptor the borrow keeps open change only its flags.
    unsafe {
        let current = check(libc::fcntl(fd.as_raw_fd(), libc::F_GETFD))?;
        check(libc::fcntl(fd.as_raw_fd(), libc::F_SETFD, current | flags))?;
    }
    Ok(())
}

#[allow(unsafe_code)]
fn add_status_flags(fd: BorrowedFd<'_>, flags: libc::c_int) -> io::Result<()> {
    // SAFETY: F_GETFL/F_SETFL on a descriptor the borrow keeps open change only its flags.
    unsafe {
        let current = check(libc::fcntl(fd.as_raw_fd(), libc::F_GETFL))?;
        check(libc::fcntl(fd.as_raw_fd(), libc::F_SETFL, current | flags))?;
    }
    Ok(())
}

/// Temporary nonblocking status, including inherited open file descriptions. Snapshot every
/// descriptor before changing any: stdin and stdout may be aliases of the same terminal.
pub struct Nonblocking(Vec<(OwnedFd, libc::c_int)>);

#[allow(unsafe_code)]
pub fn nonblocking(fds: &[BorrowedFd<'_>]) -> io::Result<Nonblocking> {
    let mut saved = Vec::with_capacity(fds.len());
    for fd in fds {
        let owned = fd.try_clone_to_owned()?;
        // SAFETY: the owned descriptor is live and F_GETFL only observes its flags.
        let flags = check(unsafe { libc::fcntl(owned.as_raw_fd(), libc::F_GETFL) })?;
        saved.push((owned, flags));
    }
    let guard = Nonblocking(saved);
    for (fd, flags) in &guard.0 {
        // SAFETY: the guard owns the descriptor and restores its original flags on every exit,
        // including a failure while configuring a later descriptor.
        check(unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK) })?;
    }
    Ok(guard)
}

impl Drop for Nonblocking {
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        for (fd, flags) in &self.0 {
            // SAFETY: descriptors stay owned and open until restoration completes.
            unsafe {
                libc::fcntl(fd.as_raw_fd(), libc::F_SETFL, *flags);
            }
        }
    }
}

#[allow(unsafe_code)]
pub fn set_window(fd: BorrowedFd<'_>, window: Window) -> io::Result<()> {
    let size = winsize(window);
    // SAFETY: TIOCSWINSZ reads one winsize from a live local; the borrow keeps the fd open.
    check(unsafe { libc::ioctl(fd.as_raw_fd(), libc::TIOCSWINSZ, &raw const size) }).map(drop)
}

#[allow(unsafe_code)]
pub fn get_window(fd: BorrowedFd<'_>) -> io::Result<Window> {
    let mut size = winsize(Window { cols: 0, rows: 0 });
    // SAFETY: TIOCGWINSZ writes one winsize into a live local; the borrow keeps the fd open.
    check(unsafe { libc::ioctl(fd.as_raw_fd(), libc::TIOCGWINSZ, &raw mut size) })?;
    Ok(Window {
        cols: size.ws_col,
        rows: size.ws_row,
    })
}

/// Sends `signal` to the process group `pgid`.
#[allow(unsafe_code)]
pub fn signal_group(pgid: u32, signal: libc::c_int) -> io::Result<()> {
    let pgid = libc::pid_t::try_from(pgid).map_err(|_| io::ErrorKind::InvalidInput)?;
    if pgid <= 1 {
        return Err(io::ErrorKind::InvalidInput.into());
    }
    // SAFETY: kill(2) takes plain integers and touches no memory; the guard above excludes the
    // values that would address the caller's own group (0) or every process (-1).
    check(unsafe { libc::kill(-pgid, signal) }).map(drop)
}

/// True once the child `pid` has exited, without reaping it. Until it is reaped its pid, and with
/// it the id of the process group it leads, cannot be given to another process.
#[allow(unsafe_code)]
pub fn has_exited(pid: u32) -> io::Result<bool> {
    // SAFETY: an all-zero siginfo_t is a valid value for waitid(2) to overwrite; the pointer
    // refers to a live local for the duration of the call.
    let mut info: libc::siginfo_t = unsafe { std::mem::zeroed() };
    // SAFETY: as above; P_PID with WNOWAIT only inspects the child, it does not reap it.
    check(unsafe {
        libc::waitid(
            libc::P_PID,
            pid,
            &raw mut info,
            libc::WEXITED | libc::WNOHANG | libc::WNOWAIT,
        )
    })?;
    // SAFETY: with WNOHANG waitid leaves si_pid zero when the child has not changed state, and
    // sets it to the child's pid when it has; the field is plain data in the live local.
    Ok(unsafe { info.si_pid() } != 0)
}

/// Makes the child of `command` a session leader whose controlling terminal is its stdin.
#[allow(unsafe_code)]
pub fn controlling_terminal_on_stdin(command: &mut Command) {
    // SAFETY: the hook runs in the forked child after std has placed the stdio descriptors and
    // before exec. It calls only setsid(2) and ioctl(2), which are async-signal-safe, allocates
    // nothing (an OS io::Error carries only the code) and takes no lock.
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1 {
                return Err(io::Error::last_os_error());
            }
            if libc::ioctl(libc::STDIN_FILENO, libc::TIOCSCTTY, 0) == -1 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
    }
}

pub fn pollfd(fd: Option<BorrowedFd<'_>>, events: libc::c_short) -> libc::pollfd {
    libc::pollfd {
        fd: fd.map_or(-1, |fd| fd.as_raw_fd()),
        events,
        revents: 0,
    }
}

/// poll(2) over descriptors the caller keeps open. An interrupting signal returns with no
/// events so the caller can look at what the signal recorded.
#[allow(unsafe_code)]
pub fn poll(fds: &mut [libc::pollfd], timeout: Duration) -> io::Result<()> {
    let millis = libc::c_int::try_from(timeout.as_millis()).unwrap_or(libc::c_int::MAX);
    let count = libc::nfds_t::try_from(fds.len()).map_err(|_| io::ErrorKind::InvalidInput)?;
    // SAFETY: the pointer and length describe a live, exclusively borrowed slice of pollfd.
    let ret = unsafe { libc::poll(fds.as_mut_ptr(), count, millis) };
    if ret == -1 {
        let err = io::Error::last_os_error();
        if err.kind() == io::ErrorKind::Interrupted {
            fds.iter_mut().for_each(|fd| fd.revents = 0);
            return Ok(());
        }
        return Err(err);
    }
    Ok(())
}

#[allow(unsafe_code)]
pub fn mkfifo(path: &Path, mode: libc::mode_t) -> io::Result<()> {
    let path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains a NUL byte"))?;
    // SAFETY: the pointer is a NUL-terminated string that outlives the call.
    check(unsafe { libc::mkfifo(path.as_ptr(), mode) }).map(drop)
}

/// A terminal in raw mode; dropping it restores the saved settings.
pub struct RawMode {
    fd: OwnedFd,
    saved: libc::termios,
}

#[allow(unsafe_code)]
pub fn raw_mode(fd: BorrowedFd<'_>) -> io::Result<RawMode> {
    // SAFETY: an all-zero termios is a valid value for tcgetattr to overwrite; tcgetattr,
    // cfmakeraw and tcsetattr only read and write the live locals passed to them, and the
    // borrow keeps the descriptor open.
    let saved = unsafe {
        let mut saved: libc::termios = std::mem::zeroed();
        check(libc::tcgetattr(fd.as_raw_fd(), &mut saved))?;
        let mut raw = saved;
        libc::cfmakeraw(&mut raw);
        check(libc::tcsetattr(fd.as_raw_fd(), libc::TCSANOW, &raw))?;
        saved
    };
    Ok(RawMode {
        fd: fd.try_clone_to_owned()?,
        saved,
    })
}

impl Drop for RawMode {
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        // SAFETY: the descriptor is owned by self and open; the termios lives in self.
        unsafe {
            libc::tcsetattr(self.fd.as_raw_fd(), libc::TCSANOW, &self.saved);
        }
    }
}
