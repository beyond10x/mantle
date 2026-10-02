//! Connects the local terminal to a Substrate PTY session.

use std::io::{Read as _, Write as _};

use anyhow::{Context, Result, bail};
use b10x_substrate_sdk::{PipeChannel, PipeFrame, PtyWindow};
use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::mpsc;

pub fn local_window() -> PtyWindow {
    let (columns, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    PtyWindow {
        columns: u64::from(columns.max(1)),
        rows: u64::from(rows.max(1)),
    }
}

struct RawMode;

impl RawMode {
    fn enable() -> Result<Self> {
        crossterm::terminal::enable_raw_mode().context("switching the terminal to raw mode")?;
        Ok(Self)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
    }
}

pub enum Ended {
    Detached,
    Exited,
    Closed,
}

/// `Ctrl-]` (design § 10); followed by `d` it detaches, followed by anything else both bytes pass.
const ESCAPE: u8 = 0x1d;

/// Splits keyboard input at the detach sequence `Ctrl-] d`, across reads.
#[derive(Default)]
struct DetachKey {
    pending_escape: bool,
}

impl DetachKey {
    /// The bytes to forward, and whether the user asked to detach.
    fn feed(&mut self, input: &[u8]) -> (Vec<u8>, bool) {
        let mut out = Vec::with_capacity(input.len() + 1);
        for &byte in input {
            if self.pending_escape {
                self.pending_escape = false;
                if byte == b'd' {
                    return (out, true);
                }
                out.push(ESCAPE);
                if byte == ESCAPE {
                    self.pending_escape = true;
                    continue;
                }
                out.push(byte);
            } else if byte == ESCAPE {
                self.pending_escape = true;
            } else {
                out.push(byte);
            }
        }
        (out, false)
    }
}

pub async fn run(mut channel: PipeChannel) -> Result<Ended> {
    let _raw = RawMode::enable()?;
    let (sender, mut receiver) = mpsc::channel::<Vec<u8>>(64);
    // Blocking stdin reads stay on their own thread; the process exits without joining it.
    std::thread::spawn(move || {
        let mut stdin = std::io::stdin().lock();
        let mut buffer = [0u8; 4096];
        loop {
            match stdin.read(&mut buffer) {
                Ok(0) | Err(_) => break,
                Ok(count) => {
                    if sender.blocking_send(buffer[..count].to_vec()).is_err() {
                        break;
                    }
                }
            }
        }
    });
    let mut resized = signal(SignalKind::window_change()).context("watching window size")?;
    let mut window = local_window();
    let mut stdout = std::io::stdout();
    let mut stdin_open = true;
    let mut detach = DetachKey::default();
    loop {
        tokio::select! {
            frame = channel.next_frame() => match frame.context("reading from the session")? {
                Some(PipeFrame::Output { bytes, .. }) => {
                    stdout.write_all(&bytes)?;
                    stdout.flush()?;
                }
                Some(PipeFrame::Exit { .. }) => return Ok(Ended::Exited),
                Some(PipeFrame::ProtocolError { code, message, .. }) => {
                    bail!("the session ended with a protocol error {code}: {message}")
                }
                Some(_) => {}
                None => return Ok(Ended::Closed),
            },
            input = receiver.recv(), if stdin_open => match input {
                Some(bytes) => {
                    let (forward, detached) = detach.feed(&bytes);
                    if !forward.is_empty() {
                        channel.write(forward).await.context("sending input")?;
                    }
                    if detached {
                        let _ = channel.close().await;
                        return Ok(Ended::Detached);
                    }
                }
                None => {
                    stdin_open = false;
                    channel.close_input().await.context("closing input")?;
                }
            },
            _ = resized.recv() => {
                let now = local_window();
                if now != window {
                    window = now;
                    channel.resize(now).await.context("resizing the remote terminal")?;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctrl_bracket_d_detaches_and_forwards_what_came_before() {
        let mut key = DetachKey::default();
        assert_eq!(key.feed(b"ab\x1dd"), (b"ab".to_vec(), true));
    }

    #[test]
    fn the_sequence_is_recognised_across_reads() {
        let mut key = DetachKey::default();
        assert_eq!(key.feed(b"x\x1d"), (b"x".to_vec(), false));
        assert_eq!(key.feed(b"d"), (Vec::new(), true));
    }

    #[test]
    fn escape_followed_by_anything_else_passes_both_bytes() {
        let mut key = DetachKey::default();
        assert_eq!(key.feed(b"\x1dq"), (b"\x1dq".to_vec(), false));
        assert_eq!(key.feed(b"\x1d\x1dd"), (b"\x1d".to_vec(), true));
    }
}
