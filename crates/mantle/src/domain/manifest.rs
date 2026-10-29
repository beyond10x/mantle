//! The session manifest (`mantle.yaml`), the subset the first slice serves.
//!
//! Fields the design declares and the slice does not serve (`peers`, `caches`, `setup`,
//! per-session `network.capabilities`) are refused by name rather than ignored, so a manifest never
//! asks for something it silently does not get.

use std::collections::BTreeSet;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use sha2::{Digest, Sha256};

pub const API_VERSION: &str = "mantle.beyond10x.dev/v1alpha1";

#[allow(
    dead_code,
    reason = "placement is accepted for manifest compatibility; the slice has one worker"
)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Manifest {
    pub api_version: String,
    pub kind: String,
    pub metadata: Metadata,
    #[serde(default)]
    pub placement: Option<Placement>,
    #[serde(default)]
    pub runtime: Option<Runtime>,
    pub workspace: Workspace,
    pub agent: Agent,
    #[serde(default)]
    pub resources: Resources,
    #[serde(default)]
    pub network: Option<Network>,
    #[serde(default)]
    pub lifecycle: Lifecycle,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub name: String,
}

#[allow(dead_code, reason = "accepted, not acted on: the slice has one worker")]
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Placement {
    pub class: String,
    #[serde(default)]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Runtime {
    pub profile: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Workspace {
    #[serde(default)]
    pub root: Option<String>,
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Repository {
    pub name: String,
    pub repository: String,
    #[serde(rename = "ref")]
    pub reference: String,
    pub mount: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Agent {
    pub kind: String,
    pub cwd: String,
    pub auth: Option<String>,
    /// The `command` kind's process, run by the launcher without a shell.
    #[serde(default)]
    pub argv: Option<Vec<String>>,
}

/// Why a manifest's agent mapping pairs kind, argv and auth illegally. Each is an ESS error of
/// `mantle.manifest.ValidateAgent`; the message is the manifest diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentRefusal {
    CommandRequiresArgv,
    CommandRefusesAuth,
    InvalidArgv,
    ArgvOutOfBounds,
    ArgvRequiresCommand,
}

/// Substrate's exec admission bounds on an argv at the pinned revision `65304edf` (0.7.10). The
/// `b10x-substrate-wire` crate does not export them: they are literals in
/// `crates/substrate-daemon/src/app/operations.rs` (`validate_exec_input`) and
/// `crates/substrate-host/src/process.rs` (`exec.argv-invalid`). Both also refuse an empty
/// element and an element containing NUL.
pub const SUBSTRATE_MAX_ARGV_ELEMENTS: usize = 256;
pub const SUBSTRATE_MAX_ARGV_ELEMENT_BYTES: usize = 4096;
/// The elements `command_argv` in `app/session.rs` places before the declared argv.
pub const COMMAND_LAUNCHER_PREFIX_ELEMENTS: usize = 13;
/// The most elements a declared `agent.argv` may have so the exec argv stays within Substrate's.
pub const MAX_COMMAND_ARGV_ELEMENTS: usize =
    SUBSTRATE_MAX_ARGV_ELEMENTS - COMMAND_LAUNCHER_PREFIX_ELEMENTS;

impl std::fmt::Display for AgentRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::CommandRequiresArgv => "agent.kind command requires agent.argv",
            Self::CommandRefusesAuth => {
                "agent.auth is refused with agent.kind command: a command session receives no agent credential"
            }
            Self::InvalidArgv => {
                "agent.argv must be a non-empty list whose first element is an absolute path"
            }
            Self::ArgvOutOfBounds => {
                return write!(
                    formatter,
                    "agent.argv must have at most {MAX_COMMAND_ARGV_ELEMENTS} elements, each non-empty, at most {SUBSTRATE_MAX_ARGV_ELEMENT_BYTES} bytes and without NUL"
                );
            }
            Self::ArgvRequiresCommand => "agent.argv is served only with agent.kind command",
        };
        formatter.write_str(text)
    }
}

impl std::error::Error for AgentRefusal {}

