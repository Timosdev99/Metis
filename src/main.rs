mod adapters;
mod commands;
mod session;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "metis",
    about = "AI context manager — switch coding CLIs without losing context",
    version
)]
struct Cli {
    /// Project root (defaults to current directory)
    #[arg(long, global = true, value_name = "PATH")]
    project: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialise Metis in this project (creates .metis/, updates .gitignore)
    Init,

    /// Start a Metis-managed session with a coding CLI
    Run {
        /// The CLI to launch (claude, codex, cursor, copilot, aider)
        #[arg(value_name = "CLI")]
        cli: String,
        /// Extra arguments passed through to the CLI
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// End current session, summarise context, switch to another CLI
    Switch {
        /// The CLI to switch to
        #[arg(value_name = "CLI")]
        cli: String,
        /// Extra arguments passed through to the new CLI
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Show current session status
    Status,

    /// Show conversation history for this session
    History {
        /// Number of most recent turns to show (default: 20)
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// Record a turn manually (useful for scripting/wrappers)
    AddTurn {
        /// Role: user | assistant | system
        role: String,
        /// Message content
        content: String,
        /// Which CLI this turn came from
        #[arg(long, default_value = "manual")]
        cli: String,
    },

    /// List supported CLIs/adapters
    Adapters,

    /// Generate shell completions
    Completion {
        /// Shell name: bash | zsh | fish | powershell | elvish
        #[arg(value_name = "SHELL")]
        shell: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let project_root = cli
        .project
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot read current dir"));

    match cli.command {
        Command::Init => {
            commands::init(&project_root)?;
        }

        Command::Run { cli, args } => {
            commands::run(&project_root, &cli, &args)?;
        }

        Command::Switch { cli, args } => {
            commands::switch(&project_root, &cli, &args)?;
        }

        Command::Status => {
            commands::status(&project_root)?;
        }

        Command::History { limit } => {
            commands::history(&project_root, limit)?;
        }

        Command::AddTurn { role, content, cli } => {
            use session::models::Role;
            let r = match role.to_lowercase().as_str() {
                "user" => Role::User,
                "assistant" => Role::Assistant,
                "system" => Role::System,
                other => anyhow::bail!("Unknown role '{}'. Use: user | assistant | system", other),
            };
            commands::add_turn(&project_root, r, &content, &cli)?;
        }

        Command::Adapters => {
            commands::adapters()?;
        }

        Command::Completion { shell } => {
            commands::completion(&shell)?;
        }
    }

    Ok(())
}
