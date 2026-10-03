use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

/// The operator's configuration. It holds every cloud-account identifier Mantle uses, which is why
/// it lives outside the repository.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub provider: Provider,
    /// One Ubuntu 24.04 build serial for every provider, so workers boot the same image build.
    pub ubuntu_serial: String,
    pub aws: Option<AwsConfig>,
    pub kubevirt: Option<KubevirtConfig>,
    pub claude: Option<ClaudeConfig>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Aws,
    Kubevirt,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KubevirtConfig {
    pub context: String,
    pub namespace: String,
    pub cpu: u32,
    pub memory_gib: u32,
    pub root_disk_gib: u32,
    pub data_disk_gib: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AwsConfig {
    pub profile: String,
    pub region: String,
    pub vpc: String,
    pub subnet: String,
    #[serde(default = "default_instance_type")]
    pub instance_type: String,
    #[serde(default = "default_root_volume")]
    pub root_volume_gib: i32,
    #[serde(default = "default_data_volume")]
    pub data_volume_gib: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaudeConfig {
    /// argv of a command that prints the token, such as a keyring lookup. Preferred.
    pub token_command: Option<Vec<String>>,
    /// A file holding the token, for hosts without a keyring.
    pub token_file: Option<String>,
}

fn default_instance_type() -> String {
    "m7i.4xlarge".to_owned()
}

const fn default_root_volume() -> i32 {
    50
}

const fn default_data_volume() -> i32 {
    200
}

impl Config {
    pub fn load(selection: &crate::profile::Selection) -> Result<Self> {
        let text = selection.read_config()?;
        Self::parse(&text)
    }

    pub fn parse(text: &str) -> Result<Self> {
        let config: Self = toml::from_str(text)
            .map_err(|_| anyhow::anyhow!("configuration has malformed or unsupported fields"))?;
        if config.ubuntu_serial.len() != 8
            || !config
                .ubuntu_serial
                .bytes()
                .all(|byte| byte.is_ascii_digit())
        {
            bail!("ubuntu_serial must be an eight-digit build serial such as 20260926");
        }
        match config.provider {
            Provider::Aws if config.aws.is_none() => {
                bail!("provider = \"aws\" needs an [aws] table")
            }
            Provider::Kubevirt if config.kubevirt.is_none() => {
                bail!("provider = \"kubevirt\" needs a [kubevirt] table")
            }
            _ => {}
        }
        Ok(config)
    }

    pub fn aws(&self) -> Result<&AwsConfig> {
        self.aws
            .as_ref()
            .context("no [aws] table in the configuration")
    }

    pub fn kubevirt(&self) -> Result<&KubevirtConfig> {
        self.kubevirt
            .as_ref()
            .context("no [kubevirt] table in the configuration")
    }

    /// The Claude Code OAuth token, from the configured command or file. Never logged; an error
    /// names where the token was looked for, not what was found.
    pub fn claude_token(&self) -> Result<Vec<u8>> {
        let claude = self.claude.as_ref().context(
            "selected Claude needs a [claude] configuration with token_command or token_file",
        )?;
        let (bytes, source) = match (&claude.token_command, &claude.token_file) {
            (Some(argv), _) => {
                let (program, args) = argv
                    .split_first()
                    .context("claude.token_command is empty")?;
                let output = std::process::Command::new(program)
                    .args(args)
                    .stdin(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .output()
                    .with_context(|| format!("running claude.token_command ({program})"))?;
                if !output.status.success() {
                    bail!(
                        "claude.token_command ({program}) exited {}; store the token from `claude setup-token` first",
                        output.status
                    );
                }
                (output.stdout, format!("claude.token_command ({program})"))
            }
            (None, Some(file)) => {
                let path = expand_home(file)?;
                let bytes = std::fs::read(&path).with_context(|| {
                    format!(
                        "reading the Claude token file {} (create it from `claude setup-token`)",
                        path.display()
                    )
                })?;
                (bytes, path.display().to_string())
            }
            (None, None) => bail!("[claude] needs token_command or token_file"),
        };
        let token = String::from_utf8(bytes)
            .with_context(|| format!("the token from {source} is not UTF-8"))?;
        let token = token.trim();
        if token.is_empty() {
            bail!("{source} gave an empty token");
        }
        Ok(token.as_bytes().to_vec())
    }
}

fn home() -> Result<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .context("HOME is not set")
}

pub fn expand_home(path: &str) -> Result<PathBuf> {
    match path.strip_prefix("~/") {
        Some(rest) => Ok(home()?.join(rest)),
        None => Ok(PathBuf::from(path)),
    }
}

/// Owner-private state: the SQLite database, the SSH key, known hosts and forwarded sockets.
#[cfg(test)]
pub fn state_dir() -> Result<PathBuf> {
    let dir = configured_path(
        std::env::var_os("MANTLE_STATE_DIR"),
        home()?.join(".local/state/mantle"),
    )?;
    ensure_private_dir(&dir)?;
    Ok(dir)
}

#[cfg(test)]
fn configured_path(override_path: Option<std::ffi::OsString>, default: PathBuf) -> Result<PathBuf> {
    let path = override_path.map_or(default, PathBuf::from);
    if !path.is_absolute() {
        bail!("Mantle configuration and state paths must be absolute");
    }
    Ok(path)
}

pub fn ensure_private_dir(dir: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))
        .with_context(|| format!("restricting {}", dir.display()))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn common_worker_configuration_needs_no_claude_credentials() {
        let config: Config = toml::from_str("provider = 'kubevirt'\nubuntu_serial = '20260926'\n[kubevirt]\ncontext = 'test'\nnamespace = 'mantle'\ncpu = 4\nmemory_gib = 8\nroot_disk_gib = 20\ndata_disk_gib = 20\n").expect("credential-independent configuration");
        assert!(
            config.claude_token().is_err(),
            "selected Claude still requires credentials"
        );
    }

    #[test]
    fn explicit_path_selection_is_absolute_and_preserves_defaults() {
        let default = PathBuf::from("/private/default");
        assert_eq!(configured_path(None, default.clone()).unwrap(), default);
        assert_eq!(
            configured_path(Some("/private/isolated".into()), default.clone()).unwrap(),
            PathBuf::from("/private/isolated")
        );
        assert!(configured_path(Some("relative".into()), default).is_err());
    }

    #[test]
    fn legacy_claude_table_remains_parseable() {
        let config: Config = toml::from_str(include_str!("../../../examples/config.toml")).unwrap();
        assert!(config.claude.unwrap().token_command.is_some());
    }
}

#[derive(Debug)]
pub struct RuntimeContext {
    pub config: Config,
    pub selection: crate::profile::Selection,
}
impl std::ops::Deref for RuntimeContext {
    type Target = Config;
    fn deref(&self) -> &Config {
        &self.config
    }
}
