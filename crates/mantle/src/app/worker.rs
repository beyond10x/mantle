//! `mantle worker up|down|status`, for every provider.
//!
//! The providers differ only in how a machine is created and reached. Everything after the first
//! SSH connection — bootstrap, binaries, credential, daemon, readiness — is one code path, so the
//! workers they produce are the same machine.

use sha2::{Digest, Sha256};
use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use aws_sdk_ec2::types::InstanceStateName;

use crate::adapters::aws::{self, Aws};
use crate::adapters::kubevirt::{self, Kubevirt};
use crate::adapters::ssh::{self, Ssh};
use crate::adapters::state::{Store, WorkerRecord};
use crate::adapters::substrate;
use crate::config::{Config, Provider};

pub const WORKER: &str = "default";
/// The signed Substrate release a worker runs: image `ghcr.io/{repository}@sha256:{manifest}`, of
/// which only the layer holding `substrate-daemon` is fetched and checked by its digest. The SDK in
/// `Cargo.toml` is pinned to the same release (commit `65304edf`).
pub const SUBSTRATE_VERSION: &str = "0.7.10";
pub const SUBSTRATE_IMAGE_REPOSITORY: &str = "beyond10x/b10x-substrate-daemon";
pub const SUBSTRATE_IMAGE_MANIFEST: &str =
    "90469e101c828c7e88fbf1e98c63ec82b9b010e022cbe8432bfc0619c38d1511";
pub const SUBSTRATE_DAEMON_LAYER: &str =
    "4812b85baaf0bdc80f08673e21f5daf1e141eaf0646b20f040b1965de99bf281";
pub const RUST_CHANNEL: &str = "1.97";
pub const CLAUDE_CODE_VERSION: &str = "2.1.287";

const CLOUD_INIT: &str = include_str!("../../../../deploy/cloud-init.yaml");
const SUBSTRATE_SERVICE: &str = include_str!("../../../../deploy/substrate.service");
const EGRESS_SERVICE: &str = include_str!("../../../../deploy/mantle-egress.service");
const BWRAP_APPARMOR: &str = include_str!("../../../../deploy/bwrap.apparmor");

pub const SECRET_PATH: &str = "/var/lib/mantle/secrets/claude";

pub struct UpOptions<'a> {
    pub worker_binaries: &'a Path,
    pub idle_stop: bool,
}

/// Where a recorded worker lives, written into the record so a changed configuration cannot point
/// Mantle at a different machine under the same name.
fn location(config: &Config) -> Result<String> {
    Ok(match config.provider {
        Provider::Aws => format!("aws/{}", config.aws()?.region),
        Provider::Kubevirt => {
            let kubevirt = config.kubevirt()?;
            format!("kubevirt/{}/{}", kubevirt.context, kubevirt.namespace)
        }
    })
}

/// The SSH channel to a recorded worker.
pub fn ssh_for(config: &Config, record: &WorkerRecord) -> Result<Ssh> {
    let expected = location(config)?;
    if record.region != expected {
        bail!(
            "the recorded worker is at {}, but the configuration selects {expected}",
            record.region
        );
    }
    let proxy = match config.provider {
        Provider::Aws => aws::proxy_command(config.aws()?),
        Provider::Kubevirt => {
            Kubevirt::new(config.kubevirt()?, &config.ubuntu_serial).proxy_command(WORKER)
        }
    };
    Ssh::new(&record.instance, proxy)
}

