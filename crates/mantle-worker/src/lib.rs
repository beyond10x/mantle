//! One bounded, atomic pinned-agent installation transaction for every provider.
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt, symlink};
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, TryLockError};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail, ensure};
use nix::fcntl::{FcntlArg, OFlag, fcntl};
use nix::sys::signal::{Signal, killpg};
use nix::sys::wait::{Id, WaitPidFlag, WaitStatus, waitid};
use nix::unistd::{Pid, Uid};
use sha2::{Digest, Sha256};

pub use mantle_worker_model::{
    MantleSessionAgentInstallationFacts as InstallationFacts,
    MantleSessionAgentInstallationOutcome as InstallationOutcome,
};

pub const VERSION: &str = "0.153.4";
pub const ARCHITECTURE: &str = "x86_64-unknown-linux-musl";
pub const ARCHIVE_SHA256: &str = "c485e889611b73ff5c3cc11fb5cea7551ef504465ad8675163766b9b1a9ec84a";
pub const BINARY_SHA256: &str = "56ef98ab4032d317ab26e9b5e5a175650717351edb16ed9cde0cb6d1734d62da";
pub const URL: &str = "https://github.com/openai/codex/releases/download/rust-v0.153.4/codex-x86_64-unknown-linux-musl.zst";
pub const MAX_HELPER_BYTES: usize = 64 * 1024 * 1024;
const MAX_ARCHIVE: u64 = 128 * 1024 * 1024;
const MAX_BINARY: u64 = 512 * 1024 * 1024;
const TRANSACTION_TIMEOUT: Duration = Duration::from_secs(180);

/// Decisions consume observations from production probes, never persisted WorkerRecord state.
pub fn worker_ready(common: bool, claude: bool, executable: bool, slot: bool) -> bool {
    common && (!claude || (executable && slot))
}

pub fn shared_upgrade_deferred(active: bool, installed: bool, changed: bool) -> bool {
    active && installed && changed
}

pub fn installation_decision(
    architecture: &str,
    current: bool,
    archive: bool,
    binary: bool,
    version: bool,
) -> InstallationOutcome {
    if architecture != "x86_64" {
        InstallationOutcome::V2
    } else if current {
        InstallationOutcome::V0
    } else if archive && binary && version {
        InstallationOutcome::V1
    } else {
        InstallationOutcome::V2
    }
}

fn candidate() -> InstallationFacts {
    InstallationFacts {
        version: VERSION.into(),
        architecture: ARCHITECTURE.into(),
        compressed_sha256: ARCHIVE_SHA256.into(),
        binary_sha256: BINARY_SHA256.into(),
    }
}

pub struct Installer {
    root: PathBuf,
    candidate: InstallationFacts,
    version_output: String,
}

impl Installer {
    pub fn production() -> Result<Self> {
        ensure!(
            Uid::effective().is_root(),
            "mantle-worker needs root for /opt/mantle installation inspection"
        );
        Ok(Self {
            root: PathBuf::from("/opt/mantle"),
            candidate: candidate(),
            version_output: format!("codex-cli {VERSION}"),
        })
    }

    fn agent_root(&self) -> PathBuf {
        self.root.join("agents/codex")
    }

    /// Verify the published bytes and coherent manifest; no model call or authentication is implied.
    pub fn inspect(&self) -> Result<Option<InstallationFacts>> {
        trusted_ancestors(&self.root)?;
        let root = self.agent_root();
        if !root.exists() {
            return Ok(None);
        }
        trusted_ancestors(&root)?;
        let current = match fs::read_link(root.join("current")) {
            Ok(path) => path,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => bail!("Codex current pointer is not a symlink"),
        };
        ensure!(
            current.to_str().is_some_and(valid_digest),
            "Codex current generation is not a digest path"
        );
        let generation = root.join(current);
        let facts = self.verify_generation(&generation)?;
        trusted_ancestors(&self.root.join("bin"))?;
        ensure!(
            fs::read_link(self.root.join("bin/codex"))?
                == Path::new("../agents/codex/current/codex"),
            "Codex executable link does not select the coherent current generation"
        );
        Ok(Some(facts))
    }

    fn verify_generation(&self, path: &Path) -> Result<InstallationFacts> {
        trusted_ancestors(path)?;
        let manifest = read_regular(&path.join("manifest.json"), 4096)?;
        let facts: InstallationFacts =
            serde_json::from_slice(&manifest).context("invalid Codex installation manifest")?;
        ensure!(
            valid_digest(&facts.binary_sha256)
                && valid_digest(&facts.compressed_sha256)
                && facts.architecture == ARCHITECTURE
                && !facts.version.is_empty(),
            "invalid Codex installation facts"
        );
        ensure!(
            path.file_name().and_then(|name| name.to_str()) == Some(facts.binary_sha256.as_str()),
            "Codex generation and manifest disagree"
        );
        let mut file = regular_file(&path.join("codex"))?;
        ensure!(
            file.metadata()?.permissions().mode() & 0o111 == 0o111,
            "Codex installed executable is not executable"
        );
        ensure!(
            digest_reader(&mut file, MAX_BINARY, Instant::now() + TRANSACTION_TIMEOUT)?
                == facts.binary_sha256,
            "Codex installed executable checksum mismatch"
        );
        Ok(facts)
    }

    pub fn install(&self) -> Result<InstallationOutcome> {
        self.install_with(std::env::consts::ARCH, |file, deadline| {
            let timeout = deadline.saturating_duration_since(Instant::now());
            let agent: ureq::Agent = ureq::Agent::config_builder()
                .https_only(true)
                .max_redirects(5)
                .timeout_global(Some(timeout))
                .build()
                .into();
            let mut response = agent
                .get(URL)
                .call()
                .map_err(|_| anyhow::anyhow!("pinned Codex download failed"))?;
            copy_bounded(response.body_mut().as_reader(), file, MAX_ARCHIVE, deadline)
                .context("reading bounded Codex archive")?;
            Ok(())
        })
    }

