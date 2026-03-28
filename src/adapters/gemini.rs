use crate::adapters::{CliAdapter, InjectionMethod, LaunchMode};

pub fn adapter() -> CliAdapter {
    CliAdapter {
        name: "gemini",
        binary: "gemini",
        // Gemini CLI injection is not standardized; default to manual paste
        injection: InjectionMethod::PrintOnly,
        launch: LaunchMode::Inherit,
    }
}
