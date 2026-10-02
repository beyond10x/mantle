//! Executes ESS's generated local scenarios against the production SQLite store and allowlist.
//! This adapter translates calls and reads persisted rows; it never evaluates ESS guards.
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail, ensure};
use mantle_egress::{Allowlist, Destination};
use serde_json::{Value, json};

use super::{SessionRecord, Store};
use crate::domain::session::SessionState;

#[derive(Default)]
struct Reply {
    command: String,
    outcome: String,
    error: Option<String>,
    events: BTreeMap<String, Value>,
}

fn text(value: &Value) -> Result<&str> {
    value.as_str().context("expected text")
}

fn resolve(value: &Value, instances: &BTreeMap<String, Value>, reply: &Reply) -> Result<Value> {
    Ok(match text(&value["kind"])? {
        "literal" => value["value"].clone(),
        "instance" => instances
            .get(text(&value["instance"])?)
            .context("uncaptured instance")?
            .clone(),
        "observed" => reply
            .events
            .get(text(&value["event"])?)
            .context("unobserved event")?
            .get(text(&value["field"])?)
            .context("unobserved field")?
            .clone(),
        other => bail!("unsupported value expression {other}"),
    })
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
    let mut reply = Reply {
        command: command.into(),
        ..Reply::default()
    };
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
        store.insert_session(&SessionRecord {
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
        })?;
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

fn scenario(scenario: &Value) -> Result<()> {
    let store = Store::in_memory()?;
    let mut instances = BTreeMap::new();
    let mut snapshots = BTreeMap::new();
    let mut queries = BTreeMap::new();
    let mut reply = Reply::default();
    for step in scenario["steps"].as_array().context("no steps")? {
        match text(&step["step"])? {
            "execute_command" => {
                let input = step["input"]
                    .as_object()
                    .context("no input")?
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), resolve(v, &instances, &reply)?)))
                    .collect::<Result<serde_json::Map<_, _>>>()?;
                reply = execute(&store, text(&step["command"])?, &Value::Object(input))?;
            }
            "expect_outcome" => {
                ensure!(reply.command == text(&step["outcome"]["command"])?);
                ensure!(
                    reply.outcome == text(&step["outcome"]["outcome"])?,
                    "outcome: {}",
                    reply.outcome
                );
            }
            "expect_error" => ensure!(reply.error.as_deref() == Some(text(&step["error"])?)),
            "expect_no_error" => ensure!(reply.error.is_none()),
            "expect_no_events" => ensure!(reply.events.is_empty()),
            "expect_no_event" => ensure!(!reply.events.contains_key(text(&step["event"])?)),
            "expect_event" => {
                let event = reply
                    .events
                    .get(text(&step["event"])?)
                    .context("event missing")?;
                for (field, shape) in step["shape"].as_object().context("no event shape")? {
                    ensure!(
                        shape == &json!({"holds": "primitive", "kind": "string"}),
                        "unsupported event shape {shape}"
                    );
                    ensure!(event[field].is_string(), "bad event field {field}");
                }
            }
            "capture_instance" => {
                let event = reply
                    .events
                    .get(text(&step["event"])?)
                    .context("capture event missing")?;
                let value = event
                    .get(text(&step["field"])?)
                    .context("capture field missing")?;
                instances.insert(text(&step["instance"])?.to_owned(), value.clone());
            }
            "query_view" => {
                let view = text(&step["view"])?;
                queries.insert(view.to_owned(), query(&store, view)?);
            }
            "expect_view" => {
                let rows = queries
                    .get(text(&step["view"])?)
                    .context("view not queried")?
                    .as_array()
                    .context("view not rows")?;
                let fields = step["expectation"]["fields"]
                    .as_object()
                    .context("no expected fields")?;
                let expected = fields
                    .iter()
                    .map(|(k, v)| Ok((k, resolve(v, &instances, &reply)?)))
                    .collect::<Result<Vec<_>>>()?;
                let contains = rows
                    .iter()
                    .any(|r| expected.iter().all(|(k, v)| r.get(*k) == Some(v)));
                match text(&step["expectation"]["expect"])? {
                    "contains" => ensure!(contains, "view lacks {expected:?}; read {rows:?}"),
                    "excludes" => ensure!(!contains, "view unexpectedly contains {expected:?}"),
                    other => bail!("unsupported view expectation {other}"),
                }
            }
            "snapshot_view" | "snapshot_complete_subject" => {
                let view = text(&step["view"])?;
                let rows = query(&store, view)?;
                if step["step"] == "snapshot_complete_subject" {
                    let subject = step["subject"].as_object().context("no subject")?;
                    let (key, value) = subject.iter().next().context("empty subject")?;
                    let identity = resolve(value, &instances, &reply)?;
                    let record = rows
                        .as_array()
                        .context("no rows")?
                        .iter()
                        .find(|r| r.get(key) == Some(&identity))
                        .context("subject absent")?;
                    for field in step["shape"]["fields"]
                        .as_array()
                        .context("no shape fields")?
                    {
                        ensure!(
                            record.get(text(&field["name"])?).is_some(),
                            "snapshot missing field"
                        );
                    }
                }
                // Comparing all rows is stronger than comparing just the subject.
                snapshots.insert(view.to_owned(), rows);
            }
            "expect_view_unchanged" | "expect_complete_subject_unchanged" => {
                let view = text(&step["view"])?;
                ensure!(
                    snapshots.get(view).context("no snapshot")? == &query(&store, view)?,
                    "{view} mutated on refusal"
                );
            }
            other => bail!("unsupported step {other}"),
        }
    }
    Ok(())
}

