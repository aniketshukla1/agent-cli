use anyhow::{Context, Result};
use futures::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;

use crate::llm::types::{StreamEvent, ToolDefinition};
use crate::session::{ChatMessage, Role};

#[derive(Clone)]
pub struct AnthropicProvider {
    pub api_key: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

#[derive(Serialize)]
struct AnthropicRequest<'a> {
    model: &'a str,
    messages: Vec<AnthropicMessage<'a>>,
    #[serde(skip_serializing_if = "str::is_empty")]
    system: &'a str,
    max_tokens: u32,
    temperature: f32,
    stream: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<AnthropicTool<'a>>,
}

#[derive(Serialize)]
struct AnthropicMessage<'a> {
    role: &'a str,
    content: AnthropicContent<'a>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum AnthropicContent<'a> {
    Text(&'a str),
    Blocks(Vec<AnthropicBlock<'a>>),
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum AnthropicBlock<'a> {
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: &'a str,
        content: &'a str,
        is_error: bool,
    },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: &'a str,
        name: &'a str,
        input: serde_json::Value,
    },
}

#[derive(Serialize)]
struct AnthropicTool<'a> {
    name: &'a str,
    description: &'a str,
    input_schema: &'a serde_json::Value,
}

#[derive(Deserialize)]
struct AnthropicEvent {
    #[serde(default)]
    r#type: String,
    delta: Option<AnthropicDelta>,
    content_block: Option<AnthropicContentBlock>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct AnthropicDelta {
    r#type: Option<String>,
    text: Option<String>,
    partial_json: Option<String>,
}

#[derive(Deserialize)]
struct AnthropicContentBlock {
    r#type: String,
    id: Option<String>,
    name: Option<String>,
}

impl AnthropicProvider {
    pub fn new(api_key: String, model: String, max_tokens: u32, temperature: f32) -> Self {
        Self {
            api_key,
            model,
            max_tokens,
            temperature,
        }
    }

    pub async fn stream_chat(
        &self,
        messages: &[ChatMessage],
        system: &str,
        tools: &[ToolDefinition],
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamEvent>> + Send>>> {
        let mut anthropic_messages = Vec::new();

        for msg in messages {
            match msg.role {
                Role::User => {
                    anthropic_messages.push(AnthropicMessage {
                        role: "user",
                        content: AnthropicContent::Text(&msg.content),
                    });
                }
                Role::Assistant => {
                    if !msg.tool_calls.is_empty() {
                        let blocks = msg
                            .tool_calls
                            .iter()
                            .map(|tc| AnthropicBlock::ToolUse {
                                id: &tc.id,
                                name: &tc.name,
                                input: serde_json::from_str(&tc.arguments).unwrap_or_default(),
                            })
                            .collect();
                        anthropic_messages.push(AnthropicMessage {
                            role: "assistant",
                            content: AnthropicContent::Blocks(blocks),
                        });
                    } else {
                        anthropic_messages.push(AnthropicMessage {
                            role: "assistant",
                            content: AnthropicContent::Text(&msg.content),
                        });
                    }
                }
                Role::Tool => {
                    let blocks = msg
                        .tool_results
                        .iter()
                        .map(|tr| AnthropicBlock::ToolResult {
                            tool_use_id: &tr.tool_call_id,
                            content: &tr.content,
                            is_error: tr.is_error,
                        })
                        .collect();
                    anthropic_messages.push(AnthropicMessage {
                        role: "user",
                        content: AnthropicContent::Blocks(blocks),
                    });
                }
                Role::System => {
                    // System messages are passed top-level in Anthropic API
                }
            }
        }

        let api_tools: Vec<AnthropicTool> = tools
            .iter()
            .map(|t| AnthropicTool {
                name: &t.name,
                description: &t.description,
                input_schema: &t.input_schema,
            })
            .collect();

        let request = AnthropicRequest {
            model: &self.model,
            messages: anthropic_messages,
            system,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            stream: true,
            tools: api_tools,
        };

        let client = reqwest::Client::new();
        let req = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request);

        let es = EventSource::new(req).context("Failed to connect to Anthropic SSE")?;

        let stream = async_stream::stream! {
            let mut es = es;
            let mut current_tool_id = String::new();
            let mut current_tool_name = String::new();
            let mut current_tool_args = String::new();

            while let Some(event) = es.next().await {
                match event {
                    Ok(Event::Open) => continue,
                    Ok(Event::Message(msg)) => {
                        if let Ok(evt) = serde_json::from_str::<AnthropicEvent>(&msg.data) {
                            match evt.r#type.as_str() {
                                "content_block_start" => {
                                    if let Some(block) = evt.content_block {
                                        if block.r#type == "tool_use" {
                                            current_tool_id = block.id.unwrap_or_default();
                                            current_tool_name = block.name.unwrap_or_default();
                                            current_tool_args.clear();
                                        }
                                    }
                                }
                                "content_block_delta" => {
                                    if let Some(delta) = evt.delta {
                                        if let Some(text) = delta.text {
                                            yield Ok(StreamEvent::Token(text));
                                        }
                                        if let Some(pjson) = delta.partial_json {
                                            current_tool_args.push_str(&pjson);
                                        }
                                    }
                                }
                                "content_block_stop" => {
                                    if !current_tool_id.is_empty() {
                                        yield Ok(StreamEvent::ToolCall {
                                            id: current_tool_id.clone(),
                                            name: current_tool_name.clone(),
                                            arguments: current_tool_args.clone(),
                                        });
                                        current_tool_id.clear();
                                        current_tool_name.clear();
                                        current_tool_args.clear();
                                    }
                                }
                                "message_stop" => {
                                    yield Ok(StreamEvent::Done);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(e) => {
                        yield Err(anyhow::anyhow!("Anthropic SSE error: {:?}", e));
                        break;
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }
}
