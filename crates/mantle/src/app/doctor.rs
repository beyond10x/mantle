//! Read-only observations. Errors become fixed stage diagnostics, never raw process output.
use std::process::Command;
use std::time::Duration;

use anyhow::{Context, Result, ensure};
use b10x_substrate_sdk::Machine;
use serde::Serialize;
use serde_json::Value;

use super::worker::{self, SUBSTRATE_VERSION, WORKER};
use crate::adapters::{
    aws,
    kubevirt::Kubevirt,
    ssh::Ssh,
    state::{Store, WorkerRecord},
    substrate,
};
use crate::config::{Config, Provider, RuntimeContext};
use crate::profile::Selection;

const STAGES: [(&str, &str); 9] = [
    (
        "selection",
        "Check --profile, MANTLE_PROFILE and legacy override conflicts.",
    ),
    (
        "configuration",
        "Check the selected bounded regular configuration file and its required fields.",
    ),
    (
        "state",
        "Check the existing state database and recorded worker; doctor never initializes state.",
    ),
    (
        "placement",
        "Select the configuration matching the recorded worker placement.",
    ),
    (
        "provider",
        "Check provider authentication, connectivity and whether the recorded worker is running.",
    ),
    (
        "ssh",
        "Check the existing private SSH key, pinned known host and provider tunnel.",
    ),
    (
        "service",
        "Check substrate.service and its socket on the worker; doctor does not restart services.",
    ),
    (
        "compatibility",
        "Use matching Mantle worker binaries and the pinned live Substrate runtime/wire contract.",
    ),
    (
        "facts",
        "Check the required common confinement capabilities; readiness does not establish agent authentication.",
    ),
];

#[derive(Debug, Serialize)]
pub struct DiagnosticCheck {
    pub stage: &'static str,
    pub status: &'static str,
    pub detail: &'static str,
}
#[derive(Debug, Serialize)]
pub struct DiagnosticReport {
    pub selection: Option<Selection>,
    pub checks: Vec<DiagnosticCheck>,
    pub healthy: bool,
}
impl DiagnosticReport {
    fn new() -> Self {
        Self {
            selection: None,
            checks: STAGES
                .iter()
                .map(|(stage, _)| DiagnosticCheck {
                    stage,
                    status: "skipped",
                    detail: "An earlier required observation failed.",
                })
                .collect(),
            healthy: false,
        }
    }
    fn observe<T>(&mut self, index: usize, result: Result<T>) -> Option<T> {
        self.checks[index].status = if result.is_ok() { "healthy" } else { "failed" };
        self.checks[index].detail = if result.is_ok() {
            "Required observation verified."
        } else {
            STAGES[index].1
        };
        result.ok()
    }
    pub fn human(&self) -> String {
        let mut text = String::new();
        for check in &self.checks {
            text.push_str(&format!(
                "{}: {} — {}\n",
                check.stage, check.status, check.detail
            ));
        }
        text.push_str("Readiness does not establish agent authentication.\n");
        text
    }
}

pub(crate) trait Probes {
    fn provider(
        &self,
        config: &RuntimeContext,
        worker: &WorkerRecord,
        timeout: Duration,
    ) -> Result<()>;
    fn ssh(&self, config: &RuntimeContext, worker: &WorkerRecord, timeout: Duration) -> Result<()>;
    fn service(
        &self,
        config: &RuntimeContext,
        worker: &WorkerRecord,
        timeout: Duration,
    ) -> Result<()>;
    async fn compatibility(
        &self,
        config: &RuntimeContext,
        worker: &WorkerRecord,
        timeout: Duration,
    ) -> Result<Machine>;
}

pub(crate) async fn diagnose(
    selection: Result<Selection>,
    probes: &impl Probes,
    timeout: Duration,
) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    let selection = selection.and_then(|selection| {
        ensure!(
            selection.config_path.to_str().is_some() && selection.state_dir.to_str().is_some(),
            "diagnostic selection paths must be representable in JSON"
        );
        Ok(selection)
    });
    let Some(selection) = report.observe(0, selection) else {
        return report;
    };
    report.selection = Some(selection.clone());
    let Some(config) = report.observe(
        1,
        crate::profile::read_bounded(&selection.config_path, 1024 * 1024)
            .and_then(|text| Config::parse(&text)),
    ) else {
        return report;
    };
    let context = RuntimeContext { config, selection };
    let Some(worker) = report.observe(
        2,
        Store::open_readonly(&context.selection.state_dir.join("state.db"))
            .and_then(|store| store.worker(WORKER))
            .and_then(|record| record.context("no worker")),
    ) else {
        return report;
    };
    let placement = (|| -> Result<()> {
        ensure!(
            worker.region == worker::location(&context)?,
            "worker placement mismatch"
        );
        ensure!(
            !worker.instance.is_empty()
                && !worker.instance.starts_with('-')
                && worker.instance.len() <= 255
                && worker
                    .instance
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b)),
            "invalid worker identifier"
        );
        Ok(())
    })();
    if report.observe(3, placement).is_none() {
        return report;
    }
    if report
        .observe(4, probes.provider(&context, &worker, timeout))
        .is_none()
    {
        return report;
    }
    if report
        .observe(5, probes.ssh(&context, &worker, timeout))
        .is_none()
    {
        return report;
    }
    if report
        .observe(6, probes.service(&context, &worker, timeout))
        .is_none()
    {
        return report;
    }
    let Some(machine) = report.observe(
        7,
        probes
            .compatibility(&context, &worker, timeout)
            .await
            .and_then(|machine| {
                ensure!(
                    machine.driver_version == SUBSTRATE_VERSION,
                    "live runtime version mismatch"
                );
                Ok(machine)
            }),
    ) else {
        return report;
    };
    if report
        .observe(
            8,
            if substrate::missing_facts(&machine).is_empty() {
                Ok(())
            } else {
                Err(anyhow::anyhow!("required facts missing"))
            },
        )
        .is_some()
    {
        report.healthy = true;
    }
    report
}