#[test]
fn ess_generated_local_conformance() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let drafts = root.join(".engineering/drafts");
    std::fs::create_dir_all(&drafts)?;
    let suite_path = drafts.join("mantle-conformance.json");
    let output = Command::new("ess")
        .current_dir(&root)
        .args(["verify", "conform", "synthesize", "--path", "spec", "--out"])
        .arg(&suite_path)
        .output()?;
    ensure!(
        output.status.success(),
        "ESS synthesis: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let synthesis = String::from_utf8(output.stdout)?;
    let refusals: Vec<_> = synthesis
        .lines()
        .filter(|line| line.starts_with("refused:"))
        .collect();
    ensure!(
        refusals
            == [
                "refused: refusal[ESS-SYNTH-013]: type mantle.launch.ServeArgs has no scenario",
                "refused: refusal[ESS-SYNTH-013]: type mantle.manifest.Resolved has no scenario",
            ],
        "review changed synthesis coverage: {synthesis}"
    );
    eprintln!("{synthesis}");
    let suite: Value = serde_json::from_slice(&std::fs::read(&suite_path)?)?;
    ensure!(
        suite["provenance"]["suite_version"] == "ess-conformance/22",
        "review runner for new suite contract"
    );
    let scenarios = suite["scenarios"].as_object().context("no scenarios")?;
    ensure!(
        scenarios.len() == 57,
        "review obligation count: {}",
        scenarios.len()
    );
    let mut outcomes = BTreeMap::new();
    for (id, steps) in scenarios {
        let result = scenario(steps);
        outcomes.insert(
            id.clone(),
            match result {
                Ok(()) => json!({"status": "passed"}),
                Err(error) => json!({"status": "failed", "reason": format!("{error:#}")}),
            },
        );
    }
    let failed = outcomes
        .values()
        .filter(|r| r["status"] == "failed")
        .count();
    let report = json!({
        "format": "mantle-local-conformance/1", "spec_digest": suite["provenance"]["spec_digest"],
        "completed_at": chrono::Utc::now().to_rfc3339(),
        "scope": "SQLite session operations and normalized default allowlist only",
        "passed": scenarios.len() - failed, "failed": failed, "skipped": 0,
        "synthesis_refusals": refusals,
        "outcomes": outcomes,
    });
    std::fs::write(
        drafts.join("mantle-conformance-report.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    ensure!(failed == 0, "conformance failures: {outcomes:#?}");
    eprintln!(
        "{} local ESS scenarios passed; 0 failed; 0 skipped",
        scenarios.len()
    );
    Ok(())
}
