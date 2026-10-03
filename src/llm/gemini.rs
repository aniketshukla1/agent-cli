use anyhow::{Context, Result};
use futures::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;

use crate::llm::types::{StreamEvent, ToolDefinition};
use crate::session::{ChatMessage, Role};

#[derive(Clone)]
pub struct GeminiProvider {
    pub api_key: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Serialize)]
struct GeminiRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiContent<'a>>,
    contents: Vec<GeminiContent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GenerationConfig>,
}

#[derive(Serialize)]
struct GenerationConfig {
    temperature: f32,
    max_output_tokens: u32,
}

#[derive(Serialize)]
struct GeminiContent<'a> {
    role: &'a str,
    parts: Vec<GeminiPart<'a>>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum GeminiPart<'a> {
    Text { text: &'a str },
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiCandidateContent>,
}

#[derive(Deserialize)]
struct GeminiCandidateContent {
    parts: Option<Vec<GeminiCandidatePart>>,
}

#[derive(Deserialize)]
struct GeminiCandidatePart {
    text: Option<String>,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: String, temperature: f32, max_tokens: u32) -> Self {
        Self {
            api_key,
            model,
            temperature,
            max_tokens,
        }
    }

    pub async fn stream_chat(
        &self,
        messages: &[ChatMessage],
        system: &str,
        _tools: &[ToolDefinition],
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamEvent>> + Send>>> {
        let system_instruction = if !system.is_empty() {
            Some(GeminiContent {
                role: "system",
                parts: vec![GeminiPart::Text { text: system }],
            })
        } else {
            None
        };

        let mut contents = Vec::new();
        for msg in messages {
            let role = match msg.role {
                Role::User => "user",
                Role::Assistant => "model",
                Role::System => "user",
                Role::Tool => "user",
            };
            contents.push(GeminiContent {
                role,
                parts: vec![GeminiPart::Text {
                    text: &msg.content,
                }],
            });
        }

        let request = GeminiRequest {
            system_instruction,
            contents,
            generation_config: Some(GenerationConfig {
                temperature: self.temperature,
                max_output_tokens: self.max_tokens,
            }),
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?alt=sse&key={}",
            self.model, self.api_key
        );

        let client = reqwest::Client::new();
        let req = client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&request);

        let es = EventSource::new(req).context("Failed to connect to Gemini SSE")?;

        let stream = async_stream::stream! {
            let mut es = es;
            while let Some(event) = es.next().await {
                match event {
                    Ok(Event::Open) => continue,
                    Ok(Event::Message(msg)) => {
                        if let Ok(resp) = serde_json::from_str::<GeminiResponse>(&msg.data) {
                            if let Some(candidates) = resp.candidates {
                                for candidate in candidates {
                                    if let Some(content) = candidate.content {
                                        if let Some(parts) = content.parts {
                                            for part in parts {
                                                if let Some(text) = part.text {
                                                    yield Ok(StreamEvent::Token(text));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        yield Err(anyhow::anyhow!("Gemini SSE error: {:?}", e));
                        break;
                    }
                }
            }
            yield Ok(StreamEvent::Done);
        };

        Ok(Box::pin(stream))
    }
}
