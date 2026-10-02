//! Executes ESS's generated local scenarios against the production SQLite store and allowlist.
//! This adapter translates calls and reads persisted rows; it never evaluates ESS guards.
use anyhow::{Context, Result, bail, ensure};
use mantle_conformance::{Boundary, Reply, integer};
use mantle_egress::{Allowlist, Destination};
use serde_json::{Value, json};

use super::{SessionRecord, SourceRecord, Store};
use crate::domain::session::SessionState;

fn text(value: &Value) -> Result<&str> {
    value.as_str().context("expected text")
}

fn row(record: SessionRecord) -> Value {
    json!({
        "session_id": record.id, "name": record.name, "state": format!("{:?}", record.state),
        "worker": record.worker, "manifest_digest": record.manifest_digest,
        "workspace": record.workspace, "agent_exec": record.agent_exec,
        "requested_json": record.requested_json, "created_at": record.created_at,
        "failure": record.failure,
    })
}

fn query(store: &Store, view: &str) -> Result<Value> {
    let records = match view {
        "mantle.session.LiveSessions" => store.live_sessions()?,
        // session_by_id underlies this internal view. Enumerating ids is test-only observation.
        "mantle.session.SessionRecords" => {
            let mut statement = store
                .connection
                .prepare("SELECT id FROM sessions ORDER BY id")?;
            let ids = statement
                .query_map([], |r| r.get::<_, String>(0))?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            ids.iter()
                .map(|id| store.session_by_id(id)?.context("row vanished"))
                .collect::<Result<Vec<_>>>()?
        }
        other => bail!("unsupported view {other}"),
    };
    Ok(Value::Array(records.into_iter().map(row).collect()))
}

fn execute(store: &Store, command: &str, input: &Value) -> Result<Reply> {
    if command.starts_with("mantle.orchestration.") {
        return super::orchestration::execute(command, input);
    }
    let mut reply = Reply::default();
    if command.starts_with("mantle.manifest.") {
        return manifest(command, input);
    }
    if command == "mantle.session.PutSource" {
        let affected = store.put_source(
            text(&input["key"]["session_id"])?,
            &SourceRecord {
                mount: text(&input["key"]["mount"])?.into(),
                name: text(&input["name"])?.into(),
                repository: text(&input["repository"])?.into(),
                declared_ref: text(&input["declared_ref"])?.into(),
                commit: text(&input["commit"])?.into(),
            },
        );
        return Ok(Reply::returned(match affected {
            Ok(affected) => json!({"affected": affected, "diagnostic": null}),
            Err(error) => json!({"affected": 0, "diagnostic": format!("{error:#}")}),
        }));
    }
    if command == "mantle.session.ReadSources" {
        let id = text(&input["session_id"])?;
        let rows: Vec<_> = store
            .sources(id)?
            .into_iter()
            .map(|s| {
                json!({
                    "session_id": id, "mount": s.mount, "name": s.name,
                    "repository": s.repository, "declared_ref": s.declared_ref, "commit": s.commit,
                })
            })
            .collect();
        return Ok(Reply::returned(json!({"rows": rows})));
    }
    if command == "mantle.egress.CheckDefaultDestination" {
        let port = input["port"].as_f64().context("port is not numeric")?;
        ensure!(port.fract() == 0.0, "port is not an integer");
        let destination = Destination {
            host: text(&input["host"])?.into(),
            port: port.to_string().parse()?,
        };
        if Allowlist::default_list().permits(&destination) {
            reply.outcome = "allowed".into();
        } else {
            reply.outcome = "denied".into();
            reply.error = Some("mantle.egress.NotAllowed".into());
        }
        return Ok(reply);
    }
    let id = text(&input["session_id"])?;
    if command == "mantle.session.InsertSession" {
        let result = store.insert_session(&SessionRecord {
            id: id.into(),
            name: text(&input["name"])?.into(),
            worker: text(&input["worker"])?.into(),
            state: SessionState::Materializing,
            manifest_digest: text(&input["manifest_digest"])?.into(),
            workspace: None,
            agent_exec: None,
            requested_json: text(&input["requested_json"])?.into(),
            created_at: text(&input["created_at"])?.into(),
            failure: None,
        });
        if let Err(error) = result {
            ensure!(
                error.to_string().starts_with("a session named "),
                "{error:#}"
            );
            reply.outcome = "conflict".into();
            reply.error = Some("mantle.session.RecordConflict".into());
            return Ok(reply);
        }
        reply.outcome = "recorded".into();
        reply.events.insert(
            "mantle.session.SessionRecorded".into(),
            json!({"session_id": id}),
        );
        return Ok(reply);
    }
    if matches!(
        command,
        "mantle.session.SetWorkspace" | "mantle.session.SetAgentExec"
    ) {
        if command == "mantle.session.SetWorkspace" {
            store.set_workspace(id, text(&input["workspace"])?)?;
        } else {
            store.set_agent_exec(id, text(&input["agent_exec"])?)?;
        }
        // Zero-row UPDATE is accepted; inspect persistence rather than assuming a write.
        if store.session_by_id(id)?.is_some() {
            reply.outcome = "recorded".into();
            reply.events.insert(
                "mantle.session.SessionChanged".into(),
                json!({"session_id": id}),
            );
        } else {
            reply.outcome = "missing".into();
        }
        return Ok(reply);
    }
    let next = match command {
        "mantle.session.Materialized" => SessionState::Starting,
        "mantle.session.MaterializationFailed" => SessionState::FailedMaterialization,
        "mantle.session.AgentStarted" => SessionState::Running,
        "mantle.session.AgentStartFailed" => SessionState::FailedAgentStart,
        "mantle.session.BeginStop" => SessionState::Stopping,
        "mantle.session.FinishStop" => SessionState::Stopped,
        other => bail!("unsupported command {other}"),
    };
    match store.move_session(id, next, input["failure"].as_str()) {
        Ok(()) => {
            reply.outcome = "moved".into();
            // Logical write facts, not an invented production event bus.
            reply.events.insert(
                "mantle.session.SessionChanged".into(),
                json!({"session_id": id}),
            );
        }
        Err(error) => {
            let message = error.to_string();
            if message == "the session record vanished" {
                reply.outcome = "missing".into();
                reply.error = Some("mantle.session.RecordVanished".into());
            } else if message.starts_with("a session cannot move from ") {
                reply.outcome = "wrong-state".into();
                reply.error = Some("mantle.session.InvalidTransition".into());
            } else {
                return Err(error);
            }
        }
    }
    Ok(reply)
}

