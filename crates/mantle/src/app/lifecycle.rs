//! Retention and admission use durable operation identities, never guessed process state.
use super::session::{RunRequest, StopPort, not_found};
use crate::adapters::state::{SessionRecord, Store, WorkerRecord, lifecycle::Attempt};
use crate::domain::session::SessionState;
use anyhow::{Context, Result, bail, ensure};
use b10x_substrate_sdk::{Client, ExecState, Operation, Workspace};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LaunchContext {
    pub version: u32,
    pub worker: WorkerRecord,
    pub request: RunRequest,
    // These bind all policy defaults used by version 1 to this persisted contract.
    pub cpu_time_secs: u64,
    pub output_bytes: u64,
}
impl LaunchContext {
    pub(crate) fn validate(&self) -> Result<()> {
        ensure!(
            self.version == 1
                && self.cpu_time_secs == b10x_substrate_sdk::MAX_EXEC_DURATION.as_secs()
                && self.output_bytes == b10x_substrate_sdk::MAX_IO_BYTES,
            "unsupported persisted launch-policy version or limits; refusing reconstruction"
        );
        Ok(())
    }
}
pub(crate) fn check_binding(
    store: &Store,
    record: &SessionRecord,
    selected: &WorkerRecord,
) -> Result<()> {
    ensure!(
        record.worker == selected.name,
        "session worker binding differs from selected worker"
    );
    if let Some(context) = store.attempt(&record.id)?.and_then(|a| a.launch_context) {
        let context: LaunchContext =
            serde_json::from_str(&context).context("invalid persisted launch context")?;
        context.validate()?;
        ensure!(
            &context.worker == selected,
            "worker instance or placement changed; session outcome remains unresolved"
        );
    }
    Ok(())
}
pub(crate) fn attachable(record: &SessionRecord) -> Result<()> {
    ensure!(
        record.state == SessionState::Running && record.agent_exec.is_some(),
        "no running agent recorded; use `mantle restart NAME` for a retained workspace (fresh process)"
    );
    Ok(())
}

pub(crate) async fn stop(
    port: &impl StopPort,
    store: &Store,
    record: &SessionRecord,
    destroy: bool,
) -> Result<()> {
    let current = store
        .session_by_id(&record.id)?
        .context("session vanished")?;
    ensure!(
        current.generation == record.generation
            && current.state == record.state
            && current.workspace == record.workspace
            && current.agent_exec == record.agent_exec,
        "stale session observation; retry from current state"
    );
    let destroy = destroy
        || matches!(
            record.state,
            SessionState::Stopping | SessionState::Destroying
        );
    if record.state == SessionState::Stopped {
        ensure!(destroy, "workspace was destroyed");
        return Ok(());
    }
    let (mut attempt, _) = store.claim(record, if destroy { "destroy" } else { "retain" })?;
    store.assert_attempt(&record.id, &attempt)?;
    if let Some(exec_id) = &record.agent_exec {
        match port.get_exec(exec_id).await {
            Ok(mut exec) => {
                let (observed, workspace) = port.binding(&exec);
                ensure!(
                    observed == exec_id && Some(workspace) == record.workspace.as_deref(),
                    "exec identity or workspace binding mismatch"
                );
                store.assert_attempt(&record.id, &attempt)?;
                if !port.terminal(&exec) {
                    port.signal(&mut exec).await?;
                    port.wait(&mut exec).await?;
                }
                ensure!(port.terminal(&exec), "agent termination remains unproven");
                let (observed, workspace) = port.binding(&exec);
                ensure!(
                    observed == exec_id && Some(workspace) == record.workspace.as_deref(),
                    "terminal exec binding mismatch"
                );
                attempt = store.terminal_proof(&record.id, &attempt, exec_id)?;
                ensure!(
                    port.retire(
                        exec,
                        attempt
                            .retirement_operation
                            .as_deref()
                            .context("retirement identity missing")?
                    )
                    .await?,
                    "agent retirement was not confirmed absent"
                );
            }
            Err(error) if not_found(&error) => {
                ensure!(
                    destroy
                        || (attempt.terminal_exec.as_deref() == Some(exec_id)
                            && attempt.retirement_operation.is_some()),
                    "agent is missing without durable terminal proof; stop remains unresolved"
                );
            }
            Err(error) => return Err(error.into()),
        }
    }
    store.assert_attempt(&record.id, &attempt)?;
    if destroy {
        super::session::destroy_workspace(port, record).await?;
    } else {
        port.get_workspace(
            record
                .workspace
                .as_deref()
                .context("session workspace binding missing")?,
        )
        .await
        .context("workspace is missing or unavailable; it will not be recreated")?;
    }
    store.lifecycle_write(
        &record.id,
        &attempt,
        if destroy {
            SessionState::Stopped
        } else {
            SessionState::Retained
        },
        None,
        if destroy {
            record.agent_exec.as_deref()
        } else {
            None
        },
        true,
    )?;
    println!(
        "Session        {} {}",
        record.name,
        if destroy {
            "destroyed"
        } else {
            "retained; restart starts a fresh agent process"
        }
    );
    Ok(())
}

