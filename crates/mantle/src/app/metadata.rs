//! Bounded metadata projection. No terminal pages, raw errors, or writable state initialization.
use super::worker;
use crate::{
    adapters::{
        aws,
        kubevirt::Kubevirt,
        ssh::Ssh,
        state::{SessionRecord, Store},
        substrate,
    },
    config::{Config, Provider, RuntimeContext},
    profile::Selection,
};
use anyhow::{Context, Result, ensure};
use mantle_acceptance::{Metadata, Observation, SelectionBinding, Session};
use std::time::Duration;
pub(crate) fn binding(selection: &Selection) -> Result<SelectionBinding> {
    configuration(selection).map(|(binding, _)| binding)
}
fn configuration(selection: &Selection) -> Result<(SelectionBinding, Config)> {
    let text = crate::profile::read_bounded(&selection.config_path, 1024 * 1024)?;
    let config = Config::parse(&text)?;
    ensure!(
        selection.config_path.to_str().is_some() && selection.state_dir.to_str().is_some(),
        "non-UTF8 selection"
    );
    Ok((
        SelectionBinding {
            profile: selection.profile.clone(),
            config_path: selection.config_path.clone(),
            state_dir: selection.state_dir.clone(),
            config_sha256: mantle_artifact::sha256(text.as_bytes()),
        },
        config,
    ))
}
pub(crate) fn record(store: &Store, row: &SessionRecord) -> Result<Session> {
    for text in [&row.id, &row.name]
        .into_iter()
        .chain(row.workspace.iter())
        .chain(row.agent_exec.iter())
    {
        ensure!(
            !text.is_empty()
                && text.len() <= 255
                && text
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"_-.".contains(&c)),
            "invalid metadata identity"
        );
    }
    let source_commits = store
        .sources(&row.id)?
        .into_iter()
        .map(|s| s.commit)
        .collect::<Vec<_>>();
    ensure!(
        source_commits.len() <= 128
            && source_commits
                .iter()
                .all(|s| (s.len() == 40 || s.len() == 64)
                    && s.bytes().all(|b| b.is_ascii_hexdigit())),
        "invalid source identity"
    );
    Ok(Session {
        id: row.id.clone(),
        name: row.name.clone(),
        agent: crate::domain::session::agent_name(&row.agent_kind).into(),
        authentication: crate::domain::session::auth_name(&row.authentication).into(),
        recorded_state: row.state.to_string(),
        generation: row.generation,
        workspace_id: row.workspace.clone(),
        exec_id: row.agent_exec.clone(),
        source_commits,
        observed: None,
    })
}
pub(crate) async fn report(
    selection: Result<Selection>,
    name: Option<&str>,
    id: Option<&str>,
    timeout: Duration,
) -> Metadata {
    let mut report = Metadata::empty();
    let Ok(selection) = selection else {
        return report;
    };
    report.outcome = "configuration-refused".into();
    let Ok((binding, config)) = configuration(&selection) else {
        return report;
    };
    report.selection = Some(binding);
    let context = RuntimeContext { config, selection };
    report.outcome = "state-unavailable".into();
    // Finish all local queries before any remote wait: readonly SQLite has a fixed query deadline.
    let snapshot = (|| -> Result<_> {
        let store = Store::open_readonly(&context.selection.state_dir.join("state.db"))?;
        let rows = if let Some(id) = id {
            vec![store.session_by_id(id)?.context("missing")?]
        } else if let Some(name) = name {
            vec![store.selected_session(name, None)?]
        } else {
            store.metadata_sessions()?
        };
        let records = rows
            .iter()
            .map(|r| record(&store, r))
            .collect::<Result<Vec<_>>>()?;
        Ok((records, store.worker(worker::WORKER)?))
    })();
    let Ok((records, worker)) = snapshot else {
        return report;
    };
    report.sessions = records;
    report.outcome = "recorded".into();
    if name.is_none() && id.is_none() {
        return bounded(report);
    }
    if report.sessions[0].recorded_state == "STOPPED" {
        return bounded(report);
    }
    report.outcome = "observation-unavailable".into();
    let observed = async {
        let worker = worker.context("no worker")?;
        ensure!(
            worker.region == worker::location(&context)?,
            "placement mismatch"
        );
        let proxy = match context.provider {
            Provider::Aws => aws::proxy_command(context.aws()?),
            Provider::Kubevirt => Kubevirt::new(context.kubevirt()?, &context.ubuntu_serial)
                .proxy_command(worker::WORKER),
        };
        let ssh = Ssh::readonly(&worker.instance, proxy, &context.selection.state_dir)?;
        let tunnel = ssh.diagnostic_tunnel(timeout)?;
        let result = tokio::time::timeout(timeout, async {
            let client = substrate::connect(tunnel.socket()).await?;
            let row = &report.sessions[0];
            let mut observation = Observation {
                workspace_state: None,
                exec_state: None,
                exit_code: None,
                exit_signal: None,
                refused: false,
            };
            if let Some(id) = &row.workspace_id {
                let ws = client.get_workspace(id).await?;
                ensure!(ws.id() == id, "workspace mismatch");
                observation.workspace_state =
                    Some(format!("{:?}", ws.observation().state).to_lowercase());
            }
            if let Some(id) = &row.exec_id {
                let exec = client.get_exec(id).await?;
                let value = exec.observation();
                ensure!(
                    value.id == *id && Some(&value.workspace) == row.workspace_id.as_ref(),
                    "exec mismatch"
                );
                observation.exec_state = Some(format!("{:?}", value.state).to_lowercase());
                observation.exit_code = value.exit.as_ref().and_then(|e| e.code);
                observation.exit_signal = value
                    .exit
                    .as_ref()
                    .and_then(|e| e.signal)
                    .map(|s| format!("{s:?}"));
                observation.refused = value.refusal.is_some();
            }
            Ok::<_, anyhow::Error>(observation)
        })
        .await;
        tunnel.finish()?;
        result.context("observation timeout")?
    }
    .await;
    match observed {
        Ok(observed) => {
            report.sessions[0].observed = Some(observed);
            report.outcome = "observed".into();
        }
        Err(error) => {
            report.outcome = match error.downcast_ref::<b10x_substrate_sdk::SdkError>() {
                Some(b10x_substrate_sdk::SdkError::Refusal(_)) => "observation-refused",
                Some(
                    b10x_substrate_sdk::SdkError::Protocol(_)
                    | b10x_substrate_sdk::SdkError::ContractMismatch { .. },
                ) => "protocol-refused",
                _ => "observation-unavailable",
            }
            .into();
        }
    }
    report.observed_at = mantle_acceptance::now();
    bounded(report)
}
fn bounded(mut report: Metadata) -> Metadata {
    if mantle_acceptance::encoded(&report).is_err() {
        report.sessions.clear();
        report.outcome = "metadata-bound-exceeded".into();
    }
    report
}
