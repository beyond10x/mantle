//! The Substrate side: readiness facts, and the fixed confinement the slice requests.

use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use b10x_substrate_sdk::{Client, ExecUsage, ExecutionPolicy, Machine};

/// The aperture the daemon declares to the worker's egress gateway.
pub const APERTURE: &str = "egress";
/// The daemon's secret slot for the Claude Code OAuth token.
pub const CLAUDE_SLOT: &str = "claude";
/// The descriptor the token arrives at inside the agent's sandbox.
pub const CLAUDE_FD: u32 = 3;
/// Toolchain, Claude Code and the launcher, projected read-only at the same path.
pub const TOOLCHAIN_ROOT: &str = "/opt/mantle";
/// Where `mantle-launch serve` keeps the agent's pipes; `attach` finds it there.
pub const AGENT_DIR: &str = "/workspace/.mantle/agent";

pub const NON_RECORDING_UNAVAILABLE: &str = "FAILED_CAPABILITY: Codex requires supported non-recording terminal capture; the pinned Substrate SDK cannot provide it";

/// An internal requirement, not an invented Machine fact. Replace this adapter only when
/// an actual supported SDK binding can enforce and observe the requested capture policy.
pub fn require_capture_binding(non_recording: bool) -> Result<()> {
    if non_recording {
        anyhow::bail!(NON_RECORDING_UNAVAILABLE);
    }
    Ok(())
}
/// The aperture listens on sandbox loopback at the destination port it was declared with.
pub const PROXY: &str = "http://127.0.0.1:3128";

pub async fn connect(socket: &Path) -> Result<Client> {
    Client::builder()
        .unix_socket(socket)
        .connect()
        .await
        .context("connecting to Substrate through the tunnel")
}

/// Every fact the slice needs from the worker. A missing one is named, never assumed.
pub fn missing_facts(machine: &Machine) -> Vec<String> {
    let facts = &machine.facts;
    let mut missing = Vec::new();
    if facts.exec_argv_only != Some(true) {
        missing.push("exec.argv_only".to_owned());
    }
    if facts.exec_no_egress != Some(true) {
        missing.push("exec.no_egress".to_owned());
    }
    match &facts.exec_cgroup_limits {
        Some(limits) if limits.processes && limits.memory => {}
        _ => missing.push("exec.cgroup_limits (processes + memory)".to_owned()),
    }
    if facts.exec_cgroup_kill != Some(true) {
        missing.push("exec.cgroup_kill".to_owned());
    }
    if facts.sessions_pty != Some(true) {
        missing.push("sessions.pty".to_owned());
    }
    if !facts
        .exec_egress_apertures
        .as_ref()
        .is_some_and(|apertures| apertures.iter().any(|aperture| aperture.name == APERTURE))
    {
        missing.push(format!("exec.egress_apertures[{APERTURE}]"));
    }
    missing
}

pub fn claude_slot_present(machine: &Machine) -> bool {
    machine
        .facts
        .secrets_slots
        .as_ref()
        .is_some_and(|slots| slots.iter().any(|slot| slot == CLAUDE_SLOT))
}

/// Capabilities observed from Substrate, without inventing an executable observation.
pub fn missing_capability_facts(machine: &Machine, claude_selected: bool) -> Vec<String> {
    let mut missing = missing_facts(machine);
    if claude_selected && !claude_slot_present(machine) {
        missing.push(format!("secrets.slots[{CLAUDE_SLOT}]"));
    }
    missing
}

pub fn selected_claude_missing_facts(machine: &Machine, executable_present: bool) -> Vec<String> {
    let mut missing = missing_capability_facts(machine, true);
    if !executable_present {
        missing.push("agent.executable[claude]".to_owned());
    }
    debug_assert_eq!(
        missing.is_empty(),
        mantle_worker::worker_ready(
            missing_facts(machine).is_empty(),
            true,
            executable_present,
            claude_slot_present(machine)
        )
    );
    missing
}

pub fn quota_served(machine: &Machine) -> bool {
    machine.facts.workspace_storage_quota.is_some()
}

pub fn policy(timeout: Duration, memory_bytes: u64, processes: u32) -> Result<ExecutionPolicy> {
    ExecutionPolicy::builder()
        .timeout(timeout)
        // Cumulative CPU is the only CPU bound Substrate 0.7.8 enforces, and its ceiling is 24h.
        .cpu_time(b10x_substrate_sdk::MAX_EXEC_DURATION)
        .memory_bytes(memory_bytes)
        .processes(processes)
        .output_bytes(b10x_substrate_sdk::MAX_IO_BYTES)
        .build()
        .context("building the execution policy")
}