pub struct SystemProbes;
fn ssh(config: &RuntimeContext, worker: &WorkerRecord) -> Result<Ssh> {
    let proxy = match config.provider {
        Provider::Aws => aws::proxy_command(config.aws()?),
        Provider::Kubevirt => {
            Kubevirt::new(config.kubevirt()?, &config.ubuntu_serial).proxy_command(WORKER)
        }
    };
    Ssh::readonly(&worker.instance, proxy, &config.selection.state_dir)
}
fn remote(ssh: &Ssh, command: &str, timeout: Duration) -> Result<Vec<u8>> {
    let output = ssh.bounded(command, None, timeout)?;
    ensure!(output.status.success(), "remote diagnostic failed");
    Ok(output.stdout)
}
impl Probes for SystemProbes {
    fn provider(
        &self,
        config: &RuntimeContext,
        worker: &WorkerRecord,
        timeout: Duration,
    ) -> Result<()> {
        let mut command = match config.provider {
            Provider::Aws => {
                let aws = config.aws()?;
                let mut command = Command::new("aws");
                command.args([
                    "ec2",
                    "describe-instances",
                    "--instance-ids",
                    &worker.instance,
                    "--profile",
                    &aws.profile,
                    "--region",
                    &aws.region,
                    "--output",
                    "json",
                    "--no-cli-pager",
                ]);
                command
            }
            Provider::Kubevirt => {
                let kube = config.kubevirt()?;
                let mut command = Command::new("kubectl");
                command.args([
                    "--context",
                    &kube.context,
                    "--namespace",
                    &kube.namespace,
                    "get",
                    "vm",
                    &crate::adapters::kubevirt::vm_name(WORKER),
                    "--output",
                    "json",
                ]);
                command
            }
        };
        let output = mantle_worker::run_bounded(&mut command, None, timeout, 1024 * 1024)?;
        ensure!(output.status.success(), "provider observation failed");
        let value: Value = serde_json::from_slice(&output.stdout)?;
        let ready = match config.provider {
            Provider::Aws => value["Reservations"]
                .as_array()
                .into_iter()
                .flatten()
                .flat_map(|r| r["Instances"].as_array().into_iter().flatten())
                .any(|v| v["InstanceId"] == worker.instance && v["State"]["Name"] == "running"),
            Provider::Kubevirt => {
                value["metadata"]["uid"] == worker.instance
                    && value["status"]["ready"] == true
                    && value["status"]["printableStatus"] == "Running"
            }
        };
        ensure!(ready, "recorded worker is not observed running");
        Ok(())
    }
    fn ssh(&self, config: &RuntimeContext, worker: &WorkerRecord, timeout: Duration) -> Result<()> {
        remote(&ssh(config, worker)?, "true", timeout).map(|_| ())
    }
    fn service(
        &self,
        config: &RuntimeContext,
        worker: &WorkerRecord,
        timeout: Duration,
    ) -> Result<()> {
        remote(&ssh(config,worker)?,"systemctl is-active --quiet substrate.service && test -S /run/substrate/substrate.sock",timeout).map(|_|())
    }
    async fn compatibility(
        &self,
        config: &RuntimeContext,
        worker: &WorkerRecord,
        timeout: Duration,
    ) -> Result<Machine> {
        let ssh = ssh(config, worker)?;
        let output = remote(
            &ssh,
            "/opt/mantle/bin/mantle-worker --version && /opt/mantle/bin/mantle-launch --version && /opt/mantle/bin/mantle-egress --version",
            timeout,
        )?;
        let versions = String::from_utf8(output)?;
        let expected = format!(
            "mantle-worker {0}\nmantle-launch {0}\nmantle-egress {0}",
            env!("CARGO_PKG_VERSION")
        );
        ensure!(
            versions.trim() == expected,
            "installed Mantle version mismatch"
        );
        let tunnel = ssh.diagnostic_tunnel(timeout)?;
        let observed = tokio::time::timeout(timeout, substrate::connect(tunnel.socket())).await;
        let cleanup = tunnel.finish();
        cleanup?;
        Ok(observed
            .context("Substrate discovery timed out")??
            .machine())
    }
}