/// The command kind names its program by absolute path and carries no credential; no other kind
/// takes an argv. Refusals apply in the order the specification lists them.
pub fn validate_agent(
    kind: &str,
    auth: Option<&str>,
    argv: Option<&[String]>,
) -> std::result::Result<(), AgentRefusal> {
    if kind != super::session::COMMAND {
        return match argv {
            Some(_) => Err(AgentRefusal::ArgvRequiresCommand),
            None => Ok(()),
        };
    }
    let argv = argv.ok_or(AgentRefusal::CommandRequiresArgv)?;
    if auth.is_some() {
        return Err(AgentRefusal::CommandRefusesAuth);
    }
    match argv.first() {
        Some(program) if program.starts_with('/') => {}
        _ => return Err(AgentRefusal::InvalidArgv),
    }
    if argv.len() > MAX_COMMAND_ARGV_ELEMENTS
        || argv.iter().any(|element| {
            element.is_empty()
                || element.contains('\0')
                || element.len() > SUBSTRATE_MAX_ARGV_ELEMENT_BYTES
        })
    {
        return Err(AgentRefusal::ArgvOutOfBounds);
    }
    Ok(())
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Resources {
    pub cpu: Option<u32>,
    pub memory: Option<String>,
    pub pids: Option<u32>,
    pub storage: Option<String>,
}

#[allow(
    dead_code,
    reason = "profile is accepted; the gateway allowlist is fixed in the slice"
)]
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Network {
    #[serde(default)]
    pub default: Option<String>,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

#[allow(
    dead_code,
    reason = "idleAfter is accepted; idle handling is the worker alarm in the slice"
)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Lifecycle {
    pub detach_keeps_running: Option<bool>,
    pub idle_after: Option<String>,
    pub retain_for: Option<String>,
}

/// The slice's fixed gateway allowlist, as manifest capabilities. A manifest may name a subset;
/// it cannot widen it.
pub const SERVED_CAPABILITIES: [&str; 3] = ["model.anthropic", "git.github", "rust.crates"];

/// What the slice runs, after every default is applied and every value bounded.
#[derive(Debug, Clone, PartialEq)]
pub struct Resolved {
    pub name: String,
    pub repositories: Vec<Repository>,
    pub agent_cwd: String,
    pub agent_kind: super::session::AgentKind,
    pub authentication: super::session::AuthenticationMethod,
    /// Present only for the `command` kind.
    pub agent_argv: Option<Vec<String>>,
    pub cpu: u32,
    pub memory_bytes: u64,
    pub pids: u32,
    pub storage_bytes: Option<u64>,
    pub retain_for: Duration,
    pub digest: String,
}

impl PartialEq for Repository {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.repository == other.repository
            && self.reference == other.reference
            && self.mount == other.mount
    }
}
impl Eq for Repository {}

pub const MAX_RETAIN: Duration = Duration::from_hours(24);
pub const DEFAULT_RETAIN: Duration = Duration::from_hours(8);
const MAX_PIDS: u32 = 4_096;

pub fn parse(text: &str) -> Result<Resolved> {
    let manifest: Manifest = serde_yaml::from_str(text).context("parsing the manifest")?;
    resolve(manifest, text)
}

