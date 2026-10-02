//! Executes ESS's generated local scenarios against the production SQLite store and allowlist.
//! This adapter translates calls and reads persisted rows; it never evaluates ESS guards.
use anyhow::{Context, Result, bail, ensure};
use mantle_conformance::{Boundary, Reply, integer};
use mantle_egress::{Allowlist, Destination};
use serde_json::{Value, json};

use super::{SessionRecord, SourceRecord, Store};
use crate::domain::session::{AgentKind, AuthenticationMethod, SessionState, validate_identity};

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
        "agent_kind": record.agent_kind, "authentication": record.authentication,
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
    if command == "mantle.session.IdentityMigration" {
        return Ok(Reply::returned(identity_migration()?));
    }
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
    if command == "mantle.session.AssessSharedBinaryUpgrade" {
        let flag = |name| {
            input[name]
                .as_bool()
                .context("upgrade input is not boolean")
        };
        reply.outcome = if mantle_worker::shared_upgrade_deferred(
            flag("shared_service_active")?,
            flag("installed")?,
            flag("artifact_changed")?,
        ) {
            "deferred"
        } else {
            "applicable"
        }
        .into();
        return Ok(reply);
    }
    if command == "mantle.session.AssessWorkerReadiness" {
        let flag = |name| {
            input[name]
                .as_bool()
                .context("readiness input is not boolean")
        };
        if mantle_worker::worker_ready(
            flag("common_ready")?,
            flag("claude_selected")?,
            flag("executable_present")?,
            flag("claude_slot_present")?,
        ) {
            reply.outcome = "ready".into();
        } else {
            reply.outcome = "missing".into();
            reply.error = Some("mantle.session.WorkerNotReady".into());
        }
        return Ok(reply);
    }
    if command == "mantle.session.AssessAgentInstallation" {
        let flag = |name| {
            input[name]
                .as_bool()
                .context("installation input is not boolean")
        };
        match mantle_worker::installation_decision(
            text(&input["architecture"])?,
            flag("current_verified")?,
            flag("archive_verified")?,
            flag("binary_verified")?,
            flag("version_verified")?,
        ) {
            mantle_worker::InstallationOutcome::V0 => reply.outcome = "already-current".into(),
            mantle_worker::InstallationOutcome::V1 => reply.outcome = "installed".into(),
            mantle_worker::InstallationOutcome::V2 => {
                reply.outcome = "refused".into();
                reply.error = Some("mantle.session.InstallationRefused".into());
            }
        }
        return Ok(reply);
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
        let agent_kind: AgentKind = serde_json::from_value(input["agent_kind"].clone())?;
        let authentication: AuthenticationMethod =
            serde_json::from_value(input["authentication"].clone())?;
        if validate_identity(&agent_kind, &authentication).is_err() {
            return Ok(Reply {
                outcome: "invalid-identity".into(),
                error: Some("mantle.session.InvalidIdentity".into()),
                ..Reply::default()
            });
        }
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
            agent_kind,
            authentication,
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
                "retain_for_secs": r.retain_for.as_secs(), "digest": r.digest,
                "agent_kind": r.agent_kind, "authentication": r.authentication
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
            agent_kind: AgentKind::V0,
            authentication: AuthenticationMethod::V1,
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
    mantle_conformance::run::<CliBoundary>("mantle-cli", 213)
}

