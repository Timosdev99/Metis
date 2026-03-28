use crate::adapters::{CliAdapter, InjectionMethod, LaunchMode, PtyMode};

pub fn adapter() -> CliAdapter {
    CliAdapter {
        name: "codex",
        binary: "codex",
        // OpenAI Codex CLI: send context as the first message via stdin.
        injection: InjectionMethod::Stdin,
        // Run in PTY passthrough to preserve UI while enabling capture.
        launch: LaunchMode::Pty,
        pty_mode: PtyMode::Passthrough,
    }
}
