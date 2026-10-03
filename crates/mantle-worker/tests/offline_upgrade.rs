#[path = "support/upgrade.rs"]
mod support;
use mantle_worker::{maintenance, upgrade};
use std::{
    fs,
    time::{Duration, Instant},
};
use support::Fixture;

#[test]
fn eligible_rendered_maintenance_applies_complete_bundle_and_preserves_agents() {
    let f = Fixture::new();
    let before = f.snapshot();
    let assessment = maintenance::assess(&f.host, Instant::now() + Duration::from_secs(10));
    assert!(assessment.applicable, "{assessment:?}");
    let report = upgrade::apply(&f.host, &f.root, &f.manifest, &()).unwrap();
    assert_eq!(report.outcome, "applied-restart-required");
    assert_eq!(
        fs::read(f.root.join("bin/claude")).unwrap(),
        before["claude"]
    );
    assert_eq!(
        fs::read_link(f.root.join("bin/codex"))
            .unwrap()
            .to_str()
            .unwrap(),
        "../agents/codex/current/codex"
    );
    assert_ne!(f.snapshot()["mantle-worker"], before["mantle-worker"]);
    assert!(
        f.host
            .calls
            .borrow()
            .iter()
            .all(|c| !c.contains(" stop") && !c.contains(" restart") && !c.contains(" mask"))
    );
}

#[test]
fn offline_refusals_do_not_mutate_installed_inventory() {
    for fault in ["active", "unknown", "incompatible", "job", "unit-job"] {
        let f = Fixture::new();
        *f.host.fault.borrow_mut() = fault.into();
        let before = f.snapshot();
        let result = upgrade::apply(&f.host, &f.root, &f.manifest, &());
        assert!(result.is_err() || !result.unwrap().applicable, "{fault}");
        assert_eq!(f.snapshot(), before, "{fault}");
    }
}

#[test]
fn deployed_units_missing_originals_pending_jobs_and_descendant_processes_refuse() {
    for case in [
        "regular-etc",
        "ineffective-runtime",
        "missing-original",
        "not-found",
        "populated",
        "drop-in",
        "absent-cgroup",
        "missing-parent",
    ] {
        let f = Fixture::new();
        let before = f.snapshot();
        match case {
            "regular-etc" | "ineffective-runtime" => {
                fs::remove_file(f.path("/etc/systemd/system/substrate.service")).unwrap();
                fs::write(
                    f.path("/etc/systemd/system/substrate.service"),
                    maintenance::substrate_service(true),
                )
                .unwrap();
                if case == "ineffective-runtime" {
                    std::os::unix::fs::symlink(
                        "/dev/null",
                        f.path("/run/systemd/system/substrate.service"),
                    )
                    .unwrap();
                }
            }
            "missing-original" => {
                fs::remove_file(f.path("/var/lib/mantle/maintenance/units/substrate.service"))
                    .unwrap()
            }
            "not-found" => *f.host.fault.borrow_mut() = "not-found".into(),
            "populated" => fs::write(
                f.path("/sys/fs/cgroup/system.slice/substrate.service/cgroup.events"),
                b"populated 1\nfrozen 0\n",
            )
            .unwrap(),
            "drop-in" => {
                fs::create_dir(f.path("/etc/systemd/system/service.d")).unwrap();
                fs::write(
                    f.path("/etc/systemd/system/service.d/override.conf"),
                    b"[Service]\nSlice=elsewhere.slice\n",
                )
                .unwrap();
            }
            "absent-cgroup" => {
                fs::remove_dir_all(f.path("/sys/fs/cgroup/system.slice/substrate.service")).unwrap()
            }
            "missing-parent" => fs::remove_dir_all(f.path("/sys/fs/cgroup/system.slice")).unwrap(),
            _ => unreachable!(),
        }
        let report = maintenance::assess(&f.host, Instant::now() + Duration::from_secs(10));
        assert_eq!(
            report.applicable,
            case == "absent-cgroup",
            "{case}: {report:?}"
        );
        if case == "not-found" {
            assert_eq!(report.outcome, "unknown");
        }
        assert_eq!(f.snapshot(), before);
    }
}

