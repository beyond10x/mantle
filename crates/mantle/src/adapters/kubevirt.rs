//! KubeVirt worker provisioning through `kubectl`. The worker is a VirtualMachine booting the same
//! Ubuntu build serial and the same rendered cloud-init as the EC2 worker.

use std::io::Write as _;
use std::process::{Command, Stdio};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use serde_json::{Value, json};

use crate::config::KubevirtConfig;

const MANAGED: &str = "mantle.beyond10x.dev/managed";
const WORKER_LABEL: &str = "mantle.beyond10x.dev/worker";

pub struct Kubevirt {
    config: KubevirtConfig,
    serial: String,
}

#[derive(Debug, Clone)]
pub struct Vm {
    pub uid: String,
    pub run_strategy: String,
    pub printable_status: String,
}

pub fn vm_name(worker: &str) -> String {
    format!("mantle-{worker}")
}

impl Kubevirt {
    pub fn new(config: &KubevirtConfig, serial: &str) -> Self {
        Self {
            config: config.clone(),
            serial: serial.to_owned(),
        }
    }

    fn kubectl(&self) -> Command {
        let mut command = Command::new("kubectl");
        command.args([
            "--context",
            &self.config.context,
            "-n",
            &self.config.namespace,
        ]);
        command
    }

    fn run(&self, args: &[&str], stdin: Option<&[u8]>) -> Result<String> {
        let mut command = self.kubectl();
        command
            .args(args)
            .stdin(if stdin.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn().context("starting kubectl")?;
        if let Some(bytes) = stdin {
            child
                .stdin
                .take()
                .context("kubectl stdin")?
                .write_all(bytes)
                .context("writing to kubectl")?;
        }
        let output = child.wait_with_output().context("waiting for kubectl")?;
        if !output.status.success() {
            bail!(
                "kubectl {} failed ({}): {}",
                args.join(" "),
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    fn apply(&self, manifest: &Value) -> Result<()> {
        let bytes = serde_json::to_vec(manifest)?;
        self.run(&["apply", "-f", "-"], Some(&bytes))?;
        Ok(())
    }

    pub fn ensure_namespace(&self) -> Result<()> {
        let mut command = Command::new("kubectl");
        command.args(["--context", &self.config.context]);
        let manifest = json!({
            "apiVersion": "v1",
            "kind": "Namespace",
            "metadata": {"name": self.config.namespace, "labels": {MANAGED: "true"}},
        });
        let mut child = command
            .args(["apply", "-f", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .context("starting kubectl")?;
        child
            .stdin
            .take()
            .context("kubectl stdin")?
            .write_all(&serde_json::to_vec(&manifest)?)?;
        let output = child.wait_with_output()?;
        if !output.status.success() {
            bail!(
                "creating namespace {}: {}",
                self.config.namespace,
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        Ok(())
    }

    pub fn find(&self, worker: &str) -> Result<Option<Vm>> {
        let text = self.run(
            &[
                "get",
                "virtualmachine",
                &vm_name(worker),
                "--ignore-not-found",
                "-o",
                "json",
            ],
            None,
        )?;
        if text.trim().is_empty() {
            return Ok(None);
        }
        let vm: Value = serde_json::from_str(&text).context("parsing the VirtualMachine")?;
        Ok(Some(Vm {
            uid: vm["metadata"]["uid"]
                .as_str()
                .unwrap_or_default()
                .to_owned(),
            run_strategy: vm["spec"]["runStrategy"]
                .as_str()
                .unwrap_or_default()
                .to_owned(),
            printable_status: vm["status"]["printableStatus"]
                .as_str()
                .unwrap_or("Unknown")
                .to_owned(),
        }))
    }

    pub fn image_url(&self) -> String {
        format!(
            "https://cloud-images.ubuntu.com/releases/noble/release-{}/ubuntu-24.04-server-cloudimg-amd64.img",
            self.serial
        )
    }

    pub fn launch(&self, worker: &str, user_data: &str) -> Result<()> {
        self.ensure_namespace()?;
        let name = vm_name(worker);
        let labels = json!({MANAGED: "true", WORKER_LABEL: worker});
        let secret = format!("{name}-cloud-init");
        self.apply(&json!({
            "apiVersion": "v1",
            "kind": "Secret",
            "metadata": {"name": secret, "labels": labels},
            "type": "Opaque",
            "stringData": {"userdata": user_data},
        }))?;
        let root = format!("{name}-root");
        let data = format!("{name}-data");
        self.apply(&json!({
            "apiVersion": "kubevirt.io/v1",
            "kind": "VirtualMachine",
            "metadata": {"name": name, "labels": labels},
            "spec": {
                "runStrategy": "Always",
                "dataVolumeTemplates": [
                    {
                        "metadata": {"name": root, "labels": labels},
                        "spec": {
                            "source": {"http": {"url": self.image_url()}},
                            "storage": {"resources": {"requests": {"storage": format!("{}Gi", self.config.root_disk_gib)}}},
                        },
                    },
                    {
                        "metadata": {"name": data, "labels": labels},
                        "spec": {
                            "source": {"blank": {}},
                            "storage": {"resources": {"requests": {"storage": format!("{}Gi", self.config.data_disk_gib)}}},
                        },
                    },
                ],
                "template": {
                    "metadata": {"labels": labels},
                    "spec": {
                        "domain": {
                            "cpu": {"cores": self.config.cpu},
                            "memory": {"guest": format!("{}Gi", self.config.memory_gib)},
                            "devices": {
                                "disks": [
                                    {"name": "root", "disk": {"bus": "virtio"}},
                                    {"name": "data", "disk": {"bus": "virtio"}, "serial": "mantle-data"},
                                    // A cdrom, so the guest sees exactly one data disk besides root.
                                    {"name": "cloudinit", "cdrom": {"bus": "sata"}},
                                ],
                                "interfaces": [{"name": "default", "masquerade": {}}],
                            },
                        },
                        "networks": [{"name": "default", "pod": {}}],
                        "volumes": [
                            {"name": "root", "dataVolume": {"name": root}},
                            {"name": "data", "dataVolume": {"name": data}},
                            {"name": "cloudinit", "cloudInitNoCloud": {"secretRef": {"name": secret}}},
                        ],
                    },
                },
            },
        }))
    }

    pub fn set_run_strategy(&self, worker: &str, strategy: &str) -> Result<()> {
        let patch = json!({"spec": {"runStrategy": strategy}}).to_string();
        self.run(
            &[
                "patch",
                "virtualmachine",
                &vm_name(worker),
                "--type",
                "merge",
                "-p",
                &patch,
            ],
            None,
        )?;
        Ok(())
    }

    /// Waits for the VirtualMachine's printable status. Image import happens before `Running`.
    pub async fn wait_for_status(
        &self,
        worker: &str,
        wanted: &str,
        timeout: Duration,
    ) -> Result<Vm> {
        let deadline = tokio::time::Instant::now() + timeout;
        let mut last = String::new();
        loop {
            if let Some(vm) = self.find(worker)? {
                if vm.printable_status == wanted {
                    return Ok(vm);
                }
                if vm.printable_status != last {
                    println!("vm          {}", vm.printable_status);
                    last.clone_from(&vm.printable_status);
                }
            }
            if tokio::time::Instant::now() > deadline {
                bail!(
                    "the VM did not reach {wanted} within {}s (last: {last})",
                    timeout.as_secs()
                );
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    /// `virtctl` tunnels port 22 of the VM over the Kubernetes API, as SSM does for EC2.
    pub fn proxy_command(&self, worker: &str) -> String {
        format!(
            "virtctl --context {} port-forward --stdio=true vmi/{}/{} %p",
            self.config.context,
            vm_name(worker),
            self.config.namespace
        )
    }
}