pub async fn up(config: &Config, store: &Store, options: &UpOptions<'_>) -> Result<()> {
    let binaries = worker_binaries(options.worker_binaries)?;
    let token = config
        .claude
        .as_ref()
        .map(|_| config.claude_token())
        .transpose()?;
    let user_data = render_user_data_for(&ssh::public_key()?, token.is_some())?;
    let (instance, data_volume) = match config.provider {
        Provider::Aws => up_aws(config, &user_data, options.idle_stop).await?,
        Provider::Kubevirt => up_kubevirt(config, &user_data).await?,
    };
    let record = WorkerRecord {
        name: WORKER.to_owned(),
        instance,
        region: location(config)?,
        data_volume,
    };
    store.put_worker(&record)?;
    let ssh = ssh_for(config, &record)?;
    wait_for_bootstrap(&ssh).await?;
    install_binaries(&ssh, &binaries)?;
    if let Some(token) = token {
        let unit = ssh.checked_bounded(
            "systemctl cat substrate.service",
            None,
            Duration::from_secs(15),
        )?;
        if !unit.contains("--secret-slot claude=/var/lib/mantle/secrets/claude") {
            bail!(
                "Claude requires a configured Substrate secret slot; schedule maintenance to add it and restart the daemon after stopping sessions; the active daemon was preserved"
            );
        }
        install_token(&ssh, &token)?;
    }
    ssh.checked_bounded(
        "sudo systemctl enable substrate.service >/dev/null 2>&1 && sudo systemctl start substrate.service && systemctl is-active substrate.service",
        None,
        Duration::from_secs(330),
    )
    .context("starting substrate.service")?;
    let result = ssh.checked_bounded(
        "sudo /opt/mantle/bin/mantle-worker install-codex",
        None,
        Duration::from_secs(210),
    )?;
    let outcome: mantle_worker::InstallationOutcome =
        serde_json::from_str(&result).context("reading Codex installation outcome")?;
    println!("codex install {}", serde_json::to_string(&outcome)?);
    report_identity(&ssh)?;
    report_machine(&ssh).await
}

async fn up_aws(
    config: &Config,
    user_data: &str,
    idle_stop: bool,
) -> Result<(String, Option<String>)> {
    let aws_config = config.aws()?;
    let aws = Aws::connect(aws_config).await;
    up_aws_with(&aws, &config.ubuntu_serial, user_data, idle_stop).await
}

pub(crate) async fn up_aws_with(
    aws: &impl AwsPort,
    serial: &str,
    user_data: &str,
    idle_stop: bool,
) -> Result<(String, Option<String>)> {
    match aws.find_instance(WORKER).await? {
        Some(found) => {
            println!("worker      {} ({})", found.id, found.state.as_str());
            if matches!(
                found.state,
                InstanceStateName::Stopped | InstanceStateName::Stopping
            ) {
                if found.state == InstanceStateName::Stopping {
                    aws.wait_for_state(
                        WORKER,
                        InstanceStateName::Stopped,
                        Duration::from_secs(600),
                    )
                    .await?;
                }
                aws.start(&found.id).await?;
                println!("starting    {}", found.id);
            }
        }
        None => {
            let created_profile = aws.ensure_instance_profile().await?;
            let group = aws.ensure_security_group().await?;
            let ami = aws.ubuntu_ami(serial).await?;
            if created_profile {
                println!(
                    "instance profile {} created; waiting for it to propagate",
                    aws::ROLE_NAME
                );
                aws.propagate().await;
            }
            let id = aws.launch(WORKER, &ami, &group, user_data).await?;
            println!("launched    {id} ({ami}, {group})");
        }
    }
    let running = aws
        .wait_for_state(WORKER, InstanceStateName::Running, Duration::from_secs(600))
        .await?;
    if idle_stop {
        aws.ensure_idle_stop_alarm(&running.id).await?;
    }
    Ok((running.id, running.data_volume))
}

async fn up_kubevirt(config: &Config, user_data: &str) -> Result<(String, Option<String>)> {
    let kubevirt = Kubevirt::new(config.kubevirt()?, &config.ubuntu_serial);
    up_kubevirt_with(&kubevirt, user_data).await
}

pub(crate) async fn up_kubevirt_with(
    kubevirt: &impl KubevirtPort,
    user_data: &str,
) -> Result<(String, Option<String>)> {
    match kubevirt.find(WORKER)? {
        Some(vm) => {
            println!(
                "worker      vm/{} ({})",
                kubevirt::vm_name(WORKER),
                vm.printable_status
            );
            if vm.run_strategy != "Always" {
                kubevirt.set_run_strategy(WORKER, "Always")?;
                println!("starting    vm/{}", kubevirt::vm_name(WORKER));
            }
        }
        None => {
            kubevirt.launch(WORKER, user_data)?;
            println!(
                "launched    vm/{} from {}",
                kubevirt::vm_name(WORKER),
                kubevirt.image_url()
            );
        }
    }
    // The first start imports the cloud image into the root volume before the VM boots.
    let vm = kubevirt
        .wait_for_status(WORKER, "Running", Duration::from_secs(40 * 60))
        .await?;
    Ok((vm.uid, Some(format!("{}-data", kubevirt::vm_name(WORKER)))))
}

