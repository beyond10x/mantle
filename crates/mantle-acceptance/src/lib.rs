//! Versioned metadata shared with the CLI; the runner invokes installed CLI processes only.
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
pub mod runner;
pub const FORMAT: &str = "mantle-session-metadata/v1";
pub const RECEIPT_FORMAT: &str = "mantle-creation-receipt/v1";
pub const MAX_METADATA: usize = 1024 * 1024;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SelectionBinding {
    pub profile: Option<String>,
    pub config_path: PathBuf,
    pub state_dir: PathBuf,
    pub config_sha256: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub agent: String,
    pub authentication: String,
    pub recorded_state: String,
    pub generation: i64,
    pub workspace_id: Option<String>,
    pub exec_id: Option<String>,
    pub source_commits: Vec<String>,
    pub observed: Option<Observation>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub workspace_state: Option<String>,
    pub exec_state: Option<String>,
    pub exit_code: Option<u8>,
    pub exit_signal: Option<String>,
    pub refused: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    pub format: String,
    pub selection: Option<SelectionBinding>,
    pub outcome: String,
    pub observed_at: String,
    pub sessions: Vec<Session>,
}
impl Metadata {
    pub fn empty() -> Self {
        Self {
            format: FORMAT.into(),
            selection: None,
            outcome: "selection-refused".into(),
            observed_at: now(),
            sessions: vec![],
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreationReceipt {
    pub format: String,
    pub selection: SelectionBinding,
    pub session: Session,
    pub created_at: String,
}
pub fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}
pub fn encoded<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let bytes = serde_json::to_vec(value)?;
    ensure!(bytes.len() <= MAX_METADATA, "metadata exceeds bound");
    Ok(bytes)
}
pub fn write_new<T: Serialize>(path: &std::path::Path, value: &T) -> Result<()> {
    use std::io::Write;
    let bytes = encoded(value)?;
    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("missing parent"))?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    temp.write_all(&bytes)?;
    temp.as_file().sync_all()?;
    temp.persist_noclobber(path)?;
    std::fs::File::open(parent)?.sync_all()?;
    Ok(())
}
