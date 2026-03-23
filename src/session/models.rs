use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
            Role::System => write!(f, "system"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Turn {
    pub role: Role,
    pub content: String,
    pub cli: String,
    pub timestamp: DateTime<Utc>,
}

impl Turn {
    pub fn new(role: Role, content: impl Into<String>, cli: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
            cli: cli.into(),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub project: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub active_cli: String,
    pub turns: Vec<Turn>,
}

impl Session {
    pub fn new(project: impl Into<String>, cli: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: generate_id(),
            project: project.into(),
            created_at: now,
            updated_at: now,
            active_cli: cli.into(),
            turns: Vec::new(),
        }
    }

    pub fn add_turn(&mut self, turn: Turn) {
        self.updated_at = Utc::now();
        self.active_cli = turn.cli.clone();
        self.turns.push(turn);
    }

    pub fn turn_count(&self) -> usize {
        self.turns.len()
    }
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{:x}", ts)
}
