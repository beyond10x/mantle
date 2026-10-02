//! AWS EC2 worker provisioning. Every resource carries `mantle:managed=true`, and every call is
//! idempotent: a second `up` finds what the first created.

use std::time::Duration;

use anyhow::{Context, Result, bail};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_cloudwatch::types::{ComparisonOperator, Dimension, Statistic};
use aws_sdk_ec2::types::{
    BlockDeviceMapping, EbsBlockDevice, Filter, HttpTokensState, IamInstanceProfileSpecification,
    InstanceMetadataEndpointState, InstanceMetadataOptionsRequest, InstanceStateName, InstanceType,
    ResourceType, Tag, TagSpecification, VolumeType,
};

use crate::config::AwsConfig;

pub const ROLE_NAME: &str = "mantle-worker";
pub const SECURITY_GROUP_NAME: &str = "mantle-worker";
const SSM_POLICY: &str = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore";
const UBUNTU_OWNER: &str = "099720109477";
const UBUNTU_NAME_PREFIX: &str = "ubuntu/images/hvm-ssd-gp3/ubuntu-noble-24.04-amd64-server-";
pub const DATA_DEVICE: &str = "/dev/sdf";
const USER_DATA_LIMIT: usize = 16 * 1024;

const TRUST_POLICY: &str = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"Service":"ec2.amazonaws.com"},"Action":"sts:AssumeRole"}]}"#;

pub struct Aws {
    config: AwsConfig,
    ec2: aws_sdk_ec2::Client,
    iam: aws_sdk_iam::Client,
    cloudwatch: aws_sdk_cloudwatch::Client,
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub id: String,
    pub state: InstanceStateName,
    pub instance_type: String,
    pub data_volume: Option<String>,
    pub availability_zone: Option<String>,
}

impl Aws {
    pub async fn connect(config: &AwsConfig) -> Self {
        let shared = aws_config::defaults(BehaviorVersion::latest())
            .profile_name(&config.profile)
            .region(Region::new(config.region.clone()))
            .load()
            .await;
        Self {
            config: config.clone(),
            ec2: aws_sdk_ec2::Client::new(&shared),
            iam: aws_sdk_iam::Client::new(&shared),
            cloudwatch: aws_sdk_cloudwatch::Client::new(&shared),
        }
    }

    /// The role and instance profile that give the worker SSM and nothing else. Returns whether
    /// the instance profile was created now, because EC2 cannot see a new profile for a while.
    pub async fn ensure_instance_profile(&self) -> Result<bool> {
        match self.iam.get_role().role_name(ROLE_NAME).send().await {
            Ok(_) => {}
            Err(error)
                if error
                    .as_service_error()
                    .is_some_and(|e| e.is_no_such_entity_exception()) =>
            {
                self.iam
                    .create_role()
                    .role_name(ROLE_NAME)
                    .assume_role_policy_document(TRUST_POLICY)
                    .description("Mantle worker: SSM only")
                    .tags(iam_tag("mantle:managed", "true")?)
                    .send()
                    .await
                    .context("creating the worker role")?;
            }
            Err(error) => return Err(error).context("reading the worker role"),
        }
        self.iam
            .attach_role_policy()
            .role_name(ROLE_NAME)
            .policy_arn(SSM_POLICY)
            .send()
            .await
            .context("attaching the SSM policy")?;
        match self
            .iam
            .get_instance_profile()
            .instance_profile_name(ROLE_NAME)
            .send()
            .await
        {
            Ok(found) => {
                let has_role = found.instance_profile().is_some_and(|profile| {
                    profile
                        .roles()
                        .iter()
                        .any(|role| role.role_name() == ROLE_NAME)
                });
                if !has_role {
                    self.add_role_to_profile().await?;
                    return Ok(true);
                }
                Ok(false)
            }
            Err(error)
                if error
                    .as_service_error()
                    .is_some_and(|e| e.is_no_such_entity_exception()) =>
            {
                self.iam
                    .create_instance_profile()
                    .instance_profile_name(ROLE_NAME)
                    .tags(iam_tag("mantle:managed", "true")?)
                    .send()
                    .await
                    .context("creating the instance profile")?;
                self.add_role_to_profile().await?;
                Ok(true)
            }
            Err(error) => Err(error).context("reading the instance profile"),
        }
    }

    async fn add_role_to_profile(&self) -> Result<()> {
        self.iam
            .add_role_to_instance_profile()
            .instance_profile_name(ROLE_NAME)
            .role_name(ROLE_NAME)
            .send()
            .await
            .context("adding the role to the instance profile")?;
        Ok(())
    }