fn manifest(command: &str, input: &Value) -> Result<Reply> {
    use crate::domain::manifest as m;
    if command == "mantle.manifest.Parse" {
        return Ok(Reply::returned(match m::parse(text(&input["text"])?) {
            Ok(r) => json!({"diagnostic": null, "resolved": {
                "name": r.name, "repositories": r.repositories.iter().map(|r| json!({
                    "name": r.name, "repository": r.repository, "reference": r.reference, "mount": r.mount
                })).collect::<Vec<_>>(), "agent_cwd": r.agent_cwd, "cpu": r.cpu,
                "memory_bytes": r.memory_bytes, "pids": r.pids, "storage_bytes": r.storage_bytes,
                "retain_for_secs": r.retain_for.as_secs(), "digest": r.digest
            }}),
            Err(e) => json!({"resolved": null, "diagnostic": format!("{e:#}")}),
        }));
    }
    let answer = match command {
        "mantle.manifest.ValidateMemory" => integer(&input["bytes"])
            .and_then(|v| Ok(u64::try_from(v)?))
            .and_then(m::validate_memory),
        "mantle.manifest.ValidatePids" => integer(&input["pids"])
            .and_then(|v| Ok(u32::try_from(v)?))
            .and_then(m::validate_pids),
        "mantle.manifest.ValidateRetention" => integer(&input["seconds"])
            .and_then(|v| Ok(u64::try_from(v)?))
            .and_then(|v| m::validate_retention(std::time::Duration::from_secs(v))),
        _ => bail!("unsupported manifest command {command}"),
    };
    Ok(if answer.is_ok() {
        Reply {
            outcome: "accepted".into(),
            ..Reply::default()
        }
    } else {
        Reply {
            outcome: "refused".into(),
            error: Some("mantle.manifest.InvalidResource".into()),
            ..Reply::default()
        }
    })
}

struct CliBoundary(Store);
impl Default for CliBoundary {
    fn default() -> Self {
        Self(Store::in_memory().expect("conformance SQLite"))
    }
}
impl Boundary for CliBoundary {
    fn external(&mut self, outcome: &str) -> Result<()> {
        ensure!(
            outcome == "mantle.session.InsertSession/conflict",
            "unsupported arrangement {outcome}"
        );
        self.0.insert_session(&SessionRecord {
            id: "pre-existing-session".into(),
            name: "conformance-session".into(),
            worker: "w".into(),
            state: SessionState::Materializing,
            manifest_digest: "existing".into(),
            workspace: None,
            agent_exec: None,
            requested_json: "{}".into(),
            created_at: "2026-10-02T00:00:00Z".into(),
            failure: None,
        })
    }
    fn execute(&mut self, command: &str, input: &Value) -> Result<Reply> {
        execute(&self.0, command, input)
    }
    fn query(&self, view: &str) -> Result<Value> {
        query(&self.0, view)
    }
}
#[test]
fn ess_generated_local_conformance() -> Result<()> {
    mantle_conformance::run::<CliBoundary>("mantle-cli", 62)
}
