//! A Rust child process for OS conformance. It reports observations, never decides a test verdict.
use clap::{Parser, Subcommand};
use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::os::unix::ffi::OsStrExt;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Mode,
}
#[derive(Subcommand)]
enum Mode {
    Emit {
        #[arg(long, default_value_t = 0)]
        exit: u8,
        #[arg(last = true)]
        args: Vec<OsString>,
    },
    Agent {
        #[arg(long)]
        child: bool,
    },
    Sleep,
    Flood,
}
static TERMINATE: AtomicBool = AtomicBool::new(false);
extern "C" fn terminated(_: libc::c_int) {
    TERMINATE.store(true, Ordering::Relaxed);
}
static RESIZE: AtomicBool = AtomicBool::new(false);
extern "C" fn resized(_: libc::c_int) {
    RESIZE.store(true, Ordering::Relaxed);
}
#[allow(unsafe_code)]
fn on_resize() {
    // SAFETY: the handler only sets a lock-free flag in this isolated fixture process.
    unsafe {
        libc::signal(libc::SIGWINCH, resized as *const () as libc::sighandler_t);
        libc::signal(libc::SIGTERM, terminated as *const () as libc::sighandler_t);
        libc::signal(libc::SIGHUP, libc::SIG_IGN);
    }
}
fn window() -> anyhow::Result<()> {
    let w = mantle_launch::sys::get_window(std::io::stdin().as_fd())?;
    println!("window:{}x{}", w.cols, w.rows);
    std::io::stdout().flush()?;
    Ok(())
}
fn main() -> anyhow::Result<()> {
    match Args::parse().command {
        Mode::Emit { exit, args } => {
            for arg in args {
                std::io::stdout().write_all(arg.as_bytes())?;
            }
            std::io::stdout().flush()?;
            std::process::exit(i32::from(exit));
        }
        Mode::Flood => {
            println!("ready");
            std::io::stdout().flush()?;
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;
            std::io::stdout().write_all(&vec![b'x'; 2 * 1024 * 1024])?;
        }
        Mode::Sleep => loop {
            std::thread::sleep(Duration::from_secs(1));
        },
        Mode::Agent { child } => {
            on_resize();
            println!("ready");
            window()?;
            println!(
                "descriptor3-open:{}",
                std::fs::metadata("/proc/self/fd/3").is_ok()
            );
            if let Ok(secret) = std::env::var("MANTLE_CONFORMANCE_SECRET") {
                println!("credential-sha256:{:x}", Sha256::digest(secret.as_bytes()));
            }
            let mut descendant = if child {
                let child = Command::new(std::env::current_exe()?)
                    .arg("sleep")
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()?;
                println!("child:{}", child.id());
                Some(child)
            } else {
                None
            };
            let mut input = std::io::stdin();
            let mut line = Vec::new();
            loop {
                if TERMINATE.load(Ordering::Relaxed) {
                    println!("signal:terminate");
                    if let Some(child) = descendant.as_mut() {
                        let deadline = std::time::Instant::now() + Duration::from_secs(1);
                        while child.try_wait()?.is_none() && std::time::Instant::now() < deadline {
                            std::thread::sleep(Duration::from_millis(5));
                        }
                        let exited = child.try_wait()?.is_some();
                        println!("descendant-exited:{exited}");
                        if !exited {
                            child.kill()?;
                        }
                        child.wait()?;
                    }
                    break;
                }
                if RESIZE.swap(false, Ordering::Relaxed) {
                    window()?;
                }
                let mut fds = [mantle_launch::sys::pollfd(
                    Some(input.as_fd()),
                    libc::POLLIN,
                )];
                mantle_launch::sys::poll(&mut fds, Duration::from_millis(20))?;
                if fds[0].revents & libc::POLLIN == 0 {
                    continue;
                }
                let mut buffer = [0; 1024];
                let n = input.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                line.extend_from_slice(&buffer[..n]);
                while let Some(end) = line.iter().position(|b| *b == b'\n') {
                    let bytes: Vec<_> = line.drain(..=end).collect();
                    if bytes == b"exit\n" {
                        return Ok(());
                    }
                    std::io::stdout().write_all(b"got:")?;
                    std::io::stdout().write_all(&bytes)?;
                    std::io::stdout().flush()?;
                }
            }
        }
    }
    Ok(())
}
