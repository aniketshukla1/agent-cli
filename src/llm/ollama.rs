use anyhow::{Context, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;
use tokio_util::codec::{FramedRead, LinesCodec};

use crate::llm::types::{StreamEvent, ToolDefinition};
use crate::session::{ChatMessage, Role};

#[derive(Clone)]
pub struct OllamaProvider {
    pub host: String,
    pub model: String,
}

#[derive(Serialize)]
struct OllamaChatRequest<'a> {
    model: &'a str,
    messages: Vec<OllamaMessage<'a>>,
    stream: bool,
}

#[derive(Serialize)]
struct OllamaMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct OllamaChatChunk {
    message: Option<OllamaDelta>,
    done: bool,
}

#[derive(Deserialize)]
struct OllamaDelta {
    content: Option<String>,
}

impl OllamaProvider {
    pub fn new(host: Option<String>, model: String) -> Self {
        let host = host
            .unwrap_or_else(|| "http://localhost:11434".to_string())
            .trim_end_matches('/')
            .to_string();
        Self { host, model }
    }

    pub async fn stream_chat(
        &self,
        messages: &[ChatMessage],
        system: &str,
        _tools: &[ToolDefinition],
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamEvent>> + Send>>> {
        let mut api_messages = Vec::new();

        if !system.is_empty() {
            api_messages.push(OllamaMessage {
                role: "system",
                content: system,
            });
        }

        for msg in messages {
            let role = match msg.role {
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::System => "system",
                Role::Tool => "user",
            };
            api_messages.push(OllamaMessage {
                role,
                content: &msg.content,
            });
        }

        let request = OllamaChatRequest {
            model: &self.model,
            messages: api_messages,
            stream: true,
        };

        let client = reqwest::Client::new();
        let url = format!("{}/api/chat", self.host);
        let resp = client
            .post(url)
            .json(&request)
            .send()
            .await
            .context("Failed to connect to local Ollama server")?;

        let byte_stream = resp
            .bytes_stream()
            .map(|result| result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));

        let stream_reader = tokio_util::io::StreamReader::new(byte_stream);
        let mut lines = FramedRead::new(stream_reader, LinesCodec::new());

        let stream = async_stream::stream! {
            while let Some(line_res) = lines.next().await {
                match line_res {
                    Ok(line) => {
                        if line.trim().is_empty() {
                            continue;
                        }
                        if let Ok(chunk) = serde_json::from_str::<OllamaChatChunk>(&line) {
                            if let Some(msg) = chunk.message {
                                if let Some(content) = msg.content {
                                    if !content.is_empty() {
                                        yield Ok(StreamEvent::Token(content));
                                    }
                                }
                            }
                            if chunk.done {
                                yield Ok(StreamEvent::Done);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        yield Err(anyhow::anyhow!("Ollama stream read error: {:?}", e));
                        break;
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }
}
