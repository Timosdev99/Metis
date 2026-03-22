use crate::session::models::{Role, Session, Turn};

/// Builds a markdown summary from raw session turns without any API calls.
/// Uses simple rule-based extraction:
///   - Last N user messages become "What was being worked on"
///   - Detects file mentions (anything ending in known extensions)
///   - Detects open threads (messages containing "TODO", "later", "next", "still need")
///   - Always captures the very last user message verbatim
pub struct ContextBuilder;

impl ContextBuilder {
    pub fn build_summary(session: &Session) -> String {
        let mut out = String::new();

        out.push_str("## Metis Session Summary\n\n");
        out.push_str(&format!("**Project:** {}\n", session.project));
        out.push_str(&format!(
            "**Started:** {}\n",
            session.created_at.format("%Y-%m-%d %H:%M UTC")
        ));
        out.push_str(&format!("**Last CLI:** {}\n", session.active_cli));
        out.push_str(&format!("**Turns:** {}\n\n", session.turn_count()));

        // --- What was worked on ---
        let user_turns: Vec<&Turn> = session
            .turns
            .iter()
            .filter(|t| t.role == Role::User)
            .collect();

        if !user_turns.is_empty() {
            out.push_str("### What was being worked on\n\n");
            // Take up to 6 most recent user messages as bullet points
            let recent = user_turns.iter().rev().take(6).rev().collect::<Vec<_>>();
            for turn in &recent {
                let snippet = first_line(&turn.content, 120);
                out.push_str(&format!("- `{}`: {}\n", turn.cli, snippet));
            }
            out.push('\n');
        }

        // --- Files mentioned ---
        let files = extract_file_mentions(session);
        if !files.is_empty() {
            out.push_str("### Files mentioned\n\n");
            for f in &files {
                out.push_str(&format!("- `{}`\n", f));
            }
            out.push('\n');
        }

        // --- Open threads ---
        let open_threads = extract_open_threads(session);
        if !open_threads.is_empty() {
            out.push_str("### Open threads\n\n");
            for thread in &open_threads {
                out.push_str(&format!("- {}\n", thread));
            }
            out.push('\n');
        }

        // --- Last user message verbatim (most important for handoff) ---
        if let Some(last) = user_turns.last() {
            out.push_str("### Last message (continue from here)\n\n");
            out.push_str("```\n");
            out.push_str(&last.content);
            out.push_str("\n```\n");
        }

        out
    }

    /// Builds the handoff prompt to prepend when launching a new CLI
    pub fn build_handoff_prompt(summary: &str, incoming_cli: &str) -> String {
        format!(
            "You are continuing a coding session managed by Metis.\n\
             The previous work was done in a different AI coding assistant.\n\
             Pick up exactly where the last message left off.\n\
             Do not re-introduce yourself or summarise unless asked.\n\n\
             {}\n\
             ---\n\
             You are now active as: {}\n",
            summary, incoming_cli
        )
    }
}

//  helper

fn first_line(s: &str, max: usize) -> String {
    let line = s.lines().next().unwrap_or("").trim();
    if line.len() > max {
        format!("{}…", &line[..max])
    } else {
        line.to_string()
    }
}

fn extract_file_mentions(session: &Session) -> Vec<String> {
    let extensions = [
        ".rs", ".ts", ".tsx", ".js", ".jsx", ".py", ".go", ".toml", ".json", ".md", ".yaml",
        ".yml", ".env", ".sql", ".sh",
    ];
    let mut files: Vec<String> = Vec::new();

    for turn in &session.turns {
        for word in turn.content.split_whitespace() {
            // strip surrounding punctuation
            let clean = word
                .trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != '_' && c != '/');
            if extensions.iter().any(|ext| clean.ends_with(ext))
                && !files.contains(&clean.to_string())
            {
                files.push(clean.to_string());
            }
        }
    }

    files
}

fn extract_open_threads(session: &Session) -> Vec<String> {
    let triggers = [
        "todo",
        "still need",
        "next step",
        "haven't",
        "not yet",
        "later",
        "should also",
        "need to",
    ];
    let mut threads: Vec<String> = Vec::new();

    for turn in session.turns.iter().filter(|t| t.role == Role::User) {
        for line in turn.content.lines() {
            let lower = line.to_lowercase();
            if triggers.iter().any(|t| lower.contains(t)) {
                let snippet = first_line(line, 100);
                if !snippet.is_empty() && !threads.contains(&snippet) {
                    threads.push(snippet);
                }
            }
        }
    }

    threads
}
