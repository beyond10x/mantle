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
const READY: &[u8] = b"\x1eMANTLE-ATTACH-READY-1\x1f";
const ACK: &[u8] = b"\x1eMANTLE-ATTACH-ACK-1\x1f";

#[derive(Default)]
struct Readiness {
    matched: usize,
}

impl Readiness {
    fn feed(&mut self, bytes: &[u8]) -> Result<bool> {
        if bytes.len() > READY.len() - self.matched
            || bytes != &READY[self.matched..self.matched + bytes.len()]
        {
            bail!("invalid launcher readiness marker; update the worker launcher");
        }
        self.matched += bytes.len();
        Ok(self.matched == READY.len())
    }
}

async fn wait_ready(channel: &mut PipeChannel) -> Result<PtyWindow> {
    let mut ready = Readiness::default();
    loop {
        match channel
            .next_frame()
            .await
            .context("waiting for launcher readiness")?
        {
            Some(PipeFrame::Output { bytes, .. }) => {
                if ready.feed(&bytes)? {
                    // The user may resize while the remote launcher starts. Synchronize before
                    // releasing replay rather than treating that new size as already applied.
                    let window = local_window();
                    channel
                        .resize(window)
                        .await
                        .context("synchronizing terminal size")?;
                    channel
                        .write(ACK.to_vec())
                        .await
                        .context("acknowledging launcher readiness")?;
                    return Ok(window);
                }
            }
            Some(PipeFrame::ProtocolError { code, message, .. }) => {
                bail!("the session ended with a protocol error {code}: {message}")
            }
            None | Some(PipeFrame::Exit { .. }) => {
                bail!("launcher ended before terminal readiness")
            }
            Some(_) => {}
        }
    }
}

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
    let mut terminated = signal(SignalKind::terminate()).context("watching termination")?;
    let mut interrupted = signal(SignalKind::interrupt()).context("watching interruption")?;
    let mut hung_up = signal(SignalKind::hangup()).context("watching hangup")?;
    let mut resized = signal(SignalKind::window_change()).context("watching window size")?;
    let _raw = RawMode::enable()?;
    let mut window = tokio::select! {
        result = tokio::time::timeout(std::time::Duration::from_secs(10), wait_ready(&mut channel)) => {
            result.context("launcher readiness timed out after 10s")??
        }
        _ = terminated.recv() => return Ok(Ended::Closed),
        _ = interrupted.recv() => return Ok(Ended::Closed),
        _ = hung_up.recv() => return Ok(Ended::Closed),
    };
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
    let mut stdout = std::io::stdout();
    let mut stdin_open = true;
    let mut detach = DetachKey::default();
    loop {
        tokio::select! {
            _ = terminated.recv() => return Ok(Ended::Closed),
            _ = interrupted.recv() => return Ok(Ended::Closed),
            _ = hung_up.recv() => return Ok(Ended::Closed),
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
    fn readiness_marker_is_consumed_across_every_split() {
        let marker = b"\x1eMANTLE-ATTACH-READY-1\x1f";
        for split in 0..marker.len() {
            let mut ready = Readiness::default();
            assert!(!ready.feed(&marker[..split]).unwrap());
            assert!(ready.feed(&marker[split..]).unwrap());
        }
        let mut ready = Readiness::default();
        assert!(ready.feed(b"not a launcher").is_err());
        let mut ready = Readiness::default();
        let mut unexpected = marker.to_vec();
        unexpected.extend_from_slice(b"premature replay");
        assert!(ready.feed(&unexpected).is_err());
    }

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