/// Environment every run in a session shares.
pub fn base_environment(cpu: u32) -> Vec<(&'static str, String)> {
    vec![
        ("HOME", "/workspace/.mantle/home".to_owned()),
        ("CARGO_HOME", "/workspace/.mantle/cargo".to_owned()),
        ("RUSTUP_HOME", format!("{TOOLCHAIN_ROOT}/rustup")),
        (
            "PATH",
            format!("{TOOLCHAIN_ROOT}/bin:{TOOLCHAIN_ROOT}/cargo/bin:/usr/local/bin:/usr/bin:/bin"),
        ),
        (
            "SSL_CERT_FILE",
            "/etc/ssl/certs/ca-certificates.crt".to_owned(),
        ),
        (
            "NODE_EXTRA_CA_CERTS",
            "/etc/ssl/certs/ca-certificates.crt".to_owned(),
        ),
        ("GIT_CONFIG_NOSYSTEM", "1".to_owned()),
        ("CARGO_BUILD_JOBS", cpu.to_string()),
        ("LANG", "C.UTF-8".to_owned()),
        ("TERM", "xterm-256color".to_owned()),
        ("SHELL", "/bin/bash".to_owned()),
        ("USER", "dev".to_owned()),
        ("TMPDIR", "/tmp".to_owned()),
        ("DISABLE_AUTOUPDATER", "1".to_owned()),
        ("CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC", "1".to_owned()),
    ]
}

// Runs request no resource-usage measurement. On a Substrate 0.7.8 worker that advertises
// `exec_resource_usage` (block_io=true), a run that requests it ends `unknown` with
// `exec.metrics-unavailable` (observed 2026-10-02 on the KubeVirt worker), and an agent whose state
// is unknown cannot be attached to or reported on. Status reports usage as not observed.

/// One line per observed usage figure; an absent observation says so.
pub fn describe_usage(usage: Option<&ExecUsage>) -> Vec<(String, String)> {
    match usage {
        None => vec![("usage".to_owned(), "not observed".to_owned())],
        Some(ExecUsage::Pending { .. }) => vec![("usage".to_owned(), "pending".to_owned())],
        Some(ExecUsage::Unavailable { code, .. }) => {
            vec![("usage".to_owned(), format!("unavailable ({code})"))]
        }
        Some(ExecUsage::Observed(observed)) => {
            let mut rows = vec![
                (
                    "cpu time".to_owned(),
                    format!("{:.1} s", observed.cpu_time_us as f64 / 1e6),
                ),
                (
                    "wall time".to_owned(),
                    format!("{:.0} s", observed.wall_time_us as f64 / 1e6),
                ),
                ("memory peak".to_owned(), bytes(observed.memory_peak_bytes)),
                (
                    "processes peak".to_owned(),
                    observed.processes_peak.to_string(),
                ),
                (
                    "oom kills".to_owned(),
                    observed.memory_oom_kills.to_string(),
                ),
            ];
            if let Some(current) = observed.memory_current_bytes {
                rows.insert(3, ("memory now".to_owned(), bytes(current)));
            }
            if let Some(current) = observed.processes_current {
                rows.push(("processes now".to_owned(), current.to_string()));
            }
            rows
        }
    }
}

pub fn bytes(value: u64) -> String {
    const GIB: f64 = (1u64 << 30) as f64;
    const MIB: f64 = (1u64 << 20) as f64;
    let value = value as f64;
    if value >= GIB {
        format!("{:.1} GiB", value / GIB)
    } else {
        format!("{:.0} MiB", value / MIB)
    }
}

#[cfg(test)]
mod worker_tests {
    use super::*;
    fn ready_machine() -> Machine {
        serde_json::from_value(serde_json::json!({
            "capability_snapshot": "test", "driver_version": "test", "configuration_generation": 1,
            "probed_at": "2026-10-02T00:00:00Z", "valid_until": null,
            "guarded_workspace_io": true, "exec_argv_only": true, "exec_no_egress": true,
            "exec_cgroup_limits": true, "exec_cgroup_kill": true, "events_pull": true, "events_stream": false,
            "facts": {
                "operation.ledger-subject-max-rows": 1, "operation.ledger-subject-max-bytes": 1,
                "operation.ledger-global-max-rows": 1, "operation.ledger-global-max-bytes": 1,
                "exec.argv-only": true, "exec.no-egress": true,
                "exec.cgroup-limits": {"cpu": true, "memory": true, "processes": true},
                "exec.cgroup-kill": true, "sessions.pty": true,
                "exec.egress-apertures": [{"name": "egress", "destination": "127.0.0.1:3128/tcp"}]
            }
        })).unwrap()
    }
    #[test]
    fn common_readiness_and_selected_claude_are_distinct_observations() {
        let mut machine = ready_machine();
        assert!(missing_facts(&machine).is_empty());
        assert_eq!(
            selected_claude_missing_facts(&machine, true),
            ["secrets.slots[claude]"]
        );
        machine.facts.secrets_slots = Some(vec!["claude".into()]);
        assert_eq!(
            selected_claude_missing_facts(&machine, false),
            ["agent.executable[claude]"]
        );
        assert!(selected_claude_missing_facts(&machine, true).is_empty());
        machine.facts.exec_no_egress = None;
        assert_eq!(missing_facts(&machine), ["exec.no_egress"]);
        assert!(!selected_claude_missing_facts(&machine, true).is_empty());
    }
}
