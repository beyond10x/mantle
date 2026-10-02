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
use crate::domain::session::SessionState;

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
    let token = config.claude_token()?;
    let worker = worker(store)?;
    let connected = connect(config, &worker).await?;
    let machine = connected.client.machine();
    let missing = sub::missing_facts(&machine);
    if !missing.is_empty() {
        bail!("FAILED_CAPABILITY: the worker lacks {}", missing.join(", "));
    }
    install_token(&connected.ssh, &token)?;

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
    })?;
    println!("Session        {} ({id})", resolved.name);
    println!("Worker         {} {}", worker.region, worker.instance);
    println!("Manifest       {}", resolved.digest);

    let mut builder = connected
        .client
        .workspace()
        .empty()
        .label("mantle-session", id.as_str())
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
            println!("Storage        requested, NOT enforced: the worker serves no workspace quota")
        }
        _ => {}
    }
    let workspace = builder.create().await.context("creating the workspace")?;
    store.set_workspace(&id, workspace.id())?;
    println!("Workspace      {}", workspace.id());

    if let Err(error) = materialize(store, &id, &workspace, &resolved).await {
        store.move_session(
            &id,
            SessionState::FailedMaterialization,
            Some(&format!("{error:#}")),
        )?;
        return Err(error.context("FAILED_MATERIALIZATION"));
    }
    store.move_session(&id, SessionState::Starting, None)?;
    match start_agent(&workspace, &resolved).await {
        Ok(exec) => {
            store.set_agent_exec(&id, &exec)?;
            store.move_session(&id, SessionState::Running, None)?;
            println!(
                "Agent          {exec} (running, ends after {}h at the latest)",
                resolved.retain_for.as_secs() / 3600
            );
        }
        Err(error) => {
            store.move_session(
                &id,
                SessionState::FailedAgentStart,
                Some(&format!("{error:#}")),
            )?;
            return Err(error.context("FAILED_AGENT_START"));
        }
    }
    if attach {
        attach_workspace(&workspace, &resolved.name).await?;
    } else {
        println!("Attach with    mantle attach {}", resolved.name);
    }
    Ok(())
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

async fn materialize(
    store: &Store,
    id: &str,
    workspace: &Workspace,
    resolved: &Resolved,
) -> Result<()> {
    run_init(
        workspace,
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
            run_init(
                workspace,
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
            run_init(
                workspace,
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
            run_init(
                workspace,
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
        let commit = run_init(
            workspace,
            &["/usr/bin/git", "-C", &path, "rev-parse", "HEAD"],
            false,
        )
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
    let mut command = workspace
        .command("/opt/mantle/bin/mantle-launch")
        .args([
            "serve",
            "--dir",
            sub::AGENT_DIR,
            "--secret-fd",
            "3",
            "--secret-env",
            "CLAUDE_CODE_OAUTH_TOKEN",
            "--cwd",
            &resolved.agent_cwd,
            "--mkdir",
            "/workspace/.mantle/home",
            "--mkdir",
            "/workspace/.mantle/cargo",
            "--proxy",
            sub::PROXY,
            "--",
            "/opt/mantle/bin/claude",
        ])
        .aperture(sub::APERTURE)
        .read_only_root(sub::toolchain_root())
        .secret_slot(SecretSlotRequest {
            slot: sub::CLAUDE_SLOT.to_owned(),
            fd: sub::CLAUDE_FD,
        })
        .policy(sub::policy(
            resolved.retain_for,
            resolved.memory_bytes,
            resolved.pids,
        )?)
        .lease(resolved.retain_for);
    for (name, value) in sub::base_environment(resolved.cpu) {
        command = command.env(name, value);
    }
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

async fn attach_workspace(workspace: &Workspace, name: &str) -> Result<()> {
    let session = workspace
        .pty_session("/opt/mantle/bin/mantle-launch", terminal::local_window())
        .args(["attach", "--dir", sub::AGENT_DIR])
        .read_only_root(sub::toolchain_root())
        .env("TERM", "xterm-256color")
        .env("LANG", "C.UTF-8")
        .env("HOME", "/workspace/.mantle/home")
        .policy(sub::policy(ATTACH_TIMEOUT, 256 << 20, 32)?)
        .lease(ATTACH_TIMEOUT)
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
    attach_workspace(&workspace, name).await
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
    let mut command = workspace
        .command(program.as_str())
        .args(args.iter().map(String::as_str))
        .aperture(sub::APERTURE)
        .read_only_root(sub::toolchain_root())
        .policy(sub::policy(INIT_TIMEOUT, 4 << 30, 512)?);
    for (key, value) in sub::base_environment(cpu) {
        command = command.env(key, value);
    }
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
        "{:<24} {:<24} {:<30} CREATED",
        "NAME", "STATE (recorded)", "ID"
    );
    for session in sessions {
        println!(
            "{:<24} {:<24} {:<30} {}",
            session.name,
            session.state.to_string(),
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
    // A stop interrupted earlier resumes from STOPPING.
    if record.state != SessionState::Stopping {
        store.move_session(&record.id, SessionState::Stopping, None)?;
    }
    if let Some(exec_id) = &record.agent_exec {
        let mut exec = match connected.client.get_exec(exec_id).await {
            Ok(exec) => exec,
            Err(error) if not_found(&error) => {
                println!("Agent          {exec_id} already retired");
                return finish_stop(&connected, store, &record, name).await;
            }
            Err(error) => return Err(error.into()),
        };
        if !exec.observation().state.terminal() {
            exec.signal(
                b10x_substrate_sdk::Signal::Terminate,
                Duration::from_secs(10),
            )
            .await
            .context("signalling the agent")?;
            let observed = exec.wait_for(Duration::from_secs(60)).await?;
            println!("Agent          {:?} {:?}", observed.state, observed.exit);
        }
        exec.retire().await.context("retiring the agent exec")?;
    }
    finish_stop(&connected, store, &record, name).await
}

/// Destroys the workspace and records the stop. A workspace Substrate no longer has is one an
/// earlier, interrupted stop already destroyed.
async fn finish_stop(
    connected: &Connected,
    store: &Store,
    record: &SessionRecord,
    name: &str,
) -> Result<()> {
    if let Some(id) = &record.workspace {
        match connected.client.get_workspace(id).await {
            Ok(workspace) => match workspace.destroy().await {
                Ok(_) => println!("Workspace      destroyed"),
                // Destroying a workspace with a built tree outlasts the operation's answer
                // (observed 2026-10-02); the outcome is read back rather than assumed.
                Err(error) if outcome_unknown(&error) => {
                    wait_until_destroyed(connected, id).await?;
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
async fn wait_until_destroyed(connected: &Connected, id: &str) -> Result<()> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(300);
    loop {
        match connected.client.get_workspace(id).await {
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
