use crate::adapters::{CliAdapter, InjectionMethod, LaunchMode};

pub fn adapter() -> CliAdapter {
    CliAdapter {
        name: "qwen",
        binary: "qwen",
        // Qwen CLI injection is not standardized; default to manual paste
        injection: InjectionMethod::PrintOnly,
        launch: LaunchMode::Pty,
    }
}