fn resolve(manifest: Manifest, text: &str) -> Result<Resolved> {
    if manifest.api_version != API_VERSION {
        bail!(
            "apiVersion {:?} is not served; this release serves {API_VERSION}",
            manifest.api_version
        );
    }
    if manifest.kind != "Session" {
        bail!("kind {:?} is not served; expected Session", manifest.kind);
    }
    let name = manifest.metadata.name;
    if !valid_name(&name) {
        bail!("metadata.name {name:?} must be 1-48 characters of a-z, 0-9 and '-'");
    }
    validate_agent(
        &manifest.agent.kind,
        manifest.agent.auth.as_deref(),
        manifest.agent.argv.as_deref(),
    )?;
    let (agent_kind, authentication) =
        super::session::resolve_identity(&manifest.agent.kind, manifest.agent.auth.as_deref())?;
    let agent_argv = manifest.agent.argv;
    if let Some(runtime) = &manifest.runtime
        && runtime.profile != "rust-dev"
    {
        bail!(
            "runtime.profile {:?} is not served; the slice serves rust-dev",
            runtime.profile
        );
    }
    if let Some(root) = &manifest.workspace.root
        && root != "/workspace"
    {
        bail!("workspace.root must be /workspace");
    }
    if let Some(network) = &manifest.network {
        if network
            .default
            .as_deref()
            .is_some_and(|value| value != "deny")
        {
            bail!("network.default must be deny");
        }
        for capability in &network.capabilities {
            if !SERVED_CAPABILITIES.contains(&capability.as_str()) {
                bail!(
                    "network capability {capability:?} is not served; the slice serves {}",
                    SERVED_CAPABILITIES.join(", ")
                );
            }
        }
    }
    let repositories = manifest.workspace.repositories;
    if repositories.is_empty() {
        bail!("workspace.repositories must name at least one repository");
    }
    let mut mounts = BTreeSet::new();
    for repository in &repositories {
        if !valid_mount(&repository.mount) {
            bail!(
                "repository {:?}: mount {:?} must be one path component of a-z, A-Z, 0-9, '.', '_' or '-'",
                repository.name,
                repository.mount
            );
        }
        if !mounts.insert(repository.mount.clone()) {
            bail!("mount {:?} is declared twice", repository.mount);
        }
        if !repository.repository.starts_with("https://") {
            bail!(
                "repository {:?}: only https:// repositories are served",
                repository.name
            );
        }
        if !valid_ref(&repository.reference) {
            bail!(
                "repository {:?}: ref {:?} is not a valid branch, tag or commit",
                repository.name,
                repository.reference
            );
        }
    }
    let cwd = manifest.agent.cwd;
    if cwd != "/workspace"
        && !cwd.strip_prefix("/workspace/").is_some_and(|rest| {
            rest.split('/').all(valid_cwd_component)
                && mounts
                    .iter()
                    .any(|mount| rest == mount || rest.starts_with(&format!("{mount}/")))
        })
    {
        bail!("agent.cwd {cwd:?} must be /workspace or lie inside a declared mount");
    }
    let resources = manifest.resources;
    let cpu = resources.cpu.unwrap_or(4).max(1);
    let memory_bytes = match resources.memory {
        Some(text) => parse_size(&text).with_context(|| format!("resources.memory {text:?}"))?,
        None => 16 << 30,
    };
    validate_memory(memory_bytes)?;
    let pids = resources.pids.unwrap_or(MAX_PIDS);
    validate_pids(pids)?;
    let storage_bytes = resources
        .storage
        .map(|text| parse_size(&text).with_context(|| format!("resources.storage {text:?}")))
        .transpose()?;
    let retain_for = match manifest.lifecycle.retain_for {
        Some(text) => {
            parse_duration(&text).with_context(|| format!("lifecycle.retainFor {text:?}"))?
        }
        None => DEFAULT_RETAIN,
    };
    validate_retention(retain_for)?;
    if manifest.lifecycle.detach_keeps_running == Some(false) {
        bail!("lifecycle.detachKeepsRunning: false is not served");
    }
    let digest = format!("sha256:{:x}", Sha256::digest(text.as_bytes()));
    Ok(Resolved {
        name,
        repositories,
        agent_cwd: cwd,
        agent_kind,
        authentication,
        agent_argv,
        cpu,
        memory_bytes,
        pids,
        storage_bytes,
        retain_for,
        digest,
    })
}

pub(crate) fn validate_memory(bytes: u64) -> Result<()> {
    if bytes < (512 << 20) {
        bail!("resources.memory must be at least 512MiB");
    }
    Ok(())
}

pub(crate) fn validate_pids(pids: u32) -> Result<()> {
    if pids == 0 || pids > MAX_PIDS {
        bail!("resources.pids must be 1-{MAX_PIDS}");
    }
    Ok(())
}

pub(crate) fn validate_retention(retain: Duration) -> Result<()> {
    if retain.is_zero() || retain > MAX_RETAIN {
        bail!("lifecycle.retainFor must be more than 0 and at most 24h (Substrate's exec bound)");
    }
    Ok(())
}

fn valid_name(name: &str) -> bool {
    (1..=48).contains(&name.len())
        && name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !name.starts_with('-')
}

fn valid_mount(mount: &str) -> bool {
    !mount.is_empty()
        && mount != "."
        && mount != ".."
        && !mount.starts_with('.')
        && mount
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

/// A prefix check on a path says nothing about where it resolves: `.`, `..` and empty components
/// are refused so the text is the location.
fn valid_cwd_component(component: &str) -> bool {
    !component.is_empty() && component != "." && component != ".."
}

/// A ref is handed to `git` as one argv element, so the check only has to keep it from being read
/// as an option or a path escape.
fn valid_ref(reference: &str) -> bool {
    !reference.is_empty()
        && !reference.starts_with('-')
        && !reference.contains("..")
        && reference
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'.' | b'_' | b'-'))
}