pub(crate) trait AdmissionPort: StopPort {
    async fn operation(&self, id: &str) -> Result<Operation>;
    async fn admit(
        &self,
        workspace: &Self::Workspace,
        request: &RunRequest,
        operation: &str,
    ) -> Result<String>;
    async fn ready(&self, exec: &mut Self::Exec) -> Result<()>;
    fn running(&self, exec: &Self::Exec) -> bool;
}

/// A pending admission is not automatically idle. Resolve its operation and bound execution
/// first; known running/terminal executions can be stopped even if readiness never succeeded.
pub(crate) async fn cleanup(
    port: &impl AdmissionPort,
    store: &Store,
    record: &SessionRecord,
    destroy: bool,
) -> Result<()> {
    let current = store
        .session_by_id(&record.id)?
        .context("session vanished")?;
    ensure!(
        current.generation == record.generation
            && current.state == record.state
            && current.workspace == record.workspace
            && current.agent_exec == record.agent_exec,
        "stale session observation; retry from current state"
    );
    let Some(attempt) = store.attempt(&record.id)?.filter(|a| a.intent == "admit") else {
        return stop(port, store, record, destroy).await;
    };
    store.assert_attempt(&record.id, &attempt)?;
    let operation = port
        .operation(&attempt.operation_id)
        .await
        .context("pending admission cannot be resolved for cleanup")?;
    ensure!(
        operation.id == attempt.operation_id
            && operation.kind == "exec.start"
            && operation.refusal.is_none(),
        "pending admission operation binding mismatch or refusal"
    );
    let id = operation
        .resource
        .context("pending admission has no observed exec; cleanup remains unresolved")?;
    let exec = port
        .get_exec(&id)
        .await
        .context("pending admission exec unavailable; cleanup remains unresolved")?;
    let (observed, workspace) = port.binding(&exec);
    ensure!(
        observed == id
            && Some(workspace) == record.workspace.as_deref()
            && record
                .agent_exec
                .as_deref()
                .is_none_or(|recorded| recorded == id),
        "pending admission exec binding mismatch"
    );
    ensure!(
        port.running(&exec) || port.terminal(&exec),
        "pending admission process outcome remains unknown"
    );
    store.lifecycle_write(
        &record.id,
        &attempt,
        if port.running(&exec) {
            SessionState::Running
        } else {
            SessionState::FailedAgentStart
        },
        None,
        Some(&id),
        true,
    )?;
    let current = store
        .session_by_id(&record.id)?
        .context("session vanished after reconciliation")?;
    stop(port, store, &current, destroy).await
}
impl AdmissionPort for Client {
    async fn operation(&self, id: &str) -> Result<Operation> {
        Ok(self.operation(id).await?)
    }
    async fn admit(
        &self,
        workspace: &Workspace,
        request: &RunRequest,
        operation: &str,
    ) -> Result<String> {
        let exec = request
            .command(workspace)?
            .operation_id(operation)
            .start()
            .await?;
        ensure!(
            exec.observation().workspace == workspace.id(),
            "admitted exec workspace binding mismatch"
        );
        Ok(exec.id().to_owned())
    }
    async fn ready(&self, exec: &mut Self::Exec) -> Result<()> {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        exec.refresh().await?;
        Ok(())
    }
    fn running(&self, exec: &Self::Exec) -> bool {
        exec.observation().state == ExecState::Running
    }
}