    fn install_with(
        &self,
        architecture: &str,
        fetch: impl FnOnce(&mut File, Instant) -> Result<()>,
    ) -> Result<InstallationOutcome> {
        ensure!(
            architecture == "x86_64",
            "Codex candidate supports x86_64 only; no download attempted"
        );
        let deadline = Instant::now() + TRANSACTION_TIMEOUT;
        create_trusted_dir(&self.root)?;
        create_trusted_dir(&self.root.join("agents"))?;
        create_trusted_dir(&self.agent_root())?;
        create_trusted_dir(&self.root.join("bin"))?;
        let root = self.agent_root();
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_NONBLOCK)
            .open(root.join("install.lock"))?;
        validate_regular(&lock)?;
        ensure!(
            lock.metadata()?.permissions().mode() & 0o077 == 0,
            "Codex installation lock must be private"
        );
        let lock_deadline = Instant::now() + Duration::from_secs(5);
        loop {
            match lock.try_lock() {
                Ok(()) => break,
                Err(std::fs::TryLockError::WouldBlock) if Instant::now() < lock_deadline => {
                    std::thread::sleep(Duration::from_millis(25))
                }
                Err(_) => bail!(
                    "Codex installation lock unavailable; retry after the current installer finishes"
                ),
            }
        }
        if self.inspect()?.as_ref() == Some(&self.candidate) {
            return Ok(installation_decision(
                architecture,
                true,
                false,
                false,
                false,
            ));
        }
        let staging = tempfile::Builder::new()
            .prefix(".staging-")
            .tempdir_in(&root)?;
        fs::set_permissions(staging.path(), fs::Permissions::from_mode(0o700))?;
        let archive_path = staging.path().join("archive.zst");
        let mut archive = OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .mode(0o600)
            .open(&archive_path)?;
        fetch(&mut archive, deadline)?;
        archive.sync_all()?;
        drop(archive);
        let mut archive = regular_file(&archive_path)?;
        ensure!(
            digest_reader(&mut archive, MAX_ARCHIVE, deadline)? == self.candidate.compressed_sha256,
            "Codex archive checksum mismatch; previous installation preserved"
        );
        let mut decoder = zstd::stream::read::Decoder::new(regular_file(&archive_path)?)?;
        decoder.window_log_max(27)?;
        let binary_path = staging.path().join("codex");
        let mut binary = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&binary_path)?;
        copy_bounded(&mut decoder, &mut binary, MAX_BINARY, deadline)?;
        binary.sync_all()?;
        drop(binary);
        ensure!(
            digest_reader(&mut regular_file(&binary_path)?, MAX_BINARY, deadline)?
                == self.candidate.binary_sha256,
            "Codex executable checksum mismatch; previous installation preserved"
        );
        fs::set_permissions(&binary_path, fs::Permissions::from_mode(0o755))?;
        let mut command = Command::new(&binary_path);
        command
            .arg("--version")
            .env_clear()
            .env("HOME", staging.path())
            .env("PATH", "/usr/bin:/bin");
        let version = run_bounded(&mut command, None, Duration::from_secs(5), 4096)?;
        ensure!(
            version.status.success()
                && String::from_utf8_lossy(&version.stdout).trim() == self.version_output,
            "Codex version check failed; previous installation preserved"
        );
        ensure!(
            Instant::now() < deadline,
            "Codex installation deadline exceeded"
        );
        let mut manifest = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o644)
            .open(staging.path().join("manifest.json"))?;
        manifest.write_all(&serde_json::to_vec(&self.candidate)?)?;
        manifest.sync_all()?;
        fs::remove_file(archive_path)?;
        fs::set_permissions(staging.path(), fs::Permissions::from_mode(0o755))?;
        File::open(staging.path())?.sync_all()?;
        let generation = root.join(&self.candidate.binary_sha256);
        if generation.exists() {
            ensure!(
                self.verify_generation(&generation)? == self.candidate,
                "existing generation has different verified facts; refusing replacement"
            );
        } else {
            fs::rename(staging.path(), &generation)?;
        }
        File::open(&root)?.sync_all()?;
        let stable = self.root.join("bin/codex");
        match fs::symlink_metadata(&stable) {
            Ok(_) => ensure!(
                fs::read_link(&stable)? == Path::new("../agents/codex/current/codex"),
                "existing Codex executable is not managed; refusing replacement"
            ),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                symlink("../agents/codex/current/codex", &stable)?
            }
            Err(error) => return Err(error.into()),
        }
        File::open(self.root.join("bin"))?.sync_all()?;
        let activation = tempfile::Builder::new()
            .prefix(".activation-")
            .tempdir_in(&root)?;
        symlink(
            &self.candidate.binary_sha256,
            activation.path().join("current"),
        )?;
        // The only activation operation: both executable and manifest select this generation.
        fs::rename(activation.path().join("current"), root.join("current"))?;
        File::open(&root)?.sync_all()?;
        Ok(installation_decision(architecture, false, true, true, true))
    }
}

fn valid_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn trusted_ancestors(path: &Path) -> Result<()> {
    ensure!(path.is_absolute(), "installation paths must be absolute");
    let mut cursor = PathBuf::new();
    for part in path.components() {
        ensure!(
            matches!(part, Component::RootDir | Component::Normal(_)),
            "unsafe installation path"
        );
        cursor.push(part.as_os_str());
        match fs::symlink_metadata(&cursor) {
            Ok(meta) => {
                ensure!(
                    meta.is_dir() && !meta.file_type().is_symlink(),
                    "installation ancestor must be a real directory"
                );
                ensure!(
                    meta.uid() == 0 || meta.uid() == Uid::effective().as_raw(),
                    "installation ancestor has an untrusted owner"
                );
                let mode = meta.mode();
                ensure!(
                    mode & 0o022 == 0 || (meta.uid() == 0 && mode & 0o1000 != 0),
                    "installation ancestor is writable by other users"
                );
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}
fn create_trusted_dir(path: &Path) -> Result<()> {
    trusted_ancestors(path)?;
    match fs::DirBuilder::new().mode(0o755).create(path) {
        Ok(()) => {
            // Preserve publication permissions even with a restrictive invoking umask. Open the
            // directory we created without following a replacement symlink before setting mode.
            let created = OpenOptions::new()
                .read(true)
                .custom_flags(nix::libc::O_DIRECTORY | nix::libc::O_NOFOLLOW)
                .open(path)?;
            ensure!(
                created.metadata()?.uid() == Uid::effective().as_raw(),
                "created directory ownership changed"
            );
            created.set_permissions(fs::Permissions::from_mode(0o755))?;
        }
        // Another first installer may win before either caller acquires the install lock.
        // Never chmod its object: revalidate its actual kind, owner and permissions below.
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(error) => return Err(error.into()),
    }
    trusted_ancestors(path)
}
fn validate_regular(file: &File) -> Result<()> {
    let meta = file.metadata()?;
    ensure!(
        meta.is_file() && meta.nlink() == 1,
        "installation input must be a regular, singly-linked file"
    );
    ensure!(
        meta.uid() == Uid::effective().as_raw() && meta.mode() & 0o022 == 0,
        "installation input ownership or permissions are unsafe"
    );
    Ok(())
}
fn regular_file(path: &Path) -> Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_NONBLOCK)
        .open(path)?;
    validate_regular(&file)?;
    Ok(file)
}
fn read_regular(path: &Path, cap: u64) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    copy_bounded(
        regular_file(path)?,
        &mut bytes,
        cap,
        Instant::now() + TRANSACTION_TIMEOUT,
    )?;
    Ok(bytes)
}

pub fn read_delivery_binary(path: &Path) -> Result<Vec<u8>> {
    // Cargo hardlinks its top-level executable to deps/. Copy into an immutable bounded buffer;
    // the remote checksum verifies these exact bytes, so source link count is not an invariant.
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_NONBLOCK)
        .open(path)?;
    let metadata = file.metadata()?;
    ensure!(metadata.is_file(), "delivery input must be a regular file");
    ensure!(
        metadata.uid() == Uid::effective().as_raw() && metadata.mode() & 0o022 == 0,
        "delivery input ownership or permissions are unsafe"
    );
    let mut bytes = Vec::new();
    copy_bounded(
        file,
        &mut bytes,
        MAX_HELPER_BYTES as u64,
        Instant::now() + TRANSACTION_TIMEOUT,
    )?;
    Ok(bytes)
}
fn digest_reader(reader: &mut impl Read, cap: u64, deadline: Instant) -> Result<String> {
    let mut hasher = Sha256::new();
    copy_bounded(reader, &mut hasher, cap, deadline)?;
    Ok(format!("{:x}", hasher.finalize()))
}
fn copy_bounded(
    mut reader: impl Read,
    mut writer: impl Write,
    cap: u64,
    deadline: Instant,
) -> Result<u64> {
    let mut bytes = [0; 32768];
    let mut count = 0;
    loop {
        ensure!(Instant::now() < deadline, "installation deadline exceeded");
        let n = reader.read(&mut bytes)?;
        if n == 0 {
            return Ok(count);
        }
        count += n as u64;
        ensure!(count <= cap, "installation input exceeds byte limit");
        writer.write_all(&bytes[..n])?;
    }
}