/// Polls until cloud-init wrote its last file and the worker runs the kernel it converged on.
/// A first boot downloads the pinned prebuilt Substrate release and reboots into that kernel.
async fn wait_for_bootstrap(ssh: &Ssh) -> Result<()> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(60 * 60);
    let mut said = String::new();
    let mut say = |text: &str| {
        if said != text {
            println!("waiting     {text}");
            said = text.to_owned();
        }
    };
    loop {
        if ssh.reachable() {
            let output = ssh.run("cat /var/lib/mantle/bootstrap.json", None)?;
            if output.status.success() {
                let bootstrap: serde_json::Value = serde_json::from_slice(&output.stdout)
                    .context("parsing /var/lib/mantle/bootstrap.json")?;
                let target = bootstrap["kernel"].as_str().unwrap_or_default().to_owned();
                let running = ssh.check("uname -r", None)?.trim().to_owned();
                if running == target {
                    println!("bootstrap   {bootstrap}");
                    return Ok(());
                }
                say(&format!(
                    "for the reboot into kernel {target} (running {running})"
                ));
            } else {
                let status = ssh.run("cloud-init status", None)?;
                if String::from_utf8_lossy(&status.stdout).contains("status: error") {
                    let log = ssh.run("sudo tail -n 40 /var/log/cloud-init-output.log", None)?;
                    bail!(
                        "cloud-init failed on the worker:\n{}",
                        String::from_utf8_lossy(&log.stdout)
                    );
                }
                say("for cloud-init (a first boot downloads pinned prebuilt binaries)");
            }
        } else {
            say("for SSH");
        }
        if tokio::time::Instant::now() > deadline {
            bail!("the worker did not finish bootstrapping within 60 minutes");
        }
        tokio::time::sleep(Duration::from_secs(20)).await;
    }
}

struct WorkerBinaries {
    egress: Vec<u8>,
    launch: Vec<u8>,
    worker: Vec<u8>,
}

fn worker_binaries(dir: &Path) -> Result<WorkerBinaries> {
    let read = |name: &str| {
        let path = dir.join(name);
        mantle_worker::read_delivery_binary(&path)
            .with_context(|| format!("reading {} (run `task build-worker` first)", path.display()))
    };
    Ok(WorkerBinaries {
        egress: read("mantle-egress")?,
        launch: read("mantle-launch")?,
        worker: read("mantle-worker")?,
    })
}

fn install_binaries(ssh: &Ssh, binaries: &WorkerBinaries) -> Result<()> {
    let active = ssh
        .bounded(
            "systemctl is-active --quiet mantle-egress.service",
            None,
            Duration::from_secs(15),
        )?
        .status
        .success();
    let daemon_active = ssh
        .bounded(
            "systemctl is-active --quiet substrate.service",
            None,
            Duration::from_secs(15),
        )?
        .status
        .success();
    let mut pending = Vec::new();
    for (name, bytes) in [
        ("mantle-egress", &binaries.egress),
        ("mantle-launch", &binaries.launch),
        ("mantle-worker", &binaries.worker),
    ] {
        let digest = format!("{:x}", Sha256::digest(bytes));
        let observed = ssh.bounded(
            &format!("sha256sum /opt/mantle/bin/{name}"),
            None,
            Duration::from_secs(15),
        )?;
        let unchanged = observed.status.success()
            && String::from_utf8_lossy(&observed.stdout)
                .split_whitespace()
                .next()
                == Some(digest.as_str());
        if unchanged {
            continue;
        }
        let shared_active =
            (name == "mantle-egress" && active) || (name == "mantle-launch" && daemon_active);
        if mantle_worker::shared_upgrade_deferred(
            shared_active,
            observed.status.success() || (name == "mantle-egress" && active),
            !unchanged,
        ) {
            println!(
                "deferred    {name} upgrade: active shared service retained; schedule maintenance to apply changed bytes"
            );
            continue;
        }
        pending.push((name, bytes, digest));
    }
    for (name, bytes, digest) in pending {
        ssh.checked_bounded(
            &format!(
                "sudo install -d -m 0755 /opt/mantle/bin && sudo tee /opt/mantle/bin/{name}.new >/dev/null \
                 && echo '{digest}  /opt/mantle/bin/{name}.new' | sha256sum -c - >/dev/null \
                 && sudo chmod 0755 /opt/mantle/bin/{name}.new && sudo mv -f /opt/mantle/bin/{name}.new /opt/mantle/bin/{name}"
            ),
            Some(bytes), Duration::from_secs(90),
        )
        .with_context(|| format!("installing {name}"))?;
    }
    ssh.checked_bounded(
        "sudo systemctl enable mantle-egress.service >/dev/null 2>&1 && sudo systemctl start mantle-egress.service \
         && systemctl is-active mantle-egress.service",
        None, Duration::from_secs(30),
    )
    .context("starting mantle-egress")?;
    println!(
        "binaries    reconciled; active shared services retained (deferred upgrades listed above)"
    );
    Ok(())
}

