//! Read-only, bounded offline-maintenance observations shared by local and SSH callers.
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    os::unix::fs::{MetadataExt, OpenOptionsExt},
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

pub const UNITS: [&str; 2] = ["substrate.service", "mantle-egress.service"];
pub const UNIT_PATHS: [&str; 11] = [
    "/etc/systemd/system.control",
    "/run/systemd/system.control",
    "/run/systemd/transient",
    "/run/systemd/generator.early",
    "/etc/systemd/system",
    "/etc/systemd/system.attached",
    "/run/systemd/system",
    "/run/systemd/system.attached",
    "/run/systemd/generator",
    "/usr/local/lib/systemd/system",
    "/usr/lib/systemd/system",
];
pub const ORIGINALS: &str = "/var/lib/mantle/maintenance/units";
pub const SUBSTRATE_VERSION: &str = "0.7.10";
pub const SUBSTRATE_REVISION: &str = "65304edf6ebdf4a95f9c2c6138b0c20ea47d157e";
pub const EGRESS_SERVICE: &str = include_str!("../../../deploy/mantle-egress.service");
pub fn substrate_service(claude: bool) -> String {
    include_str!("../../../deploy/substrate.service")
        .replace(
            "{{CLAUDE_CONDITION}}",
            if claude {
                "ConditionFileNotEmpty=/var/lib/mantle/secrets/claude"
            } else {
                ""
            },
        )
        .replace(
            "{{CLAUDE_SLOT}}\n",
            if claude {
                "    --secret-slot claude=/var/lib/mantle/secrets/claude \\\n"
            } else {
                ""
            },
        )
}

