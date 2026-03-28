use crate::adapters::{CliAdapter, InjectionMethod, LaunchMode, PtyMode};

pub fn adapter() -> CliAdapter {
    CliAdapter {
        name: "gemini",
        binary: "gemini",
        // Gemini CLI injection is not standardized; default to manual paste
        injection: InjectionMethod::Stdin,
        // PTY passthrough preserves UI while allowing capture.
        launch: LaunchMode::Pty,
        pty_mode: PtyMode::Passthrough,
    }
}
