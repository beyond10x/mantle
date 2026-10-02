//! `mantle start|attach|list|status|stop`.

use std::time::Duration;

use anyhow::{Context, Result, bail};
use b10x_substrate_sdk::{
    Client, ExecState, OutputStream, SecretSlotRequest, StorageLimit, Workspace,
};
use serde_json::json;

use crate::adapters::ssh::Ssh;
use crate::adapters::state::{SessionRecord, SourceRecord, Store, WorkerRecord};
use crate::adapters::substrate::{self as sub, bytes};
use crate::app::terminal::{self, Ended};
use crate::app::worker::{WORKER, install_token};
use crate::config::Config;
use crate::domain::manifest::{self, Resolved};
use crate::domain::session::{AgentKind, SessionState, agent_name, auth_name, validate_identity};

/// Substrate admits no proxy variable in a request, so git takes the gateway as a config value.
const GIT_PROXY: &str = "http.proxy=http://127.0.0.1:3128";
const INIT_TIMEOUT: Duration = Duration::from_secs(30 * 60);
const ATTACH_TIMEOUT: Duration = Duration::from_hours(24);

struct Connected {
    ssh: Ssh,
    _tunnel: crate::adapters::ssh::Tunnel,
    client: Client,
}

fn worker(store: &Store) -> Result<WorkerRecord> {
    store
        .worker(WORKER)?
        .context("no worker recorded; run `mantle worker up` first")
}

async fn connect(config: &Config, worker: &WorkerRecord) -> Result<Connected> {
    let ssh = crate::app::worker::ssh_for(config, worker)?;
    // A worker that is still bootstrapping, or whose daemon is down, answers a socket forward with
    // a reset. Say which, before the transport error does.
    let ready = ssh.run(
        "test -s /var/lib/mantle/bootstrap.json && systemctl is-active --quiet substrate.service",
        None,
    );
    match ready {
        Ok(output) if output.status.success() => {}
        Ok(_) => bail!(
            "the worker is not ready: it is still being set up, or substrate.service is not running. \
             `mantle worker up` finishes the setup; `mantle worker status` shows the state."
        ),
        Err(error) => {
            return Err(error
                .context("the worker is not reachable; `mantle worker status` shows its state"));
        }
    }
    let tunnel = ssh.tunnel()?;
    let client = sub::connect(tunnel.socket()).await?;
    Ok(Connected {
        ssh,
        _tunnel: tunnel,
        client,
    })
}

pub async fn start(
    config: &Config,
    store: &Store,
    manifest_path: &str,
    attach: bool,
) -> Result<()> {
    let text = std::fs::read_to_string(manifest_path)
        .with_context(|| format!("reading {manifest_path}"))?;
    let resolved = manifest::parse(&text)?;
    let token = selected_credentials(&resolved, || config.claude_token())?;
    let worker = worker(store)?;
    let connected = connect(config, &worker).await?;
    let machine = connected.client.machine();
    let executable = connected
        .ssh
        .bounded(
            match resolved.agent_kind {
                AgentKind::V0 => "test -x /opt/mantle/bin/claude",
                AgentKind::V1 => "test -x /opt/mantle/bin/codex",
            },
            None,
            Duration::from_secs(15),
        )?
        .status
        .success();
    let missing = match resolved.agent_kind {
        AgentKind::V0 => sub::selected_claude_missing_facts(&machine, executable),
        AgentKind::V1 => {
            let mut missing = sub::missing_facts(&machine);
            if !executable {
                missing.push("agent.executable[codex]".into());
            }
            missing
        }
    };
    if !missing.is_empty() {
        bail!("FAILED_CAPABILITY: the worker lacks {}", missing.join(", "));
    }
    if let Some(token) = token {
        install_token(&connected.ssh, &token)?;
    }

    let id = format!("ses_{}", ulid::Ulid::new().to_string().to_lowercase());
    store.insert_session(&SessionRecord {
        id: id.clone(),
        name: resolved.name.clone(),
        worker: WORKER.to_owned(),
        state: SessionState::Materializing,
        manifest_digest: resolved.digest.clone(),
        workspace: None,
        agent_exec: None,
        requested_json: requested_json(&resolved).to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        failure: None,
        agent_kind: resolved.agent_kind.clone(),
        authentication: resolved.authentication.clone(),
    })?;
    println!("Session        {} ({id})", resolved.name);
    println!("Worker         {} {}", worker.region, worker.instance);
    println!("Manifest       {}", resolved.digest);

    start_recorded(store, &id, &resolved, &connected, attach).await
}

pub(crate) trait StartPort {
    type Workspace: InitPort;
    async fn create(&self, id: &str, resolved: &Resolved) -> Result<Self::Workspace>;
    fn workspace_id<'a>(&self, workspace: &'a Self::Workspace) -> &'a str;
    async fn start_agent(&self, workspace: &Self::Workspace, resolved: &Resolved)
    -> Result<String>;
    async fn attach(
        &self,
        workspace: &Self::Workspace,
        name: &str,
        agent: &AgentKind,
    ) -> Result<()>;
}
impl StartPort for Connected {
    type Workspace = Workspace;
    async fn create(&self, id: &str, resolved: &Resolved) -> Result<Workspace> {
        let machine = self.client.machine();
        let mut builder = self
            .client
            .workspace()
            .empty()
            .label("mantle-session", id)
            .label("mantle-name", resolved.name.as_str());
        match (
            &machine.facts.workspace_storage_quota,
            resolved.storage_bytes,
        ) {
            (Some(quota), Some(requested)) => {
                builder = builder.storage(StorageLimit {
                    max_bytes: requested.min(quota.max_bytes),
                    max_inodes: quota.max_inodes,
                });
            }
            (None, Some(_)) => {
                println!(
                    "Storage        requested, NOT enforced: the worker serves no workspace quota"
                )
            }
            _ => {}
        }
        builder.create().await.context("creating the workspace")
    }
    fn workspace_id<'a>(&self, workspace: &'a Workspace) -> &'a str {
        workspace.id()
    }
    async fn start_agent(&self, workspace: &Workspace, resolved: &Resolved) -> Result<String> {
        start_agent(workspace, resolved).await
    }
    async fn attach(&self, workspace: &Workspace, name: &str, agent: &AgentKind) -> Result<()> {
        attach_workspace(workspace, name, agent).await
    }
}