struct Fault<'a> {
    point: &'a str,
    rollback: bool,
}
impl upgrade::Hooks for Fault<'_> {
    fn boundary(&self, name: &str) -> anyhow::Result<()> {
        anyhow::ensure!(
            name != self.point && !(self.rollback && name == "before-rollback"),
            "controlled failure at {name}"
        );
        Ok(())
    }
}
#[test]
fn staging_exchange_postcheck_and_rollback_failures_report_observed_state() {
    for (point, rollback, outcome) in [
        ("before-journal", false, "error"),
        ("after-journal", false, "error"),
        ("before-exchange", false, "error"),
        ("after-exchange", false, "rolled-back"),
        ("postcheck", false, "rolled-back"),
        ("postcheck", true, "recovery-required"),
    ] {
        let f = Fixture::new();
        let before = f.snapshot();
        let result = upgrade::apply(&f.host, &f.root, &f.manifest, &Fault { point, rollback });
        if outcome == "error" {
            assert!(result.is_err(), "{point}");
        } else {
            assert_eq!(result.unwrap().outcome, outcome, "{point}");
        }
        if outcome != "recovery-required" {
            assert_eq!(f.snapshot(), before, "{point}");
        } else {
            assert_ne!(f.snapshot(), before);
            assert!(f.root.join("upgrade-pending.json").exists());
            let recovered = upgrade::recover(&f.host, &f.root, &()).unwrap();
            assert_eq!(recovered.outcome, "applied-restart-required");
        }
    }
}

#[test]
fn interrupted_upgrade_child() {
    let Some(host) = std::env::var_os("MANTLE_UPGRADE_FIXTURE_HOST") else {
        return;
    };
    let host = support::FixtureHost {
        fs: maintenance::LocalHost { root: host.into() },
        fault: std::cell::RefCell::new(String::new()),
        calls: std::cell::RefCell::new(Vec::new()),
    };
    struct ExitAt(String);
    impl upgrade::Hooks for ExitAt {
        fn boundary(&self, name: &str) -> anyhow::Result<()> {
            if name == self.0 {
                std::process::exit(77);
            }
            Ok(())
        }
    }
    let root = std::env::var_os("MANTLE_UPGRADE_FIXTURE_ROOT").unwrap();
    let manifest = std::env::var_os("MANTLE_UPGRADE_FIXTURE_MANIFEST").unwrap();
    let result = upgrade::apply(
        &host,
        std::path::Path::new(&root),
        std::path::Path::new(&manifest),
        &ExitAt(std::env::var("MANTLE_UPGRADE_FIXTURE_POINT").unwrap()),
    );
    if std::env::var("MANTLE_UPGRADE_FIXTURE_POINT").unwrap() == "locked" {
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("host installation lock unavailable")
        );
        return;
    }
    result.unwrap();
    panic!("selected interruption boundary did not execute");
}

#[test]
fn crash_recovery_uses_actual_directory_identity_before_and_after_exchange() {
    for point in ["after-journal", "after-exchange", "after-marker"] {
        let f = Fixture::new();
        let before = f.snapshot();
        let mut child = std::process::Command::new(std::env::current_exe().unwrap());
        child
            .args(["--exact", "interrupted_upgrade_child", "--nocapture"])
            .env("MANTLE_UPGRADE_FIXTURE_HOST", &f.host.fs.root)
            .env("MANTLE_UPGRADE_FIXTURE_ROOT", &f.root)
            .env("MANTLE_UPGRADE_FIXTURE_MANIFEST", &f.manifest)
            .env("MANTLE_UPGRADE_FIXTURE_POINT", point);
        let out =
            mantle_worker::run_bounded(&mut child, None, Duration::from_secs(30), 65536).unwrap();
        assert_eq!(
            out.status.code(),
            Some(77),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        if point == "after-marker" {
            let pending = upgrade::check(&f.host, &f.root, &f.manifest).unwrap();
            assert_ne!(
                pending.outcome, "current",
                "pending journal requires recovery even when marker was written"
            );
            assert!(pending.diagnostic.contains("recovery"));
        }
        let report = upgrade::recover(&f.host, &f.root, &()).unwrap();
        assert_eq!(
            report.outcome,
            if point == "after-journal" {
                "not-applied"
            } else {
                "applied-restart-required"
            }
        );
        if point == "after-journal" {
            assert_eq!(f.snapshot(), before);
        } else {
            assert_ne!(f.snapshot(), before);
        }
    }
}

#[test]
fn concurrent_first_adoption_rechecks_marker_only_under_host_lock() {
    let f = Fixture::new();
    let source = f.temp.path().join("delivery");
    fs::create_dir(&source).unwrap();
    for name in upgrade::HELPERS {
        fs::copy(f.root.join("bin").join(name), source.join(name)).unwrap();
    }
    // The provisioning path enters before publication, then waits while the bundle is applied.
    struct ApplyBeforeLock<'a>(&'a Fixture);
    impl upgrade::Hooks for ApplyBeforeLock<'_> {
        fn boundary(&self, name: &str) -> anyhow::Result<()> {
            if name == "before-host-lock" {
                assert_eq!(
                    upgrade::apply(&self.0.host, &self.0.root, &self.0.manifest, &())?.outcome,
                    "applied-restart-required"
                );
            }
            Ok(())
        }
    }
    let outcome = upgrade::reconcile_helpers(&f.host, &f.root, &source, &ApplyBeforeLock(&f));
    assert!(
        outcome
            .unwrap_err()
            .to_string()
            .contains("managed helper bundles")
    );
    assert_eq!(
        upgrade::check(&f.host, &f.root, &f.manifest)
            .unwrap()
            .outcome,
        "current"
    );
}

