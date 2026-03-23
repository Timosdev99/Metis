use anyhow::Result;

use crate::adapters::known_adapters;

pub fn handle() -> Result<()> {
    println!("Supported CLIs:");
    for name in known_adapters() {
        println!("- {}", name);
    }
    Ok(())
}
