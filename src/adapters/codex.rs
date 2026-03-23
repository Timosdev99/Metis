use crate::adapters::{CliAdapter, InjectionMethod, LaunchMode};

pub fn adapter() -> CliAdapter {
    CliAdapter {
        name: "codex",
        binary: "codex",
        // OpenAI Codex CLI: pass context as the first positional message
        injection: InjectionMethod::PrintOnly,
        // Codex expects a real terminal (cursor position queries).
        launch: LaunchMode::Inherit,
    }
}