const CLEANUP_TIMEOUT: Duration = Duration::from_secs(2);

struct TransportChild {
    child: Child,
    retiring: bool,
    retirement_requested: Arc<AtomicBool>,
}

struct TransportRegistry {
    children: Mutex<BTreeMap<i32, TransportChild>>,
    interrupted: Arc<AtomicUsize>,
    default_when_idle: Arc<AtomicBool>,
}

impl Default for TransportRegistry {
    fn default() -> Self {
        Self {
            children: Mutex::default(),
            interrupted: Arc::new(AtomicUsize::new(0)),
            default_when_idle: Arc::new(AtomicBool::new(true)),
        }
    }
}

impl TransportRegistry {
    fn lock_until(
        &self,
        deadline: Instant,
    ) -> Result<MutexGuard<'_, BTreeMap<i32, TransportChild>>> {
        loop {
            match self.children.try_lock() {
                Ok(children) => return Ok(children),
                Err(TryLockError::Poisoned(error)) => return Ok(error.into_inner()),
                Err(TryLockError::WouldBlock) => {
                    ensure!(
                        Instant::now() < deadline,
                        "bounded child registry unavailable"
                    );
                    std::thread::sleep(Duration::from_millis(2));
                }
            }
        }
    }

    fn retire(child: &mut TransportChild) {
        child.retirement_requested.store(true, Ordering::Release);
        if !child.retiring {
            // No observer reaps a live entry before this kill. Its PID cannot be reused here.
            let _ = killpg(Pid::from_raw(child.child.id() as i32), Signal::SIGKILL);
            let _ = child.child.kill();
            child.retiring = true;
        }
    }

    fn reap_retired(&self) {
        if let Ok(mut children) = self.children.try_lock() {
            children.retain(|_, owned| {
                if owned.retirement_requested.load(Ordering::Acquire) {
                    Self::retire(owned);
                }
                !owned.retiring || !matches!(owned.child.try_wait(), Ok(Some(_)))
            });
            self.default_when_idle
                .store(children.is_empty(), Ordering::Release);
        }
    }

    fn terminate(&self, signal: i32) -> ! {
        self.interrupted.store(signal as usize, Ordering::Release);
        let deadline = Instant::now() + CLEANUP_TIMEOUT;
        if let Ok(mut children) = self.lock_until(deadline) {
            for owned in children.values_mut() {
                Self::retire(owned);
            }
        }
        // The coordinator owns the Child handles: cleanup does not depend on caller progress.
        // A kernel-uninterruptible child cannot justify an unbounded registry or signal wait.
        while Instant::now() < deadline {
            self.reap_retired();
            if self
                .children
                .try_lock()
                .is_ok_and(|children| children.is_empty())
            {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let _ = signal_hook::low_level::emulate_default_handler(signal);
        signal_hook::low_level::exit(128 + signal)
    }
}

static TRANSPORTS: OnceLock<std::result::Result<Arc<TransportRegistry>, String>> = OnceLock::new();

/// Initialize before starting application threads. INT, TERM and HUP keep their default
/// termination semantics, after bounded cleanup of every registered transport group.
/// The library also initializes lazily, so an isolated library caller gets the same protection.
pub fn initialize_transport_signals() -> Result<()> {
    transport_registry().map(|_| ())
}

fn transport_registry() -> Result<&'static Arc<TransportRegistry>> {
    TRANSPORTS
        .get_or_init(|| {
            (|| -> Result<Arc<TransportRegistry>> {
                use signal_hook::consts::signal::{SIGHUP, SIGINT, SIGTERM};
                let signals = [SIGINT, SIGTERM, SIGHUP];
                // Registration or thread creation failure must not leave an idle ignored interrupt.
                // Before initialization completes there are no owned children to clean.
                let registry = Arc::new(TransportRegistry::default());
                for signal in signals {
                    signal_hook::flag::register_conditional_default(
                        signal,
                        Arc::clone(&registry.default_when_idle),
                    )?;
                    signal_hook::flag::register_usize(
                        signal,
                        Arc::clone(&registry.interrupted),
                        signal as usize,
                    )?;
                }
                let mut notifications = signal_hook::iterator::Signals::new(signals)?;
                let watched = Arc::clone(&registry);
                std::thread::Builder::new()
                    .name("mantle-child-cleanup".into())
                    .spawn(move || {
                        loop {
                            if let Some(signal) = notifications.pending().next() {
                                watched.terminate(signal);
                            }
                            // Retain and eventually reap an owned child even if a caller's cleanup deadline
                            // expired. Normal calls and signal cleanup never target another process group.
                            watched.reap_retired();
                            std::thread::sleep(Duration::from_millis(10));
                        }
                    })
                    .context("starting bounded-child signal coordinator")?;
                Ok(registry)
            })()
            .map_err(|error| format!("{error:#}"))
        })
        .as_ref()
        .map_err(|error| anyhow::anyhow!("{error}"))
}

struct OwnedTransport {
    registry: Arc<TransportRegistry>,
    pid: Pid,
    finished: bool,
    retirement_requested: Arc<AtomicBool>,
}

impl OwnedTransport {
    fn finish(&mut self) -> Result<()> {
        if self.finished {
            return Ok(());
        }
        self.finished = true;
        // Record retirement without taking the registry lock. Even if a registry wait expires,
        // its monitor retains the Child and must finish this cleanup once it can progress.
        self.retirement_requested.store(true, Ordering::Release);
        let deadline = Instant::now() + CLEANUP_TIMEOUT;
        if let Some(owned) = self
            .registry
            .lock_until(deadline)?
            .get_mut(&self.pid.as_raw())
            .filter(|owned| Arc::ptr_eq(&owned.retirement_requested, &self.retirement_requested))
        {
            TransportRegistry::retire(owned);
        }
        loop {
            self.registry.reap_retired();
            if !self
                .registry
                .lock_until(deadline)?
                .get(&self.pid.as_raw())
                .is_some_and(|owned| {
                    Arc::ptr_eq(&owned.retirement_requested, &self.retirement_requested)
                })
            {
                return Ok(());
            }
            ensure!(
                Instant::now() < deadline,
                "bounded child cleanup deadline exceeded; registry retains ownership"
            );
            std::thread::sleep(Duration::from_millis(2));
        }
    }
}

impl Drop for OwnedTransport {
    fn drop(&mut self) {
        let _ = self.finish();
    }
}