/// Replaces the Claude token atomically. Substrate re-reads the slot file on every exec.
pub fn install_token(ssh: &Ssh, token: &[u8]) -> Result<()> {
    ssh.check(
        &format!(
            "sudo tee {SECRET_PATH}.new >/dev/null && sudo chown substrate:substrate {SECRET_PATH}.new \
             && sudo chmod 0600 {SECRET_PATH}.new && sudo mv -f {SECRET_PATH}.new {SECRET_PATH}"
        ),
        Some(token),
    )
    .context("installing the Claude token on the worker")?;
    Ok(())
}

/// What makes two workers the same machine: image build, OS, kernel and installed versions.
fn report_identity(ssh: &Ssh) -> Result<()> {
    let text = ssh.check(
        "printf 'image %s\\n' \"$(sed -n 's/^serial: *//p' /etc/cloud/build.info)\"; \
         printf 'os %s\\n' \"$(. /etc/os-release && echo \"$PRETTY_NAME\")\"; \
         printf 'kernel %s\\n' \"$(uname -r)\"; \
         printf 'bubblewrap %s\\n' \"$(bwrap --version)\"; \
         printf 'egress %s\\n' \"$(sha256sum /opt/mantle/bin/mantle-egress | cut -c1-16)\"; \
         printf 'launch %s\\n' \"$(sha256sum /opt/mantle/bin/mantle-launch | cut -c1-16)\"; \
         printf 'daemon %s\\n' \"$(sha256sum /usr/local/bin/substrate-daemon | cut -c1-16)\"",
        None,
    )?;
    for line in text.lines() {
        if let Some((key, value)) = line.split_once(' ') {
            println!("{key:<11} {value}");
        }
    }
    let agent = ssh.bounded(
        "sudo /opt/mantle/bin/mantle-worker inspect-codex",
        None,
        Duration::from_secs(190),
    )?;
    if agent.status.success() {
        let installed: Option<mantle_worker::InstallationFacts> =
            serde_json::from_slice(&agent.stdout).context("parsing observed Codex facts")?;
        match installed {
            Some(facts) => println!(
                "codex       {} {} binary={} archive={} (verified installation; authentication unknown)",
                facts.version, facts.architecture, facts.binary_sha256, facts.compressed_sha256
            ),
            None => println!("codex       not installed"),
        }
    } else {
        println!(
            "codex       installation unverified (worker helper absent or inspection refused)"
        );
    }
    Ok(())
}

