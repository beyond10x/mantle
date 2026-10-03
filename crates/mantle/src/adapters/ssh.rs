//! SSH to a worker through a provider-supplied ProxyCommand: AWS SSM (`AWS-StartSSHSession`) for
//! EC2, `virtctl port-forward --stdio` for KubeVirt. The worker has no inbound port either way.
//!
//! One channel serves both worker administration (remote commands) and the Substrate control
//! plane (a forwarded Unix socket). Mantle uses its own key and its own known-hosts file, so it
//! neither reads nor changes the operator's SSH configuration.

use std::io::Write as _;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};

#[cfg(test)]
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
    /// `instance` names the worker for host-key pinning; `proxy` is the whole
    /// ProxyCommand, in which ssh expands `%h` and `%p`.
    pub fn new(instance: &str, proxy: String, dir: &Path) -> Result<Self> {
        crate::profile::validate_state_path(dir)?;
        Ok(Self {
            instance: instance.to_owned(),
            proxy,
            key: ensure_key(dir)?,
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
            .arg(format!(
                "UserKnownHostsFile=\"{}\"",
                self.known_hosts.display()
            ))
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

    /// Bounded administration transport. Failures omit remote output, which may contain secrets.
    pub fn bounded(&self, remote: &str, stdin: Option<&[u8]>, timeout: Duration) -> Result<Output> {
        let mut command = self.command();
        command.arg(self.target()).arg("--").arg(remote);
        mantle_worker::run_bounded(&mut command, stdin, timeout, 64 * 1024)
    }

    pub fn checked_bounded(
        &self,
        remote: &str,
        stdin: Option<&[u8]>,
        timeout: Duration,
    ) -> Result<String> {
        let output = self.bounded(remote, stdin, timeout)?;
        if !output.status.success() {
            bail!("bounded remote command failed ({})", output.status);
        }
        String::from_utf8(output.stdout).context("remote output is not UTF-8")
    }

    /// Forwards the worker's Substrate socket to a private local socket for as long as the tunnel
    /// lives.
    pub fn tunnel(&self) -> Result<Tunnel> {
        self.tunnel_with(Command::spawn)
    }

    fn tunnel_with(
        &self,
        spawn: impl FnOnce(&mut Command) -> std::io::Result<Child>,
    ) -> Result<Tunnel> {
        let allocation = self.allocate_tunnel_socket()?;
        let local = allocation.local.clone();
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
        let child = spawn(&mut command).context("starting the ssh tunnel")?;
        let mut tunnel = Tunnel {
            child,
            local,
            _allocation: allocation,
        };
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

    fn allocate_tunnel_socket(&self) -> Result<SocketAllocation> {
        // Unix socket addresses have a small byte limit, and SSH's -L syntax uses colons.
        // Neither persistent state paths, worker identifiers nor ambient TMPDIR belong here.
        let directory = tempfile::Builder::new()
            .prefix("mantle-")
            .permissions(std::fs::Permissions::from_mode(0o700))
            .tempdir_in("/tmp")
            .context("allocating a private tunnel socket directory")?;
        let local = directory.path().join("s");
        Ok(SocketAllocation {
            local,
            _directory: directory,
        })
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
    _allocation: SocketAllocation,
}

struct SocketAllocation {
    local: PathBuf,
    _directory: tempfile::TempDir,
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
        // SocketAllocation's private directory is removed after the child has been reaped.
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

pub fn public_key(dir: &Path) -> Result<String> {
    let key = ensure_key(dir)?;
    let path = key.with_extension("pub");
    Ok(std::fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?
        .trim()
        .to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::net::UnixListener;

    #[test]
    fn known_hosts_option_preserves_one_literal_path_in_openssh_parser() {
        let ssh = fixture(Path::new("/tmp/profile # with spaces:λ"));
        let mut command = ssh.command();
        command.args(["-G", "fixture.invalid"]);
        let output =
            mantle_worker::run_bounded(&mut command, None, Duration::from_secs(5), 64 * 1024)
                .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let printed = String::from_utf8(output.stdout).unwrap();
        let line = printed
            .lines()
            .find(|line| line.starts_with("userknownhostsfile "))
            .unwrap();
        assert_eq!(
            line,
            "userknownhostsfile /tmp/profile # with spaces:λ/known_hosts"
        );
    }

    fn fixture(state: &Path) -> Ssh {
        Ssh {
            instance: "21c01ed2-df0f-4ff1-a79a-0c397c186073".into(),
            proxy: "fixture".into(),
            key: state.join("id_ed25519"),
            known_hosts: state.join("known_hosts"),
        }
    }

    #[test]
    fn tunnel_socket_binds_with_live_length_state_prefix() {
        const MARKER: &str = "MANTLE_SOCKET_ALLOCATION_TEST";
        if std::env::var_os(MARKER).is_some() {
            let state = state_dir().unwrap();
            assert_eq!(state.as_os_str().len(), 61);
            let ssh = fixture(&state);
            let allocation = ssh.allocate_tunnel_socket().unwrap();
            let _listener = UnixListener::bind(&allocation.local).expect(
                "production tunnel socket must bind at the observed live state-prefix length",
            );
            return;
        }
        let base = tempfile::Builder::new()
            .prefix("mantle-bind-")
            .tempdir_in("/tmp")
            .unwrap();
        let state = base
            .path()
            .join("s".repeat(61 - base.path().as_os_str().len() - 1));
        let thread = std::thread::current();
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args(["--exact", thread.name().unwrap(), "--nocapture"])
            .env(MARKER, "1")
            .env("MANTLE_STATE_DIR", &state);
        let output =
            mantle_worker::run_bounded(&mut command, None, Duration::from_secs(10), 64 * 1024)
                .unwrap();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn socket_allocations_are_private_unique_and_state_path_independent() {
        const MARKER: &str = "MANTLE_SOCKET_PATH_CLASS_TEST";
        if std::env::var_os(MARKER).is_some() {
            let state = state_dir().unwrap();
            let ssh = fixture(&state);
            std::fs::write(&ssh.key, b"test identity marker").unwrap();
            std::fs::write(&ssh.known_hosts, b"test host marker").unwrap();
            let allocations = std::thread::scope(|scope| {
                let threads: Vec<_> = (0..8)
                    .map(|_| scope.spawn(|| ssh.allocate_tunnel_socket().unwrap()))
                    .collect();
                threads
                    .into_iter()
                    .map(|thread| thread.join().unwrap())
                    .collect::<Vec<_>>()
            });
            let paths: std::collections::BTreeSet<_> =
                allocations.iter().map(|a| a.local.clone()).collect();
            assert_eq!(
                paths.len(),
                8,
                "concurrent tunnels must not replace one another"
            );
            let mut listeners = Vec::new();
            let mut directories = Vec::new();
            for allocation in &allocations {
                assert!(allocation.local.as_os_str().len() < 64);
                assert!(!allocation.local.to_str().unwrap().contains(':'));
                let directory = allocation.local.parent().unwrap();
                assert_eq!(directory.parent(), Some(Path::new("/tmp")));
                let metadata = directory.metadata().unwrap();
                assert_eq!(metadata.mode() & 0o777, 0o700);
                assert_eq!(metadata.uid(), state.metadata().unwrap().uid());
                listeners.push(UnixListener::bind(&allocation.local).unwrap());
                directories.push(directory.to_owned());
            }
            assert!(
                ssh.command()
                    .get_args()
                    .any(|arg| arg == ssh.key.as_os_str())
            );
            assert_eq!(std::fs::read(&ssh.key).unwrap(), b"test identity marker");
            assert_eq!(
                std::fs::read(&ssh.known_hosts).unwrap(),
                b"test host marker"
            );
            drop(listeners);
            drop(allocations);
            assert!(directories.iter().all(|directory| !directory.exists()));
            return;
        }
        let base = tempfile::Builder::new()
            .prefix("mantle-path-test-")
            .tempdir_in("/tmp")
            .unwrap();
        let thread = std::thread::current();
        for leaf in [
            "s".repeat(100),
            "é".repeat(60),
            "state:worker:identity".into(),
        ] {
            let state = base.path().join(leaf);
            let mut command = Command::new(std::env::current_exe().unwrap());
            command
                .args(["--exact", thread.name().unwrap(), "--nocapture"])
                .env(MARKER, "1")
                .env("MANTLE_STATE_DIR", &state)
                .env(
                    "TMPDIR",
                    state.join("deliberately-missing:ambient-directory"),
                );
            let output =
                mantle_worker::run_bounded(&mut command, None, Duration::from_secs(10), 64 * 1024)
                    .unwrap();
            assert!(
                output.status.success(),
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    fn forwarded_socket(command: &Command) -> PathBuf {
        let args: Vec<_> = command.get_args().collect();
        let forward = args.iter().position(|arg| *arg == "-L").unwrap();
        let value = args[forward + 1].to_str().unwrap();
        PathBuf::from(
            value
                .strip_suffix(&format!(":{REMOTE_SUBSTRATE_SOCKET}"))
                .unwrap(),
        )
    }

    #[test]
    fn tunnel_socket_directory_lives_until_child_cleanup_and_is_removed_on_errors() {
        let ssh = fixture(Path::new("/unused:state/é"));
        let mut listener = None;
        let tunnel = ssh
            .tunnel_with(|command| {
                listener = Some(UnixListener::bind(forwarded_socket(command))?);
                Command::new("/usr/bin/sleep").arg("30").spawn()
            })
            .unwrap();
        let directory = tunnel.socket().parent().unwrap().to_owned();
        let pid = tunnel.child.id();
        assert!(directory.exists());
        drop(listener);
        drop(tunnel);
        assert!(
            !Path::new(&format!("/proc/{pid}")).exists(),
            "SSH child must be reaped before return from drop"
        );
        assert!(!directory.exists());

        let mut failed_directory = None;
        let error = ssh
            .tunnel_with(|command| {
                failed_directory = Some(forwarded_socket(command).parent().unwrap().to_owned());
                Command::new("/definitely-absent-mantle-ssh-fixture").spawn()
            })
            .err()
            .expect("spawn failure must be returned");
        assert!(error.to_string().contains("starting the ssh tunnel"));
        assert!(!failed_directory.unwrap().exists());

        let mut exited_directory = None;
        let error = ssh
            .tunnel_with(|command| {
                exited_directory = Some(forwarded_socket(command).parent().unwrap().to_owned());
                Command::new("/usr/bin/false").spawn()
            })
            .err()
            .expect("early SSH exit must be returned");
        assert!(error.to_string().contains("the ssh tunnel exited"));
        assert!(!exited_directory.unwrap().exists());
    }
}
