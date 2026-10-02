//! Local session and worker records in `~/.local/state/mantle/state.db`.
//!
//! Substrate resource ids are stored as opaque strings; nothing here interprets them.

use std::path::Path;

use anyhow::{Context, Result, bail};
use rusqlite::{Connection, OptionalExtension as _, params};

use crate::domain::session::SessionState;

pub struct Store {
    connection: Connection,
}

#[derive(Debug, Clone)]
pub struct WorkerRecord {
    pub name: String,
    pub instance: String,
    pub region: String,
    pub data_volume: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SessionRecord {
    pub id: String,
    pub name: String,
    pub worker: String,
    pub state: SessionState,
    pub manifest_digest: String,
    pub workspace: Option<String>,
    pub agent_exec: Option<String>,
    pub requested_json: String,
    pub created_at: String,
    pub failure: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SourceRecord {
    pub name: String,
    pub repository: String,
    pub declared_ref: String,
    pub commit: String,
    pub mount: String,
}

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS workers (
    name TEXT PRIMARY KEY,
    instance TEXT NOT NULL,
    region TEXT NOT NULL,
    data_volume TEXT
);
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    worker TEXT NOT NULL,
    state TEXT NOT NULL,
    manifest_digest TEXT NOT NULL,
    workspace TEXT,
    agent_exec TEXT,
    requested_json TEXT NOT NULL,
    created_at TEXT NOT NULL,
    failure TEXT
);
CREATE UNIQUE INDEX IF NOT EXISTS sessions_live_name ON sessions(name) WHERE state <> 'STOPPED';
CREATE TABLE IF NOT EXISTS sources (
    session_id TEXT NOT NULL REFERENCES sessions(id),
    name TEXT NOT NULL,
    repository TEXT NOT NULL,
    declared_ref TEXT NOT NULL,
    commit_id TEXT NOT NULL,
    mount TEXT NOT NULL,
    PRIMARY KEY (session_id, mount)
);
";

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        let connection =
            Connection::open(path).with_context(|| format!("opening {}", path.display()))?;
        connection
            .execute_batch(SCHEMA)
            .context("creating the state schema")?;
        Ok(Self { connection })
    }

    #[cfg(test)]
    fn in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        connection.execute_batch(SCHEMA)?;
        Ok(Self { connection })
    }

    pub fn put_worker(&self, worker: &WorkerRecord) -> Result<()> {
        self.connection.execute(
            "INSERT INTO workers(name, instance, region, data_volume) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(name) DO UPDATE SET instance = ?2, region = ?3, data_volume = ?4",
            params![
                worker.name,
                worker.instance,
                worker.region,
                worker.data_volume
            ],
        )?;
        Ok(())
    }

    pub fn worker(&self, name: &str) -> Result<Option<WorkerRecord>> {
        Ok(self
            .connection
            .query_row(
                "SELECT name, instance, region, data_volume FROM workers WHERE name = ?1",
                params![name],
                |row| {
                    Ok(WorkerRecord {
                        name: row.get(0)?,
                        instance: row.get(1)?,
                        region: row.get(2)?,
                        data_volume: row.get(3)?,
                    })
                },
            )
            .optional()?)
    }

    pub fn insert_session(&self, session: &SessionRecord) -> Result<()> {
        let result = self.connection.execute(
            "INSERT INTO sessions(id, name, worker, state, manifest_digest, workspace, agent_exec,
                                  requested_json, created_at, failure)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                session.id,
                session.name,
                session.worker,
                session.state.to_string(),
                session.manifest_digest,
                session.workspace,
                session.agent_exec,
                session.requested_json,
                session.created_at,
                session.failure,
            ],
        );
        match result {
            Err(rusqlite::Error::SqliteFailure(error, _))
                if error.code == rusqlite::ErrorCode::ConstraintViolation =>
            {
                bail!(
                    "a session named {:?} already exists and is not stopped",
                    session.name
                )
            }
            other => {
                other?;
                Ok(())
            }
        }
    }

    /// Moves a session, refusing a transition the lifecycle does not allow.
    pub fn move_session(&self, id: &str, next: SessionState, failure: Option<&str>) -> Result<()> {
        let current = self
            .session_by_id(id)?
            .context("the session record vanished")?;
        current.state.checked_move(next)?;
        self.connection.execute(
            "UPDATE sessions SET state = ?2, failure = COALESCE(?3, failure) WHERE id = ?1",
            params![id, next.to_string(), failure],
        )?;
        Ok(())
    }

    pub fn set_workspace(&self, id: &str, workspace: &str) -> Result<()> {
        self.connection.execute(
            "UPDATE sessions SET workspace = ?2 WHERE id = ?1",
            params![id, workspace],
        )?;
        Ok(())
    }

    pub fn set_agent_exec(&self, id: &str, exec: &str) -> Result<()> {
        self.connection.execute(
            "UPDATE sessions SET agent_exec = ?2 WHERE id = ?1",
            params![id, exec],
        )?;
        Ok(())
    }

    pub fn put_source(&self, session: &str, source: &SourceRecord) -> Result<()> {
        self.connection.execute(
            "INSERT INTO sources(session_id, name, repository, declared_ref, commit_id, mount)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                session,
                source.name,
                source.repository,
                source.declared_ref,
                source.commit,
                source.mount
            ],
        )?;
        Ok(())
    }

    pub fn sources(&self, session: &str) -> Result<Vec<SourceRecord>> {
        let mut statement = self.connection.prepare(
            "SELECT name, repository, declared_ref, commit_id, mount FROM sources
             WHERE session_id = ?1 ORDER BY mount",
        )?;
        let rows = statement.query_map(params![session], |row| {
            Ok(SourceRecord {
                name: row.get(0)?,
                repository: row.get(1)?,
                declared_ref: row.get(2)?,
                commit: row.get(3)?,
                mount: row.get(4)?,
            })
        })?;
        rows.collect::<Result<_, _>>().map_err(Into::into)
    }

    /// The live (not stopped) session with this name.
    pub fn live_session(&self, name: &str) -> Result<Option<SessionRecord>> {
        self.one_session("WHERE name = ?1 AND state <> 'STOPPED'", name)
    }

    fn session_by_id(&self, id: &str) -> Result<Option<SessionRecord>> {
        self.one_session("WHERE id = ?1", id)
    }

    fn one_session(&self, clause: &str, value: &str) -> Result<Option<SessionRecord>> {
        let sql = format!("{SESSION_COLUMNS} {clause}");
        let row = self
            .connection
            .query_row(&sql, params![value], read_session)
            .optional()?;
        row.map(finish_session).transpose()
    }

    pub fn live_sessions(&self) -> Result<Vec<SessionRecord>> {
        let sql = format!("{SESSION_COLUMNS} WHERE state <> 'STOPPED' ORDER BY created_at");
        let mut statement = self.connection.prepare(&sql)?;
        let rows = statement.query_map([], read_session)?;
        rows.map(|row| finish_session(row?)).collect()
    }
}

