use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String, // JSON string
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool_call_id: String,
    pub content: String,
    pub is_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: Role,
    pub content: String,
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
    #[serde(default)]
    pub tool_results: Vec<ToolResult>,
    pub timestamp: DateTime<Utc>,
    #[serde(skip)]
    pub is_streaming: bool,
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role: Role::User,
            content: content.into(),
            tool_calls: Vec::new(),
            tool_results: Vec::new(),
            timestamp: Utc::now(),
            is_streaming: false,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role: Role::Assistant,
            content: content.into(),
            tool_calls: Vec::new(),
            tool_results: Vec::new(),
            timestamp: Utc::now(),
            is_streaming: false,
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role: Role::System,
            content: content.into(),
            tool_calls: Vec::new(),
            tool_results: Vec::new(),
            timestamp: Utc::now(),
            is_streaming: false,
        }
    }

    pub fn tool_response(tool_call_id: impl Into<String>, content: impl Into<String>, is_error: bool) -> Self {
        let content_str = content.into();
        Self {
            id: Uuid::new_v4().to_string(),
            role: Role::Tool,
            content: content_str.clone(),
            tool_calls: Vec::new(),
            tool_results: vec![ToolResult {
                tool_call_id: tool_call_id.into(),
                content: content_str,
                is_error,
            }],
            timestamp: Utc::now(),
            is_streaming: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<ChatMessage>,
    pub active_model: String,
}

impl Session {
    pub fn new(model: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title: "New Conversation".to_string(),
            created_at: now,
            updated_at: now,
            messages: Vec::new(),
            active_model: model.into(),
        }
    }

    pub fn estimate_tokens(&self) -> u32 {
        // Approximate 1 token ~= 4 characters for English / code
        let total_chars: usize = self
            .messages
            .iter()
            .map(|m| {
                m.content.len()
                    + m.tool_calls.iter().map(|tc| tc.arguments.len() + tc.name.len()).sum::<usize>()
                    + m.tool_results.iter().map(|tr| tr.content.len()).sum::<usize>()
            })
            .sum();
        ((total_chars as f64) / 3.8).ceil() as u32
    }

    pub fn sessions_dir() -> PathBuf {
        if let Some(proj_dirs) = directories::ProjectDirs::from("com", "agent", "agent-cli") {
            proj_dirs.data_dir().join("sessions")
        } else {
            PathBuf::from(".sessions")
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let dir = Self::sessions_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let file_path = dir.join(format!("{}.json", self.id));
        let content = serde_json::to_string_pretty(self)?;
        fs::write(file_path, content)?;
        Ok(())
    }

    pub fn load_latest() -> Option<Self> {
        let dir = Self::sessions_dir();
        if !dir.exists() {
            return None;
        }
        let mut entries = fs::read_dir(dir).ok()?.filter_map(|e| e.ok()).collect::<Vec<_>>();
        entries.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
        if let Some(latest) = entries.last() {
            if let Ok(content) = fs::read_to_string(latest.path()) {
                if let Ok(sess) = serde_json::from_str::<Session>(&content) {
                    return Some(sess);
                }
            }
        }
        None
    }
}