pub(crate) async fn start_recorded(
    store: &Store,
    id: &str,
    resolved: &Resolved,
    port: &impl StartPort,
    attach: bool,
) -> Result<()> {
    validate_identity(&resolved.agent_kind, &resolved.authentication)?;
    let workspace = port.create(id, resolved).await?;
    store.set_workspace(id, port.workspace_id(&workspace))?;
    println!("Workspace      {}", port.workspace_id(&workspace));

    if let Err(error) = materialize(store, id, &workspace, resolved).await {
        store.move_session(
            id,
            SessionState::FailedMaterialization,
            Some(&format!("{error:#}")),
        )?;
        return Err(error.context("FAILED_MATERIALIZATION"));
    }
    store.move_session(id, SessionState::Starting, None)?;
    match port.start_agent(&workspace, resolved).await {
        Ok(exec) => {
            store.set_agent_exec(id, &exec)?;
            store.move_session(id, SessionState::Running, None)?;
            println!(
                "Agent          {exec} (running, ends after {}h at the latest)",
                resolved.retain_for.as_secs() / 3600
            );
        }
        Err(error) => {
            store.move_session(
                id,
                SessionState::FailedAgentStart,
                Some(&format!("{error:#}")),
            )?;
            return Err(error.context("FAILED_AGENT_START"));
        }
    }
    if attach {
        port.attach(&workspace, &resolved.name, &resolved.agent_kind)
            .await?;
    } else {
        println!("Attach with    mantle attach {}", resolved.name);
        if resolved.agent_kind == AgentKind::V1 {
            println!(
                "Authentication complete only after attached Codex login; process readiness does not establish it."
            );
        }
    }
    Ok(())
}

/// Selection is checked before calling the credential source, connecting to a provider or
/// inserting a session. Codex does not read or install Claude credentials.
pub(crate) fn selected_credentials(
    resolved: &Resolved,
    read_claude: impl FnOnce() -> Result<Vec<u8>>,
) -> Result<Option<Vec<u8>>> {
    validate_identity(&resolved.agent_kind, &resolved.authentication)?;
    match resolved.agent_kind {
        AgentKind::V0 => read_claude().map(Some),
        AgentKind::V1 => Ok(None),
    }
}

fn requested_json(resolved: &Resolved) -> serde_json::Value {
    json!({
        "cpu": resolved.cpu,
        "memory_bytes": resolved.memory_bytes,
        "pids": resolved.pids,
        "storage_bytes": resolved.storage_bytes,
        "retain_for_secs": resolved.retain_for.as_secs(),
        "agent_cwd": resolved.agent_cwd,
    })
}

