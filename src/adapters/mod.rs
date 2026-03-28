/// How each CLI receives its initial context.
/// Some CLIs accept a system prompt via flag, some via stdin, some via a temp file.
#[derive(Debug, Clone)]
pub enum InjectionMethod {
    /// Pass context as a --system or similar flag: `<cli> --system "<ctx>"`
    Flag { flag: &'static str },
    /// No automatic injection — Metis prints the context for the user to paste
    PrintOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchMode {
    /// Run the CLI inside a PTY (pseudo-terminal).
    Pty,
    /// Run the CLI with inherited stdio (real terminal).
    Inherit,
}

pub struct CliAdapter {
    pub name: &'static str,
    pub binary: &'static str,
    pub injection: InjectionMethod,
    pub launch: LaunchMode,
}

impl CliAdapter {
    /// Build the command + args to launch this CLI with context injected.
    /// Returns (binary, args, optional_stdin_content).
    pub fn build_launch(
        &self,
        handoff_prompt: &str,
        extra_args: &[String],
    ) -> (String, Vec<String>, Option<String>) {
        let mut args: Vec<String> = Vec::new();

        let stdin = match &self.injection {
            InjectionMethod::Flag { flag } => {
                args.push(flag.to_string());
                args.push(handoff_prompt.to_string());
                args.extend_from_slice(extra_args);
                None
            }
            InjectionMethod::PrintOnly => {
                args.extend_from_slice(extra_args);
                None
            }
        };

        (self.binary.to_string(), args, stdin)
    }
}

pub mod claude;
pub mod codex;
pub mod gemini;
pub mod qwen;

// Adapter registry

pub fn get_adapter(name: &str) -> Option<CliAdapter> {
    match name.to_lowercase().as_str() {
        "claude" => Some(claude::adapter()),
        "codex" => Some(codex::adapter()),
        "gemini" => Some(gemini::adapter()),
        "qwen" => Some(qwen::adapter()),
        _ => None,
    }
}

pub fn known_adapters() -> Vec<&'static str> {
    vec!["claude", "codex", "gemini", "qwen"]
}
