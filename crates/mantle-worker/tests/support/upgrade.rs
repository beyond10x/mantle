use anyhow::{Result, ensure};
use mantle_worker::maintenance::{self, Host, LocalHost, Metadata};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt, symlink},
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
    time::{Duration, Instant},
};

pub struct Fixture {
    pub temp: tempfile::TempDir,
    pub root: PathBuf,
    pub manifest: PathBuf,
    pub host: FixtureHost,
}
pub struct FixtureHost {
    pub fs: LocalHost,
    pub fault: RefCell<String>,
    pub calls: RefCell<Vec<String>>,
}
impl Host for FixtureHost {
    fn owner(&self) -> u32 {
        fs::metadata(&self.fs.root).unwrap().uid()
    }
    fn read(&self, path: &str, cap: usize, d: Instant) -> Result<Vec<u8>> {
        self.fs.read(path, cap, d)
    }
    fn metadata(&self, path: &str, d: Instant) -> Result<Metadata> {
        self.fs.metadata(path, d)
    }
    fn entries(&self, path: &str, d: Instant) -> Result<Vec<String>> {
        self.fs.entries(path, d)
    }
    fn link(&self, path: &str, d: Instant) -> Result<String> {
        self.fs.link(path, d)
    }
    fn command(&self, args: &[&str], _: Instant) -> Result<Vec<u8>> {
        self.calls.borrow_mut().push(args.join(" "));
        let fault = self.fault.borrow();
        ensure!(*fault != "unknown", "controlled observation failure");
        if args[0] == "/usr/local/bin/substrate-daemon" {
            return Ok(format!(
                "substrate-daemon {}\n",
                if *fault == "incompatible" {
                    "0.0.0"
                } else {
                    maintenance::SUBSTRATE_VERSION
                }
            )
            .into_bytes());
        }
        if args.contains(&"list-jobs") {
            return Ok(if *fault == "job" {
                b"42 substrate.service start waiting\n".to_vec()
            } else {
                Vec::new()
            });
        }
        ensure!(
            args.contains(&"show"),
            "fixture refuses mutating service command"
        );
        if args.contains(&"--property=ActiveState") {
            return Ok(if *fault == "active" {
                b"ActiveState=active\n".to_vec()
            } else {
                b"ActiveState=inactive\n".to_vec()
            });
        }
        let unit = args.last().unwrap();
        if *fault == "real-fragment" {
            return Ok(format!("Id={unit}\nNames={unit}\nLoadState=masked\nActiveState=inactive\nSubState=dead\nUnitFileState=masked\nFragmentPath=/etc/systemd/system/{unit}\nNeedDaemonReload=no\nJob=\nMainPID=0\nControlPID=0\nControlGroup=\nTransient=no\n").into_bytes());
        }
        Ok(format!("Id={unit}\nNames={unit}\nLoadState={}\nActiveState={}\nSubState={}\nUnitFileState=masked\nFragmentPath=/etc/systemd/system/{unit}\nNeedDaemonReload=no\nJob={}\nMainPID={}\nControlPID=0\nControlGroup=\nTransient=no\n",if *fault=="not-found" {"not-found"}else{"masked"},if *fault=="active" {"active"}else{"inactive"},if *fault=="active" {"running"}else{"dead"},if *fault=="unit-job" {"42"}else{""},if *fault=="active" {"12"}else{"0"}).into_bytes())
    }
}
fn binary() -> &'static Vec<u8> {
    static BINARY: OnceLock<Vec<u8>> = OnceLock::new();
    BINARY.get_or_init(|| {
        let temp=tempfile::tempdir().unwrap(); let source=temp.path().join("binary.rs");let out=temp.path().join("binary");
        fs::write(&source, "fn main() { let p=std::env::current_exe().unwrap(); println!(\"{} 0.1.4\",p.file_name().unwrap().to_string_lossy()); }\n").unwrap();
        let result=mantle_worker::run_bounded(Command::new("rustc").arg(source).args(["--target",mantle_artifact::MUSL,"-o"]).arg(&out),None,Duration::from_secs(30),1024*1024).unwrap();
        assert!(result.status.success(),"{}",String::from_utf8_lossy(&result.stderr));fs::read(out).unwrap()
    })
}
impl Fixture {
    pub fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let hostroot = temp.path().join("host");
        for path in [
            "etc/systemd/system",
            "run/systemd/system",
            "var/lib/mantle/maintenance/units",
            "sys/fs/cgroup/system.slice/substrate.service",
            "sys/fs/cgroup/system.slice/mantle-egress.service",
            "proc/sys/kernel/random",
            "proc/self",
            "opt/mantle/bin",
            "opt/mantle/agents/codex/fixture",
        ] {
            fs::create_dir_all(hostroot.join(path)).unwrap();
        }
        fs::set_permissions(
            hostroot.join("var/lib/mantle/maintenance/units"),
            fs::Permissions::from_mode(0o700),
        )
        .unwrap();
        for unit in maintenance::UNITS {
            let bytes = if unit == maintenance::UNITS[0] {
                maintenance::substrate_service(true)
            } else {
                maintenance::EGRESS_SERVICE.into()
            };
            fs::write(
                hostroot.join("var/lib/mantle/maintenance/units").join(unit),
                bytes,
            )
            .unwrap();
            symlink("/dev/null", hostroot.join("etc/systemd/system").join(unit)).unwrap();
            fs::write(
                hostroot
                    .join("sys/fs/cgroup/system.slice")
                    .join(unit)
                    .join("cgroup.events"),
                b"populated 0\nfrozen 0\n",
            )
            .unwrap();
        }
        fs::write(
            hostroot.join("proc/sys/kernel/random/boot_id"),
            b"11111111-1111-1111-1111-111111111111\n",
        )
        .unwrap();
        fs::write(
            hostroot.join("proc/self/mountinfo"),
            b"29 23 0:26 / /sys/fs/cgroup rw - cgroup2 cgroup rw\n",
        )
        .unwrap();
        let root = hostroot.join("opt/mantle");
        for name in ["mantle-worker", "mantle-egress", "mantle-launch", "claude"] {
            fs::write(root.join("bin").join(name), format!("previous {name}")).unwrap();
            fs::set_permissions(
                root.join("bin").join(name),
                fs::Permissions::from_mode(0o755),
            )
            .unwrap();
        }
        symlink("../agents/codex/current/codex", root.join("bin/codex")).unwrap();
        symlink("fixture", root.join("agents/codex/current")).unwrap();
        fs::write(root.join("agents/codex/fixture/codex"), b"preserved agent").unwrap();
        let bundle = temp.path().join("bundle");
        fs::create_dir(&bundle).unwrap();
        let mut manifest = mantle_artifact::Manifest {
            format: "mantle-release/1".into(),
            version: "0.1.4".into(),
            source_commit: "1111111111111111111111111111111111111111".into(),
            substrate_revision: maintenance::SUBSTRATE_REVISION.into(),
            substrate_version: maintenance::SUBSTRATE_VERSION.into(),
            rustc: "rustc fixture".into(),
            gnu_runtime: "glibc 2.44".into(),
            musl_runtime: "1.2.5".into(),
            artifacts: Vec::new(),
        };
        for target in [mantle_artifact::GNU, mantle_artifact::MUSL] {
            let mut tar = tar::Builder::new(Vec::new());
            let mut payloads = Vec::new();
            for (path, mode) in mantle_artifact::inventory(target).unwrap() {
                let bytes = if path.starts_with("bin/") {
                    binary().clone()
                } else {
                    b"fixture notice\n".to_vec()
                };
                let mut h = tar::Header::new_ustar();
                h.set_size(bytes.len() as u64);
                h.set_mode(mode);
                h.set_uid(0);
                h.set_gid(0);
                h.set_mtime(0);
                h.set_cksum();
                tar.append_data(&mut h, &path, bytes.as_slice()).unwrap();
                payloads.push(mantle_artifact::Payload {
                    path,
                    mode,
                    size_bytes: bytes.len() as u64,
                    sha256: mantle_artifact::sha256(&bytes),
                });
            }
            let bytes = tar.into_inner().unwrap();
            let name = format!("mantle-0.1.4-{target}.tar");
            fs::write(bundle.join(&name), &bytes).unwrap();
            manifest.artifacts.push(mantle_artifact::Artifact {
                name,
                target: target.into(),
                sha256: mantle_artifact::sha256(&bytes),
                size_bytes: bytes.len() as u64,
                payloads,
            });
        }
        let manifest_path = bundle.join("manifest.json");
        fs::write(&manifest_path, serde_json::to_vec(&manifest).unwrap()).unwrap();
        Self {
            temp,
            root,
            manifest: manifest_path,
            host: FixtureHost {
                fs: LocalHost { root: hostroot },
                fault: RefCell::new(String::new()),
                calls: RefCell::new(Vec::new()),
            },
        }
    }
    pub fn snapshot(&self) -> BTreeMap<String, Vec<u8>> {
        fs::read_dir(self.root.join("bin"))
            .unwrap()
            .map(|e| {
                let e = e.unwrap();
                let p = e.path();
                (
                    e.file_name().to_string_lossy().into_owned(),
                    if e.file_type().unwrap().is_symlink() {
                        fs::read_link(&p)
                            .unwrap()
                            .as_os_str()
                            .as_encoded_bytes()
                            .to_vec()
                    } else {
                        fs::read(&p).unwrap()
                    },
                )
            })
            .collect()
    }
    pub fn path(&self, path: &str) -> PathBuf {
        self.host
            .fs
            .root
            .join(Path::new(path).strip_prefix("/").unwrap_or(Path::new(path)))
    }
}
