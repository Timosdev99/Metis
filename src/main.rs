use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const CONTEXT_FILENAME: &str = "context.md";
const GITIGNORE_FILENAME: &str = ".gitignore";

#[derive(Parser, Debug)]
#[command(
    name = "context",
    version,
    about = "Simple context manager for AI agents"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize context.md and ensure it is gitignored
    Init,

    /// Show the current context
    Show,

    /// Replace context with provided text or stdin
    Set {
        /// Text to set. If omitted, reads from stdin.
        text: Option<String>,
    },

    /// Append text to the context
    Add {
        /// Text to append. If omitted, reads from stdin.
        text: Option<String>,
    },

    /// Clear the context file
    Clear,

    /// Show current agent (gemini or codex)
    Agent,

    /// Switch current agent (gemini or codex)
    Use {
        /// Agent name: gemini or codex
        agent: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => cmd_init(),
        Commands::Show => cmd_show(),
        Commands::Set { text } => cmd_set(text),
        Commands::Add { text } => cmd_add(text),
        Commands::Clear => cmd_clear(),
        Commands::Agent => cmd_agent(),
        Commands::Use { agent } => cmd_use(agent),
    };

    if let Err(err) = result {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn cmd_init() -> io::Result<()> {
    ensure_context_file()?;
    Ok(())
}

fn cmd_show() -> io::Result<()> {
    ensure_context_file()?;
    let content = fs::read_to_string(CONTEXT_FILENAME)?;
    print!("{content}");
    Ok(())
}

fn cmd_set(text: Option<String>) -> io::Result<()> {
    ensure_context_file()?;
    let content = read_text_or_stdin(text)?;
    fs::write(CONTEXT_FILENAME, content)?;
    Ok(())
}

fn cmd_add(text: Option<String>) -> io::Result<()> {
    ensure_context_file()?;
    let addition = read_text_or_stdin(text)?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(CONTEXT_FILENAME)?;

    if !addition.is_empty() {
        let needs_newline = !file_ends_with_newline(CONTEXT_FILENAME)?;
        if needs_newline {
            writeln!(file)?;
        }
        write!(file, "{addition}")?;
    }

    Ok(())
}

fn cmd_clear() -> io::Result<()> {
    ensure_context_file()?;
    fs::write(CONTEXT_FILENAME, "")?;
    Ok(())
}

fn cmd_agent() -> io::Result<()> {
    let agent = read_agent()?;
    println!("{agent}");
    Ok(())
}

fn cmd_use(agent: String) -> io::Result<()> {
    let agent = normalize_agent(&agent).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "agent must be 'gemini' or 'codex'",
        )
    })?;

    write_agent(&agent)?;
    println!("Switched to {agent}");
    Ok(())
}

fn ensure_context_file() -> io::Result<()> {
    if !Path::new(CONTEXT_FILENAME).exists() {
        fs::write(CONTEXT_FILENAME, "")?;
    }
    ensure_gitignore()?;
    Ok(())
}

fn ensure_gitignore() -> io::Result<()> {
    let path = Path::new(GITIGNORE_FILENAME);
    if !path.exists() {
        fs::write(path, "")?;
    }

    let content = fs::read_to_string(path)?;
    if !content.lines().any(|line| line.trim() == CONTEXT_FILENAME) {
        let mut file = fs::OpenOptions::new().append(true).open(path)?;
        if !content.ends_with('\n') && !content.is_empty() {
            writeln!(file)?;
        }
        writeln!(file, "{CONTEXT_FILENAME}")?;
    }

    Ok(())
}

fn read_text_or_stdin(text: Option<String>) -> io::Result<String> {
    match text {
        Some(t) => Ok(t),
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}

fn file_ends_with_newline(path: &str) -> io::Result<bool> {
    let content = fs::read_to_string(path)?;
    Ok(content.ends_with('\n') || content.is_empty())
}

fn agent_path() -> PathBuf {
    Path::new(CONTEXT_FILENAME).with_file_name(".context_agent")
}

fn read_agent() -> io::Result<String> {
    let path = agent_path();
    if !path.exists() {
        return Ok("codex".to_string());
    }
    let raw = fs::read_to_string(path)?;
    let agent = normalize_agent(raw.trim()).unwrap_or("codex".to_string());
    Ok(agent)
}

fn write_agent(agent: &str) -> io::Result<()> {
    fs::write(agent_path(), agent)?;
    Ok(())
}

fn normalize_agent(input: &str) -> Option<String> {
    match input.trim().to_lowercase().as_str() {
        "gemini" => Some("gemini".to_string()),
        "codex" => Some("codex".to_string()),
        _ => None,
    }
}
