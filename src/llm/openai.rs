use anyhow::{Context, Result};
use futures::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;

use crate::llm::types::{StreamEvent, ToolDefinition};
use crate::session::{ChatMessage, Role};

#[derive(Clone)]
pub struct OpenAiProvider {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<OpenAiMessage<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    stream: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<OpenAiTool<'a>>,
}

#[derive(Serialize)]
struct OpenAiMessage<'a> {
    role: &'a str,
    content: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<&'a str>,
}

#[derive(Serialize)]
struct OpenAiTool<'a> {
    r#type: &'static str,
    function: OpenAiFunction<'a>,
}

#[derive(Serialize)]
struct OpenAiFunction<'a> {
    name: &'a str,
    description: &'a str,
    parameters: &'a serde_json::Value,
}

#[derive(Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

#[derive(Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
}

#[derive(Deserialize)]
struct StreamDelta {
    content: Option<String>,
    tool_calls: Option<Vec<StreamToolCallDelta>>,
}

#[derive(Deserialize)]
struct StreamToolCallDelta {
    id: Option<String>,
    function: Option<StreamFunctionDelta>,
}

#[derive(Deserialize)]
struct StreamFunctionDelta {
    name: Option<String>,
    arguments: Option<String>,
}

impl OpenAiProvider {
    pub fn new(
        api_key: String,
        base_url: Option<String>,
        model: String,
        temperature: f32,
        max_tokens: u32,
    ) -> Self {
        let base_url = base_url
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string())
            .trim_end_matches('/')
            .to_string();
        Self {
            api_key,
            base_url,
            model,
            temperature,
            max_tokens,
        }
    }

    pub async fn stream_chat(
        &self,
        messages: &[ChatMessage],
        system: &str,
        tools: &[ToolDefinition],
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamEvent>> + Send>>> {
        let mut api_messages = Vec::new();

        if !system.is_empty() {
            api_messages.push(OpenAiMessage {
                role: "system",
                content: system,
                tool_call_id: None,
            });
        }

        for msg in messages {
            let role = match msg.role {
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::System => "system",
                Role::Tool => "tool",
            };
            let tool_call_id = msg.tool_results.first().map(|r| r.tool_call_id.as_str());

            api_messages.push(OpenAiMessage {
                role,
                content: &msg.content,
                tool_call_id,
            });
        }

        let api_tools: Vec<OpenAiTool> = tools
            .iter()
            .map(|t| OpenAiTool {
                r#type: "function",
                function: OpenAiFunction {
                    name: &t.name,
                    description: &t.description,
                    parameters: &t.input_schema,
                },
            })
            .collect();

        let request = ChatRequest {
            model: &self.model,
            messages: api_messages,
            temperature: Some(self.temperature),
            max_tokens: Some(self.max_tokens),
            stream: true,
            tools: api_tools,
        };

        let client = reqwest::Client::new();
        let url = format!("{}/chat/completions", self.base_url);
        let req = client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request);

        let es = EventSource::new(req).context("Failed to create SSE connection to OpenAI")?;

        let stream = async_stream::stream! {
            let mut es = es;
            let mut current_tool_id = String::new();
            let mut current_tool_name = String::new();
            let mut current_tool_args = String::new();

            while let Some(event) = es.next().await {
                match event {
                    Ok(Event::Open) => continue,
                    Ok(Event::Message(msg)) => {
                        if msg.data.trim() == "[DONE]" {
                            if !current_tool_id.is_empty() {
                                yield Ok(StreamEvent::ToolCall {
                                    id: current_tool_id.clone(),
                                    name: current_tool_name.clone(),
                                    arguments: current_tool_args.clone(),
                                });
                            }
                            yield Ok(StreamEvent::Done);
                            break;
                        }

                        if let Ok(chunk) = serde_json::from_str::<StreamChunk>(&msg.data) {
                            for choice in chunk.choices {
                                if let Some(content) = choice.delta.content {
                                    if !content.is_empty() {
                                        yield Ok(StreamEvent::Token(content));
                                    }
                                }
                                if let Some(tool_calls) = choice.delta.tool_calls {
                                    for tc in tool_calls {
                                        if let Some(id) = tc.id {
                                            current_tool_id = id;
                                        }
                                        if let Some(func) = tc.function {
                                            if let Some(name) = func.name {
                                                current_tool_name.push_str(&name);
                                            }
                                            if let Some(args) = func.arguments {
                                                current_tool_args.push_str(&args);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        yield Err(anyhow::anyhow!("OpenAI SSE Error: {:?}", e));
                        break;
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }
}
