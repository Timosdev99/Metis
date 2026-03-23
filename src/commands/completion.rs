use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;

use crate::Cli;

pub fn handle(shell: &str) -> Result<()> {
    let mut cmd = Cli::command();
    let shell = match shell.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" | "pwsh" => Shell::PowerShell,
        "elvish" => Shell::Elvish,
        other => {
            anyhow::bail!(
                "Unknown shell '{}'. Supported: bash, zsh, fish, powershell, elvish",
                other
            );
        }
    };
    generate(shell, &mut cmd, "metis", &mut io::stdout());
    Ok(())
}
