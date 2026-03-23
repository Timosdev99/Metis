use anyhow::Result;
use std::path::Path;

use crate::session::models::Role;
use crate::session::store::SessionStore;

pub fn handle(project_root: &Path, limit: usize) -> Result<()> {
    let store = SessionStore::new(project_root);
    guard_initialised(&store)?;

    let session = store
        .load()?
        .ok_or_else(|| anyhow::anyhow!("No session found. Run `metis run <cli>` first."))?;

    let turns = &session.turns;
    let start = turns.len().saturating_sub(limit);

    for turn in &turns[start..] {
        let time = turn.timestamp.format("%H:%M");
        let role_label = match turn.role {
            Role::User => "you",
            Role::Assistant => turn.cli.as_str(),
            Role::System => "system",
        };
        let snippet = first_line(&turn.content, 120);
        println!("[{}] [{}] {}", time, role_label, snippet);
    }

    Ok(())
}

fn guard_initialised(store: &SessionStore) -> Result<()> {
    if !store.is_initialised() {
        anyhow::bail!("This directory has not been initialised with Metis.\nRun `metis init` first.");
    }
    Ok(())
}

fn first_line(s: &str, max: usize) -> String {
    let line = s.lines().next().unwrap_or("").trim();
    if line.len() > max {
        format!("{}…", &line[..max])
    } else {
        line.to_string()
    }
}
