use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use tokio::process::Command;

use crate::llm::types::ToolDefinition;

pub struct BuiltinTools;

impl BuiltinTools {
    pub fn definitions() -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                name: "run_command".to_string(),
                description: "Execute a shell command (zsh/bash) in the current directory and return stdout/stderr".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "description": "The exact shell command to execute"
                        }
                    },
                    "required": ["command"]
                }),
            },
            ToolDefinition {
                name: "read_file".to_string(),
                description: "Read the entire UTF-8 content of a file from disk".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to read"
                        }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "write_file".to_string(),
                description: "Write content to a file on disk (creates parent directories if needed)".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to write"
                        },
                        "content": {
                            "type": "string",
                            "description": "Text content to write into the file"
                        }
                    },
                    "required": ["path", "content"]
                }),
            },
            ToolDefinition {
                name: "list_directory".to_string(),
                description: "List directory contents including files and subdirectories".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Directory path to list (defaults to current directory if omitted)"
                        }
                    }
                }),
            },
        ]
    }

    pub async fn execute(name: &str, args_json: &str) -> Result<String> {
        let args: serde_json::Value = serde_json::from_str(args_json).unwrap_or_default();

        match name {
            "run_command" => {
                let cmd_str = args
                    .get("command")
                    .and_then(|v| v.as_str())
                    .context("Missing 'command' argument")?;

                let output = Command::new("sh")
                    .arg("-c")
                    .arg(cmd_str)
                    .output()
                    .await
                    .with_context(|| format!("Failed to run command: {}", cmd_str))?;

                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let code = output.status.code().unwrap_or(-1);

                let mut res = format!("Exit code: {}\n", code);
                if !stdout.is_empty() {
                    res.push_str(&format!("--- STDOUT ---\n{}\n", stdout));
                }
                if !stderr.is_empty() {
                    res.push_str(&format!("--- STDERR ---\n{}\n", stderr));
                }
                Ok(res)
            }
            "read_file" => {
                let path_str = args
                    .get("path")
                    .and_then(|v| v.as_str())
                    .context("Missing 'path' argument")?;
                let content = fs::read_to_string(path_str)
                    .with_context(|| format!("Failed to read file: {}", path_str))?;
                Ok(content)
            }
            "write_file" => {
                let path_str = args
                    .get("path")
                    .and_then(|v| v.as_str())
                    .context("Missing 'path' argument")?;
                let content = args
                    .get("content")
                    .and_then(|v| v.as_str())
                    .context("Missing 'content' argument")?;

                let path = Path::new(path_str);
                if let Some(parent) = path.parent() {
                    if !parent.as_os_str().is_empty() {
                        fs::create_dir_all(parent)?;
                    }
                }
                fs::write(path, content)?;
                Ok(format!("Successfully wrote {} bytes to {}", content.len(), path_str))
            }
            "list_directory" => {
                let path_str = args
                    .get("path")
                    .and_then(|v| v.as_str())
                    .unwrap_or(".");
                let dir = Path::new(path_str);
                let mut entries_summary = Vec::new();
                for entry in fs::read_dir(dir)? {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        let file_type = entry.file_type()?;
                        let kind = if file_type.is_dir() { "dir" } else { "file" };
                        entries_summary.push(format!("[{}] {}", kind, file_name));
                    }
                }
                entries_summary.sort();
                Ok(entries_summary.join("\n"))
            }
            other => Err(anyhow::anyhow!("Unknown tool '{}'", other)),
        }
    }
}