async fn report_machine(ssh: &Ssh) -> Result<()> {
    let tunnel = ssh.tunnel()?;
    let client = substrate::connect(tunnel.socket()).await?;
    let machine = client.machine();
    println!(
        "substrate   driver {} snapshot {}",
        machine.driver_version, machine.capability_snapshot
    );
    println!(
        "quota       {}",
        if substrate::quota_served(&machine) {
            "workspace storage quota served"
        } else {
            "workspace storage quota NOT served; storage requests are not enforced"
        }
    );
    println!(
        "usage       {}",
        match &machine.facts.exec_resource_usage {
            Some(facts) => format!("served (block_io={})", facts.block_io),
            None => "NOT served; runs report no usage".to_owned(),
        }
    );
    let missing = substrate::missing_facts(&machine);
    if mantle_worker::worker_ready(missing.is_empty(), false, false, false) {
        println!(
            "state       READY (common confinement/toolchain; agent authentication not checked)"
        );
        Ok(())
    } else {
        bail!("worker is not ready; missing facts: {}", missing.join(", "))
    }
}

pub async fn down(config: &Config) -> Result<()> {
    match config.provider {
        Provider::Aws => {
            let aws = Aws::connect(config.aws()?).await;
            down_aws_with(&aws).await?;
        }
        Provider::Kubevirt => {
            let kubevirt = Kubevirt::new(config.kubevirt()?, &config.ubuntu_serial);
            down_kubevirt_with(&kubevirt).await?;
        }
    }
    Ok(())
}

pub async fn status(config: &Config, store: &Store) -> Result<()> {
    let running = match config.provider {
        Provider::Aws => {
            let aws = Aws::connect(config.aws()?).await;
            let Some(found) = aws.find_instance(WORKER).await? else {
                println!("no worker");
                return Ok(());
            };
            println!("worker      {}", found.id);
            println!("state       {}", found.state.as_str());
            println!("type        {}", found.instance_type);
            println!(
                "zone        {}",
                found.availability_zone.as_deref().unwrap_or("?")
            );
            println!(
                "data volume {}",
                found.data_volume.as_deref().unwrap_or("?")
            );
            found.state == InstanceStateName::Running
        }
        Provider::Kubevirt => {
            let kubevirt = Kubevirt::new(config.kubevirt()?, &config.ubuntu_serial);
            let Some(vm) = kubevirt.find(WORKER)? else {
                println!("no worker");
                return Ok(());
            };
            println!("worker      vm/{} ({})", kubevirt::vm_name(WORKER), vm.uid);
            println!("state       {}", vm.printable_status);
            vm.printable_status == "Running"
        }
    };
    if running {
        let record = store
            .worker(WORKER)?
            .context("the worker runs but is not recorded; run `mantle worker up`")?;
        let ssh = ssh_for(config, &record)?;
        report_identity(&ssh)?;
        report_machine(&ssh).await?;
    }
    Ok(())
}

/// A shell on the worker host as `ubuntu`, through the same tunnel Mantle uses.
pub fn ssh(config: &Config, store: &Store) -> Result<()> {
    let record = store
        .worker(WORKER)?
        .context("no worker recorded; run `mantle worker up` first")?;
    let status = ssh_for(config, &record)?.interactive()?;
    if !status.success() {
        bail!("ssh ended {status}");
    }
    Ok(())
}

