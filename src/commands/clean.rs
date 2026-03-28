use anyhow::Result;
use chrono::Utc;
use std::path::Path;

use crate::session::clean::{sanitize_assistant_output, sanitize_user_input, strip_ansi};
use crate::session::models::Role;
use crate::session::store::SessionStore;

pub fn handle(project_root: &Path) -> Result<()> {
    let store = SessionStore::new(project_root);
    guard_initialised(&store)?;

    let mut session = store
        .load()?
        .ok_or_else(|| anyhow::anyhow!("No session found. Run `metis run <cli>` first."))?;

    let original = session.turns.len();
    let mut cleaned = Vec::with_capacity(original);

    for mut turn in session.turns.into_iter() {
        let content = match turn.role {
            Role::User => sanitize_user_input(&turn.content),
            Role::Assistant | Role::System => {
                sanitize_assistant_output(&turn.cli, &turn.content)
            }
        };
        let content = strip_ansi(&content).trim().to_string();
        if !content.is_empty() {
            turn.content = content;
            cleaned.push(turn);
        }
    }

    let removed = original.saturating_sub(cleaned.len());
    session.turns = cleaned;
    session.updated_at = Utc::now();
    store.save(&session)?;

    println!(
        "Metis: cleaned session turns (kept {}, removed {})",
        session.turns.len(),
        removed
    );
    Ok(())
}

fn guard_initialised(store: &SessionStore) -> Result<()> {
    if !store.is_initialised() {
        anyhow::bail!("This directory has not been initialised with Metis.\nRun `metis init` first.");
    }
    Ok(())
}
