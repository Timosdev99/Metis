use crate::session::models::Session;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub struct SessionStore {
    pub metis_dir: PathBuf,
}

impl SessionStore {
    pub fn new(project_root: &Path) -> Self {
        Self {
            metis_dir: project_root.join(".metis"),
        }
    }

    fn session_path(&self) -> PathBuf {
        self.metis_dir.join("session.json")
    }

    fn summary_path(&self) -> PathBuf {
        self.metis_dir.join("summary.md")
    }

    /// Load existing session, or None if no session exists yet
    pub fn load(&self) -> Result<Option<Session>> {
        let path = self.session_path();
        if !path.exists() {
            return Ok(None);
        }
        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let session: Session =
            serde_json::from_str(&raw).with_context(|| "session.json is malformed")?;
        Ok(Some(session))
    }

    /// Persist session to session.json
    pub fn save(&self, session: &Session) -> Result<()> {
        std::fs::create_dir_all(&self.metis_dir)?;
        let json = serde_json::to_string_pretty(session)?;
        std::fs::write(self.session_path(), json)
            .with_context(|| "Failed to write session.json")?;
        Ok(())
    }

    /// Write the summary.md file
    pub fn write_summary(&self, content: &str) -> Result<()> {
        std::fs::create_dir_all(&self.metis_dir)?;
        std::fs::write(self.summary_path(), content)
            .with_context(|| "Failed to write summary.md")?;
        Ok(())
    }

    /// Read summary.md if it exists
    pub fn read_summary(&self) -> Result<Option<String>> {
        let path = self.summary_path();
        if !path.exists() {
            return Ok(None);
        }
        Ok(Some(std::fs::read_to_string(path)?))
    }

    /// Check whether this directory has been initialised with metis init
    pub fn is_initialised(&self) -> bool {
        self.metis_dir.exists()
    }

    /// Returns the path so callers can display it
    pub fn metis_dir(&self) -> &Path {
        &self.metis_dir
    }
}