    /// A security group with no ingress rule. EC2 gives a new group one egress rule that allows
    /// everything, which the host needs for SSM, packages and the egress gateway.
    pub async fn ensure_security_group(&self) -> Result<String> {
        let found = self
            .ec2
            .describe_security_groups()
            .filters(filter("group-name", SECURITY_GROUP_NAME))
            .filters(filter("vpc-id", &self.config.vpc))
            .send()
            .await
            .context("listing security groups")?;
        if let Some(group) = found.security_groups().first() {
            if !group.ip_permissions().is_empty() {
                bail!(
                    "security group {} has ingress rules; the worker must have none",
                    group.group_id().unwrap_or("?")
                );
            }
            return group
                .group_id()
                .map(str::to_owned)
                .context("security group without an id");
        }
        let created = self
            .ec2
            .create_security_group()
            .group_name(SECURITY_GROUP_NAME)
            .description("Mantle worker: no ingress, reached through SSM")
            .vpc_id(&self.config.vpc)
            .tag_specifications(tags(
                ResourceType::SecurityGroup,
                &[("mantle:managed", "true")],
            ))
            .send()
            .await
            .context("creating the security group")?;
        created
            .group_id()
            .map(str::to_owned)
            .context("created security group has no id")
    }

    pub async fn find_instance(&self, worker: &str) -> Result<Option<Instance>> {
        let found = self
            .ec2
            .describe_instances()
            .filters(filter("tag:mantle:managed", "true"))
            .filters(filter("tag:mantle:worker", worker))
            .filters(
                Filter::builder()
                    .name("instance-state-name")
                    .values("pending")
                    .values("running")
                    .values("stopping")
                    .values("stopped")
                    .build(),
            )
            .send()
            .await
            .context("listing instances")?;
        let instances: Vec<Instance> = found
            .reservations()
            .iter()
            .flat_map(|reservation| reservation.instances())
            .map(|instance| Instance {
                id: instance.instance_id().unwrap_or_default().to_owned(),
                state: instance
                    .state()
                    .and_then(|state| state.name())
                    .cloned()
                    .unwrap_or(InstanceStateName::Pending),
                instance_type: instance
                    .instance_type()
                    .map(|kind| kind.as_str().to_owned())
                    .unwrap_or_default(),
                data_volume: instance
                    .block_device_mappings()
                    .iter()
                    .find(|mapping| mapping.device_name() == Some(DATA_DEVICE))
                    .and_then(|mapping| mapping.ebs())
                    .and_then(|ebs| ebs.volume_id())
                    .map(str::to_owned),
                availability_zone: instance
                    .placement()
                    .and_then(|placement| placement.availability_zone())
                    .map(str::to_owned),
            })
            .collect();
        if instances.len() > 1 {
            bail!(
                "{} instances carry mantle:worker={worker}: {}",
                instances.len(),
                instances
                    .iter()
                    .map(|i| i.id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        Ok(instances.into_iter().next())
    }

    /// The AMI Canonical built from the given Ubuntu build serial, the same build the KubeVirt
    /// provider imports from cloud-images.ubuntu.com.
    pub async fn ubuntu_ami(&self, serial: &str) -> Result<String> {
        let name = format!("{UBUNTU_NAME_PREFIX}{serial}");
        let images = self
            .ec2
            .describe_images()
            .owners(UBUNTU_OWNER)
            .filters(filter("name", &name))
            .filters(filter("state", "available"))
            .send()
            .await
            .context("listing Ubuntu images")?;
        images
            .images()
            .first()
            .and_then(|image| image.image_id())
            .map(str::to_owned)
            .with_context(|| format!("no AMI named {name} in {}", self.config.region))
    }

    pub async fn launch(
        &self,
        worker: &str,
        ami: &str,
        security_group: &str,
        user_data: &str,
    ) -> Result<String> {
        if user_data.len() > USER_DATA_LIMIT {
            bail!(
                "rendered user data is {} bytes; EC2 accepts at most {USER_DATA_LIMIT}",
                user_data.len()
            );
        }
        use base64::Engine as _;
        let encoded = base64::engine::general_purpose::STANDARD.encode(user_data);
        let common = [("mantle:managed", "true"), ("mantle:worker", worker)];
        let name = format!("mantle-{worker}");
        let mut instance_tags = common.to_vec();
        instance_tags.push(("Name", name.as_str()));
        // A new instance profile is invisible to EC2 for some seconds; that refusal is retried.
        let mut attempt = 0;
        loop {
            attempt += 1;
            let result = self
                .ec2
                .run_instances()
                .image_id(ami)
                .instance_type(InstanceType::from(self.config.instance_type.as_str()))
                .min_count(1)
                .max_count(1)
                .subnet_id(&self.config.subnet)
                .security_group_ids(security_group)
                .iam_instance_profile(
                    IamInstanceProfileSpecification::builder()
                        .name(ROLE_NAME)
                        .build(),
                )
                .block_device_mappings(volume("/dev/sda1", self.config.root_volume_gib, true))
                .block_device_mappings(volume(DATA_DEVICE, self.config.data_volume_gib, false))
                .metadata_options(
                    InstanceMetadataOptionsRequest::builder()
                        .http_tokens(HttpTokensState::Required)
                        .http_put_response_hop_limit(1)
                        .http_endpoint(InstanceMetadataEndpointState::Enabled)
                        .build(),
                )
                .user_data(&encoded)
                .tag_specifications(tags(ResourceType::Instance, &instance_tags))
                .tag_specifications(tags(ResourceType::Volume, &common))
                .send()
                .await;
            match result {
                Ok(output) => {
                    return output
                        .instances()
                        .first()
                        .and_then(|instance| instance.instance_id())
                        .map(str::to_owned)
                        .context("RunInstances returned no instance");
                }
                Err(error) => {
                    let message = format!("{}", aws_sdk_ec2::error::DisplayErrorContext(&error));
                    if attempt < 12 && message.contains("Invalid IAM Instance Profile") {
                        tokio::time::sleep(Duration::from_secs(10)).await;
                        continue;
                    }
                    bail!("launching the worker: {message}");
                }
            }
        }
    }

    pub async fn start(&self, instance: &str) -> Result<()> {
        self.ec2
            .start_instances()
            .instance_ids(instance)
            .send()
            .await
            .context("starting the instance")?;
        Ok(())
    }

    pub async fn stop(&self, instance: &str) -> Result<()> {
        self.ec2
            .stop_instances()
            .instance_ids(instance)
            .send()
            .await
            .context("stopping the instance")?;
        Ok(())
    }

    pub async fn wait_for_state(
        &self,
        worker: &str,
        wanted: InstanceStateName,
        timeout: Duration,
    ) -> Result<Instance> {
        let deadline = tokio::time::Instant::now() + timeout;
        loop {
            if let Some(instance) = self.find_instance(worker).await?
                && instance.state == wanted
            {
                return Ok(instance);
            }
            if tokio::time::Instant::now() > deadline {
                bail!(
                    "the worker did not reach {} within {}s",
                    wanted.as_str(),
                    timeout.as_secs()
                );
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    /// Stops the instance after two hours below 2% average CPU. The data volume survives a stop.
    pub async fn ensure_idle_stop_alarm(&self, instance: &str) -> Result<()> {
        self.cloudwatch
            .put_metric_alarm()
            .alarm_name(format!("mantle-{instance}-idle-stop"))
            .alarm_description("Mantle: stop an idle worker")
            .namespace("AWS/EC2")
            .metric_name("CPUUtilization")
            .dimensions(
                Dimension::builder()
                    .name("InstanceId")
                    .value(instance)
                    .build(),
            )
            .statistic(Statistic::Average)
            .period(300)
            .evaluation_periods(24)
            .threshold(2.0)
            .comparison_operator(ComparisonOperator::LessThanThreshold)
            .treat_missing_data("notBreaching")
            .alarm_actions(format!("arn:aws:automate:{}:ec2:stop", self.config.region))
            .send()
            .await
            .context("creating the idle-stop alarm")?;
        Ok(())
    }
}

fn filter(name: &str, value: &str) -> Filter {
    Filter::builder().name(name).values(value).build()
}

fn tags(resource: ResourceType, pairs: &[(&str, &str)]) -> TagSpecification {
    let mut builder = TagSpecification::builder().resource_type(resource);
    for (key, value) in pairs {
        builder = builder.tags(Tag::builder().key(*key).value(*value).build());
    }
    builder.build()
}

fn iam_tag(key: &str, value: &str) -> Result<aws_sdk_iam::types::Tag> {
    aws_sdk_iam::types::Tag::builder()
        .key(key)
        .value(value)
        .build()
        .context("building an IAM tag")
}

fn volume(device: &str, size_gib: i32, delete_on_termination: bool) -> BlockDeviceMapping {
    BlockDeviceMapping::builder()
        .device_name(device)
        .ebs(
            EbsBlockDevice::builder()
                .volume_size(size_gib)
                .volume_type(VolumeType::Gp3)
                .encrypted(true)
                .delete_on_termination(delete_on_termination)
                .build(),
        )
        .build()
}

/// SSM carries SSH to the instance; `%h` is the instance id.
pub fn proxy_command(config: &AwsConfig) -> String {
    format!(
        "aws ssm start-session --target %h --document-name AWS-StartSSHSession \
         --parameters portNumber=%p --profile {} --region {}",
        config.profile, config.region
    )
}