const SESSION_COLUMNS: &str =
    "SELECT id, name, worker, state, manifest_digest, workspace, agent_exec,
        requested_json, created_at, failure FROM sessions";

type RawSession = (
    String,
    String,
    String,
    String,
    String,
    Option<String>,
    Option<String>,
    String,
    String,
    Option<String>,
);

fn read_session(row: &rusqlite::Row<'_>) -> rusqlite::Result<RawSession> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
        row.get(9)?,
    ))
}

fn finish_session(raw: RawSession) -> Result<SessionRecord> {
    let (
        id,
        name,
        worker,
        state,
        manifest_digest,
        workspace,
        agent_exec,
        requested_json,
        created_at,
        failure,
    ) = raw;
    Ok(SessionRecord {
        id,
        name,
        worker,
        state: state.parse()?,
        manifest_digest,
        workspace,
        agent_exec,
        requested_json,
        created_at,
        failure,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session(id: &str, name: &str) -> SessionRecord {
        SessionRecord {
            id: id.to_owned(),
            name: name.to_owned(),
            worker: "default".to_owned(),
            state: SessionState::Materializing,
            manifest_digest: "sha256:0".to_owned(),
            workspace: None,
            agent_exec: None,
            requested_json: "{}".to_owned(),
            created_at: "2026-10-02T00:00:00Z".to_owned(),
            failure: None,
        }
    }

    #[test]
    fn a_live_name_is_unique_until_stopped() {
        let store = Store::in_memory().expect("store");
        store.insert_session(&session("a", "s1")).expect("first");
        assert!(store.insert_session(&session("b", "s1")).is_err());
        store
            .move_session("a", SessionState::Stopping, None)
            .expect("stopping");
        store
            .move_session("a", SessionState::Stopped, None)
            .expect("stopped");
        store
            .insert_session(&session("b", "s1"))
            .expect("name reusable");
        assert_eq!(
            store.live_session("s1").expect("query").expect("live").id,
            "b"
        );
    }

    #[test]
    fn an_illegal_move_is_refused_and_not_written() {
        let store = Store::in_memory().expect("store");
        store.insert_session(&session("a", "s1")).expect("insert");
        assert!(
            store
                .move_session("a", SessionState::Running, None)
                .is_err()
        );
        let live = store.live_session("s1").expect("query").expect("live");
        assert_eq!(live.state, SessionState::Materializing);
    }

    #[test]
    fn sources_round_trip() {
        let store = Store::in_memory().expect("store");
        store.insert_session(&session("a", "s1")).expect("insert");
        let source = SourceRecord {
            name: "substrate".to_owned(),
            repository: "https://example.com/r.git".to_owned(),
            declared_ref: "main".to_owned(),
            commit: "4b7f".repeat(10),
            mount: "substrate".to_owned(),
        };
        store.put_source("a", &source).expect("put");
        let read = store.sources("a").expect("read");
        assert_eq!(read.len(), 1);
        assert_eq!(read[0].commit, source.commit);
    }
}
