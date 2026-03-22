use anyhow::Result;
use std::path::Path;

use crate::session::models::{Role, Turn};
use crate::session::store::SessionStore;

pub fn handle(project_root: &Path, role: Role, content: &str, cli: &str) -> Result<()> {
    let store = SessionStore::new(project_root);
    let mut session = store
        .load()?
        .ok_or_else(|| anyhow::anyhow!("No active session."))?;
    session.add_turn(Turn::new(role, content, cli));
    store.save(&session)?;
    Ok(())
}
