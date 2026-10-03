//! Local session and worker records in `~/.local/state/mantle/state.db`.
//!
//! Substrate resource ids are stored as opaque strings; nothing here interprets them.

use std::path::Path;

use anyhow::{Context, Result, bail};
use rusqlite::{Connection, OptionalExtension as _, params};

pub(crate) mod lifecycle;

use crate::domain::session::{
    AgentKind, AuthenticationMethod, SessionState, agent_name, auth_name, resolve_identity,
    validate_identity,
};

pub struct Store {
    connection: Connection,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorkerRecord {
    pub name: String,
    pub instance: String,
    pub region: String,
    pub data_volume: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SessionRecord {
    pub generation: i64,
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
    pub agent_kind: AgentKind,
    pub authentication: AuthenticationMethod,
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
    /// Diagnostics use SQLite's supported current committed view, including WAL. This permits
    /// ordinary SQLite lock/SHM coordination, but neither schema nor application writes.
    pub fn open_readonly(path: &Path) -> Result<Self> {
        let _file = crate::profile::inspect_regular(path, 64 * 1024 * 1024)?;
        for suffix in ["-wal", "-shm", "-journal"] {
            let mut name = path.as_os_str().to_os_string();
            name.push(suffix);
            let sidecar = std::path::PathBuf::from(name);
            match std::fs::symlink_metadata(&sidecar) {
                Ok(_) => {
                    crate::profile::inspect_regular(&sidecar, 64 * 1024 * 1024)?;
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
        }
        let connection = Connection::open_with_flags(
            path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NOFOLLOW,
        )?;
        connection.busy_timeout(std::time::Duration::from_millis(250))?;
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(250);
        connection.progress_handler(1000, Some(move || std::time::Instant::now() >= deadline));
        connection.pragma_update(None, "query_only", true)?;
        let kind: String = connection.query_row(
            "SELECT type FROM sqlite_schema WHERE name='workers'",
            [],
            |row| row.get(0),
        )?;
        anyhow::ensure!(kind == "table", "worker records must be a table");
        Ok(Self { connection })
    }

    pub fn open(path: &Path) -> Result<Self> {
        let mut connection =
            Connection::open(path).with_context(|| format!("opening {}", path.display()))?;
        Self::initialize(&mut connection)?;
        Ok(Self { connection })
    }

    fn initialize(connection: &mut Connection) -> Result<()> {
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        let tx = connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        tx.execute_batch(SCHEMA)
            .context("creating the state schema")?;
        let columns = tx
            .prepare("PRAGMA table_info(sessions)")?
            .query_map([], |row| row.get::<_, String>(1))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        match (columns.iter().any(|s| s == "agent_kind"), columns.iter().any(|s| s == "authentication")) {
            (false, false) => tx.execute_batch("ALTER TABLE sessions ADD COLUMN agent_kind TEXT NOT NULL DEFAULT 'claude-code'; ALTER TABLE sessions ADD COLUMN authentication TEXT NOT NULL DEFAULT 'claude-oauth';")?,
            (true, true) => {},
            _ => bail!("incomplete session identity schema"),
        }
        {
            let mut rows =
                tx.prepare("SELECT DISTINCT agent_kind, authentication FROM sessions")?;
            for row in rows.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })? {
                let (agent, auth) = row?;
                resolve_identity(&agent, Some(&auth))
                    .context("invalid session identity in state database")?;
            }
        }
        lifecycle::migrate(&tx)?;
        tx.commit()?;
        Ok(())
    }

    #[cfg(test)]
    fn in_memory() -> Result<Self> {
        let mut connection = Connection::open_in_memory()?;
        Self::initialize(&mut connection)?;
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
        validate_identity(&session.agent_kind, &session.authentication)?;
        let result = self.connection.execute(
            "INSERT INTO sessions(id, name, worker, state, manifest_digest, workspace, agent_exec,
                                  requested_json, created_at, failure, agent_kind, authentication)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
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
                agent_name(&session.agent_kind),
                auth_name(&session.authentication),
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
    #[cfg_attr(not(test), allow(dead_code))] // Retained ESS local-store command boundary.
    pub fn move_session(&self, id: &str, next: SessionState, failure: Option<&str>) -> Result<()> {
        let tx = rusqlite::Transaction::new_unchecked(
            &self.connection,
            rusqlite::TransactionBehavior::Immediate,
        )?;
        let current = self
            .session_by_id(id)?
            .context("the session record vanished")?;
        current.state.checked_move(next)?;
        anyhow::ensure!(tx.execute(
            "UPDATE sessions SET state = ?2, failure = COALESCE(?3, failure) WHERE id = ?1 AND state=?4",
            params![id, next.to_string(), failure,current.state.to_string()],
        )?==1,"stale session transition");
        tx.commit()?;
        Ok(())
    }

    #[cfg_attr(not(test), allow(dead_code))] // Retained ESS legacy local-store command boundary.
    pub fn set_workspace(&self, id: &str, workspace: &str) -> Result<()> {
        self.connection.execute(
            "UPDATE sessions SET workspace = ?2 WHERE id = ?1",
            params![id, workspace],
        )?;
        Ok(())
    }

    #[cfg_attr(not(test), allow(dead_code))] // Runtime completions use lifecycle_write with a fence.
    pub fn set_agent_exec(&self, id: &str, exec: &str) -> Result<()> {
        self.connection.execute(
            "UPDATE sessions SET agent_exec = ?2 WHERE id = ?1",
            params![id, exec],
        )?;
        Ok(())
    }

    pub fn put_source(&self, session: &str, source: &SourceRecord) -> Result<usize> {
        let affected = self.connection.execute(
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
        Ok(affected)
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

    pub(crate) fn session_by_id(&self, id: &str) -> Result<Option<SessionRecord>> {
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
        requested_json, created_at, failure, agent_kind, authentication,
        COALESCE((SELECT generation FROM session_lifecycle WHERE session_id=sessions.id),0) FROM sessions";

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
    String,
    String,
    i64,
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
        row.get(10)?,
        row.get(11)?,
        row.get(12)?,
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
        agent,
        auth,
        generation,
    ) = raw;
    let (agent_kind, authentication) = resolve_identity(&agent, Some(&auth))
        .context("invalid session identity in state database")?;
    Ok(SessionRecord {
        generation,
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
        agent_kind,
        authentication,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readonly_diagnostics_observe_wal_and_refuse_writes_without_migrating_legacy_schema() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("state.db");
        assert!(Store::open_readonly(&path).is_err());
        assert!(!path.exists());
        let writer = Connection::open(&path).unwrap();
        writer.execute_batch("CREATE TABLE workers(name TEXT PRIMARY KEY,instance TEXT,region TEXT,data_volume TEXT); PRAGMA journal_mode=WAL; INSERT INTO workers VALUES('default','fresh-wal','kubevirt/fixture/fixture',NULL)").unwrap();
        let store = Store::open_readonly(&path).unwrap();
        assert_eq!(
            store.worker("default").unwrap().unwrap().instance,
            "fresh-wal"
        );
        assert!(
            store
                .connection
                .execute("UPDATE workers SET instance='changed'", [])
                .is_err()
        );
        assert!(
            store
                .connection
                .execute("CREATE TABLE forbidden(value TEXT)", [])
                .is_err()
        );
        assert_eq!(writer.query_row("SELECT COUNT(*) FROM sqlite_master WHERE name IN ('sessions','sources','forbidden')",[],|r|r.get::<_,i64>(0)).unwrap(),0);
        assert_eq!(
            writer
                .query_row("SELECT instance FROM workers", [], |r| r
                    .get::<_, String>(0))
                .unwrap(),
            "fresh-wal"
        );
        assert!(store.connection.query_row("WITH RECURSIVE x(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM x) SELECT sum(n) FROM x",[],|r|r.get::<_,i64>(0)).is_err(),"query VM budget must interrupt unbounded read");
    }

    fn session(id: &str, name: &str) -> SessionRecord {
        SessionRecord {
            generation: 0,
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
            agent_kind: AgentKind::V0,
            authentication: AuthenticationMethod::V1,
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

#[cfg(test)]
#[path = "conformance.rs"]
mod conformance;

#[cfg(test)]
#[path = "orchestration.rs"]
mod orchestration;
