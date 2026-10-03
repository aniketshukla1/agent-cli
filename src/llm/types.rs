use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum StreamEvent {
    Token(String),
    Thinking(String),
    ToolCall {
        id: String,
        name: String,
        arguments: String,
    },
    Done,
}
