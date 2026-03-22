use anyhow::Result;
use std::path::Path;

use crate::session::store::SessionStore;

pub fn handle(project_root: &Path) -> Result<()> {
    let store = SessionStore::new(project_root);

    if store.is_initialised() {
        println!(
            "Metis already initialised in {}",
            store.metis_dir().display()
        );
        return Ok(());
    }

    std::fs::create_dir_all(store.metis_dir())?;

    let gitignore = project_root.join(".gitignore");
    let entry = "\n# Metis AI context manager\n.metis/\n";
    if gitignore.exists() {
        let existing = std::fs::read_to_string(&gitignore)?;
        if !existing.contains(".metis/") {
            std::fs::write(&gitignore, format!("{}{}", existing, entry))?;
            println!("Added .metis/ to .gitignore");
        }
    } else {
        std::fs::write(&gitignore, entry.trim_start())?;
        println!("Created .gitignore with .metis/ entry");
    }

    println!("Metis initialised at {}", store.metis_dir().display());
    println!("Run `metis run <cli>` to start a tracked session.");
    Ok(())
}