async fn run_init(workspace: &Workspace, argv: &[&str], network: bool) -> Result<String> {
    let (program, args) = argv.split_first().context("empty argv")?;
    let mut command = workspace
        .command(*program)
        .args(args.iter().copied())
        .policy(sub::policy(INIT_TIMEOUT, 4 << 30, 512)?);
    for (name, value) in sub::base_environment(4) {
        command = command.env(name, value);
    }
    if network {
        command = command.aperture(sub::APERTURE);
    }
    let output = command
        .run()
        .await
        .with_context(|| format!("running {}", argv.join(" ")))?;
    let succeeded = output.exec.state == ExecState::Exited
        && output
            .exec
            .exit
            .as_ref()
            .is_some_and(|exit| exit.code == Some(0));
    if !succeeded {
        bail!(
            "`{}` ended {:?} {:?}: {}",
            argv.join(" "),
            output.exec.state,
            output.exec.exit,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

pub(crate) trait InitPort {
    async fn run(&self, argv: &[&str], network: bool) -> Result<String>;
}
impl InitPort for Workspace {
    async fn run(&self, argv: &[&str], network: bool) -> Result<String> {
        run_init(self, argv, network).await
    }
}

pub(crate) async fn materialize(
    store: &Store,
    id: &str,
    workspace: &impl InitPort,
    resolved: &Resolved,
) -> Result<()> {
    workspace
        .run(
            &[
                "/usr/bin/mkdir",
                "-p",
                "/workspace/.mantle/home",
                "/workspace/.mantle/cargo",
            ],
            false,
        )
        .await?;
    for repository in &resolved.repositories {
        let path = format!("/workspace/{}", repository.mount);
        if manifest::is_commit(&repository.reference) {
            workspace
                .run(
                    &[
                        "/usr/bin/git",
                        "-c",
                        GIT_PROXY,
                        "clone",
                        "--no-tags",
                        "--",
                        &repository.repository,
                        &path,
                    ],
                    true,
                )
                .await?;
            workspace
                .run(
                    &[
                        "/usr/bin/git",
                        "-C",
                        &path,
                        "checkout",
                        "--detach",
                        &repository.reference,
                    ],
                    false,
                )
                .await?;
        } else {
            workspace
                .run(
                    &[
                        "/usr/bin/git",
                        "-c",
                        GIT_PROXY,
                        "clone",
                        "--no-tags",
                        "--branch",
                        &repository.reference,
                        "--",
                        &repository.repository,
                        &path,
                    ],
                    true,
                )
                .await?;
        }
        let commit = workspace
            .run(&["/usr/bin/git", "-C", &path, "rev-parse", "HEAD"], false)
            .await?;
        if !manifest::is_commit(&commit) {
            bail!(
                "{}: rev-parse answered {commit:?}, not a commit",
                repository.name
            );
        }
        store.put_source(
            id,
            &SourceRecord {
                name: repository.name.clone(),
                repository: repository.repository.clone(),
                declared_ref: repository.reference.clone(),
                commit: commit.clone(),
                mount: repository.mount.clone(),
            },
        )?;
        println!("Source         {path} {}@{commit}", repository.reference);
    }
    Ok(())
}

async fn start_agent(workspace: &Workspace, resolved: &Resolved) -> Result<String> {
    let command = agent_request(resolved).command(workspace)?;
    let mut exec = command.start().await.context("starting the agent")?;
    // The launcher refuses quickly (bad secret, stale server); give it that long to do so.
    tokio::time::sleep(Duration::from_secs(3)).await;
    let observed = exec.refresh().await?.clone();
    if observed.state != ExecState::Running {
        let stderr = exec
            .output_page(OutputStream::Stderr, 0, 16 * 1024)
            .await
            .map(|page| String::from_utf8_lossy(&page.bytes).into_owned())
            .unwrap_or_default();
        bail!(
            "the agent exec is {:?} {:?}: {}",
            observed.state,
            observed.exit,
            stderr.trim()
        );
    }
    Ok(exec.id().to_owned())
}

async fn attach_workspace(workspace: &Workspace, name: &str, agent: &AgentKind) -> Result<()> {
    let session = attach_request(agent)
        .pty(workspace)?
        .input_limit_bytes(b10x_substrate_sdk::MAX_SESSION_INPUT_BYTES)
        .frame_limit_bytes(b10x_substrate_sdk::MAX_SESSION_FRAME_BYTES)
        .queued_frames(b10x_substrate_sdk::MAX_SESSION_QUEUED_FRAMES)
        .start()
        .await
        .context("starting the terminal session")?;
    let channel = session.attach().await.context("attaching")?;
    let ended = terminal::run(channel).await;
    let _ = session.retire().await;
    match ended? {
        Ended::Detached => println!(
            "\r\nDetached from {name}. The agent keeps running; `mantle attach {name}` returns to it."
        ),
        Ended::Exited | Ended::Closed => println!(
            "\r\nThe terminal session for {name} ended; `mantle status {name}` shows whether the agent still runs."
        ),
    }
    Ok(())
}

pub async fn attach(config: &Config, store: &Store, name: &str) -> Result<()> {
    let record = live(store, name)?;
    let connected = connect(config, &worker(store)?).await?;
    let workspace = connected
        .client
        .get_workspace(
            record
                .workspace
                .as_deref()
                .context("session has no workspace")?,
        )
        .await?;
    if let Some(exec) = &record.agent_exec {
        let observed = connected.client.get_exec(exec).await?;
        if observed.observation().state != ExecState::Running {
            bail!(
                "the agent exec is {:?}; nothing to attach to (`mantle stop {name}` cleans up)",
                observed.observation().state
            );
        }
    }
    attach_workspace(&workspace, name, &record.agent_kind).await
}

fn live(store: &Store, name: &str) -> Result<SessionRecord> {
    store
        .live_session(name)?
        .with_context(|| format!("no live session named {name:?}"))
}

/// Runs one command in a session's workspace under the agent's confinement (toolchain root,
/// aperture, environment) but without its credential, and prints what Substrate observed.
pub async fn exec(config: &Config, store: &Store, name: &str, argv: &[String]) -> Result<()> {
    let record = live(store, name)?;
    let (program, args) = argv.split_first().context("no command given")?;
    let connected = connect(config, &worker(store)?).await?;
    let workspace = connected
        .client
        .get_workspace(
            record
                .workspace
                .as_deref()
                .context("session has no workspace")?,
        )
        .await?;
    let requested: serde_json::Value = serde_json::from_str(&record.requested_json)?;
    let cpu = requested["cpu"]
        .as_u64()
        .and_then(|cpu| u32::try_from(cpu).ok())
        .unwrap_or(4);
    let command = exec_request(program, args, cpu).command(&workspace)?;
    let output = command.run().await.context("running the command")?;
    use std::io::Write as _;
    std::io::stdout().write_all(&output.stdout)?;
    std::io::stderr().write_all(&output.stderr)?;
    eprintln!(
        "[mantle exec] {} {:?} {:?} refusal={:?}",
        output.exec.id, output.exec.state, output.exec.exit, output.exec.refusal
    );
    Ok(())
}

pub fn list(store: &Store) -> Result<()> {
    let sessions = store.live_sessions()?;
    if sessions.is_empty() {
        println!("no sessions");
        return Ok(());
    }
    println!(
        "{:<24} {:<24} {:<12} {:<16} {:<30} CREATED",
        "NAME", "STATE (recorded)", "AGENT", "AUTH METHOD", "ID"
    );
    for session in sessions {
        println!(
            "{:<24} {:<24} {:<12} {:<16} {:<30} {}",
            session.name,
            session.state.to_string(),
            agent_name(&session.agent_kind),
            auth_name(&session.authentication),
            session.id,
            session.created_at
        );
    }
    Ok(())
}

pub async fn status(config: &Config, store: &Store, name: &str) -> Result<()> {
    let record = live(store, name)?;
    let worker = worker(store)?;
    println!("Session");
    println!("  id              {}", record.id);
    println!("  recorded state  {}", record.state);
    println!("  agent           {}", agent_name(&record.agent_kind));
    println!(
        "  auth method     {} (not an authenticated-status observation)",
        auth_name(&record.authentication)
    );
    println!("  manifest        {}", record.manifest_digest);
    if let Some(failure) = &record.failure {
        println!("  failure         {failure}");
    }
    println!("Placement");
    println!("  worker          {} {}", worker.region, worker.instance);
    println!("Sources");
    for source in store.sources(&record.id)? {
        println!(
            "  /workspace/{:<12} {}@{}",
            source.mount, source.declared_ref, source.commit
        );
    }
    let requested: serde_json::Value = serde_json::from_str(&record.requested_json)?;
    let connected = match connect(config, &worker).await {
        Ok(connected) => connected,
        Err(error) => {
            println!("Substrate");
            println!("  UNREACHABLE     {error:#}");
            return Ok(());
        }
    };
    let machine = connected.client.machine();
    let limits = machine.facts.exec_cgroup_limits.clone();
    println!("Resources       requested              enforcement");
    println!(
        "  cpu           {:<22} not enforced (Substrate 0.7.8 bounds cumulative CPU time only); CARGO_BUILD_JOBS={}",
        requested["cpu"], requested["cpu"]
    );
    println!(
        "  memory        {:<22} {}",
        bytes(requested["memory_bytes"].as_u64().unwrap_or(0)),
        if limits.as_ref().is_some_and(|l| l.memory) {
            "cgroup memory limit (fact exec.cgroup_limits.memory)"
        } else {
            "NOT enforced"
        }
    );
    println!(
        "  pids          {:<22} {}",
        requested["pids"],
        if limits.as_ref().is_some_and(|l| l.processes) {
            "cgroup pids limit (fact exec.cgroup_limits.processes)"
        } else {
            "NOT enforced"
        }
    );
    println!(
        "  storage       {:<22} {}",
        requested["storage_bytes"]
            .as_u64()
            .map_or("none".to_owned(), bytes),
        if sub::quota_served(&machine) {
            "workspace project quota"
        } else {
            "NOT enforced (no quota fact)"
        }
    );
    println!("Network");
    println!(
        "  aperture        {} -> worker gateway; gateway allowlist is fixed (see mantle-egress)",
        sub::APERTURE
    );
    println!("  everything else no route (Substrate exec.no_egress)");
    println!("Agent");
    match &record.agent_exec {
        None => println!("  exec            none"),
        Some(exec_id) => match connected.client.get_exec(exec_id).await {
            Ok(exec) => {
                let observed = exec.observation();
                println!("  exec            {exec_id}");
                println!(
                    "  state           {:?} (observed {})",
                    observed.state, observed.observed_at
                );
                if let Some(exit) = &observed.exit {
                    println!("  exit            {exit:?}");
                }
                if let Some(applied) = &observed.applied {
                    println!("  cgroup          {}", applied.cgroup);
                }
                if let Some(lease) = &observed.lease {
                    println!("  lease ends by   {}", lease.renew_by);
                }
                for (key, value) in sub::describe_usage(observed.usage.as_ref()) {
                    println!("  {key:<15} {value}");
                }
                if observed.state.terminal() {
                    // The launcher's own diagnostics; the agent's terminal output goes to its pty.
                    if let Ok(page) = exec.output_page(OutputStream::Stderr, 0, 16 * 1024).await {
                        for line in String::from_utf8_lossy(&page.bytes).lines() {
                            println!("  stderr          {line}");
                        }
                    }
                }
            }
            Err(error) => println!("  exec            {exec_id}: {error}"),
        },
    }
    Ok(())
}

pub async fn stop(config: &Config, store: &Store, name: &str) -> Result<()> {
    let record = live(store, name)?;
    let connected = connect(config, &worker(store)?).await?;
    stop_recorded(&connected.client, store, &record, name).await
}

pub(crate) trait StopPort {
    type Exec;
    type Workspace;
    async fn get_exec(&self, id: &str) -> Result<Self::Exec, b10x_substrate_sdk::SdkError>;
    fn terminal(&self, exec: &Self::Exec) -> bool;
    async fn signal(&self, exec: &mut Self::Exec) -> Result<()>;
    async fn wait(&self, exec: &mut Self::Exec) -> Result<()>;
    async fn retire(&self, exec: Self::Exec) -> Result<()>;
    async fn get_workspace(
        &self,
        id: &str,
    ) -> Result<Self::Workspace, b10x_substrate_sdk::SdkError>;
    async fn destroy(&self, workspace: Self::Workspace)
    -> Result<(), b10x_substrate_sdk::SdkError>;
}
impl StopPort for Client {
    type Exec = b10x_substrate_sdk::Exec;
    type Workspace = Workspace;
    async fn get_exec(&self, id: &str) -> Result<Self::Exec, b10x_substrate_sdk::SdkError> {
        self.get_exec(id).await
    }
    fn terminal(&self, exec: &Self::Exec) -> bool {
        exec.observation().state.terminal()
    }
    async fn signal(&self, exec: &mut Self::Exec) -> Result<()> {
        exec.signal(
            b10x_substrate_sdk::Signal::Terminate,
            Duration::from_secs(10),
        )
        .await?;
        Ok(())
    }
    async fn wait(&self, exec: &mut Self::Exec) -> Result<()> {
        let observed = exec.wait_for(Duration::from_secs(60)).await?;
        println!("Agent          {:?} {:?}", observed.state, observed.exit);
        Ok(())
    }
    async fn retire(&self, exec: Self::Exec) -> Result<()> {
        exec.retire().await?;
        Ok(())
    }
    async fn get_workspace(&self, id: &str) -> Result<Workspace, b10x_substrate_sdk::SdkError> {
        self.get_workspace(id).await
    }
    async fn destroy(&self, workspace: Workspace) -> Result<(), b10x_substrate_sdk::SdkError> {
        workspace.destroy().await?;
        Ok(())
    }
}

pub(crate) async fn stop_recorded(
    port: &impl StopPort,
    store: &Store,
    record: &SessionRecord,
    name: &str,
) -> Result<()> {
    // A stop interrupted earlier resumes from STOPPING.
    if record.state != SessionState::Stopping {
        store.move_session(&record.id, SessionState::Stopping, None)?;
    }
    if let Some(exec_id) = &record.agent_exec {
        let mut exec = match port.get_exec(exec_id).await {
            Ok(exec) => exec,
            Err(error) if not_found(&error) => {
                println!("Agent          {exec_id} already retired");
                return finish_stop(port, store, record, name).await;
            }
            Err(error) => return Err(error.into()),
        };
        if !port.terminal(&exec) {
            port.signal(&mut exec)
                .await
                .context("signalling the agent")?;
            port.wait(&mut exec).await?;
        }
        port.retire(exec).await.context("retiring the agent exec")?;
    }
    finish_stop(port, store, record, name).await
}

/// Destroys the workspace and records the stop. A workspace Substrate no longer has is one an
/// earlier, interrupted stop already destroyed.
async fn finish_stop(
    port: &impl StopPort,
    store: &Store,
    record: &SessionRecord,
    name: &str,
) -> Result<()> {
    if let Some(id) = &record.workspace {
        match port.get_workspace(id).await {
            Ok(workspace) => match port.destroy(workspace).await {
                Ok(_) => println!("Workspace      destroyed"),
                // Destroying a workspace with a built tree outlasts the operation's answer
                // (observed 2026-10-02); the outcome is read back rather than assumed.
                Err(error) if outcome_unknown(&error) => {
                    wait_until_destroyed(port, id).await?;
                    println!("Workspace      destroyed (confirmed by read-back)");
                }
                Err(error) => return Err(error).context("destroying the workspace"),
            },
            Err(error) if not_found(&error) => {
                println!("Workspace      {id} already destroyed")
            }
            Err(error) => return Err(error.into()),
        }
    }
    // The token file stays: substrate.service will not start without it.
    store.move_session(&record.id, SessionState::Stopped, None)?;
    println!("Session        {name} stopped");
    Ok(())
}

fn outcome_unknown(error: &b10x_substrate_sdk::SdkError) -> bool {
    match error {
        b10x_substrate_sdk::SdkError::Refusal(refusal) => {
            refusal.code == "operation.outcome-unknown"
        }
        b10x_substrate_sdk::SdkError::UnknownOperation { .. } => true,
        _ => false,
    }
}

/// Reads the workspace back until Substrate no longer has it, for at most five minutes.
async fn wait_until_destroyed(port: &impl StopPort, id: &str) -> Result<()> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(300);
    loop {
        match port.get_workspace(id).await {
            Err(error) if not_found(&error) => return Ok(()),
            Err(error) => return Err(error).context("reading the workspace back"),
            Ok(_) if tokio::time::Instant::now() > deadline => {
                bail!("workspace {id} still exists five minutes after its destroy was accepted")
            }
            Ok(_) => tokio::time::sleep(Duration::from_secs(2)).await,
        }
    }
}

fn not_found(error: &b10x_substrate_sdk::SdkError) -> bool {
    matches!(error, b10x_substrate_sdk::SdkError::Refusal(refusal) if refusal.code == "resource.not-found")
}

/// The exact command request Mantle passes to the pinned SDK. No credential value belongs here.
#[derive(serde::Serialize)]
pub(crate) struct RunRequest {
    argv: Vec<String>,
    environment: std::collections::BTreeMap<String, String>,
    aperture: Option<String>,
    root: String,
    secret_slot: Option<String>,
    secret_fd: Option<u32>,
    timeout_secs: u64,
    memory_bytes: u64,
    processes: u32,
    lease_secs: Option<u64>,
}
impl RunRequest {
    fn command(&self, workspace: &Workspace) -> Result<b10x_substrate_sdk::CommandBuilder> {
        let (program, args) = self.argv.split_first().context("empty argv")?;
        let mut command = workspace
            .command(program.as_str())
            .args(args.iter().map(String::as_str))
            .read_only_root(b10x_substrate_sdk::ReadOnlyRoot {
                host_path: self.root.clone(),
                mount: self.root.clone(),
            })
            .policy(sub::policy(
                Duration::from_secs(self.timeout_secs),
                self.memory_bytes,
                self.processes,
            )?);
        for (key, value) in &self.environment {
            command = command.env(key, value);
        }
        if let Some(aperture) = &self.aperture {
            command = command.aperture(aperture);
        }
        if let Some(slot) = &self.secret_slot {
            command = command.secret_slot(SecretSlotRequest {
                slot: slot.clone(),
                fd: self.secret_fd.context("secret fd")?,
            });
        }
        if let Some(lease) = self.lease_secs {
            command = command.lease(Duration::from_secs(lease));
        }
        Ok(command)
    }
    fn pty(&self, workspace: &Workspace) -> Result<b10x_substrate_sdk::PipeSessionBuilder> {
        let (program, args) = self.argv.split_first().context("empty argv")?;
        let mut command = workspace
            .pty_session(program.as_str(), terminal::local_window())
            .args(args.iter().map(String::as_str))
            .read_only_root(b10x_substrate_sdk::ReadOnlyRoot {
                host_path: self.root.clone(),
                mount: self.root.clone(),
            })
            .policy(sub::policy(
                Duration::from_secs(self.timeout_secs),
                self.memory_bytes,
                self.processes,
            )?);
        for (key, value) in &self.environment {
            command = command.env(key, value);
        }
        if let Some(lease) = self.lease_secs {
            command = command.lease(Duration::from_secs(lease));
        }
        Ok(command)
    }
}
pub(crate) fn agent_request(resolved: &Resolved) -> RunRequest {
    let mut request = RunRequest {
        argv: [
            "/opt/mantle/bin/mantle-launch",
            "serve",
            "--dir",
            sub::AGENT_DIR,
            "--secret-fd",
            "3",
            "--secret-env",
            "CLAUDE_CODE_OAUTH_TOKEN",
            "--cwd",
            resolved.agent_cwd.as_str(),
            "--mkdir",
            "/workspace/.mantle/home",
            "--mkdir",
            "/workspace/.mantle/cargo",
            "--proxy",
            sub::PROXY,
            "--",
            "/opt/mantle/bin/claude",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
        environment: sub::base_environment(resolved.cpu)
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v))
            .collect(),
        aperture: Some(sub::APERTURE.into()),
        root: sub::TOOLCHAIN_ROOT.into(),
        secret_slot: Some(sub::CLAUDE_SLOT.into()),
        secret_fd: Some(sub::CLAUDE_FD),
        timeout_secs: resolved.retain_for.as_secs(),
        memory_bytes: resolved.memory_bytes,
        processes: resolved.pids,
        lease_secs: Some(resolved.retain_for.as_secs()),
    };
    if resolved.agent_kind == AgentKind::V1 {
        request.argv = codex_argv(&resolved.agent_cwd);
        request.secret_slot = None;
        request.secret_fd = None;
        request
            .environment
            .remove("CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC");
        request.environment.remove("DISABLE_AUTOUPDATER");
        request
            .environment
            .insert("CODEX_HOME".into(), "/workspace/.mantle/home/.codex".into());
        request.environment.insert("RUST_LOG".into(), "off".into());
        request
            .environment
            .insert("CODEX_TUI_RECORD_SESSION".into(), "0".into());
    }
    request
}
pub(crate) fn exec_request(program: &str, args: &[String], cpu: u32) -> RunRequest {
    RunRequest {
        argv: std::iter::once(program.to_owned())
            .chain(args.iter().cloned())
            .collect(),
        environment: sub::base_environment(cpu)
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v))
            .collect(),
        aperture: Some(sub::APERTURE.into()),
        root: sub::TOOLCHAIN_ROOT.into(),
        secret_slot: None,
        secret_fd: None,
        timeout_secs: INIT_TIMEOUT.as_secs(),
        memory_bytes: 4 << 30,
        processes: 512,
        lease_secs: None,
    }
}
pub(crate) fn attach_request(_agent: &AgentKind) -> RunRequest {
    RunRequest {
        argv: vec![
            "/opt/mantle/bin/mantle-launch".into(),
            "attach".into(),
            "--dir".into(),
            sub::AGENT_DIR.into(),
        ],
        environment: [
            ("TERM", "xterm-256color"),
            ("LANG", "C.UTF-8"),
            ("HOME", "/workspace/.mantle/home"),
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect(),
        aperture: None,
        root: sub::TOOLCHAIN_ROOT.into(),
        secret_slot: None,
        secret_fd: None,
        timeout_secs: ATTACH_TIMEOUT.as_secs(),
        memory_bytes: 256 << 20,
        processes: 32,
        lease_secs: Some(ATTACH_TIMEOUT.as_secs()),
    }
}

fn codex_argv(cwd: &str) -> Vec<String> {
    let mut argv: Vec<String> = [
        "/opt/mantle/bin/mantle-launch",
        "serve",
        "--dir",
        sub::AGENT_DIR,
        "--volatile-replay",
        "--cwd",
        cwd,
        "--private-dir",
        "/workspace/.mantle",
        "--private-dir",
        "/workspace/.mantle/home",
        "--private-dir",
        "/workspace/.mantle/home/.codex",
        "--volatile-dir",
        "/tmp/mantle-codex/sqlite",
        "--volatile-dir",
        "/tmp/mantle-codex/log",
        "--check-private-file",
        "/workspace/.mantle/home/.codex/auth.json",
        "--check-private-file",
        "/workspace/.mantle/home/.codex/config.toml",
        "--check-private-file",
        "/workspace/.mantle/home/.codex/environments.toml",
        "--mkdir",
        "/workspace/.mantle/cargo",
        "--proxy",
        sub::PROXY,
        "--",
        "/opt/mantle/bin/codex",
        "--strict-config",
        "--sandbox",
        "danger-full-access",
        "--ask-for-approval",
        "on-request",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    // Outer confinement remains Substrate's responsibility. This requests approvals and
    // never uses Codex's flag that bypasses both approvals and sandboxing.
    for setting in [
        "forced_login_method=\"chatgpt\"",
        "cli_auth_credentials_store=\"file\"",
        "history.persistence=\"none\"",
        "analytics.enabled=false",
        "feedback.enabled=false",
        "check_for_update_on_startup=false",
        "sqlite_home=\"/tmp/mantle-codex/sqlite\"",
        "log_dir=\"/tmp/mantle-codex/log\"",
        "model_provider=\"openai\"",
        "chatgpt_base_url=\"https://chatgpt.com/backend-api/\"",
        "openai_base_url=\"https://chatgpt.com/backend-api/codex\"",
    ] {
        argv.push("-c".into());
        argv.push(setting.into());
    }
    argv
}

#[cfg(test)]
mod codex_transport_tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn codex_start_and_attach_construct_real_sdk_builders() {
        let dir = tempfile::tempdir().unwrap();
        let socket = dir.path().join("sdk.sock");
        let listener = tokio::net::UnixListener::bind(&socket).unwrap();
        let server = tokio::spawn(async move {
            for (path, result) in [
                (
                    "/v1/machine",
                    json!({
                        "snapshot":format!("sha256:{}", "7".repeat(64)), "driver":"host",
                        "driver_version":"fixture", "config_generation":1,
                        "probed_at":"2026-10-02T00:00:00Z", "facts": {
                            "operation.ledger-subject-max-rows":1000,
                            "operation.ledger-subject-max-bytes":1048576,
                            "operation.ledger-global-max-rows":10000,
                            "operation.ledger-global-max-bytes":10485760
                        }
                    }),
                ),
                (
                    "/v1/workspaces/ws-fixture",
                    json!({
                        "id":"ws-fixture", "kind":"workspace", "labels":{},
                        "observed_at":"2026-10-02T00:00:00Z", "state":"ready"
                    }),
                ),
            ] {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut request = Vec::new();
                loop {
                    let byte = stream.read_u8().await.unwrap();
                    request.push(byte);
                    if request.ends_with(b"\r\n\r\n") {
                        break;
                    }
                    assert!(request.len() < 8192);
                }
                assert!(
                    String::from_utf8(request)
                        .unwrap()
                        .starts_with(&format!("GET {path} HTTP/1.1"))
                );
                let body =
                    json!({"api_version":"v1","request_id":"fixture","result":result}).to_string();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nx-b10x-contract: {}\r\nx-b10x-contract-bundle-sha256: {}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                    b10x_substrate_sdk::CONTRACT,
                    b10x_substrate_sdk::CONTRACT_SHA256,
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).await.unwrap();
            }
        });
        tokio::time::timeout(Duration::from_secs(5), async {
            let client = Client::builder()
                .unix_socket(socket)
                .connect()
                .await
                .unwrap();
            let workspace = client.get_workspace("ws-fixture").await.unwrap();
            let resolved =
                manifest::parse(include_str!("../../../../examples/codex.yaml")).unwrap();
            let start = agent_request(&resolved);
            let attach = attach_request(&AgentKind::V1);
            let start_result = start.command(&workspace);
            let attach_result = attach.pty(&workspace);
            assert!(
                start_result.is_ok(),
                "Codex start builder refused: {:?}",
                start_result.err()
            );
            assert!(
                attach_result.is_ok(),
                "Codex attach builder refused: {:?}",
                attach_result.err()
            );
            assert!(start.secret_slot.is_none() && start.secret_fd.is_none());
            assert!(attach.secret_slot.is_none() && attach.secret_fd.is_none());
            assert_eq!(
                start.environment["CODEX_HOME"],
                "/workspace/.mantle/home/.codex"
            );
            assert_eq!(start.aperture.as_deref(), Some(sub::APERTURE));
            assert_eq!(start.memory_bytes, resolved.memory_bytes);
            assert_eq!(start.processes, resolved.pids);
            server.await.unwrap();
        })
        .await
        .unwrap();
    }
}

