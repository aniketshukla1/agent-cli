use anyhow::{Context, Result};
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::Mutex;

use crate::mcp::protocol::{
    JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, McpTool, McpToolCallResult,
    McpToolsListResult,
};

pub struct McpClient {
    pub name: String,
    _child: Mutex<Child>,
    stdin: Mutex<ChildStdin>,
    stdout_reader: Mutex<BufReader<ChildStdout>>,
    req_counter: AtomicU64,
    pub tools: Arc<Mutex<Vec<McpTool>>>,
}

impl McpClient {
    pub async fn spawn(
        name: String,
        command: &str,
        args: &[String],
        env: &HashMap<String, String>,
    ) -> Result<Arc<Self>> {
        let mut cmd = Command::new(command);
        cmd.args(args);
        for (k, v) in env {
            cmd.env(k, v);
        }
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::null());

        let mut child = cmd
            .spawn()
            .with_context(|| format!("Failed to spawn MCP server '{}' ({})", name, command))?;

        let stdin = child
            .stdin
            .take()
            .context("Failed to open child stdin for MCP server")?;
        let stdout = child
            .stdout
            .take()
            .context("Failed to open child stdout for MCP server")?;

        let client = Arc::new(Self {
            name,
            _child: Mutex::new(child),
            stdin: Mutex::new(stdin),
            stdout_reader: Mutex::new(BufReader::new(stdout)),
            req_counter: AtomicU64::new(1),
            tools: Arc::new(Mutex::new(Vec::new())),
        });

        // Initialize handshake
        client.initialize().await?;

        // List initial tools
        if let Ok(tools) = client.list_tools().await {
            let mut stored_tools = client.tools.lock().await;
            *stored_tools = tools;
        }

        Ok(client)
    }

    async fn send_request(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let req_id = self.req_counter.fetch_add(1, Ordering::SeqCst);
        let req = JsonRpcRequest::new(req_id, method, params);
        let serialized = serde_json::to_string(&req)? + "\n";

        {
            let mut stdin = self.stdin.lock().await;
            stdin.write_all(serialized.as_bytes()).await?;
            stdin.flush().await?;
        }

        let mut stdout = self.stdout_reader.lock().await;
        let mut line = String::new();

        loop {
            line.clear();
            let bytes_read = stdout.read_line(&mut line).await?;
            if bytes_read == 0 {
                return Err(anyhow::anyhow!("MCP server closed connection unexpectedly"));
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if let Ok(resp) = serde_json::from_str::<JsonRpcResponse>(trimmed) {
                if resp.id == Some(req_id) {
                    if let Some(err) = resp.error {
                        return Err(anyhow::anyhow!(
                            "MCP error (code {}): {}",
                            err.code,
                            err.message
                        ));
                    }
                    return Ok(resp.result.unwrap_or(serde_json::Value::Null));
                }
            }
        }
    }

    async fn send_notification(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<()> {
        let notif = JsonRpcNotification::new(method, params);
        let serialized = serde_json::to_string(&notif)? + "\n";

        let mut stdin = self.stdin.lock().await;
        stdin.write_all(serialized.as_bytes()).await?;
        stdin.flush().await?;
        Ok(())
    }

    pub async fn initialize(&self) -> Result<()> {
        let init_params = serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "roots": { "listChanged": true }
            },
            "clientInfo": {
                "name": "agent-cli",
                "version": "0.1.0"
            }
        });

        self.send_request("initialize", Some(init_params)).await?;
        self.send_notification("notifications/initialized", None).await?;
        Ok(())
    }

    pub async fn list_tools(&self) -> Result<Vec<McpTool>> {
        let res = self.send_request("tools/list", None).await?;
        let parsed: McpToolsListResult = serde_json::from_value(res)
            .context("Failed to parse tools/list result from MCP server")?;
        Ok(parsed.tools)
    }

    pub async fn call_tool(
        &self,
        name: &str,
        arguments: serde_json::Value,
    ) -> Result<McpToolCallResult> {
        let params = serde_json::json!({
            "name": name,
            "arguments": arguments,
        });

        let res = self.send_request("tools/call", Some(params)).await?;
        let parsed: McpToolCallResult = serde_json::from_value(res)
            .context("Failed to parse tools/call response from MCP server")?;
        Ok(parsed)
    }
}