pub(crate) async fn restart(
    port: &impl AdmissionPort,
    store: &Store,
    record: &SessionRecord,
) -> Result<()> {
    let current = store
        .attempt(&record.id)?
        .context("legacy session has no validated restart context")?;
    let context: LaunchContext = serde_json::from_str(
        current
            .launch_context
            .as_deref()
            .context("legacy session has no validated restart context")?,
    )?;
    context.validate()?;
    let (attempt, fresh) = store.claim(record, "admit")?;
    admit_recorded(port, store, record, &attempt, &context.request, fresh).await
}

pub(crate) async fn admit_recorded(
    port: &impl AdmissionPort,
    store: &Store,
    record: &SessionRecord,
    attempt: &Attempt,
    request: &RunRequest,
    fresh: bool,
) -> Result<()> {
    store.assert_attempt(&record.id, attempt)?;
    let workspace_id = record
        .workspace
        .as_deref()
        .context("session workspace binding missing")?;
    let workspace = port
        .get_workspace(workspace_id)
        .await
        .context("retained workspace is missing or unavailable; refusing recreation")?;
    let exec_id = if fresh {
        store.assert_attempt(&record.id, attempt)?;
        port.admit(&workspace, request, &attempt.operation_id)
            .await
            .context("agent admission unresolved; retry observes the recorded operation")?
    } else {
        // Not-found after reconnect does not prove that a start was never accepted.
        let operation = port
            .operation(&attempt.operation_id)
            .await
            .context("recorded admission cannot be resolved; refusing another start")?;
        ensure!(
            operation.id == attempt.operation_id && operation.kind == "exec.start",
            "recorded operation binding mismatch"
        );
        ensure!(
            operation.refusal.is_none()
                && operation.state != b10x_substrate_sdk::OperationState::Refused,
            "recorded admission was refused; outcome requires explicit inspection"
        );
        operation
            .resource
            .context("recorded admission has no observed exec; remains unresolved")?
    };
    let pending_state = if record.state == SessionState::Starting {
        SessionState::Starting
    } else {
        SessionState::Restarting
    };
    if fresh {
        // The admission response has already bound this ID to the selected workspace.
        // Persist it before another network call can fail or the caller can be interrupted.
        store.lifecycle_write(
            &record.id,
            attempt,
            pending_state,
            None,
            Some(&exec_id),
            false,
        )?;
    }
    let mut exec = port
        .get_exec(&exec_id)
        .await
        .context("admitted exec is unavailable; admission remains unresolved")?;
    let (observed, workspace) = port.binding(&exec);
    ensure!(
        observed == exec_id && workspace == workspace_id,
        "admitted exec identity or workspace binding mismatch"
    );
    ensure!(
        record
            .agent_exec
            .as_deref()
            .is_none_or(|previous| previous == exec_id),
        "recorded admission exec differs from operation resource"
    );
    // Record before readiness sleeps/refreshes; a killed caller can reconcile this exact admission.
    store.lifecycle_write(
        &record.id,
        attempt,
        pending_state,
        None,
        Some(&exec_id),
        false,
    )?;
    port.ready(&mut exec).await?;
    if !port.running(&exec) {
        bail!(
            "agent readiness not observed; execution is recorded and admission remains incomplete"
        );
    }
    store.lifecycle_write(
        &record.id,
        attempt,
        SessionState::Running,
        None,
        Some(&exec_id),
        true,
    )?;
    println!("Agent          {exec_id} running (fresh process, not conversation resume)");
    Ok(())
}
