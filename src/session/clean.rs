pub fn strip_ansi(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    while let Some(ch) = chars.next() {
                        if ('@'..='~').contains(&ch) {
                            break;
                        }
                    }
                }
                Some(']') => {
                    chars.next();
                    while let Some(ch) = chars.next() {
                        if ch == '\x07' {
                            break;
                        }
                        if ch == '\x1b' {
                            if let Some('\\') = chars.peek() {
                                chars.next();
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
            continue;
        }
        out.push(c);
    }
    out
}

pub fn sanitize_user_input(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    let mut chars = trimmed.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '[' {
            // Strip CSI-like sequences that sometimes appear without ESC.
            if let Some(&next) = chars.peek() {
                if next.is_ascii_digit() || next == ';' || next == '?' {
                    while let Some(ch) = chars.next() {
                        if ch.is_ascii_alphabetic() {
                            break;
                        }
                    }
                    continue;
                }
            }
        }
        if c == ']' {
            // Strip OSC-like sequences without ESC, until backslash.
            while let Some(ch) = chars.next() {
                if ch == '\\' || ch == '\u{0007}' {
                    break;
                }
            }
            continue;
        }
        if c.is_control() && c != '\t' {
            continue;
        }
        out.push(c);
    }

    strip_ansi(&out).trim().to_string()
}

pub fn sanitize_assistant_output(cli: &str, input: &str) -> String {
    let cleaned = strip_ansi(input);
    let mut lines: Vec<String> = Vec::new();
    for line in cleaned.lines() {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        if is_generic_noise_line(l) {
            continue;
        }
        if is_cli_noise_line(cli, l) {
            continue;
        }
        lines.push(l.to_string());
    }

    let joined = lines.join("\n");
    joined.trim().to_string()
}

fn is_generic_noise_line(line: &str) -> bool {
    // Drop lines with no alphanumeric content (borders, separators, etc.)
    if !line.chars().any(|c| c.is_ascii_alphanumeric()) {
        return true;
    }

    // Drop obvious progress spam.
    let lower = line.to_lowercase();
    if lower.contains("press enter to continue") {
        return true;
    }

    false
}

fn is_cli_noise_line(cli: &str, line: &str) -> bool {
    let lower = line.to_lowercase();
    match cli {
        "codex" => {
            let patterns = [
                "openai codex",
                "write tests for @filename",
                "context left",
                "token usage",
                "to continue this session, run codex resume",
                "working",
                "model:",
                "directory:",
                "tip:",
                "update available",
                "release notes",
            ];
            patterns.iter().any(|p| lower.contains(p))
        }
        "claude" => {
            let patterns = ["update available", "release notes"];
            patterns.iter().any(|p| lower.contains(p))
        }
        "gemini" => {
            let patterns = [
                "waiting for auth",
                "gemini cli update available",
                "attempting to automatically update",
                "automatic update failed",
                "type your message or @path/to/file",
                "press ctrl+c again to exit",
                "interaction summary",
                "tool calls",
                "success rate",
                "model usage",
                "wall time",
                "agent powering down",
            ];
            patterns.iter().any(|p| lower.contains(p))
        }
        "qwen" => {
            let patterns = ["update available", "press ctrl+c again to exit"];
            patterns.iter().any(|p| lower.contains(p))
        }
        _ => false,
    }
}
