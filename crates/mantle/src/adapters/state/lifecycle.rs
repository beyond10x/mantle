//! Durable, short SQLite claims. No transaction spans remote IO.
use super::*;
use anyhow::ensure;

#[derive(Clone, Debug)]
pub(crate) struct Attempt {
    pub generation: i64,
    pub intent: String,
    pub operation_id: String,
    pub launch_context: Option<String>,
    pub terminal_exec: Option<String>,
    pub retirement_operation: Option<String>,
}

pub(super) fn migrate(tx: &rusqlite::Transaction<'_>) -> Result<()> {
    let version: i64 = tx.pragma_query_value(None, "user_version", |row| row.get(0))?;
    ensure!(version <= 1, "unsupported session state schema version");
    tx.execute_batch(
        "CREATE TABLE IF NOT EXISTS session_lifecycle (
        session_id TEXT PRIMARY KEY REFERENCES sessions(id),
        generation INTEGER NOT NULL,
        intent TEXT NOT NULL,
        operation_id TEXT NOT NULL,
        launch_context TEXT,
        terminal_exec TEXT,
        retirement_operation TEXT
    );",
    )?;
    if version == 0 {
        // STOPPING was destructive in 0.1.4. Convert exactly once, never reinterpret it.
        tx.execute(
            "UPDATE sessions SET state='DESTROYING' WHERE state='STOPPING'",
            [],
        )?;
        tx.execute(
            "INSERT OR IGNORE INTO session_lifecycle(session_id,generation,intent,operation_id)
                    SELECT id,1,'destroy','legacy-' || id FROM sessions WHERE state='DESTROYING'",
            [],
        )?;
        tx.pragma_update(None, "user_version", 1)?;
    }
    Ok(())
}

fn read(connection: &Connection, id: &str) -> Result<Option<Attempt>> {
    Ok(connection.query_row("SELECT generation,intent,operation_id,launch_context,terminal_exec,retirement_operation FROM session_lifecycle WHERE session_id=?1", [id], |row| Ok(Attempt {
        generation: row.get(0)?, intent: row.get(1)?, operation_id: row.get(2)?,
        launch_context: row.get(3)?, terminal_exec: row.get(4)?, retirement_operation: row.get(5)?,
    })).optional()?)
}

impl Store {
    pub(crate) fn attempt(&self, id: &str) -> Result<Option<Attempt>> {
        read(&self.connection, id)
    }

    pub(crate) fn initialize_session(&self, id: &str, context: &str) -> Result<Attempt> {
        let tx = rusqlite::Transaction::new_unchecked(
            &self.connection,
            rusqlite::TransactionBehavior::Immediate,
        )?;
        let state: String =
            tx.query_row("SELECT state FROM sessions WHERE id=?1", [id], |r| r.get(0))?;
        ensure!(
            state == "MATERIALIZING",
            "initialization no longer owns this session"
        );
        tx.execute("INSERT INTO session_lifecycle(session_id,generation,intent,operation_id,launch_context) VALUES(?1,1,'initialize',?2,?3)", params![id, operation_id(), context])?;
        let attempt = read(&tx, id)?.context("initialization claim vanished")?;
        tx.commit()?;
        Ok(attempt)
    }

    pub(crate) fn assert_attempt(&self, id: &str, expected: &Attempt) -> Result<()> {
        let actual = self.attempt(id)?.context("lifecycle ownership vanished")?;
        ensure!(
            actual.generation == expected.generation
                && actual.intent == expected.intent
                && actual.operation_id == expected.operation_id,
            "stale lifecycle attempt; observe the current session before retrying"
        );
        Ok(())
    }