#[derive(Clone, Debug)]
pub struct Metadata {
    pub kind: String,
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
    pub links: u64,
    pub size: u64,
}
pub trait Host {
    fn command(&self, argv: &[&str], deadline: Instant) -> Result<Vec<u8>>;
    fn read(&self, path: &str, cap: usize, deadline: Instant) -> Result<Vec<u8>>;
    fn metadata(&self, path: &str, deadline: Instant) -> Result<Metadata>;
    fn entries(&self, path: &str, deadline: Instant) -> Result<Vec<String>>;
    fn link(&self, path: &str, deadline: Instant) -> Result<String>;
    fn owner(&self) -> u32 {
        0
    }
}
pub fn remaining(deadline: Instant) -> Result<Duration> {
    let left = deadline.saturating_duration_since(Instant::now());
    ensure!(!left.is_zero(), "assessment deadline exceeded");
    Ok(left.min(Duration::from_secs(15)))
}
/// `root` is an explicit dependency-injection seam; shipped commands always use `/`.
pub struct LocalHost {
    pub root: PathBuf,
}
impl LocalHost {
    fn path(&self, path: &str) -> PathBuf {
        self.root.join(path.trim_start_matches('/'))
    }
}
impl Host for LocalHost {
    fn command(&self, argv: &[&str], deadline: Instant) -> Result<Vec<u8>> {
        let out = crate::run_bounded(
            Command::new(argv[0]).args(&argv[1..]).env("LC_ALL", "C"),
            None,
            remaining(deadline)?,
            64 * 1024,
        )?;
        ensure!(out.status.success(), "host observation command failed");
        Ok(out.stdout)
    }
    fn read(&self, path: &str, cap: usize, deadline: Instant) -> Result<Vec<u8>> {
        remaining(deadline)?;
        let file = fs::OpenOptions::new()
            .read(true)
            .custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_NONBLOCK)
            .open(self.path(path))?;
        ensure!(
            file.metadata()?.is_file(),
            "observation requires a regular file"
        );
        let mut out = Vec::new();
        file.take(cap as u64 + 1).read_to_end(&mut out)?;
        ensure!(out.len() <= cap, "observation exceeds bound");
        remaining(deadline)?;
        Ok(out)
    }
    fn metadata(&self, path: &str, deadline: Instant) -> Result<Metadata> {
        remaining(deadline)?;
        let m = fs::symlink_metadata(self.path(path))?;
        Ok(Metadata {
            kind: if m.is_file() {
                "file"
            } else if m.is_dir() {
                "directory"
            } else if m.file_type().is_symlink() {
                "link"
            } else {
                "other"
            }
            .into(),
            uid: m.uid(),
            gid: m.gid(),
            mode: m.mode() & 0o7777,
            links: m.nlink(),
            size: m.len(),
        })
    }
    fn entries(&self, path: &str, deadline: Instant) -> Result<Vec<String>> {
        remaining(deadline)?;
        let mut names = Vec::new();
        for entry in fs::read_dir(self.path(path))? {
            ensure!(names.len() < 4096, "directory observation exceeds bound");
            names.push(
                entry?
                    .file_name()
                    .into_string()
                    .map_err(|_| anyhow::anyhow!("non-UTF8 layout"))?,
            );
        }
        remaining(deadline)?;
        Ok(names)
    }
    fn link(&self, path: &str, deadline: Instant) -> Result<String> {
        remaining(deadline)?;
        fs::read_link(self.path(path))?
            .into_os_string()
            .into_string()
            .map_err(|_| anyhow::anyhow!("non-UTF8 link"))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnitObservation {
    pub name: String,
    pub state: String,
    pub mask: String,
    pub job: String,
    pub pids: String,
    pub cgroup: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Assessment {
    pub applicable: bool,
    pub boot_id: String,
    pub observed_at: String,
    pub units: Vec<UnitObservation>,
    pub jobs: String,
    pub layout: String,
    pub compatibility: String,
    pub outcome: String,
    pub diagnostic: String,
    pub candidate_source: String,
    pub candidate_manifest: String,
    pub installed_versions: Vec<String>,
}
impl Default for Assessment {
    fn default() -> Self {
        Self {
            applicable: false,
            boot_id: String::new(),
            observed_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string(),
            units: Vec::new(),
            jobs: "unknown".into(),
            layout: "unknown".into(),
            compatibility: "unknown".into(),
            outcome: "unknown".into(),
            diagnostic: "Required observations are incomplete.".into(),
            candidate_source: String::new(),
            candidate_manifest: String::new(),
            installed_versions: Vec::new(),
        }
    }
}
fn text(bytes: Vec<u8>) -> Result<String> {
    Ok(String::from_utf8(bytes)?.trim_end_matches('\n').to_owned())
}
fn properties(bytes: Vec<u8>, required: &[&str]) -> Result<BTreeMap<String, String>> {
    let mut map = BTreeMap::new();
    for line in String::from_utf8(bytes)?.lines() {
        let (key, value) = line.split_once('=').context("malformed property")?;
        ensure!(
            required.contains(&key) && map.insert(key.to_owned(), value.to_owned()).is_none(),
            "unexpected or duplicate property"
        );
    }
    ensure!(map.len() == required.len(), "missing property");
    Ok(map)
}
/// Absence is inferred only from a successful bounded listing of each existing parent.
fn exists(host: &impl Host, path: &str, deadline: Instant) -> Result<bool> {
    let mut parent = String::from("/");
    for component in Path::new(path).components() {
        let std::path::Component::Normal(name) = component else {
            continue;
        };
        let name = name.to_str().context("path encoding")?;
        if !host
            .entries(&parent, deadline)?
            .iter()
            .any(|entry| entry == name)
        {
            return Ok(false);
        }
        if parent != "/" {
            parent.push('/');
        }
        parent.push_str(name);
    }
    Ok(true)
}
fn directory(host: &impl Host, path: &str, mode: Option<u32>, deadline: Instant) -> Result<()> {
    let m = host.metadata(path, deadline)?;
    ensure!(
        m.kind == "directory"
            && m.uid == host.owner()
            && m.mode & 0o022 == 0
            && mode.is_none_or(|v| m.mode == v),
        "unsupported directory layout"
    );
    Ok(())
}
fn layout(host: &impl Host, deadline: Instant) -> Result<()> {
    for parent in [
        "/var",
        "/var/lib",
        "/var/lib/mantle",
        "/var/lib/mantle/maintenance",
    ] {
        directory(host, parent, None, deadline)?;
    }
    directory(host, ORIGINALS, Some(0o700), deadline)?;
    for unit in UNITS {
        let path = format!("{ORIGINALS}/{unit}");
        let m = host.metadata(&path, deadline)?;
        ensure!(
            m.kind == "file"
                && m.uid == host.owner()
                && m.gid == host.owner()
                && m.mode == 0o644
                && m.links == 1,
            "unsupported preserved original"
        );
        let bytes = host.read(&path, 16384, deadline)?;
        ensure!(
            if unit == UNITS[0] {
                bytes == substrate_service(false).as_bytes()
                    || bytes == substrate_service(true).as_bytes()
            } else {
                bytes == EGRESS_SERVICE.as_bytes()
            },
            "unsupported original unit bytes"
        );
    }
    for base in UNIT_PATHS
        .into_iter()
        .chain(["/run/systemd/generator.late"])
    {
        if !exists(host, base, deadline)? {
            continue;
        }
        directory(host, base, None, deadline)?;
        let entries = host.entries(base, deadline)?;
        for dropin in [
            "service.d",
            "mantle-.service.d",
            "substrate.service.d",
            "mantle-egress.service.d",
        ] {
            if entries.iter().any(|name| name == dropin) {
                ensure!(
                    host.entries(&format!("{base}/{dropin}"), deadline)?
                        .is_empty(),
                    "unsupported unit drop-in"
                );
            }
        }
        for unit in UNITS {
            if entries.iter().any(|name| name == unit) {
                let path = format!("{base}/{unit}");
                let m = host.metadata(&path, deadline)?;
                ensure!(
                    (base == "/etc/systemd/system" || base == "/run/systemd/system")
                        && m.kind == "link"
                        && m.uid == host.owner()
                        && host.link(&path, deadline)? == "/dev/null",
                    "unit file overrides effective mask"
                );
            }
        }
    }
    Ok(())
}
fn observe(host: &impl Host, report: &mut Assessment, deadline: Instant) -> Result<()> {
    let boot = text(host.read("/proc/sys/kernel/random/boot_id", 128, deadline)?)?;
    ensure!(
        boot.len() == 36 && boot.bytes().all(|b| b.is_ascii_hexdigit() || b == b'-'),
        "invalid boot identity"
    );
    report.boot_id = boot.clone();
    layout(host, deadline)?;
    report.layout = "supported".into();
    let mounts = text(host.read("/proc/self/mountinfo", 65536, deadline)?)?;
    ensure!(
        mounts
            .lines()
            .filter(|l| l.split_whitespace().nth(4) == Some("/sys/fs/cgroup"))
            .count()
            == 1
            && mounts.lines().any(|line| {
                let fields: Vec<_> = line.split_whitespace().collect();
                fields.get(3) == Some(&"/")
                    && fields.get(4) == Some(&"/sys/fs/cgroup")
                    && line.contains(" - cgroup2 ")
            }),
        "unsupported cgroup mount"
    );
    directory(host, "/sys/fs/cgroup/system.slice", None, deadline)?;
    let parents = host.entries("/sys/fs/cgroup/system.slice", deadline)?;
    let keys = [
        "Id",
        "Names",
        "LoadState",
        "ActiveState",
        "SubState",
        "UnitFileState",
        "FragmentPath",
        "NeedDaemonReload",
        "Job",
        "MainPID",
        "ControlPID",
        "ControlGroup",
        "Transient",
    ];
    let property = format!("--property={}", keys.join(","));
    let mut safe = true;
    for unit in UNITS {
        let p = properties(
            host.command(
                &[
                    "systemctl",
                    "--system",
                    "--no-pager",
                    "--no-ask-password",
                    "show",
                    "--all",
                    &property,
                    unit,
                ],
                deadline,
            )?,
            &keys,
        )?;
        let cg = format!("/system.slice/{unit}");
        ensure!(
            p["Id"] == unit
                && p["Names"] == unit
                && matches!(p["LoadState"].as_str(), "masked" | "loaded")
                && p["Transient"] == "no"
                && (p["ControlGroup"].is_empty() || p["ControlGroup"] == cg),
            "unsupported or absent unit identity"
        );
        ensure!(
            p["MainPID"].parse::<u32>().is_ok() && p["ControlPID"].parse::<u32>().is_ok(),
            "malformed unit PID"
        );
        let fragment = &p["FragmentPath"];
        let etc_mask = format!("/etc/systemd/system/{unit}");
        let run_mask = format!("/run/systemd/system/{unit}");
        let effective_source = if exists(host, &etc_mask, deadline)? {
            &etc_mask
        } else {
            &run_mask
        };
        let mask = p["LoadState"] == "masked"
            && matches!(p["UnitFileState"].as_str(), "masked" | "masked-runtime")
            && fragment == effective_source
            && host.metadata(fragment, deadline)?.kind == "link"
            && host.metadata(fragment, deadline)?.uid == host.owner()
            && host.link(fragment, deadline)? == "/dev/null"
            && p["NeedDaemonReload"] == "no";
        let quiet = p["ActiveState"] == "inactive"
            && p["SubState"] == "dead"
            && p["MainPID"] == "0"
            && p["ControlPID"] == "0"
            && p["Job"].is_empty();
        let cgroup = if parents.iter().any(|name| name == unit) {
            let path = format!("/sys/fs/cgroup/system.slice/{unit}");
            directory(host, &path, None, deadline)?;
            let events = text(host.read(&format!("{path}/cgroup.events"), 4096, deadline)?)?;
            let mut seen = BTreeSet::new();
            let mut populated = None;
            for line in events.lines() {
                let (key, value) = line.split_once(' ').context("malformed cgroup event")?;
                ensure!(
                    seen.insert(key) && matches!(value, "0" | "1"),
                    "malformed cgroup event"
                );
                if key == "populated" {
                    populated = Some(value);
                }
            }
            match populated.context("missing populated observation")? {
                "0" => "empty",
                _ => "populated",
            }
        } else {
            "absent"
        };
        safe &= mask && quiet && cgroup != "populated";
        report.units.push(UnitObservation {
            name: unit.into(),
            state: p["ActiveState"].clone(),
            mask: if mask { "effective" } else { "ineffective" }.into(),
            job: if p["Job"].is_empty() {
                "none"
            } else {
                "pending"
            }
            .into(),
            pids: if p["MainPID"] == "0" && p["ControlPID"] == "0" {
                "none"
            } else {
                "present"
            }
            .into(),
            cgroup: cgroup.into(),
        });
    }
    let jobs = text(host.command(
        &[
            "systemctl",
            "--system",
            "--no-ask-password",
            "list-jobs",
            "--all",
            "--full",
            "--no-legend",
            "--no-pager",
            "--plain",
        ],
        deadline,
    )?)?;
    let mut pending = false;
    for line in jobs.lines().filter(|l| !l.is_empty()) {
        let fields: Vec<_> = line.split_whitespace().collect();
        ensure!(
            fields.len() == 4 && fields[0].parse::<u64>().is_ok(),
            "malformed jobs observation"
        );
        pending |= UNITS.contains(&fields[1]);
    }
    report.jobs = if pending { "pending" } else { "none" }.into();
    safe &= !pending;
    let version = text(host.command(&["/usr/local/bin/substrate-daemon", "--version"], deadline)?)?;
    report.compatibility = if version == format!("substrate-daemon {SUBSTRATE_VERSION}")
        || version == format!("substrate {SUBSTRATE_VERSION}")
    {
        "matching"
    } else {
        "incompatible"
    }
    .into();
    ensure!(
        text(host.read("/proc/sys/kernel/random/boot_id", 128, deadline)?)? == boot,
        "boot changed during observation"
    );
    report.applicable = safe && report.compatibility == "matching";
    report.outcome = if report.compatibility != "matching" {
        "incompatible"
    } else if safe {
        "eligible"
    } else {
        "refused"
    }
    .into();
    report.diagnostic=if report.applicable {"Offline maintenance verified; applying requires a recheck under the host lock."}else{"Both units must be effectively masked, inactive and job-free with empty delegated cgroups; Substrate must match the pin."}.into();
    Ok(())
}
pub fn assess(host: &impl Host, deadline: Instant) -> Assessment {
    let mut report = Assessment::default();
    if observe(host, &mut report, deadline).is_err() {
        report.applicable = false;
        report.outcome = "unknown".into();
        report.diagnostic =
            "A required bounded observation failed or the unit/cgroup layout is unsupported."
                .into();
    }
    report
}
