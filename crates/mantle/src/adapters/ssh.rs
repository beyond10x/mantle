//! SSH to a worker through a provider-supplied ProxyCommand: AWS SSM (`AWS-StartSSHSession`) for
//! EC2, `virtctl port-forward --stdio` for KubeVirt. The worker has no inbound port either way.
//!
//! One channel serves both worker administration (remote commands) and the Substrate control
//! plane (a forwarded Unix socket). Mantle uses its own key and its own known-hosts file, so it
//! neither reads nor changes the operator's SSH configuration.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};

use crate::config::state_dir;

pub const REMOTE_USER: &str = "ubuntu";
pub const REMOTE_SUBSTRATE_SOCKET: &str = "/run/substrate/substrate.sock";

#[derive(Debug, Clone)]
pub struct Ssh {
    instance: String,
    proxy: String,
    key: PathBuf,
    known_hosts: PathBuf,
}

impl Ssh {
    /// `instance` names the worker for host-key pinning and socket names; `proxy` is the whole
    /// ProxyCommand, in which ssh expands `%h` and `%p`.
    pub fn new(instance: &str, proxy: String) -> Result<Self> {
        let dir = state_dir()?;
        Ok(Self {
            instance: instance.to_owned(),
            proxy,
            key: ensure_key(&dir)?,
            known_hosts: dir.join("known_hosts"),
        })
    }

    fn command(&self) -> Command {
        let mut command = Command::new("ssh");
        command
            .arg("-i")
            .arg(&self.key)
            .args(["-o", "IdentitiesOnly=yes", "-o", "BatchMode=yes"])
            .arg("-o")
            .arg(format!("UserKnownHostsFile={}", self.known_hosts.display()))
            .args([
                "-o",
                "StrictHostKeyChecking=accept-new",
                "-o",
                "ServerAliveInterval=30",
                "-o",
                "ConnectTimeout=60",
                "-o",
                "LogLevel=ERROR",
                "-F",
                "/dev/null",
            ])
            .arg("-o")
            .arg(format!("ProxyCommand={}", self.proxy))
            .arg("-o")
            .arg(format!("HostKeyAlias={}", self.instance));
        command
    }

    fn target(&self) -> String {
        format!("{REMOTE_USER}@{}", self.instance)
    }

    /// Runs one remote command. `remote` is interpreted by the worker's login shell, so callers
    /// pass only fixed strings and values they have validated.
    pub fn run(&self, remote: &str, stdin: Option<&[u8]>) -> Result<Output> {
        let mut command = self.command();
        command
            .arg(self.target())
            .arg("--")
            .arg(remote)
            .stdin(if stdin.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn().context("starting ssh")?;
        if let Some(bytes) = stdin {
            let mut pipe = child.stdin.take().context("ssh stdin")?;
            pipe.write_all(bytes).context("writing to ssh")?;
        }
        child.wait_with_output().context("waiting for ssh")
    }

    /// Runs a remote command and fails with its stderr when it exits non-zero.
    pub fn check(&self, remote: &str, stdin: Option<&[u8]>) -> Result<String> {
        let output = self.run(remote, stdin)?;
        if !output.status.success() {
            bail!(
                "remote command failed ({}): {}",
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Forwards the worker's Substrate socket to a private local socket for as long as the tunnel
    /// lives.
    pub fn tunnel(&self) -> Result<Tunnel> {
        let dir = state_dir()?.join("run");
        crate::config::ensure_private_dir(&dir)?;
        let local = dir.join(format!("{}-{}.sock", self.instance, std::process::id()));
        let _ = std::fs::remove_file(&local);
        let mut command = self.command();
        command
            .args([
                "-N",
                "-o",
                "ExitOnForwardFailure=yes",
                "-o",
                "StreamLocalBindUnlink=yes",
                "-o",
                "StreamLocalBindMask=0177",
            ])
            .arg("-L")
            .arg(format!("{}:{REMOTE_SUBSTRATE_SOCKET}", local.display()))
            .arg(self.target())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped());
        let child = command.spawn().context("starting the ssh tunnel")?;
        let mut tunnel = Tunnel { child, local };
        let deadline = Instant::now() + Duration::from_secs(90);
        while !tunnel.local.exists() {
            if let Some(status) = tunnel.child.try_wait()? {
                let mut stderr = String::new();
                if let Some(mut pipe) = tunnel.child.stderr.take() {
                    use std::io::Read as _;
                    let _ = pipe.read_to_string(&mut stderr);
                }
                bail!("the ssh tunnel exited ({status}): {}", stderr.trim());
            }
            if Instant::now() > deadline {
                bail!("the ssh tunnel did not come up within 90s");
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(tunnel)
    }

    /// An interactive login shell on the worker, on the operator's terminal.
    pub fn interactive(&self) -> Result<std::process::ExitStatus> {
        self.command()
            .arg("-t")
            .arg(self.target())
            .status()
            .context("starting ssh")
    }

    /// Whether the worker answers SSH at all.
    pub fn reachable(&self) -> bool {
        self.run("true", None)
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

pub struct Tunnel {
    child: Child,
    local: PathBuf,
}

impl Tunnel {
    pub fn socket(&self) -> &Path {
        &self.local
    }
}

impl Drop for Tunnel {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_file(&self.local);
    }
}

fn ensure_key(dir: &Path) -> Result<PathBuf> {
    let key = dir.join("id_ed25519");
    if !key.exists() {
        let status = Command::new("ssh-keygen")
            .args(["-q", "-t", "ed25519", "-N", "", "-C", "mantle", "-f"])
            .arg(&key)
            .stdin(Stdio::null())
            .status()
            .context("running ssh-keygen")?;
        if !status.success() {
            bail!("ssh-keygen failed ({status})");
        }
    }
    Ok(key)
}

pub fn public_key() -> Result<String> {
    let key = ensure_key(&state_dir()?)?;
    let path = key.with_extension("pub");
    Ok(std::fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?
        .trim()
        .to_owned())
}