fn identity_migration() -> Result<Value> {
    fn legacy_payload(connection: &rusqlite::Connection) -> Result<Vec<Vec<Option<String>>>> {
        let mut rows = Vec::new();
        for sql in [
            "SELECT id,name,worker,state,manifest_digest,workspace,agent_exec,requested_json,created_at,failure FROM sessions WHERE id IN ('live','stopped') ORDER BY id",
            "SELECT session_id,name,repository,declared_ref,commit_id,mount FROM sources ORDER BY session_id,mount",
            "SELECT name,instance,region,data_volume FROM workers ORDER BY name",
        ] {
            let mut statement = connection.prepare(sql)?;
            let columns = statement.column_count();
            rows.extend(
                statement
                    .query_map([], |r| {
                        (0..columns)
                            .map(|i| r.get(i))
                            .collect::<rusqlite::Result<Vec<Option<String>>>>()
                    })?
                    .collect::<rusqlite::Result<Vec<_>>>()?,
            );
        }
        Ok(rows)
    }
    let temp = tempfile::tempdir()?;
    let path = temp.path().join("legacy.db");
    let legacy = rusqlite::Connection::open(&path)?;
    legacy.execute_batch(super::SCHEMA)?;
    legacy.execute_batch("INSERT INTO sessions VALUES ('live','legacy','w','RUNNING','digest','ws','exec','{}','time',NULL); INSERT INTO sessions VALUES ('stopped','old','w','STOPPED','other',NULL,NULL,'{}','old-time','old failure'); INSERT INTO sources VALUES ('live','r','https://example.com/r.git','main','commit','r');")?;
    legacy.execute_batch("INSERT INTO workers VALUES ('w','instance','region','volume');")?;
    let original = legacy_payload(&legacy)?;
    drop(legacy);
    let store = Store::open(&path)?;
    let before = store.session_by_id("live")?.context("migrated row")?;
    let old = store.session_by_id("stopped")?.context("stopped row")?;
    let legacy_preserved = original == legacy_payload(&store.connection)?
        && before.agent_kind == AgentKind::V0
        && before.authentication == AuthenticationMethod::V1
        && before.state == SessionState::Running
        && before.workspace.as_deref() == Some("ws")
        && before.agent_exec.as_deref() == Some("exec")
        && before.manifest_digest == "digest"
        && old.state == SessionState::Stopped
        && old.failure.as_deref() == Some("old failure")
        && store.sources("live")?.len() == 1;
    let mut codex = before.clone();
    codex.id = "codex".into();
    codex.name = "codex".into();
    codex.agent_kind = AgentKind::V1;
    codex.authentication = AuthenticationMethod::V0;
    store.insert_session(&codex)?;
    let mut invalid = codex.clone();
    invalid.id = "invalid".into();
    invalid.name = "invalid".into();
    invalid.authentication = AuthenticationMethod::V1;
    let bad_pair_refused = store.insert_session(&invalid).is_err();
    drop(store);
    let store = Store::open(&path)?;
    let legacy_preserved = legacy_preserved && original == legacy_payload(&store.connection)?;
    let stored = store.session_by_id("codex")?.context("Codex row")?;
    let codex_preserved = stored.agent_kind == codex.agent_kind
        && stored.authentication == codex.authentication
        && store.live_sessions()?.len() == 2;
    let indexes_preserved = store.connection.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type='index' AND name='sessions_live_name'",
        [],
        |r| r.get::<_, i64>(0),
    )? == 1
        && store
            .insert_session(&SessionRecord {
                id: "duplicate".into(),
                ..before
            })
            .is_err();
    store.connection.execute(
        "UPDATE sessions SET agent_kind='unknown' WHERE id='codex'",
        [],
    )?;
    let unknown_refused = store.session_by_id("codex").is_err() && Store::open(&path).is_err();
    store.connection.execute(
        "UPDATE sessions SET agent_kind='codex', authentication='unknown' WHERE id='codex'",
        [],
    )?;
    let unknown_refused =
        unknown_refused && store.live_sessions().is_err() && Store::open(&path).is_err();
    store.connection.execute(
        "UPDATE sessions SET agent_kind='codex', authentication='claude-oauth' WHERE id='codex'",
        [],
    )?;
    let bad_pair_refused =
        bad_pair_refused && store.live_sessions().is_err() && Store::open(&path).is_err();
    Ok(
        json!({"legacy_preserved":legacy_preserved,"codex_preserved":codex_preserved,
        "indexes_preserved":indexes_preserved,"unknown_refused":unknown_refused,"bad_pair_refused":bad_pair_refused}),
    )
}