#[cfg(test)]
mod adversary_activation_wire {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn adversary_codex_dispatches_confined_secretless_command_and_pty() {
        let dir = tempfile::tempdir().unwrap();
        let socket = dir.path().join("wire.sock");
        let listener = tokio::net::UnixListener::bind(&socket).unwrap();
        let server = tokio::spawn(async move {
            let mut posts = Vec::new();
            for index in 0..4 {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut header = Vec::new();
                while !header.ends_with(b"\r\n\r\n") {
                    header.push(stream.read_u8().await.unwrap());
                    assert!(header.len() < 16384);
                }
                let header = String::from_utf8(header).unwrap();
                let length = header
                    .lines()
                    .find_map(|line| {
                        let (key, value) = line.split_once(':')?;
                        key.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().unwrap())
                    })
                    .unwrap_or(0);
                assert!(length < 65536);
                let mut body = vec![0; length];
                stream.read_exact(&mut body).await.unwrap();
                let result = match index {
                    0 => {
                        assert!(header.starts_with("GET /v1/machine HTTP/1.1"));
                        json!({"snapshot":format!("sha256:{}", "7".repeat(64)),
                            "driver":"host", "driver_version":"fixture", "config_generation":1,
                            "probed_at":"2026-10-02T00:00:00Z", "facts": {
                                "operation.ledger-subject-max-rows":1000,
                                "operation.ledger-subject-max-bytes":1048576,
                                "operation.ledger-global-max-rows":10000,
                                "operation.ledger-global-max-bytes":10485760}})
                    }
                    1 => {
                        assert!(header.starts_with("GET /v1/workspaces/ws-fixture HTTP/1.1"));
                        json!({"id":"ws-fixture", "kind":"workspace", "labels":{},
                            "observed_at":"2026-10-02T00:00:00Z", "state":"ready"})
                    }
                    _ => {
                        let path = if index == 2 {
                            "/v1/execs"
                        } else {
                            "/v1/sessions"
                        };
                        assert!(header.starts_with(&format!("POST {path} HTTP/1.1")));
                        posts.push(serde_json::from_slice::<serde_json::Value>(&body).unwrap());
                        // Intentionally no executable response: this fixture observes dispatch only.
                        json!({})
                    }
                };
                let body = json!({"api_version":"v1", "request_id":"fixture", "result":result})
                    .to_string();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nx-b10x-contract: {}\r\nx-b10x-contract-bundle-sha256: {}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                    b10x_substrate_sdk::CONTRACT,
                    b10x_substrate_sdk::CONTRACT_SHA256,
                    body.len(),
                    body
                );
                stream.write_all(response.as_bytes()).await.unwrap();
            }
            posts
        });
        tokio::time::timeout(Duration::from_secs(5), async {
            let client = Client::builder()
                .unix_socket(socket)
                .connect()
                .await
                .unwrap();
            let workspace = client.get_workspace("ws-fixture").await.unwrap();
            let resolved =
                manifest::parse(include_str!("../../../../examples/codex.yaml")).unwrap();
            let result = agent_request(&resolved)
                .command(&workspace)
                .unwrap()
                .start()
                .await;
            assert!(matches!(
                result,
                Err(b10x_substrate_sdk::SdkError::Protocol(_))
            ));
            let result = attach_request(&AgentKind::V1)
                .pty(&workspace)
                .unwrap()
                .input_limit_bytes(b10x_substrate_sdk::MAX_SESSION_INPUT_BYTES)
                .frame_limit_bytes(b10x_substrate_sdk::MAX_SESSION_FRAME_BYTES)
                .queued_frames(b10x_substrate_sdk::MAX_SESSION_QUEUED_FRAMES)
                .start()
                .await;
            assert!(matches!(
                result,
                Err(b10x_substrate_sdk::SdkError::Protocol(_))
            ));
            let posts = server.await.unwrap();
            assert_eq!(posts.len(), 2);
            let start = &posts[0]["input"];
            let attach = &posts[1]["input"]["exec"];
            for request in [start, attach] {
                assert_eq!(request["workspace"], "ws-fixture");
                assert_eq!(request["sandbox"]["require"], true);
                assert_eq!(request["sandbox"]["profile"], "workspace");
                assert!(request.get("secret_slots").is_none());
                assert_eq!(request["env"]["allow"], json!([]));
                assert_eq!(
                    request["read_only_roots"],
                    json!([{"host_path":"/opt/mantle", "mount":"/opt/mantle"}])
                );
                assert!(request["limits"]["output_bytes"].as_u64().unwrap() > 0);
                assert!(request["limits"]["memory_bytes"].as_u64().unwrap() > 0);
                assert!(request["limits"]["processes"].as_u64().unwrap() > 0);
                assert!(request["lease_ttl_ms"].as_u64().unwrap() > 0);
                assert!(
                    request["env"]["set"]
                        .get("CLAUDE_CODE_OAUTH_TOKEN")
                        .is_none()
                );
            }
            assert_eq!(start["sandbox"]["network"], "aperture");
            assert_eq!(start["sandbox"]["aperture"], "egress");
            assert_eq!(start["limits"]["memory_bytes"], resolved.memory_bytes);
            assert_eq!(start["limits"]["processes"], resolved.pids);
            assert_eq!(
                start["env"]["set"]["CODEX_HOME"],
                "/workspace/.mantle/home/.codex"
            );
            assert_eq!(start["env"]["set"]["RUST_LOG"], "off");
            assert_eq!(start["env"]["set"]["CODEX_TUI_RECORD_SESSION"], "0");
            let argv = start["argv"].as_array().unwrap();
            assert!(argv.iter().any(|v| v == "/opt/mantle/bin/codex"));
            assert!(
                !argv
                    .iter()
                    .any(|v| v == "--secret-fd" || v == "--secret-env")
            );
            assert_eq!(attach["sandbox"]["network"], "none");
            assert_eq!(
                attach["argv"],
                json!([
                    "/opt/mantle/bin/mantle-launch",
                    "attach",
                    "--dir",
                    "/workspace/.mantle/agent"
                ])
            );
            assert_eq!(posts[1]["input"]["mode"], "pty");
            assert!(posts[1]["input"]["input_limit_bytes"].as_u64().unwrap() > 0);
            assert!(posts[1]["input"]["frame_limit_bytes"].as_u64().unwrap() > 0);
            assert!(posts[1]["input"]["queued_frames"].as_u64().unwrap() > 0);
        })
        .await
        .unwrap();
    }
}