#[test]
fn systemd_mask_fragment_is_its_effective_source_path() {
    let f = Fixture::new();
    *f.host.fault.borrow_mut() = "real-fragment".into();
    let report = maintenance::assess(&f.host, Instant::now() + Duration::from_secs(10));
    assert!(report.applicable, "{report:?}");
}

#[test]
fn separate_process_installer_cannot_exchange_while_host_lock_is_held() {
    let f = Fixture::new();
    let before = f.snapshot();
    let _lock = upgrade::HostLock::acquire(&f.root).unwrap();
    let mut child = std::process::Command::new(std::env::current_exe().unwrap());
    child
        .args(["--exact", "interrupted_upgrade_child", "--nocapture"])
        .env("MANTLE_UPGRADE_FIXTURE_HOST", &f.host.fs.root)
        .env("MANTLE_UPGRADE_FIXTURE_ROOT", &f.root)
        .env("MANTLE_UPGRADE_FIXTURE_MANIFEST", &f.manifest)
        .env("MANTLE_UPGRADE_FIXTURE_POINT", "locked");
    let out = mantle_worker::run_bounded(&mut child, None, Duration::from_secs(15), 65536).unwrap();
    assert!(
        out.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(f.snapshot(), before);
}

#[test]
fn changed_prerequisites_and_unsupported_or_tampered_inputs_leave_prior_bytes() {
    for case in [
        "changed-maintenance",
        "extra-entry",
        "tampered-archive",
        "other-pin",
        "unsafe-mode",
    ] {
        let f = Fixture::new();
        match case {
            "extra-entry" => fs::write(f.root.join("bin/unmanaged"), b"untouched").unwrap(),
            "tampered-archive" => {
                let m: serde_json::Value =
                    serde_json::from_slice(&fs::read(&f.manifest).unwrap()).unwrap();
                fs::write(
                    f.manifest
                        .parent()
                        .unwrap()
                        .join(m["artifacts"][0]["name"].as_str().unwrap()),
                    b"invalid",
                )
                .unwrap();
            }
            "other-pin" => {
                let mut m: serde_json::Value =
                    serde_json::from_slice(&fs::read(&f.manifest).unwrap()).unwrap();
                m["substrate_revision"] = serde_json::Value::String("a".repeat(40));
                fs::write(&f.manifest, serde_json::to_vec(&m).unwrap()).unwrap();
            }
            "unsafe-mode" => {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(f.root.join("bin/claude"), fs::Permissions::from_mode(0o777))
                    .unwrap();
            }
            _ => {}
        }
        struct Changed<'a>(&'a Fixture, bool);
        impl upgrade::Hooks for Changed<'_> {
            fn boundary(&self, name: &str) -> anyhow::Result<()> {
                if self.1 && name == "after-journal" {
                    *self.0.host.fault.borrow_mut() = "active".into();
                }
                Ok(())
            }
        }
        let before = f.snapshot();
        let result = upgrade::apply(
            &f.host,
            &f.root,
            &f.manifest,
            &Changed(&f, case == "changed-maintenance"),
        );
        assert!(result.is_err() || !result.unwrap().applicable, "{case}");
        assert_eq!(f.snapshot(), before, "{case}");
    }
}

#[test]
fn fresh_provisioning_uses_private_delivery_and_defers_active_existing_helpers() {
    for active in [false, true] {
        let f = Fixture::new();
        let source = f.temp.path().join("fresh-delivery");
        fs::create_dir(&source).unwrap();
        let bundle = mantle_artifact::verify(&f.manifest).unwrap();
        for name in upgrade::HELPERS {
            fs::write(
                source.join(name),
                &bundle.artifacts[mantle_artifact::MUSL][&format!("bin/{name}")],
            )
            .unwrap();
        }
        let before = f.snapshot();
        if active {
            *f.host.fault.borrow_mut() = "active".into();
        }
        let deferred = upgrade::reconcile_helpers(&f.host, &f.root, &source, &()).unwrap();
        assert_eq!(deferred.len(), if active { 3 } else { 0 });
        if active {
            assert_eq!(f.snapshot(), before);
        } else {
            assert_ne!(f.snapshot()["mantle-worker"], before["mantle-worker"]);
            assert_eq!(f.snapshot()["claude"], before["claude"]);
            assert_eq!(f.snapshot()["codex"], before["codex"]);
        }
        assert!(!f.root.join("bin/mantle-worker.new").exists());
    }
}