pub fn is_commit(reference: &str) -> bool {
    reference.len() == 40 && reference.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub fn parse_size(text: &str) -> Result<u64> {
    let text = text.trim();
    let split = text
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(text.len());
    let (number, unit) = text.split_at(split);
    let number: u64 = number.parse().context("expected a whole number")?;
    let multiplier: u64 = match unit {
        "" => 1,
        "KiB" => 1 << 10,
        "MiB" => 1 << 20,
        "GiB" => 1 << 30,
        "TiB" => 1 << 40,
        _ => bail!("unit {unit:?} is not one of KiB, MiB, GiB, TiB"),
    };
    number.checked_mul(multiplier).context("size overflows")
}

pub fn parse_duration(text: &str) -> Result<Duration> {
    let text = text.trim();
    let (number, unit) = text.split_at(text.len().saturating_sub(1));
    let number: u64 = number
        .parse()
        .context("expected a whole number and s, m or h")?;
    let seconds = match unit {
        "s" => number,
        "m" => number.checked_mul(60).context("duration overflows")?,
        "h" => number.checked_mul(3_600).context("duration overflows")?,
        _ => bail!("unit {unit:?} is not one of s, m, h"),
    };
    Ok(Duration::from_secs(seconds))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../../../examples/substrate.yaml");

    #[test]
    fn the_example_manifest_resolves() {
        let resolved = parse(EXAMPLE).expect("example resolves");
        assert_eq!(resolved.name, "substrate-work");
        assert_eq!(resolved.repositories.len(), 1);
        assert_eq!(resolved.agent_cwd, "/workspace/substrate");
        assert_eq!(resolved.memory_bytes, 48 << 30);
        assert_eq!(resolved.retain_for, Duration::from_hours(8));
        assert!(resolved.digest.starts_with("sha256:"));
    }

    fn minimal(extra: &str) -> String {
        format!(
            "apiVersion: {API_VERSION}\nkind: Session\nmetadata:\n  name: s1\nworkspace:\n  repositories:\n    - name: r\n      repository: https://example.com/r.git\n      ref: main\n      mount: r\nagent:\n  kind: claude-code\n  cwd: /workspace/r\n{extra}"
        )
    }

    #[test]
    fn defaults_apply() {
        let resolved = parse(&minimal("")).expect("minimal resolves");
        assert_eq!(resolved.cpu, 4);
        assert_eq!(resolved.pids, 4_096);
        assert_eq!(resolved.storage_bytes, None);
        assert_eq!(resolved.retain_for, DEFAULT_RETAIN);
    }

    #[test]
    fn unserved_fields_are_refused_by_name() {
        let error = parse(&minimal("peers:\n  allow: []\n")).expect_err("peers refused");
        assert!(format!("{error:#}").contains("peers"), "{error:#}");
        let error = parse(&minimal("network:\n  capabilities: [apt.ubuntu]\n"))
            .expect_err("capability refused");
        assert!(format!("{error:#}").contains("apt.ubuntu"), "{error:#}");
    }

    #[test]
    fn retain_for_is_bounded_by_the_exec_limit() {
        assert!(parse(&minimal("lifecycle:\n  retainFor: 25h\n")).is_err());
        let resolved = parse(&minimal("lifecycle:\n  retainFor: 30m\n")).expect("30m resolves");
        assert_eq!(resolved.retain_for, Duration::from_secs(1_800));
    }

    #[test]
    fn mounts_and_refs_cannot_escape() {
        for mount in ["..", "a/b", ".hidden", ""] {
            let text = minimal("").replace("mount: r", &format!("mount: {mount:?}"));
            assert!(parse(&text).is_err(), "mount {mount:?} accepted");
        }
        for reference in ["--upload-pack=x", "a..b", "a b"] {
            let text = minimal("").replace("ref: main", &format!("ref: {reference:?}"));
            assert!(parse(&text).is_err(), "ref {reference:?} accepted");
        }
    }

    #[test]
    fn cwd_must_be_inside_a_mount() {
        let text = minimal("").replace("cwd: /workspace/r", "cwd: /workspace/other");
        assert!(parse(&text).is_err());
        let text = minimal("").replace("cwd: /workspace/r", "cwd: /workspace/r/sub");
        assert!(parse(&text).is_ok());
    }

    #[test]
    fn cwd_with_dot_dot_or_empty_components_is_refused() {
        for cwd in [
            "/workspace/r/../..",
            "/workspace/r/..",
            "/workspace/r/./sub",
            "/workspace/r/.",
            "/workspace/r//sub",
            "/workspace/r/",
            "/workspace/r/sub/../../../etc",
        ] {
            let text = minimal("").replace("cwd: /workspace/r", &format!("cwd: {cwd:?}"));
            let error = parse(&text).expect_err(cwd);
            assert!(format!("{error:#}").contains("agent.cwd"), "{error:#}");
        }
    }

    const SHELL: &str = include_str!("../../../../examples/shell.yaml");

    fn command(agent: &str) -> String {
        minimal("").replace("  kind: claude-code\n", agent)
    }

    #[test]
    fn the_shell_example_resolves_to_a_command_session_without_credential() {
        let resolved = parse(SHELL).expect("shell example resolves");
        assert_eq!(
            super::super::session::agent_name(&resolved.agent_kind),
            "command"
        );
        assert_eq!(
            super::super::session::auth_name(&resolved.authentication),
            "none"
        );
        assert_eq!(resolved.agent_cwd, "/workspace/substrate");
    }

    #[test]
    fn argv_without_command_kind_is_refused_by_name() {
        for kind in ["claude-code", "codex"] {
            let text = command(&format!("  kind: {kind}\n  argv: [/bin/bash]\n"));
            let error = parse(&text).expect_err(kind);
            assert_eq!(
                format!("{error:#}"),
                "agent.argv is served only with agent.kind command"
            );
        }
    }

    #[test]
    fn command_kind_without_argv_is_refused_by_name() {
        let error = parse(&command("  kind: command\n")).expect_err("argv required");
        assert_eq!(
            format!("{error:#}"),
            "agent.kind command requires agent.argv"
        );
    }

    #[test]
    fn command_argv_must_name_an_absolute_program() {
        for argv in ["[]", "[bash, -l]", "['']", "[./bash]"] {
            let error =
                parse(&command(&format!("  kind: command\n  argv: {argv}\n"))).expect_err(argv);
            assert_eq!(
                format!("{error:#}"),
                "agent.argv must be a non-empty list whose first element is an absolute path",
                "{argv}"
            );
        }
    }

    #[test]
    fn command_argv_stays_within_substrate_exec_bounds_at_the_boundary() {
        let refused = "agent.argv must have at most 243 elements, each non-empty, at most 4096 bytes and without NUL";
        let declared = |argv: Vec<String>| validate_agent("command", None, Some(&argv));
        let elements = |n: usize| {
            std::iter::once("/bin/echo".to_owned())
                .chain(std::iter::repeat_n("x".to_owned(), n - 1))
                .collect::<Vec<_>>()
        };
        assert_eq!(declared(elements(243)), Ok(()));
        assert_eq!(declared(elements(244)).unwrap_err().to_string(), refused);
        let element = |bytes: usize| vec!["/bin/echo".to_owned(), "a".repeat(bytes)];
        assert_eq!(declared(element(4096)), Ok(()));
        assert_eq!(declared(element(4097)).unwrap_err().to_string(), refused);
        for argv in [
            vec!["/bin/bash".to_owned(), "-c".to_owned(), String::new()],
            vec!["/bin/bash".to_owned(), "a\0b".to_owned()],
            vec![format!("/{}", "a".repeat(4096))],
        ] {
            assert_eq!(
                declared(argv.clone()),
                Err(AgentRefusal::ArgvOutOfBounds),
                "{argv:?}"
            );
        }
        let error = parse(&command(
            "  kind: command\n  argv: [/bin/bash, \"a\\0b\"]\n",
        ))
        .expect_err("NUL refused");
        assert_eq!(format!("{error:#}"), refused);
    }

    #[test]
    fn command_kind_refuses_any_agent_auth() {
        for auth in ["claude-oauth", "chatgpt-device", "none"] {
            let text = command(&format!(
                "  kind: command\n  argv: [/bin/bash]\n  auth: {auth}\n"
            ));
            let error = parse(&text).expect_err(auth);
            assert_eq!(
                format!("{error:#}"),
                "agent.auth is refused with agent.kind command: a command session receives no agent credential",
                "{auth}"
            );
        }
    }

    #[test]
    fn existing_agent_kinds_resolve_unchanged() {
        let claude = parse(&minimal("")).expect("claude resolves");
        assert_eq!(
            super::super::session::agent_name(&claude.agent_kind),
            "claude-code"
        );
        let codex = parse(&command("  kind: codex\n")).expect("codex resolves");
        assert_eq!(
            super::super::session::agent_name(&codex.agent_kind),
            "codex"
        );
        assert!(parse(&command("  kind: shell\n")).is_err());
    }

    #[test]
    fn sizes_and_durations_parse() {
        assert_eq!(parse_size("48GiB").expect("size"), 48 << 30);
        assert_eq!(parse_size("1024").expect("size"), 1024);
        assert!(parse_size("1GB").is_err());
        assert_eq!(
            parse_duration("8h").expect("duration"),
            Duration::from_hours(8)
        );
        assert!(parse_duration("8d").is_err());
    }
}