/// Substitutes the cloud-init template. A placeholder alone on a line is replaced by a whole file,
/// indented to the placeholder's column; a placeholder inside a line is replaced by a value.
fn render_user_data_for(public_key: &str, claude: bool) -> Result<String> {
    let service = SUBSTRATE_SERVICE
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
        );
    let blocks = [
        ("{{SUBSTRATE_SERVICE}}", service.as_str()),
        ("{{EGRESS_SERVICE}}", EGRESS_SERVICE),
        ("{{BWRAP_APPARMOR}}", BWRAP_APPARMOR),
    ];
    let values = [
        ("{{SSH_PUBLIC_KEY}}", public_key),
        ("{{SUBSTRATE_IMAGE_REPOSITORY}}", SUBSTRATE_IMAGE_REPOSITORY),
        ("{{SUBSTRATE_DAEMON_LAYER}}", SUBSTRATE_DAEMON_LAYER),
        ("{{SUBSTRATE_VERSION}}", SUBSTRATE_VERSION),
        ("{{SUBSTRATE_IMAGE_MANIFEST}}", SUBSTRATE_IMAGE_MANIFEST),
        ("{{RUST_CHANNEL}}", RUST_CHANNEL),
        ("{{CLAUDE_CODE_VERSION}}", CLAUDE_CODE_VERSION),
    ];
    let mut out = String::with_capacity(CLOUD_INIT.len() * 2);
    for line in CLOUD_INIT.lines() {
        let trimmed = line.trim();
        if let Some((_, body)) = blocks.iter().find(|(name, _)| *name == trimmed) {
            let indent = &line[..line.len() - line.trim_start().len()];
            for body_line in body.lines() {
                if body_line.is_empty() {
                    out.push('\n');
                } else {
                    out.push_str(indent);
                    out.push_str(body_line);
                    out.push('\n');
                }
            }
            continue;
        }
        let mut rendered = line.to_owned();
        for (name, value) in values {
            rendered = rendered.replace(name, value);
        }
        out.push_str(&rendered);
        out.push('\n');
    }
    if let Some(position) = out.find("{{") {
        let end = out[position..]
            .find("}}")
            .map_or(out.len(), |end| position + end + 2);
        bail!(
            "cloud-init template has an unknown placeholder {}",
            &out[position..end]
        );
    }
    Ok(out)
}

pub(crate) async fn down_aws_with(aws: &impl AwsPort) -> Result<()> {
    let Some(found) = aws.find_instance(WORKER).await? else {
        println!("no worker");
        return Ok(());
    };
    if found.state != InstanceStateName::Stopped {
        aws.stop(&found.id).await?;
        aws.wait_for_state(WORKER, InstanceStateName::Stopped, Duration::from_secs(600))
            .await?;
    }
    println!(
        "worker      {} stopped; data volume {} retained",
        found.id,
        found.data_volume.as_deref().unwrap_or("?")
    );
    Ok(())
}
pub(crate) async fn down_kubevirt_with(kubevirt: &impl KubevirtPort) -> Result<()> {
    if kubevirt.find(WORKER)?.is_none() {
        println!("no worker");
        return Ok(());
    }
    kubevirt.set_run_strategy(WORKER, "Halted")?;
    kubevirt
        .wait_for_status(WORKER, "Stopped", Duration::from_secs(600))
        .await?;
    println!(
        "worker      vm/{} stopped; root and data volumes retained",
        kubevirt::vm_name(WORKER)
    );
    Ok(())
}

