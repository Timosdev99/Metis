use anyhow::Result;
use std::path::Path;

use crate::session::store::SessionStore;

pub fn handle(project_root: &Path) -> Result<()> {
    let store = SessionStore::new(project_root);

    if !store.is_initialised() {
        println!("No Metis session in this directory.");
        println!("Run `metis init` to initialise.");
        return Ok(());
    }

    match store.load()? {
        None => println!("Metis initialised but no session started yet."),
        Some(s) => {
            println!("Session ID  : {}", s.id);
            println!("Project     : {}", s.project);
            println!("Active CLI  : {}", s.active_cli);
            println!("Turns       : {}", s.turn_count());
            println!(
                "Created     : {}",
                s.created_at.format("%Y-%m-%d %H:%M UTC")
            );
            println!(
                "Updated     : {}",
                s.updated_at.format("%Y-%m-%d %H:%M UTC")
            );
            if store.read_summary()?.is_some() {
                println!("Summary     : .metis/summary.md exists");
            }
        }
    }

    Ok(())
}
