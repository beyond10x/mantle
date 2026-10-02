use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use anyhow::{Result, bail};

static TERMINATE: AtomicI32 = AtomicI32::new(0);
static RESIZED: AtomicBool = AtomicBool::new(false);

extern "C" fn record(signal: libc::c_int) {
    if signal == libc::SIGWINCH {
        RESIZED.store(true, Ordering::SeqCst);
    } else {
        TERMINATE.store(signal, Ordering::SeqCst);
    }
}

/// Records the given signals instead of taking their default action. SIGWINCH is recorded as a
/// resize, every other signal as a request to stop. A pending signal interrupts poll(2).
/// Handlers reset to the default on exec, so children are unaffected.
pub fn install(signals: &[libc::c_int]) -> Result<()> {
    for &signal in signals {
        set_action(
            signal,
            record as extern "C" fn(libc::c_int) as libc::sighandler_t,
        )?;
    }
    Ok(())
}

/// Ignores `signal`; used for SIGPIPE so a departed reader shows up as EPIPE.
pub fn ignore(signal: libc::c_int) -> Result<()> {
    set_action(signal, libc::SIG_IGN)
}

#[allow(unsafe_code)]
fn set_action(signal: libc::c_int, handler: libc::sighandler_t) -> Result<()> {
    // SAFETY: an all-zero sigaction is a valid value; the only handler installed performs atomic
    // stores, which are async-signal-safe; the pointers passed to libc outlive the calls.
    let installed = unsafe {
        let mut action: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = handler;
        action.sa_flags = libc::SA_RESTART;
        libc::sigemptyset(&mut action.sa_mask);
        libc::sigaction(signal, &action, std::ptr::null_mut())
    };
    if installed != 0 {
        bail!(
            "cannot install handler for signal {signal}: {}",
            std::io::Error::last_os_error()
        );
    }
    Ok(())
}

pub fn take_terminate() -> Option<i32> {
    match TERMINATE.swap(0, Ordering::SeqCst) {
        0 => None,
        signal => Some(signal),
    }
}

pub fn take_resize() -> bool {
    RESIZED.swap(false, Ordering::SeqCst)
}