pub(crate) trait AwsPort {
    async fn find_instance(&self, worker: &str) -> Result<Option<aws::Instance>>;
    async fn ensure_instance_profile(&self) -> Result<bool>;
    async fn ensure_security_group(&self) -> Result<String>;
    async fn ubuntu_ami(&self, serial: &str) -> Result<String>;
    async fn launch(&self, worker: &str, ami: &str, group: &str, user_data: &str)
    -> Result<String>;
    async fn start(&self, id: &str) -> Result<()>;
    async fn stop(&self, id: &str) -> Result<()>;
    async fn wait_for_state(
        &self,
        worker: &str,
        state: InstanceStateName,
        timeout: Duration,
    ) -> Result<aws::Instance>;
    async fn ensure_idle_stop_alarm(&self, id: &str) -> Result<()>;
    async fn propagate(&self);
}
impl AwsPort for Aws {
    async fn find_instance(&self, worker: &str) -> Result<Option<aws::Instance>> {
        self.find_instance(worker).await
    }
    async fn ensure_instance_profile(&self) -> Result<bool> {
        self.ensure_instance_profile().await
    }
    async fn ensure_security_group(&self) -> Result<String> {
        self.ensure_security_group().await
    }
    async fn ubuntu_ami(&self, serial: &str) -> Result<String> {
        self.ubuntu_ami(serial).await
    }
    async fn launch(
        &self,
        worker: &str,
        ami: &str,
        group: &str,
        user_data: &str,
    ) -> Result<String> {
        self.launch(worker, ami, group, user_data).await
    }
    async fn start(&self, id: &str) -> Result<()> {
        self.start(id).await
    }
    async fn stop(&self, id: &str) -> Result<()> {
        self.stop(id).await
    }
    async fn wait_for_state(
        &self,
        worker: &str,
        state: InstanceStateName,
        timeout: Duration,
    ) -> Result<aws::Instance> {
        self.wait_for_state(worker, state, timeout).await
    }
    async fn ensure_idle_stop_alarm(&self, id: &str) -> Result<()> {
        self.ensure_idle_stop_alarm(id).await
    }
    async fn propagate(&self) {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

pub(crate) trait KubevirtPort {
    fn find(&self, worker: &str) -> Result<Option<kubevirt::Vm>>;
    fn launch(&self, worker: &str, user_data: &str) -> Result<()>;
    fn set_run_strategy(&self, worker: &str, strategy: &str) -> Result<()>;
    fn image_url(&self) -> String;
    async fn wait_for_status(
        &self,
        worker: &str,
        status: &str,
        timeout: Duration,
    ) -> Result<kubevirt::Vm>;
}
impl KubevirtPort for Kubevirt {
    fn find(&self, worker: &str) -> Result<Option<kubevirt::Vm>> {
        self.find(worker)
    }
    fn launch(&self, worker: &str, user_data: &str) -> Result<()> {
        self.launch(worker, user_data)
    }
    fn set_run_strategy(&self, worker: &str, strategy: &str) -> Result<()> {
        self.set_run_strategy(worker, strategy)
    }
    fn image_url(&self) -> String {
        self.image_url()
    }
    async fn wait_for_status(
        &self,
        worker: &str,
        status: &str,
        timeout: Duration,
    ) -> Result<kubevirt::Vm> {
        self.wait_for_status(worker, status, timeout).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_template_renders_every_placeholder() {
        let rendered = render_user_data_for("ssh-ed25519 AAAAtest mantle", false).expect("renders");
        assert!(
            rendered.starts_with("#cloud-config"),
            "first line must be #cloud-config"
        );
        assert!(rendered.contains("ssh-ed25519 AAAAtest mantle"));
        assert!(rendered.contains(SUBSTRATE_DAEMON_LAYER));
        assert!(
            !rendered.contains("cargo build"),
            "a worker must not compile Substrate"
        );
        assert!(!rendered.contains("{{"));
        assert!(!rendered.contains("--secret-slot"));
        assert!(!rendered.contains("ConditionFileNotEmpty"));
        assert!(!rendered.contains("systemctl start substrate.service"));
        assert!(!rendered.contains("systemctl enable substrate.service"));
        assert!(
            rendered.len() <= 16 * 1024,
            "user data is {} bytes",
            rendered.len()
        );
    }

    #[test]
    fn legacy_claude_bootstrap_retains_required_slot() {
        let rendered = render_user_data_for("ssh-ed25519 AAAAtest mantle", true).unwrap();
        assert!(rendered.contains("--secret-slot claude=/var/lib/mantle/secrets/claude"));
        assert!(rendered.contains("ConditionFileNotEmpty=/var/lib/mantle/secrets/claude"));
        assert!(!rendered.contains("{{"));
    }

    #[test]
    fn both_profiles_keep_aperture_and_ca_in_the_daemon_command() {
        for claude in [false, true] {
            let rendered = render_user_data_for("ssh-ed25519 AAAAtest mantle", claude).unwrap();
            let yaml: serde_yaml::Value = serde_yaml::from_str(&rendered).unwrap();
            let service = yaml["write_files"]
                .as_sequence()
                .unwrap()
                .iter()
                .find(|file| file["path"] == "/etc/systemd/system/substrate.service")
                .unwrap()["content"]
                .as_str()
                .unwrap();
            let mut lines = service
                .lines()
                .skip_while(|line| !line.starts_with("ExecStart="));
            let mut line = lines.next().unwrap();
            let mut command = line.to_owned();
            while line.ends_with('\\') {
                line = lines.next().unwrap();
                command.push_str(line);
            }
            assert!(
                command.contains("--egress-aperture egress=127.0.0.1:3128/tcp"),
                "daemon lost aperture after optional slot line"
            );
            assert!(
                command.contains("--ca-bundle /etc/ssl/certs/ca-certificates.crt"),
                "daemon lost CA bundle after optional slot line"
            );
        }
    }
}