    pub(crate) fn claim(&self, record: &SessionRecord, intent: &str) -> Result<(Attempt, bool)> {
        let tx = rusqlite::Transaction::new_unchecked(
            &self.connection,
            rusqlite::TransactionBehavior::Immediate,
        )?;
        let current = self
            .session_by_id(&record.id)?
            .context("session vanished")?;
        ensure!(
            current.generation == record.generation
                && current.state == record.state
                && current.workspace == record.workspace
                && current.agent_exec == record.agent_exec,
            "stale session observation; retry from current state"
        );
        let old = read(&tx, &record.id)?;
        if let Some(old) = &old {
            if old.intent == intent {
                tx.commit()?;
                return Ok((old.clone(), false));
            }
            ensure!(
                old.intent == "idle",
                "session has pending {} intent; reconcile it before {intent}",
                old.intent
            );
        }
        let next = match intent {
            "admit" => {
                ensure!(
                    record.state == SessionState::Retained,
                    "restart requires a retained workspace"
                );
                SessionState::Restarting
            }
            "retain" => {
                ensure!(
                    record.agent_exec.is_some() || record.state == SessionState::Retained,
                    "agent termination is unproven; initialization remains incomplete"
                );
                SessionState::Retaining
            }
            "destroy" => SessionState::Destroying,
            _ => bail!("unsupported lifecycle intent"),
        };
        record.state.checked_move(next)?;
        let generation = old.as_ref().map_or(1, |a| a.generation + 1);
        let context = old.and_then(|a| a.launch_context);
        tx.execute("INSERT INTO session_lifecycle(session_id,generation,intent,operation_id,launch_context) VALUES(?1,?2,?3,?4,?5)
            ON CONFLICT(session_id) DO UPDATE SET generation=excluded.generation,intent=excluded.intent,operation_id=excluded.operation_id,terminal_exec=NULL,retirement_operation=NULL", params![record.id, generation, intent, operation_id(), context])?;
        ensure!(
            tx.execute(
                "UPDATE sessions SET state=?2 WHERE id=?1 AND state=?3",
                params![record.id, next.to_string(), record.state.to_string()]
            )? == 1,
            "stale session claim"
        );
        let attempt = read(&tx, &record.id)?.context("claim vanished")?;
        tx.commit()?;
        Ok((attempt, true))
    }

    /// Used for initialization as well as lifecycle completions; CAS is checked under the write lock.
    pub(crate) fn lifecycle_write(
        &self,
        id: &str,
        attempt: &Attempt,
        state: SessionState,
        workspace: Option<&str>,
        exec: Option<&str>,
        complete: bool,
    ) -> Result<()> {
        let tx = rusqlite::Transaction::new_unchecked(
            &self.connection,
            rusqlite::TransactionBehavior::Immediate,
        )?;
        self.assert_attempt(id, attempt)?;
        ensure!(tx.execute("UPDATE sessions SET state=?2,workspace=COALESCE(?3,workspace),agent_exec=?4 WHERE id=?1",params![id,state.to_string(),workspace,exec])?==1,"session vanished during lifecycle completion");
        if complete {
            ensure!(tx.execute("UPDATE session_lifecycle SET intent='idle' WHERE session_id=?1 AND generation=?2 AND intent=?3",params![id,attempt.generation,attempt.intent])?==1,"stale lifecycle completion");
        }
        tx.commit()?;
        Ok(())
    }

    pub(crate) fn begin_initial_admission(&self, id: &str, attempt: &Attempt) -> Result<Attempt> {
        let tx = rusqlite::Transaction::new_unchecked(
            &self.connection,
            rusqlite::TransactionBehavior::Immediate,
        )?;
        self.assert_attempt(id, attempt)?;
        ensure!(
            attempt.intent == "initialize",
            "initial admission requires initialization ownership"
        );
        tx.execute("UPDATE session_lifecycle SET intent='admit',generation=generation+1,operation_id=?2 WHERE session_id=?1",params![id,operation_id()])?;
        tx.execute("UPDATE sessions SET state='STARTING' WHERE id=?1", [id])?;
        let attempt = read(&tx, id)?.context("admission claim vanished")?;
        tx.commit()?;
        Ok(attempt)
    }

    pub(crate) fn terminal_proof(
        &self,
        id: &str,
        attempt: &Attempt,
        exec: &str,
    ) -> Result<Attempt> {
        let tx = rusqlite::Transaction::new_unchecked(
            &self.connection,
            rusqlite::TransactionBehavior::Immediate,
        )?;
        self.assert_attempt(id, attempt)?;
        tx.execute("UPDATE session_lifecycle SET terminal_exec=?2,retirement_operation=COALESCE(retirement_operation,?3) WHERE session_id=?1",params![id,exec,operation_id()])?;
        let attempt = read(&tx, id)?.context("terminal proof vanished")?;
        tx.commit()?;
        Ok(attempt)
    }

    pub(crate) fn selected_session(
        &self,
        name: &str,
        expected: Option<&str>,
    ) -> Result<SessionRecord> {
        let record = self
            .live_session(name)?
            .or(self.one_session("WHERE name=?1 ORDER BY created_at DESC LIMIT 1", name)?)
            .with_context(|| format!("no session named {name:?}"))?;
        ensure!(
            expected.is_none_or(|id| id == record.id),
            "expected session ID does not match; no remote mutation attempted"
        );
        Ok(record)
    }
}

pub(crate) fn operation_id() -> String {
    format!("mantle-{}", ulid::Ulid::new().to_string().to_lowercase())
}
