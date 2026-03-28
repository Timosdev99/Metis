use anyhow::Result;
use std::path::Path;

use crate::adapters::{get_adapter, known_adapters};
use crate::session::context::ContextBuilder;
use crate::session::store::SessionStore;

pub fn handle(
    project_root: &Path,
    target_cli: &str,
    extra_args: &[String],
    inject_delay_ms: u64,
) -> Result<()> {
    let store = SessionStore::new(project_root);
    guard_initialised(&store)?;

    let mut session = store
        .load()?
        .ok_or_else(|| anyhow::anyhow!("No active session. Run `metis run <cli>` first."))?;

    if session.turn_count() == 0 {
        println!("Metis: session has no turns yet — switching anyway.");
    }

    let adapter = require_adapter(target_cli)?;
    session.active_cli = target_cli.to_string();
    store.save(&session)?;

    println!(
        "Metis: summarising {} turns from {} session…",
        session.turn_count(),
        session.active_cli
    );
    let summary = ContextBuilder::build_summary(&session);
    store.write_summary(&summary)?;
    println!("Metis: summary written to .metis/summary.md");

    let handoff = ContextBuilder::build_handoff_prompt(&summary, target_cli);
    println!("Metis: switching to {} with full context injected.", target_cli);

    crate::commands::run::launch_cli(
        &adapter,
        &handoff,
        extra_args,
        project_root,
        session,
        inject_delay_ms,
    )?;
    Ok(())
}

fn guard_initialised(store: &SessionStore) -> Result<()> {
    if !store.is_initialised() {
        anyhow::bail!("This directory has not been initialised with Metis.\nRun `metis init` first.");
    }
    Ok(())
}

fn require_adapter(name: &str) -> Result<crate::adapters::CliAdapter> {
    get_adapter(name).ok_or_else(|| {
        anyhow::anyhow!(
            "Unknown CLI: '{}'\nSupported: {}",
            name,
            known_adapters().join(", ")
        )
    })
}
