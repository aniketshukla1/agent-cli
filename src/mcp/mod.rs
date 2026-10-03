pub mod client;
pub mod protocol;

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::llm::types::ToolDefinition;
use crate::mcp::client::McpClient;
use crate::mcp::protocol::McpToolCallResult;

pub struct McpManager {
    clients: Arc<RwLock<HashMap<String, Arc<McpClient>>>>,
}

impl McpManager {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn init_from_config(&self, config: &Config) {
        for (name, srv) in &config.mcp_servers {
            if srv.disabled {
                continue;
            }
            match McpClient::spawn(name.clone(), &srv.command, &srv.args, &srv.env).await {
                Ok(client) => {
                    let mut lock = self.clients.write().await;
                    lock.insert(name.clone(), client);
                }
                Err(e) => {
                    tracing::warn!("Failed to initialize MCP server '{}': {:?}", name, e);
                }
            }
        }
    }

    pub async fn get_all_tools(&self) -> Vec<ToolDefinition> {
        let clients = self.clients.read().await;
        let mut tools = Vec::new();

        for (srv_name, client) in clients.iter() {
            let srv_tools = client.tools.lock().await;
            for t in srv_tools.iter() {
                // Name tool with prefix or clean name: {srv_name}__{t.name}
                let tool_name = format!("{}__{}", srv_name, t.name);
                tools.push(ToolDefinition {
                    name: tool_name,
                    description: format!("[From MCP Server '{}'] {}", srv_name, t.description),
                    input_schema: t.input_schema.clone(),
                });
            }
        }

        tools
    }

    pub async fn call_tool(
        &self,
        full_tool_name: &str,
        args_json: &str,
    ) -> Result<McpToolCallResult> {
        let parts: Vec<&str> = full_tool_name.split("__").collect();
        let (server_name, actual_tool_name) = if parts.len() >= 2 {
            (parts[0], parts[1..].join("__"))
        } else {
            ("", full_tool_name.to_string())
        };

        let clients = self.clients.read().await;
        if let Some(client) = clients.get(server_name) {
            let args_value: serde_json::Value =
                serde_json::from_str(args_json).unwrap_or_else(|_| serde_json::json!({}));
            client.call_tool(&actual_tool_name, args_value).await
        } else {
            Err(anyhow::anyhow!(
                "MCP server '{}' not found for tool '{}'",
                server_name,
                full_tool_name
            ))
        }
    }

    pub async fn server_count(&self) -> usize {
        self.clients.read().await.len()
    }

    pub async fn get_server_statuses(&self) -> Vec<(String, usize)> {
        let clients = self.clients.read().await;
        let mut res = Vec::new();
        for (name, client) in clients.iter() {
            let tool_count = client.tools.lock().await.len();
            res.push((name.clone(), tool_count));
        }
        res
    }
}