/// Bounded process transport shared by SSH delivery and executable validation.
/// Pipes are nonblocking and pumped together, so a stalled writer cannot prevent draining output.
pub fn run_bounded(
    command: &mut Command,
    input: Option<&[u8]>,
    timeout: Duration,
    output_cap: usize,
) -> Result<Output> {
    ensure!(
        input.is_none_or(|bytes| bytes.len() <= MAX_HELPER_BYTES),
        "process input exceeds 64 MiB"
    );
    command
        .process_group(0)
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let registry = Arc::clone(transport_registry()?);
    // Spawn and registration are serialized against signal cleanup. No child can be created
    // between the cancellation snapshot and termination of the process enforcing its deadline.
    let mut children = registry.lock_until(Instant::now() + CLEANUP_TIMEOUT)?;
    ensure!(
        registry.interrupted.load(Ordering::Acquire) == 0,
        "bounded child cancelled"
    );
    registry.default_when_idle.store(false, Ordering::Release);
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            registry
                .default_when_idle
                .store(children.is_empty(), Ordering::Release);
            return Err(error).context("starting bounded child");
        }
    };
    let pid = Pid::from_raw(child.id() as i32);
    let mut stdin = child.stdin.take();
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let retirement_requested = Arc::new(AtomicBool::new(false));
    children.insert(
        pid.as_raw(),
        TransportChild {
            child,
            retiring: false,
            retirement_requested: Arc::clone(&retirement_requested),
        },
    );
    drop(children);
    let mut owned = OwnedTransport {
        registry,
        pid,
        finished: false,
        retirement_requested,
    };
    let result = (|| {
        let mut stdout = stdout.context("child stdout")?;
        let mut stderr = stderr.context("child stderr")?;
        for fd in [
            &stdout as &dyn std::os::fd::AsFd,
            &stderr as &dyn std::os::fd::AsFd,
        ] {
            fcntl(fd, FcntlArg::F_SETFL(OFlag::O_NONBLOCK))?;
        }
        if let Some(pipe) = &stdin {
            fcntl(pipe, FcntlArg::F_SETFL(OFlag::O_NONBLOCK))?;
        }
        let mut out = Vec::new();
        let mut err = Vec::new();
        let mut out_done = false;
        let mut err_done = false;
        let mut written = 0;
        let deadline = Instant::now() + timeout;
        let mut status = None;
        loop {
            ensure!(
                owned.registry.interrupted.load(Ordering::Acquire) == 0,
                "bounded child cancelled"
            );
            ensure!(Instant::now() < deadline, "bounded child exceeded deadline");
            if let Some(pipe) = &mut stdin {
                let bytes = input.unwrap_or_default();
                match pipe.write(&bytes[written..]) {
                    Ok(n) => written += n,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                    Err(error) => return Err(error.into()),
                }
                if written == bytes.len() {
                    stdin = None;
                }
            }
            for (pipe, buffer, done) in [
                (&mut stdout as &mut dyn Read, &mut out, &mut out_done),
                (&mut stderr as &mut dyn Read, &mut err, &mut err_done),
            ] {
                if !*done {
                    let mut bytes = [0; 8192];
                    match pipe.read(&mut bytes) {
                        Ok(0) => *done = true,
                        Ok(n) => {
                            ensure!(
                                buffer.len() + n <= output_cap,
                                "bounded child exceeded output limit"
                            );
                            buffer.extend_from_slice(&bytes[..n]);
                        }
                        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                        Err(error) => return Err(error.into()),
                    }
                }
            }
            if status.is_none() {
                // WNOWAIT retains the owned PID until process-group cleanup, preventing PID reuse.
                status = match waitid(
                    Id::Pid(pid),
                    WaitPidFlag::WEXITED | WaitPidFlag::WNOHANG | WaitPidFlag::WNOWAIT,
                )? {
                    WaitStatus::Exited(_, code) => {
                        Some(std::process::ExitStatus::from_raw(code << 8))
                    }
                    WaitStatus::Signaled(_, signal, core) => {
                        Some(std::process::ExitStatus::from_raw(
                            signal as i32 | if core { 128 } else { 0 },
                        ))
                    }
                    _ => None,
                };
            }
            ensure!(
                out.len() + err.len() <= output_cap,
                "bounded child exceeded combined output limit"
            );
            if let Some(status) = status.filter(|_| out_done && err_done) {
                ensure!(
                    written == input.map_or(0, <[u8]>::len),
                    "bounded child did not consume input"
                );
                return Ok(Output {
                    status,
                    stdout: out,
                    stderr: err,
                });
            }
            std::thread::sleep(Duration::from_millis(2));
        }
    })();
    let cleanup = owned.finish();
    let signal = owned.registry.interrupted.load(Ordering::Acquire);
    if signal != 0 {
        owned.registry.terminate(signal as i32);
    }
    result.and_then(|output| {
        cleanup?;
        Ok(output)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adversary_second_successful_child_exit_cleans_its_remaining_group() {
        const LEAF_FILE: &str = "MANTLE_ADVERSARY_SUCCESS_LEAF";
        if let Some(path) = std::env::var_os(LEAF_FILE) {
            let leaf = Command::new("/usr/bin/sleep")
                .arg("30")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap();
            fs::write(path, leaf.id().to_string()).unwrap();
            // Deliberate fixture parent exit: its child keeps the owned process group alive.
            std::process::exit(0);
        }
        let dir = tempfile::tempdir().unwrap();
        let leaf_file = dir.path().join("leaf.pid");
        let result = run_bounded(
            Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "tests::adversary_second_successful_child_exit_cleans_its_remaining_group",
                ])
                .env(LEAF_FILE, &leaf_file),
            None,
            Duration::from_secs(3),
            4096,
        );
        let leaf = fs::read_to_string(&leaf_file)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let deadline = Instant::now() + Duration::from_millis(500);
        while fixture_running(leaf) && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(2));
        }
        let remained = fixture_running(leaf);
        if remained {
            let _ = nix::sys::signal::kill(Pid::from_raw(leaf), Signal::SIGKILL);
        }
        assert!(result.unwrap().status.success());
        assert!(
            !remained,
            "successful direct-child exit leaked its owned descendant"
        );
    }

    #[test]
    fn adversary_second_failed_spawn_preserves_following_transport_and_idle_state() {
        if isolated_installer_test() {
            return;
        }
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("absent-executable");
        let failed = run_bounded(
            &mut Command::new(missing),
            None,
            Duration::from_secs(1),
            128,
        );
        assert!(
            failed
                .unwrap_err()
                .to_string()
                .contains("starting bounded child")
        );
        let output = run_bounded(
            Command::new("/usr/bin/printf").arg("transport recovered"),
            None,
            Duration::from_secs(1),
            128,
        )
        .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, b"transport recovered");
        let registry = transport_registry().unwrap();
        assert!(registry.children.lock().unwrap().is_empty());
        assert!(registry.default_when_idle.load(Ordering::Acquire));
        assert_eq!(registry.interrupted.load(Ordering::Acquire), 0);
    }

    #[test]
    fn adversary_second_update_observers_only_see_complete_verified_generations() {
        if isolated_installer_test() {
            return;
        }
        let (_dir, original, archive) = fixture();
        install_fixture(&original, &archive).unwrap();
        let binary = fs::read("/usr/bin/printf").unwrap();
        let archive = zstd::stream::encode_all(&binary[..], 1).unwrap();
        let version = Command::new("/usr/bin/printf")
            .arg("--version")
            .env_clear()
            .output()
            .unwrap();
        assert!(version.status.success());
        let replacement = Installer {
            root: original.root.clone(),
            candidate: InstallationFacts {
                version: "replacement".into(),
                architecture: ARCHITECTURE.into(),
                compressed_sha256: format!("{:x}", Sha256::digest(&archive)),
                binary_sha256: format!("{:x}", Sha256::digest(&binary)),
            },
            version_output: String::from_utf8(version.stdout).unwrap().trim().to_owned(),
        };
        assert_ne!(
            original.candidate.binary_sha256,
            replacement.candidate.binary_sha256
        );
        let barrier = std::sync::Barrier::new(2);
        let finished = AtomicBool::new(false);
        std::thread::scope(|scope| {
            let observer = scope.spawn(|| {
                let first = original.inspect().unwrap().unwrap();
                assert_eq!(first, original.candidate);
                barrier.wait();
                while !finished.load(Ordering::Acquire) {
                    let observed = original.inspect().unwrap().unwrap();
                    assert!(
                        observed == original.candidate || observed == replacement.candidate,
                        "inspection mixed different generation facts"
                    );
                }
            });
            barrier.wait();
            let installed = install_fixture(&replacement, &archive);
            finished.store(true, Ordering::Release);
            observer.join().unwrap();
            assert_eq!(installed.unwrap(), InstallationOutcome::V1);
        });
        assert_eq!(
            original.inspect().unwrap(),
            Some(replacement.candidate.clone())
        );
        assert!(
            original
                .agent_root()
                .join(&original.candidate.binary_sha256)
                .join("codex")
                .exists()
        );
        assert_eq!(fs::read(original.root.join("bin/codex")).unwrap(), binary);
    }

    // Production runs one installer per helper process. Keep unrelated signal/fork fixtures
    // from inheriting another test's writable executable while preserving concurrency inside
    // the explicit two-installer case. Every original assertion executes in the child.
    fn isolated_installer_test() -> bool {
        const MARKER: &str = "MANTLE_ISOLATED_INSTALLER_TEST";
        let thread = std::thread::current();
        let name = thread.name().expect("named installer test");
        if std::env::var(MARKER).as_deref() == Ok(name) {
            return false;
        }
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args(["--exact", name, "--nocapture"])
            .env(MARKER, name);
        let output = run_bounded(&mut command, None, Duration::from_secs(20), 256 * 1024)
            .expect("bounded isolated installer test");
        assert!(
            output.status.success(),
            "isolated {name} exited {}\n{}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        true
    }

    #[test]
    fn stale_transport_guard_cannot_retire_a_reused_pid_registration() {
        let registry = Arc::new(TransportRegistry::default());
        let child = Command::new("/usr/bin/sleep")
            .arg("30")
            .process_group(0)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let pid = Pid::from_raw(child.id() as i32);
        let current_registration = Arc::new(AtomicBool::new(false));
        registry.children.lock().unwrap().insert(
            pid.as_raw(),
            TransportChild {
                child,
                retiring: false,
                retirement_requested: current_registration,
            },
        );
        // Model PID reuse after the previous Child was reaped: the old guard holds a different
        // registration identity even though the operating system gave a new child the same PID.
        let mut stale = OwnedTransport {
            registry: Arc::clone(&registry),
            pid,
            finished: false,
            retirement_requested: Arc::new(AtomicBool::new(false)),
        };
        let cleanup = stale.finish();
        let survived = registry
            .children
            .lock()
            .unwrap()
            .get_mut(&pid.as_raw())
            .is_some_and(|owned| !owned.retiring && owned.child.try_wait().unwrap().is_none());
        if let Some(mut remaining) = registry.children.lock().unwrap().remove(&pid.as_raw()) {
            TransportRegistry::retire(&mut remaining);
            let _ = remaining.child.wait();
        }
        assert!(cleanup.is_ok());
        assert!(
            survived,
            "stale cleanup targeted a different registration that reused its PID"
        );
    }

    #[test]
    fn retirement_survives_a_bounded_registry_wait_timeout() {
        let registry = Arc::new(TransportRegistry::default());
        let child = Command::new("/usr/bin/sleep")
            .arg("30")
            .process_group(0)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let pid = Pid::from_raw(child.id() as i32);
        let requested = Arc::new(AtomicBool::new(false));
        registry.children.lock().unwrap().insert(
            pid.as_raw(),
            TransportChild {
                child,
                retiring: false,
                retirement_requested: Arc::clone(&requested),
            },
        );
        let mut owned = OwnedTransport {
            registry: Arc::clone(&registry),
            pid,
            finished: false,
            retirement_requested: Arc::clone(&requested),
        };
        let held = registry.children.lock().unwrap();
        let started = Instant::now();
        let refused = owned.finish().is_err();
        let elapsed = started.elapsed();
        let retirement_retained = requested.load(Ordering::Acquire);
        drop(held);
        let deadline = Instant::now() + CLEANUP_TIMEOUT;
        while Instant::now() < deadline && !registry.children.lock().unwrap().is_empty() {
            registry.reap_retired();
            std::thread::sleep(Duration::from_millis(2));
        }
        let reaped = registry.children.lock().unwrap().is_empty();
        if !reaped
            && let Some(mut remaining) = registry.children.lock().unwrap().remove(&pid.as_raw())
        {
            TransportRegistry::retire(&mut remaining);
            let _ = remaining.child.wait();
        }
        assert!(
            refused && elapsed < CLEANUP_TIMEOUT + Duration::from_secs(1),
            "registry wait must be bounded even while its owner cannot progress"
        );
        assert!(
            retirement_retained && reaped,
            "deadline expiry must retain cleanup ownership"
        );
    }

    #[test]
    fn termination_coordinator_cleans_concurrent_groups_and_preserves_signal_status() {
        const MODE: &str = "MANTLE_TEST_TERMINATION_MODE";
        if std::env::var_os(MODE).is_some() {
            initialize_transport_signals().unwrap();
            std::thread::scope(|scope| {
                for _ in 0..2 {
                    scope.spawn(|| {
                        let _ = run_bounded(
                            Command::new("/usr/bin/sleep").arg("30"),
                            None,
                            Duration::from_secs(40),
                            4096,
                        );
                    });
                }
            });
            panic!("termination signal was swallowed");
        }
        for signal in [Signal::SIGINT, Signal::SIGTERM, Signal::SIGHUP] {
            let mut driver = Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "tests::termination_coordinator_cleans_concurrent_groups_and_preserves_signal_status"])
                .env(MODE, "concurrent").process_group(0).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap();
            let mut unrelated = Command::new("/usr/bin/sleep")
                .arg("30")
                .process_group(0)
                .spawn()
                .unwrap();
            let deadline = Instant::now() + Duration::from_secs(3);
            let children = loop {
                let children = fixture_children(driver.id());
                if children.len() == 2 {
                    break children;
                }
                if Instant::now() >= deadline {
                    let _ = killpg(Pid::from_raw(driver.id() as i32), Signal::SIGKILL);
                    let _ = driver.wait();
                    for pid in children {
                        let _ = killpg(Pid::from_raw(pid), Signal::SIGKILL);
                    }
                    let _ = unrelated.kill();
                    let _ = unrelated.wait();
                    panic!("two bounded children were not observed");
                }
                std::thread::sleep(Duration::from_millis(2));
            };
            killpg(Pid::from_raw(driver.id() as i32), signal).unwrap();
            let status = wait_fixture(&mut driver, &children);
            let reaped = children
                .iter()
                .all(|pid| !Path::new(&format!("/proc/{pid}")).exists());
            let unrelated_untouched = unrelated.try_wait().unwrap().is_none();
            let _ = unrelated.kill();
            let _ = unrelated.wait();
            for pid in &children {
                if Path::new(&format!("/proc/{pid}")).exists() {
                    let _ = killpg(Pid::from_raw(*pid), Signal::SIGKILL);
                }
            }
            assert_eq!(
                status.signal(),
                Some(signal as i32),
                "termination must retain default signal status"
            );
            assert!(
                reaped,
                "every direct child must be reaped before the caller terminates"
            );
            assert!(
                unrelated_untouched,
                "cleanup reached an unrelated process group"
            );
        }
    }

    #[test]
    fn idle_transport_coordinator_does_not_swallow_termination() {
        const READY: &str = "MANTLE_TEST_IDLE_SIGNAL_READY";
        if let Some(path) = std::env::var_os(READY) {
            assert!(
                run_bounded(
                    &mut Command::new("/usr/bin/true"),
                    None,
                    Duration::from_secs(2),
                    4096
                )
                .unwrap()
                .status
                .success()
            );
            fs::write(path, b"ready").unwrap();
            loop {
                std::thread::park();
            }
        }
        for signal in [Signal::SIGINT, Signal::SIGTERM, Signal::SIGHUP] {
            let dir = tempfile::tempdir().unwrap();
            let ready = dir.path().join("ready");
            let mut driver = Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "tests::idle_transport_coordinator_does_not_swallow_termination",
                ])
                .env(READY, &ready)
                .process_group(0)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap();
            let deadline = Instant::now() + Duration::from_secs(3);
            while !ready.exists() && Instant::now() < deadline {
                std::thread::sleep(Duration::from_millis(2));
            }
            let was_ready = ready.exists();
            let _ = killpg(Pid::from_raw(driver.id() as i32), signal);
            let status = wait_fixture(&mut driver, &[]);
            assert!(was_ready, "idle fixture never reached its ready point");
            assert_eq!(status.signal(), Some(signal as i32));
        }
    }

    #[test]
    fn termination_cleanup_includes_owned_descendants() {
        const MODE: &str = "MANTLE_TEST_DESCENDANT_MODE";
        const PID_FILE: &str = "MANTLE_TEST_DESCENDANT_PID";
        match std::env::var(MODE).as_deref() {
            Ok("leaf-parent") => {
                let mut leaf = Command::new("/usr/bin/sleep").arg("30").spawn().unwrap();
                fs::write(std::env::var_os(PID_FILE).unwrap(), leaf.id().to_string()).unwrap();
                let _ = leaf.wait();
                return;
            }
            Ok("driver") => {
                let _ = run_bounded(
                    Command::new(std::env::current_exe().unwrap())
                        .args([
                            "--exact",
                            "tests::termination_cleanup_includes_owned_descendants",
                        ])
                        .env(MODE, "leaf-parent"),
                    None,
                    Duration::from_secs(40),
                    4096,
                );
                panic!("termination was swallowed");
            }
            _ => {}
        }
        let dir = tempfile::tempdir().unwrap();
        let leaf_file = dir.path().join("leaf.pid");
        let mut driver = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "tests::termination_cleanup_includes_owned_descendants",
            ])
            .env(MODE, "driver")
            .env(PID_FILE, &leaf_file)
            .process_group(0)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(3);
        while !leaf_file.exists() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(2));
        }
        let children = fixture_children(driver.id());
        let leaf = fs::read_to_string(&leaf_file)
            .ok()
            .and_then(|pid| pid.parse::<i32>().ok());
        let _ = killpg(Pid::from_raw(driver.id() as i32), Signal::SIGTERM);
        let status = wait_fixture(&mut driver, &children);
        let leaf_running = leaf.is_some_and(fixture_running);
        for pid in &children {
            if fixture_running(*pid) {
                let _ = killpg(Pid::from_raw(*pid), Signal::SIGKILL);
            }
        }
        if let Some(pid) = leaf.filter(|_| leaf_running) {
            let _ = nix::sys::signal::kill(Pid::from_raw(pid), Signal::SIGKILL);
        }
        assert!(leaf.is_some(), "descendant fixture never started");
        assert_eq!(status.signal(), Some(Signal::SIGTERM as i32));
        assert!(
            !leaf_running,
            "owned process-group descendant survived cleanup"
        );
        assert!(
            children
                .iter()
                .all(|pid| !Path::new(&format!("/proc/{pid}")).exists()),
            "direct child was not reaped"
        );
    }

    fn fixture_children(pid: u32) -> Vec<i32> {
        fs::read_dir(format!("/proc/{pid}/task"))
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|entry| fs::read_to_string(entry.path().join("children")).ok())
            .flat_map(|text| {
                text.split_whitespace()
                    .filter_map(|pid| pid.parse().ok())
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn fixture_running(pid: i32) -> bool {
        fs::read_to_string(format!("/proc/{pid}/stat"))
            .ok()
            .is_some_and(|stat| {
                stat.split_once(") ")
                    .is_some_and(|(_, rest)| !rest.starts_with('Z'))
            })
    }

    fn wait_fixture(driver: &mut Child, children: &[i32]) -> std::process::ExitStatus {
        let deadline = Instant::now() + Duration::from_secs(4);
        loop {
            if let Some(status) = driver.try_wait().unwrap() {
                return status;
            }
            if Instant::now() >= deadline {
                let _ = killpg(Pid::from_raw(driver.id() as i32), Signal::SIGKILL);
                let _ = driver.wait();
                for pid in children {
                    if fixture_running(*pid) {
                        let _ = killpg(Pid::from_raw(*pid), Signal::SIGKILL);
                    }
                }
                panic!("signal coordinator exceeded its bounded cleanup deadline");
            }
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    #[test]
    fn concurrent_directory_winners_are_revalidated_without_permission_changes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("winner");
        let barrier = std::sync::Barrier::new(8);
        std::thread::scope(|scope| {
            let barrier = &barrier;
            for _ in 0..8 {
                let path = &path;
                scope.spawn(move || {
                    barrier.wait();
                    create_trusted_dir(path).unwrap();
                });
            }
        });
        fs::set_permissions(&path, fs::Permissions::from_mode(0o777)).unwrap();
        assert!(create_trusted_dir(&path).is_err());
        assert_eq!(fs::metadata(&path).unwrap().mode() & 0o777, 0o777);
        fs::remove_dir(&path).unwrap();
        symlink(dir.path(), &path).unwrap();
        assert!(create_trusted_dir(&path).is_err());
        fs::remove_file(&path).unwrap();
        fs::write(&path, b"not a directory").unwrap();
        assert!(create_trusted_dir(&path).is_err());
        assert_eq!(fs::read(path).unwrap(), b"not a directory");
    }

    #[test]
    fn adversary_concurrent_installation_fetches_once_and_publishes_one_generation() {
        if isolated_installer_test() {
            return;
        }
        let (_dir, installer, archive) = fixture();
        let calls = std::sync::atomic::AtomicUsize::new(0);
        let barrier = std::sync::Barrier::new(2);
        let outcomes = std::thread::scope(|scope| {
            let install = || {
                barrier.wait();
                installer
                    .install_with("x86_64", |file, _| {
                        calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        std::thread::sleep(Duration::from_millis(75));
                        file.write_all(&archive)?;
                        Ok(())
                    })
                    .unwrap()
            };
            let first = scope.spawn(install);
            let second = scope.spawn(install);
            [first.join().unwrap(), second.join().unwrap()]
        });
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert!(outcomes.contains(&InstallationOutcome::V0));
        assert!(outcomes.contains(&InstallationOutcome::V1));
        assert_eq!(
            installer.inspect().unwrap(),
            Some(installer.candidate.clone())
        );
        assert_eq!(
            fs::read_link(installer.agent_root().join("current")).unwrap(),
            PathBuf::from(&installer.candidate.binary_sha256)
        );
    }

    #[test]
    fn adversary_live_lock_refuses_without_fetching_or_changing_current() {
        if isolated_installer_test() {
            return;
        }
        let (_dir, installer, archive) = fixture();
        install_fixture(&installer, &archive).unwrap();
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .open(installer.agent_root().join("install.lock"))
            .unwrap();
        lock.lock().unwrap();
        let before = installer.inspect().unwrap();
        let start = Instant::now();
        let error = installer
            .install_with("x86_64", |_, _| panic!("held lock must preclude download"))
            .unwrap_err();
        assert!(error.to_string().contains("lock unavailable"));
        assert!(start.elapsed() < Duration::from_secs(7));
        assert_eq!(installer.inspect().unwrap(), before);
    }

    #[test]
    fn adversary_fifo_and_symlink_inputs_refuse_without_waiting_for_writers() {
        if isolated_installer_test() {
            return;
        }
        let (dir, installer, archive) = fixture();
        install_fixture(&installer, &archive).unwrap();
        let fifo = dir.path().join("fifo");
        nix::unistd::mkfifo(
            &fifo,
            nix::sys::stat::Mode::S_IRUSR | nix::sys::stat::Mode::S_IWUSR,
        )
        .unwrap();
        let start = Instant::now();
        assert!(read_delivery_binary(&fifo).is_err());
        let generation = installer
            .agent_root()
            .join(&installer.candidate.binary_sha256);
        fs::rename(
            generation.join("manifest.json"),
            generation.join("manifest.saved"),
        )
        .unwrap();
        symlink(&fifo, generation.join("manifest.json")).unwrap();
        assert!(installer.inspect().is_err());
        fs::remove_file(generation.join("manifest.json")).unwrap();
        nix::unistd::mkfifo(
            &generation.join("manifest.json"),
            nix::sys::stat::Mode::S_IRUSR | nix::sys::stat::Mode::S_IWUSR,
        )
        .unwrap();
        assert!(installer.inspect().is_err());
        assert!(start.elapsed() < Duration::from_secs(2));
    }

    #[test]
    fn adversary_unpublished_verified_generation_recovers_after_interrupted_activation() {
        if isolated_installer_test() {
            return;
        }
        let (_dir, installer, archive) = fixture();
        install_fixture(&installer, &archive).unwrap();
        fs::remove_file(installer.agent_root().join("current")).unwrap();
        assert!(installer.inspect().unwrap().is_none());
        assert_eq!(
            install_fixture(&installer, &archive).unwrap(),
            InstallationOutcome::V1
        );
        assert_eq!(
            installer.inspect().unwrap(),
            Some(installer.candidate.clone())
        );
    }

    #[test]
    fn adversary_unmanaged_executable_is_preserved_on_activation_refusal() {
        if isolated_installer_test() {
            return;
        }
        let (_dir, installer, archive) = fixture();
        create_trusted_dir(&installer.root).unwrap();
        create_trusted_dir(&installer.root.join("bin")).unwrap();
        let unrelated = installer.root.join("bin/codex");
        fs::write(&unrelated, b"operator-owned executable").unwrap();
        assert!(install_fixture(&installer, &archive).is_err());
        assert_eq!(fs::read(&unrelated).unwrap(), b"operator-owned executable");
        assert!(!installer.agent_root().join("current").exists());
        assert!(installer.inspect().unwrap().is_none());
    }

    #[test]
    fn adversary_refuses_binary_mismatch_after_valid_archive_before_execution() {
        if isolated_installer_test() {
            return;
        }
        let (_dir, mut installer, archive) = fixture();
        installer.candidate.binary_sha256 = "0".repeat(64);
        let error = install_fixture(&installer, &archive).unwrap_err();
        assert!(error.to_string().contains("executable checksum mismatch"));
        assert!(installer.inspect().unwrap().is_none());
        assert!(!installer.root.join("bin/codex").exists());
    }

    #[test]
    fn adversary_operator_interrupt_reaps_bounded_child() {
        const MODE: &str = "MANTLE_ADVERSARY_CANCEL_DRIVER";
        if std::env::var_os(MODE).is_some() {
            let _ = run_bounded(
                Command::new("/usr/bin/sleep").arg("30"),
                None,
                Duration::from_secs(40),
                4096,
            );
            return;
        }
        let mut driver = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "tests::adversary_operator_interrupt_reaps_bounded_child",
                "--nocapture",
            ])
            .env(MODE, "1")
            .process_group(0)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let driver_pid = Pid::from_raw(driver.id() as i32);
        let start = Instant::now();
        let child_pid = loop {
            let children = fs::read_dir(format!("/proc/{}/task", driver.id()))
                .unwrap()
                .filter_map(Result::ok)
                .filter_map(|entry| fs::read_to_string(entry.path().join("children")).ok())
                .flat_map(|text| {
                    text.split_whitespace()
                        .filter_map(|pid| pid.parse::<i32>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            if let Some(pid) = children.first() {
                break Pid::from_raw(*pid);
            }
            if start.elapsed() > Duration::from_secs(3) {
                let _ = killpg(driver_pid, Signal::SIGKILL);
                let _ = driver.wait();
                panic!("bounded child was not observed");
            }
            std::thread::sleep(Duration::from_millis(5));
        };
        // A terminal interrupt reaches the foreground CLI group; the bounded child has its own.
        killpg(driver_pid, Signal::SIGINT).unwrap();
        let _ = driver.wait().unwrap();
        std::thread::sleep(Duration::from_millis(100));
        let remaining = fs::read_to_string(format!("/proc/{child_pid}/stat"))
            .ok()
            .is_some_and(|stat| {
                stat.split_once(") ")
                    .is_some_and(|(_, rest)| !rest.starts_with('Z'))
            });
        // Always clean up the exact fixture group before asserting, even when the attack succeeds.
        let _ = killpg(child_pid, Signal::SIGKILL);
        assert!(
            !remaining,
            "operator SIGINT left the owned bounded child running after its parent exited"
        );
    }

    fn fixture() -> (tempfile::TempDir, Installer, Vec<u8>) {
        let dir = tempfile::tempdir().unwrap();
        let binary = fs::read("/usr/bin/true").unwrap();
        let archive = zstd::stream::encode_all(&binary[..], 1).unwrap();
        let observed = Command::new("/usr/bin/true")
            .arg("--version")
            .env_clear()
            .output()
            .unwrap();
        let installer = Installer {
            root: dir.path().join("mantle"),
            candidate: InstallationFacts {
                version: "fixture".into(),
                architecture: ARCHITECTURE.into(),
                compressed_sha256: format!("{:x}", Sha256::digest(&archive)),
                binary_sha256: format!("{:x}", Sha256::digest(&binary)),
            },
            version_output: String::from_utf8(observed.stdout)
                .unwrap()
                .trim()
                .to_owned(),
        };
        (dir, installer, archive)
    }
    fn install_fixture(installer: &Installer, archive: &[u8]) -> Result<InstallationOutcome> {
        installer.install_with("x86_64", |file, _| {
            file.write_all(archive)?;
            Ok(())
        })
    }
    #[test]
    fn installation_is_verified_atomic_and_idempotent() {
        if isolated_installer_test() {
            return;
        }
        let (_dir, installer, archive) = fixture();
        assert!(installer.inspect().unwrap().is_none());
        assert_eq!(
            install_fixture(&installer, &archive).unwrap(),
            InstallationOutcome::V1
        );
        assert_eq!(
            installer.inspect().unwrap(),
            Some(installer.candidate.clone())
        );
        assert_eq!(
            installer
                .install_with("x86_64", |_, _| panic!("already-current must not fetch"))
                .unwrap(),
            InstallationOutcome::V0
        );
    }
    #[test]
    fn corrupt_archive_never_activates() {
        if isolated_installer_test() {
            return;
        }
        let (_dir, installer, _) = fixture();
        assert!(install_fixture(&installer, b"not a zstd archive").is_err());
        assert!(installer.inspect().unwrap().is_none());
        assert!(!installer.agent_root().join("current").exists());
    }
    #[test]
    fn failed_update_preserves_previous_binary_and_facts() {
        if isolated_installer_test() {
            return;
        }
        let (_dir, mut installer, archive) = fixture();
        install_fixture(&installer, &archive).unwrap();
        let before = installer.inspect().unwrap();
        let pointer = fs::read_link(installer.agent_root().join("current")).unwrap();
        installer.candidate.compressed_sha256 = "0".repeat(64);
        assert!(
            installer
                .install_with("x86_64", |file, _| {
                    file.write_all(b"partial")?;
                    bail!("interrupted fetch")
                })
                .is_err()
        );
        assert_eq!(installer.inspect().unwrap(), before);
        assert_eq!(
            fs::read_link(installer.agent_root().join("current")).unwrap(),
            pointer
        );
        assert!(install_fixture(&installer, &archive).is_err());
        assert_eq!(installer.inspect().unwrap(), before);
    }
    #[test]
    fn unsupported_architecture_never_fetches_or_creates_paths() {
        if isolated_installer_test() {
            return;
        }
        let (_dir, installer, _) = fixture();
        assert!(
            installer
                .install_with("aarch64", |_, _| panic!("must not fetch"))
                .is_err()
        );
        assert!(!installer.root.exists());
    }
    #[test]
    fn tampered_installed_bytes_are_refused() {
        if isolated_installer_test() {
            return;
        }
        let (_dir, installer, archive) = fixture();
        install_fixture(&installer, &archive).unwrap();
        fs::write(
            installer
                .agent_root()
                .join(&installer.candidate.binary_sha256)
                .join("codex"),
            b"tampered",
        )
        .unwrap();
        assert!(installer.inspect().is_err());
        assert!(
            installer
                .install_with("x86_64", |_, _| panic!("must not fetch"))
                .is_err()
        );
    }
    #[test]
    fn version_mismatch_never_activates() {
        if isolated_installer_test() {
            return;
        }
        let (_dir, mut installer, archive) = fixture();
        installer.version_output = "wrong version".into();
        assert!(install_fixture(&installer, &archive).is_err());
        assert!(installer.inspect().unwrap().is_none());
    }
    #[test]
    fn bounded_child_refuses_output_flood_and_timeout() {
        assert!(
            run_bounded(
                &mut Command::new("/usr/bin/yes"),
                None,
                Duration::from_secs(2),
                4096
            )
            .is_err()
        );
        let start = Instant::now();
        assert!(
            run_bounded(
                Command::new("/usr/bin/sleep").arg("10"),
                None,
                Duration::from_millis(100),
                4096
            )
            .is_err()
        );
        assert!(start.elapsed() < Duration::from_secs(2));
    }
    #[test]
    fn bounded_child_pumps_input_and_output_together() {
        let input = vec![b'x'; 128 * 1024];
        let output = run_bounded(
            &mut Command::new("/usr/bin/cat"),
            Some(&input),
            Duration::from_secs(5),
            input.len(),
        )
        .unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, input);
    }
    #[test]
    fn symlinked_root_is_refused() {
        if isolated_installer_test() {
            return;
        }
        let (dir, mut installer, archive) = fixture();
        symlink(dir.path(), dir.path().join("unsafe")).unwrap();
        installer.root = dir.path().join("unsafe/mantle");
        assert!(install_fixture(&installer, &archive).is_err());
    }
    #[test]
    fn byte_limit_detects_growth_beyond_declared_size() {
        assert!(
            copy_bounded(
                &b"abcd"[..],
                Vec::new(),
                3,
                Instant::now() + Duration::from_secs(1)
            )
            .is_err()
        );
    }

    #[test]
    fn delivery_accepts_cargo_hardlinked_artifacts() {
        let dir = tempfile::tempdir().unwrap();
        let artifact = dir.path().join("mantle-worker");
        fs::write(&artifact, b"compiled artifact bytes").unwrap();
        fs::hard_link(&artifact, dir.path().join("cargo-deps-artifact")).unwrap();
        assert_eq!(
            read_delivery_binary(&artifact).unwrap(),
            b"compiled artifact bytes"
        );
    }
    #[test]
    fn generated_model_has_no_drift() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let temporary = tempfile::tempdir().unwrap();
        let output = Command::new("ess")
            .current_dir(&root)
            .args([
                "generate",
                "types",
                "--path",
                "spec",
                "--root",
                "mantle.session.AgentInstallationFacts",
                "--root",
                "mantle.session.AgentInstallationOutcome",
                "--target",
                "rust",
                "--package",
                "mantle-worker-model",
                "--out",
            ])
            .arg(temporary.path().join("model"))
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        // ESS operational state contains output-path/inode/random-anchor metadata, not model source.
        for path in [
            "Cargo.toml",
            "types.rs",
            "source.schema.json",
            "types-report.json",
        ] {
            assert_eq!(
                fs::read(root.join("generated/worker-model").join(path)).unwrap(),
                fs::read(temporary.path().join("model").join(path)).unwrap(),
                "generated drift: {path}"
            );
        }
    }
}
