use crate::adapters::{CliAdapter, InjectionMethod, LaunchMode};

pub fn adapter() -> CliAdapter {
    CliAdapter {
        name: "claude",
        binary: "claude",
        // Claude Code accepts a system prompt via --system flag
        injection: InjectionMethod::Flag { flag: "--system" },
        launch: LaunchMode::Pty,
    }
}
