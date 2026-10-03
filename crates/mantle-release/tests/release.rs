use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::OnceLock,
};
use tar::{Builder, Header};
use tempfile::TempDir;
const GNU: &str = "x86_64-unknown-linux-gnu";
const MUSL: &str = "x86_64-unknown-linux-musl";
fn binaries() -> &'static (TempDir, PathBuf, PathBuf) {
    static BINARIES: OnceLock<(TempDir, PathBuf, PathBuf)> = OnceLock::new();
    BINARIES.get_or_init(|| {
        let temp = tempfile::tempdir().unwrap();
        let gnu = temp.path().join("gnu");
        let musl = temp.path().join("musl");
        for (target, out) in [(GNU, &gnu), (MUSL, &musl)] {
            let status = Command::new("rustc")
                .args(["--edition=2024", "--target", target])
                .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/support/program.rs"))
                .arg("-o")
                .arg(out)
                .status()
                .unwrap();
            assert!(status.success());
        }
        (temp, gnu, musl)
    })
}
fn sha(bytes: &[u8]) -> String {
    // Independent system checksum oracle; production uses Rust SHA256.
    use std::io::Write;
    let mut child = Command::new("sha256sum")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(bytes).unwrap();
    String::from_utf8(child.wait_with_output().unwrap().stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .into()
}
struct Fixture {
    dir: TempDir,
    manifest: Value,
}
impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let mut artifacts = Vec::new();
        for (target, binary, names) in [
            (
                GNU,
                &binaries().1,
                vec!["mantle", "mantle-release", "mantle-acceptance"],
            ),
            (
                MUSL,
                &binaries().2,
                vec!["mantle-egress", "mantle-launch", "mantle-worker"],
            ),
        ] {
            let mut files = std::collections::BTreeMap::new();
            for name in names {
                files.insert(format!("bin/{name}"), (fs::read(binary).unwrap(), 0o755));
            }
            for name in [
                "LICENSE",
                "THIRD_PARTY_LICENSES.html",
                "RUST_RUNTIME_LICENSE.html",
            ] {
                files.insert(name.into(), (b"fixture notice\n".to_vec(), 0o644));
            }
            if target == MUSL {
                files.insert(
                    "MUSL_COPYRIGHT".into(),
                    (b"fixture musl notice\n".to_vec(), 0o644),
                );
            }
            let mut builder = Builder::new(Vec::new());
            let mut payloads = Vec::new();
            for (path, (bytes, mode)) in files {
                let mut header = Header::new_ustar();
                header.set_size(bytes.len() as u64);
                header.set_mode(mode);
                header.set_uid(0);
                header.set_gid(0);
                header.set_mtime(0);
                header.set_cksum();
                builder
                    .append_data(&mut header, &path, bytes.as_slice())
                    .unwrap();
                payloads.push(
                    json!({"path":path,"sha256":sha(&bytes),"size_bytes":bytes.len(),"mode":mode}),
                );
            }
            let bytes = builder.into_inner().unwrap();
            let name = format!("mantle-0.1.4-{target}.tar");
            fs::write(dir.path().join(&name), &bytes).unwrap();
            artifacts.push(json!({"name":name,"target":target,"sha256":sha(&bytes),"size_bytes":bytes.len(),"payloads":payloads}));
        }
        let manifest = json!({"format":"mantle-release/1","version":"0.1.4","source_commit":"1111111111111111111111111111111111111111","substrate_revision":"65304edf6ebdf4a95f9c2c6138b0c20ea47d157e","substrate_version":"0.7.10","rustc":"rustc 1.97.1 fixture","gnu_runtime":"glibc 2.42","musl_runtime":"1.2.5","artifacts":artifacts});
        let result = Self { dir, manifest };
        result.save();
        result
    }
    fn save(&self) {
        fs::write(
            self.dir.path().join("manifest.json"),
            serde_json::to_vec_pretty(&self.manifest).unwrap(),
        )
        .unwrap();
    }
    fn cli(&self, command: &str, extra: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mantle-release"))
            .arg(command)
            .arg("--manifest")
            .arg(self.dir.path().join("manifest.json"))
            .args(extra)
            .output()
            .unwrap()
    }
}
#[test]
fn valid_packages_are_verified_through_cli() {
    let fixture = Fixture::new();
    let out = fixture.cli("verify", &[]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stdout).contains("verified"));
}
#[test]
fn strict_manifest_and_checksum_refusals_are_not_cli_parse_errors() {
    let mut fixture = Fixture::new();
    fixture.manifest["unexpected"] = json!(true);
    fixture.save();
    let out = fixture.cli("verify", &[]);
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("unknown field"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    fixture
        .manifest
        .as_object_mut()
        .unwrap()
        .remove("unexpected");
    fixture.save();
    fs::write(
        fixture.dir.path().join(format!("mantle-0.1.4-{GNU}.tar")),
        b"tampered",
    )
    .unwrap();
    let out = fixture.cli("verify", &[]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("archive"));
}
#[test]
fn same_length_archive_padding_corruption_is_refused() {
    let fixture = Fixture::new();
    let artifact = &fixture.manifest["artifacts"][0];
    let path = fixture.dir.path().join(artifact["name"].as_str().unwrap());
    let mut bytes = fs::read(&path).unwrap();
    let length = bytes.len();
    let digest = sha(&bytes);
    let padding = {
        let mut archive = tar::Archive::new(bytes.as_slice());
        let entry = archive.entries().unwrap().next().unwrap().unwrap();
        assert_ne!(entry.size() % 512, 0, "fixture needs TAR padding");
        usize::try_from(entry.raw_file_position() + entry.size()).unwrap()
    };
    // Padding belongs to the archive digest, but not to any extracted payload digest.
    bytes[padding] ^= 1;
    assert_eq!(bytes.len(), length);
    assert_ne!(sha(&bytes), digest);
    fs::write(path, bytes).unwrap();
    let out = fixture.cli("verify", &[]);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("archive checksum or size mismatch"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn install_is_managed_and_atomic_and_refuses_unmanaged_collision() {
    let fixture = Fixture::new();
    let prefix = tempfile::tempdir().unwrap();
    fs::create_dir(prefix.path().join("bin")).unwrap();
    fs::write(prefix.path().join("bin/mantle"), b"unmanaged").unwrap();
    let path = prefix.path().to_str().unwrap();
    let out = fixture.cli("install", &["--prefix", path, "--target", GNU]);
    assert!(!out.status.success());
    assert_eq!(
        fs::read(prefix.path().join("bin/mantle")).unwrap(),
        b"unmanaged"
    );
    fs::remove_file(prefix.path().join("bin/mantle")).unwrap();
    let out = fixture.cli("install", &["--prefix", path, "--target", GNU]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        fs::symlink_metadata(prefix.path().join("bin/mantle"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(
        fs::read_link(prefix.path().join("bin/mantle-acceptance")).unwrap(),
        std::path::PathBuf::from("../.mantle/current/bin/mantle-acceptance")
    );
    assert!(
        Command::new(prefix.path().join("bin/mantle-acceptance"))
            .arg("--version")
            .output()
            .unwrap()
            .status
            .success()
    );
    let out = Command::new(prefix.path().join("bin/mantle"))
        .arg("--version")
        .output()
        .unwrap();
    assert_eq!(out.stdout, b"mantle 0.1.4\n");
}

#[test]
fn source_identity_refuses_dirty_or_wrong_revision_even_same_version() {
    let source = tempfile::tempdir().unwrap();
    let git = |args: &[&str]| {
        Command::new("git")
            .arg("-C")
            .arg(source.path())
            .args(args)
            .output()
            .unwrap()
    };
    assert!(git(&["init", "--quiet"]).status.success());
    fs::write(
        source.path().join("Cargo.toml"),
        "[workspace.package]\nversion = \"0.1.4\"\n",
    )
    .unwrap();
    assert!(git(&["add", "Cargo.toml"]).status.success());
    assert!(
        git(&[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--quiet",
            "-m",
            "fixture"
        ])
        .status
        .success()
    );
    let head = String::from_utf8(git(&["rev-parse", "HEAD"]).stdout).unwrap();
    assert_eq!(
        mantle_release::build::source_identity(source.path(), head.trim()).unwrap(),
        head.trim()
    );
    assert!(
        mantle_release::build::source_identity(
            source.path(),
            "1111111111111111111111111111111111111111"
        )
        .unwrap_err()
        .to_string()
        .contains("differs")
    );
    fs::write(
        source.path().join("Cargo.toml"),
        "[workspace.package]\nversion = \"0.1.4\"\n# same version, different bytes\n",
    )
    .unwrap();
    assert!(
        mantle_release::build::source_identity(source.path(), head.trim())
            .unwrap_err()
            .to_string()
            .contains("dirty")
    );
}
#[test]
fn unsafe_archive_members_and_nonstatic_workers_are_refused() {
    let mut fixture = Fixture::new();
    for alteration in [
        "duplicate",
        "link",
        "traversal",
        "extra",
        "oversize",
        "nonstatic",
        "trailer",
        "missing",
    ] {
        let index = 1;
        let original = Fixture::new();
        fixture.manifest = original.manifest.clone();
        let name = fixture.manifest["artifacts"][index]["name"]
            .as_str()
            .unwrap()
            .to_owned();
        let data = fs::read(original.dir.path().join(&name)).unwrap();
        let mut builder = Builder::new(Vec::new());
        let mut first = true;
        for entry in tar::Archive::new(data.as_slice()).entries().unwrap() {
            use std::io::Read;
            let mut entry = entry.unwrap();
            let path = entry.path().unwrap().into_owned();
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes).unwrap();
            let mut header = entry.header().clone();
            if first && alteration == "missing" {
                first = false;
                continue;
            }
            if first && alteration == "nonstatic" && path.starts_with("bin") {
                unreachable!();
            }
            if alteration == "nonstatic" && path == Path::new("bin/mantle-worker") {
                bytes = fs::read(&binaries().1).unwrap();
                header.set_size(bytes.len() as u64);
                header.set_cksum();
                for payload in fixture.manifest["artifacts"][index]["payloads"]
                    .as_array_mut()
                    .unwrap()
                {
                    if payload["path"] == "bin/mantle-worker" {
                        payload["sha256"] = json!(sha(&bytes));
                        payload["size_bytes"] = json!(bytes.len());
                    }
                }
            }
            builder
                .append_data(&mut header, &path, bytes.as_slice())
                .unwrap();
            if first {
                match alteration {
                    "duplicate" => builder
                        .append_data(&mut header, &path, bytes.as_slice())
                        .unwrap(),
                    "link" => {
                        let mut link = Header::new_ustar();
                        link.set_size(0);
                        link.set_mode(0o777);
                        link.set_entry_type(tar::EntryType::Symlink);
                        link.set_link_name("/etc/passwd").unwrap();
                        link.set_cksum();
                        builder
                            .append_data(&mut link, "unsafe-link", &[][..])
                            .unwrap();
                    }
                    "extra" => {
                        builder
                            .append_data(&mut header, "unexpected", bytes.as_slice())
                            .unwrap();
                    }
                    "traversal" => {
                        let mut raw = header.clone();
                        raw.as_mut_bytes()[..100].fill(0);
                        raw.as_mut_bytes()[..10].copy_from_slice(b"../escaped");
                        raw.set_cksum();
                        builder.append(&raw, bytes.as_slice()).unwrap();
                    }
                    _ => {}
                }
            }
            first = false;
        }
        let mut bytes = builder.into_inner().unwrap();
        if alteration == "trailer" {
            bytes.extend_from_slice(b"unexpected trailer");
        }
        fs::write(fixture.dir.path().join(&name), &bytes).unwrap();
        fixture.manifest["artifacts"][index]["sha256"] = json!(sha(&bytes));
        fixture.manifest["artifacts"][index]["size_bytes"] = json!(bytes.len());
        if alteration == "oversize" {
            fixture.manifest["artifacts"][index]["payloads"][0]["size_bytes"] =
                json!(mantle_artifact::MAX_PAYLOAD + 1);
        }
        fixture.save();
        let out = fixture.cli("verify", &[]);
        assert!(!out.status.success(), "accepted {alteration}");
        assert!(!String::from_utf8_lossy(&out.stderr).contains("unexpected argument"));
    }
}
#[test]
fn bot_publication_refuses_remote_tag_mismatch_existing_and_unknown_release_and_reports_upload_failure()
 {
    let mut fixture = Fixture::new();
    let source = tempfile::tempdir().unwrap();
    let git = |args: &[&str]| {
        Command::new("git")
            .arg("-C")
            .arg(source.path())
            .args(args)
            .output()
            .unwrap()
    };
    assert!(git(&["init", "--quiet"]).status.success());
    fs::write(source.path().join("source"), b"same version fixture").unwrap();
    git(&["add", "source"]);
    assert!(
        git(&[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--quiet",
            "-m",
            "fixture"
        ])
        .status
        .success()
    );
    let head = String::from_utf8(git(&["rev-parse", "HEAD"]).stdout).unwrap();
    assert!(git(&["tag", "0.1.4"]).status.success());
    fixture.manifest["source_commit"] = json!(head.trim());
    fixture.save();
    let sums = fixture.manifest["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| {
            format!(
                "{}  {}\n",
                a["sha256"].as_str().unwrap(),
                a["name"].as_str().unwrap()
            )
        })
        .collect::<String>()
        + &format!(
            "{}  manifest.json\n",
            sha(&fs::read(fixture.dir.path().join("manifest.json")).unwrap())
        );
    fs::write(fixture.dir.path().join("SHA256SUMS"), sums).unwrap();
    let bin = tempfile::tempdir().unwrap();
    assert!(
        Command::new("rustc")
            .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/support/bot.rs"))
            .arg("-o")
            .arg(bin.path().join("b10x-gates"))
            .status()
            .unwrap()
            .success()
    );
    for mode in [
        "wrong-remote",
        "existing",
        "network",
        "create-failure",
        "success",
        "annotated",
        "bad-digest",
        "changed-after-create",
    ] {
        let trace = bin.path().join(format!("{mode}.trace"));
        let out = Command::new(env!("CARGO_BIN_EXE_mantle-release"))
            .args(["publish", "--manifest"])
            .arg(fixture.dir.path().join("manifest.json"))
            .arg("--source")
            .arg(source.path())
            .args(["--tag", "0.1.4", "--policy", "/unused/fixture-policy"])
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    bin.path().display(),
                    std::env::var("PATH").unwrap()
                ),
            )
            .env("FIXTURE_MODE", mode)
            .env("FIXTURE_TRACE", &trace)
            .env("FIXTURE_HEAD", head.trim())
            .output()
            .unwrap();
        assert_eq!(
            out.status.success(),
            matches!(mode, "success" | "annotated"),
            "{mode}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let trace = fs::read_to_string(trace).unwrap();
        assert_eq!(
            trace
                .lines()
                .filter(|line| line.starts_with("release create"))
                .count(),
            usize::from(!matches!(mode, "wrong-remote" | "existing" | "network")),
            "{mode}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        if mode == "wrong-remote" {
            assert!(String::from_utf8_lossy(&out.stderr).contains("remote tag differs"));
        }
        if mode == "create-failure" {
            assert!(String::from_utf8_lossy(&out.stderr).contains("partial release"));
        }
    }
}
#[test]
fn concurrent_installers_and_interrupted_activation_preserve_a_complete_generation() {
    let fixture = Fixture::new();
    let prefix = tempfile::tempdir().unwrap();
    let prefix_str = prefix.path().to_str().unwrap();
    assert!(
        fixture
            .cli("install", &["--prefix", prefix_str])
            .status
            .success()
    );
    let before = fs::read_link(prefix.path().join(".mantle/current")).unwrap();
    // A prior process left its private activation stage but never performed the single rename.
    fs::create_dir(prefix.path().join(".mantle/.activation-interrupted")).unwrap();
    std::os::unix::fs::symlink(
        "generations/nonexistent",
        prefix.path().join(".mantle/.activation-interrupted/next"),
    )
    .unwrap();
    let mut children = Vec::new();
    for _ in 0..4 {
        children.push(
            Command::new(env!("CARGO_BIN_EXE_mantle-release"))
                .args(["install", "--manifest"])
                .arg(fixture.dir.path().join("manifest.json"))
                .arg("--prefix")
                .arg(prefix.path())
                .stdout(std::process::Stdio::null())
                .spawn()
                .unwrap(),
        );
    }
    for mut child in children {
        assert!(child.wait().unwrap().success());
    }
    assert_eq!(
        fs::read_link(prefix.path().join(".mantle/current")).unwrap(),
        before
    );
    assert_eq!(
        Command::new(prefix.path().join("bin/mantle"))
            .arg("--version")
            .output()
            .unwrap()
            .stdout,
        b"mantle 0.1.4\n"
    );
}

#[test]
fn deterministic_packaging_and_interrupted_process_keep_previous_activation() {
    use std::time::{Duration, Instant};
    let mut fixture = Fixture::new();
    let verified = mantle_artifact::verify(&fixture.dir.path().join("manifest.json")).unwrap();
    let first = mantle_release::build::archive(&verified.artifacts[GNU], GNU, "0.1.4").unwrap();
    let second = mantle_release::build::archive(&verified.artifacts[GNU], GNU, "0.1.4").unwrap();
    assert_eq!(first, second);
    let prefix = tempfile::tempdir().unwrap();
    let path = prefix.path().to_str().unwrap();
    assert!(fixture.cli("install", &["--prefix", path]).status.success());
    let before = fs::read_link(prefix.path().join(".mantle/current")).unwrap();
    fixture.manifest["source_commit"] = json!("2222222222222222222222222222222222222222");
    fixture.save();
    let lock_path = prefix.path().join(".mantle/lock");
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&lock_path)
        .unwrap();
    rustix::fs::flock(&lock, rustix::fs::FlockOperation::LockExclusive).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_mantle-release"))
        .args(["install", "--manifest"])
        .arg(fixture.dir.path().join("manifest.json"))
        .arg("--prefix")
        .arg(prefix.path())
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(8);
    loop {
        let waiting = fs::read_dir(format!("/proc/{}/fd", child.id()))
            .unwrap()
            .flatten()
            .any(|entry| fs::read_link(entry.path()).is_ok_and(|target| target == lock_path));
        if waiting {
            break;
        }
        assert!(Instant::now() < deadline, "installer never reached lock");
        assert!(child.try_wait().unwrap().is_none());
        std::thread::sleep(Duration::from_millis(10));
    }
    child.kill().unwrap();
    assert!(!child.wait().unwrap().success());
    assert_eq!(
        fs::read_link(prefix.path().join(".mantle/current")).unwrap(),
        before
    );
    drop(lock);
    assert!(fixture.cli("install", &["--prefix", path]).status.success());
    assert_ne!(
        fs::read_link(prefix.path().join(".mantle/current")).unwrap(),
        before
    );
    assert!(prefix.path().join(".mantle").join(before).is_dir());
}
#[test]
fn missing_linked_fifo_and_oversized_inputs_and_unsupported_target_refuse() {
    use std::os::unix::fs::symlink;
    let fixture = Fixture::new();
    let name = fixture.manifest["artifacts"][0]["name"].as_str().unwrap();
    let archive = fixture.dir.path().join(name);
    let saved = fixture.dir.path().join("saved.tar");
    fs::rename(&archive, &saved).unwrap();
    assert!(!fixture.cli("verify", &[]).status.success());
    symlink(&saved, &archive).unwrap();
    assert!(!fixture.cli("verify", &[]).status.success());
    fs::remove_file(&archive).unwrap();
    rustix::fs::mknodat(
        rustix::fs::CWD,
        &archive,
        rustix::fs::FileType::Fifo,
        rustix::fs::Mode::RUSR,
        0,
    )
    .unwrap();
    assert!(!fixture.cli("verify", &[]).status.success());
    fs::remove_file(&archive).unwrap();
    let large = fs::File::create(&archive).unwrap();
    large.set_len(mantle_artifact::MAX_ARCHIVE + 1).unwrap();
    assert!(!fixture.cli("verify", &[]).status.success());
    drop(large);
    fs::remove_file(&archive).unwrap();
    fs::rename(saved, archive).unwrap();
    let prefix = tempfile::tempdir().unwrap();
    let output = fixture.cli(
        "install",
        &[
            "--prefix",
            prefix.path().to_str().unwrap(),
            "--target",
            MUSL,
        ],
    );
    assert!(!output.status.success());
    assert_eq!(fs::read_dir(prefix.path()).unwrap().count(), 0);
}

#[test]
fn actual_git_archive_exports_exact_source_and_still_refuses_source_symlinks() {
    let source = tempfile::tempdir().unwrap();
    let work = tempfile::tempdir().unwrap();
    let git = |args: &[&str]| {
        Command::new("git")
            .arg("-C")
            .arg(source.path())
            .args(args)
            .output()
            .unwrap()
    };
    assert!(git(&["init", "--quiet"]).status.success());
    let content = b"tracked exact source, same version 0.1.4\n";
    fs::write(source.path().join("source.txt"), content).unwrap();
    assert!(git(&["add", "source.txt"]).status.success());
    assert!(
        git(&[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--quiet",
            "-m",
            "fixture"
        ])
        .status
        .success()
    );
    let head = String::from_utf8(git(&["rev-parse", "HEAD"]).stdout).unwrap();
    mantle_release::build::export_source(source.path(), head.trim(), &work.path().join("export"))
        .unwrap();
    assert_eq!(
        fs::read(work.path().join("export/source.txt")).unwrap(),
        content
    );
    assert!(!work.path().join("export/pax_global_header").exists());
    std::os::unix::fs::symlink("/outside", source.path().join("link")).unwrap();
    assert!(git(&["add", "link"]).status.success());
    assert!(
        git(&[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--quiet",
            "-m",
            "link fixture"
        ])
        .status
        .success()
    );
    let head = String::from_utf8(git(&["rev-parse", "HEAD"]).stdout).unwrap();
    assert!(
        mantle_release::build::export_source(
            source.path(),
            head.trim(),
            &work.path().join("refused")
        )
        .unwrap_err()
        .to_string()
        .contains("unsupported links")
    );
}

#[test]
fn adversary_untracked_git_attributes_cannot_change_exact_source_export() {
    let source = tempfile::tempdir().unwrap();
    let work = tempfile::tempdir().unwrap();
    let git = |args: &[&str]| {
        Command::new("git")
            .arg("-C")
            .arg(source.path())
            .args(args)
            .output()
            .unwrap()
    };
    assert!(git(&["init", "--quiet"]).status.success());
    let tracked = b"fn main() { println!(\"$Format:%H$\"); }\n";
    fs::write(source.path().join("main.rs"), tracked).unwrap();
    assert!(git(&["add", "main.rs"]).status.success());
    assert!(
        git(&[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--quiet",
            "-m",
            "exact source fixture"
        ])
        .status
        .success()
    );
    let head = String::from_utf8(git(&["rev-parse", "HEAD"]).stdout).unwrap();
    // Git permits local, untracked attributes that do not make the checkout dirty.
    fs::write(
        source.path().join(".git/info/attributes"),
        b"main.rs export-subst\n",
    )
    .unwrap();
    assert!(
        git(&["status", "--porcelain=v1", "--untracked-files=all"])
            .stdout
            .is_empty()
    );
    assert_eq!(
        mantle_release::build::source_identity(source.path(), head.trim()).unwrap(),
        head.trim()
    );
    let export = work.path().join("export");
    let result = mantle_release::build::export_source(source.path(), head.trim(), &export);
    if result.is_ok() {
        assert_eq!(
            fs::read(export.join("main.rs")).unwrap(),
            tracked,
            "accepted exact-source export changed committed Rust bytes using untracked Git attributes"
        );
    }
}

#[test]
fn adversary_modified_generation_inventory_is_refused_without_activation() {
    let fixture = Fixture::new();
    let prefix = tempfile::tempdir().unwrap();
    let prefix_arg = prefix.path().to_str().unwrap();
    assert!(
        fixture
            .cli("install", &["--prefix", prefix_arg])
            .status
            .success()
    );
    let active = prefix.path().join(".mantle/current");
    let before = fs::read_link(&active).unwrap();
    let unverified = prefix
        .path()
        .join(".mantle")
        .join(&before)
        .join("bin/unverified-helper");
    fs::write(
        &unverified,
        b"unmanaged bytes outside the verified generation inventory",
    )
    .unwrap();
    let result = fixture.cli("install", &["--prefix", prefix_arg]);
    assert_eq!(fs::read_link(&active).unwrap(), before);
    assert_eq!(
        fs::read(&unverified).unwrap(),
        b"unmanaged bytes outside the verified generation inventory"
    );
    assert!(
        !result.status.success(),
        "installer accepted an existing generation with an extra unmanaged payload"
    );
}

#[test]
fn exact_source_export_checks_attributes_inventory_modes_and_replacement_objects() {
    use std::os::unix::fs::PermissionsExt;
    for case in [
        "export-ignore",
        "tracked-export-subst",
        "executable",
        "replace-blob",
        "replace-commit",
    ] {
        let source = tempfile::tempdir().unwrap();
        let work = tempfile::tempdir().unwrap();
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .arg("-C")
                .arg(source.path())
                .args(args)
                .output()
                .unwrap();
            assert!(
                out.status.success(),
                "{case}: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            out.stdout
        };
        git(&["init", "--quiet"]);
        fs::create_dir(source.path().join("nested")).unwrap();
        let tracked = b"fn main() { println!(\"$Format:%H$\"); }\n";
        fs::write(source.path().join("nested/main.rs"), tracked).unwrap();
        if case == "tracked-export-subst" {
            fs::write(
                source.path().join(".gitattributes"),
                b"nested/main.rs export-subst\n",
            )
            .unwrap();
        }
        if case == "executable" {
            fs::set_permissions(
                source.path().join("nested/main.rs"),
                fs::Permissions::from_mode(0o755),
            )
            .unwrap();
        }
        git(&["add", "."]);
        git(&[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--quiet",
            "-m",
            "source fixture",
        ]);
        let head = String::from_utf8(git(&["rev-parse", "HEAD"]))
            .unwrap()
            .trim()
            .to_owned();
        if case == "export-ignore" {
            fs::write(
                source.path().join(".git/info/attributes"),
                b"nested/main.rs export-ignore\n",
            )
            .unwrap();
        }
        if case.starts_with("replace-") {
            let old_blob = String::from_utf8(git(&["rev-parse", "HEAD:nested/main.rs"]))
                .unwrap()
                .trim()
                .to_owned();
            fs::write(
                source.path().join("nested/main.rs"),
                b"fn main() { panic!(\"replacement\"); }\n",
            )
            .unwrap();
            git(&["add", "."]);
            git(&[
                "-c",
                "user.name=Fixture",
                "-c",
                "user.email=fixture@example.invalid",
                "commit",
                "--quiet",
                "-m",
                "replacement fixture",
            ]);
            let replacement = String::from_utf8(git(&[
                "rev-parse",
                if case == "replace-blob" {
                    "HEAD:nested/main.rs"
                } else {
                    "HEAD"
                },
            ]))
            .unwrap()
            .trim()
            .to_owned();
            git(&["reset", "--hard", &head]);
            git(&[
                "replace",
                if case == "replace-blob" {
                    &old_blob
                } else {
                    &head
                },
                &replacement,
            ]);
        }
        let export = work.path().join("export");
        let result = mantle_release::build::export_source(source.path(), &head, &export);
        if matches!(case, "export-ignore" | "tracked-export-subst") {
            assert!(
                result.is_err(),
                "altered or incomplete export accepted: {case}"
            );
        } else {
            result.unwrap();
            assert_eq!(
                fs::read(export.join("nested/main.rs")).unwrap(),
                tracked,
                "{case}"
            );
            assert_eq!(
                fs::metadata(export.join("nested/main.rs"))
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                if case == "executable" { 0o755 } else { 0o644 }
            );
        }
    }
}

#[test]
fn generation_reuse_checks_all_entry_locations_and_types() {
    let fixture = Fixture::new();
    for case in [
        "root-file",
        "root-directory",
        "bin-directory",
        "extra-symlink",
        "payload-symlink",
        "missing-payload",
    ] {
        let prefix = tempfile::tempdir().unwrap();
        let prefix_arg = prefix.path().to_str().unwrap();
        assert!(
            fixture
                .cli("install", &["--prefix", prefix_arg])
                .status
                .success()
        );
        let active = prefix.path().join(".mantle/current");
        let before = fs::read_link(&active).unwrap();
        let root = prefix.path().join(".mantle").join(&before);
        match case {
            "root-file" => fs::write(root.join("extra"), b"extra").unwrap(),
            "root-directory" => fs::create_dir(root.join("extra")).unwrap(),
            "bin-directory" => fs::create_dir(root.join("bin/extra")).unwrap(),
            "extra-symlink" => std::os::unix::fs::symlink("/outside", root.join("extra")).unwrap(),
            "payload-symlink" => {
                fs::remove_file(root.join("LICENSE")).unwrap();
                std::os::unix::fs::symlink("RUST_RUNTIME_LICENSE.html", root.join("LICENSE"))
                    .unwrap();
            }
            "missing-payload" => fs::remove_file(root.join("LICENSE")).unwrap(),
            _ => unreachable!(),
        }
        assert!(
            !fixture
                .cli("install", &["--prefix", prefix_arg])
                .status
                .success(),
            "{case}"
        );
        assert_eq!(fs::read_link(&active).unwrap(), before, "{case}");
    }
}

#[test]
fn adversary_exact_export_preserves_shared_blobs_and_per_path_modes() {
    use std::os::unix::fs::PermissionsExt;
    let source = tempfile::tempdir().unwrap();
    let work = tempfile::tempdir().unwrap();
    let git = |args: &[&str]| {
        let result = Command::new("git")
            .arg("-C")
            .arg(source.path())
            .args(args)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        result.stdout
    };
    git(&["init", "--quiet"]);
    let bytes = b"fn main() { println!(\"identical committed blob\"); }\n";
    let files = [
        ("nested dir/λ.rs", 0o644),
        ("nested dir/tab\tname.rs", 0o755),
        ("second/same.rs", 0o644),
    ];
    for (name, mode) in files {
        let path = source.path().join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, bytes).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(mode)).unwrap();
    }
    git(&["add", "."]);
    git(&[
        "-c",
        "user.name=Fixture",
        "-c",
        "user.email=fixture@example.invalid",
        "commit",
        "--quiet",
        "-m",
        "shared blob fixture",
    ]);
    let head = String::from_utf8(git(&["rev-parse", "HEAD"])).unwrap();
    let export = work.path().join("export");
    mantle_release::build::export_source(source.path(), head.trim(), &export).unwrap();
    for (name, mode) in files {
        let path = export.join(name);
        assert_eq!(fs::read(&path).unwrap(), bytes, "{name}");
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            mode,
            "{name}"
        );
    }
    assert_eq!(fs::read_dir(&export).unwrap().count(), 2);
    assert_eq!(fs::read_dir(export.join("nested dir")).unwrap().count(), 2);
    assert_eq!(fs::read_dir(export.join("second")).unwrap().count(), 1);
}
