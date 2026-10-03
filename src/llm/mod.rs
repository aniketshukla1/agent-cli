pub mod anthropic;
pub mod gemini;
pub mod mock;
pub mod ollama;
pub mod openai;
pub mod types;

use anyhow::Result;
use std::pin::Pin;
use tokio_stream::Stream;

pub use types::{StreamEvent, ToolDefinition};

use crate::config::Config;
use crate::llm::anthropic::AnthropicProvider;
use crate::llm::gemini::GeminiProvider;
use crate::llm::mock::MockProvider;
use crate::llm::ollama::OllamaProvider;
use crate::llm::openai::OpenAiProvider;
use crate::session::ChatMessage;

pub enum ProviderInstance {
    Anthropic(AnthropicProvider),
    OpenAi(OpenAiProvider),
    Gemini(GeminiProvider),
    Ollama(OllamaProvider),
    Mock(MockProvider),
}

impl ProviderInstance {
    pub fn create(config: &Config, model_id: &str) -> Self {
        let temp = config.general.temperature;
        let max_tokens = config.general.max_tokens;

        // 1. Anthropic models (claude-*)
        if model_id.starts_with("claude-") {
            if let Some(key) = &config.providers.anthropic_api_key {
                if !key.trim().is_empty() {
                    return ProviderInstance::Anthropic(AnthropicProvider::new(
                        key.clone(),
                        model_id.to_string(),
                        max_tokens,
                        temp,
                    ));
                }
            }
        }

        // 2. Gemini models (gemini-*)
        if model_id.starts_with("gemini-") {
            if let Some(key) = &config.providers.gemini_api_key {
                if !key.trim().is_empty() {
                    return ProviderInstance::Gemini(GeminiProvider::new(
                        key.clone(),
                        model_id.to_string(),
                        temp,
                        max_tokens,
                    ));
                }
            }
        }

        // 3. Ollama models (*:latest, or model contains :)
        if model_id.contains(':') || model_id.contains("llama") || model_id.contains("qwen") || model_id.contains("deepseek") {
            return ProviderInstance::Ollama(OllamaProvider::new(
                config.providers.ollama_host.clone(),
                model_id.to_string(),
            ));
        }

        // 4. OpenAI / OpenRouter / Custom compatible endpoint
        if let Some(key) = &config.providers.openai_api_key {
            if !key.trim().is_empty() {
                return ProviderInstance::OpenAi(OpenAiProvider::new(
                    key.clone(),
                    config.providers.openai_base_url.clone(),
                    model_id.to_string(),
                    temp,
                    max_tokens,
                ));
            }
        }

        if let Some(key) = &config.providers.openrouter_api_key {
            if !key.trim().is_empty() {
                return ProviderInstance::OpenAi(OpenAiProvider::new(
                    key.clone(),
                    Some("https://openrouter.ai/api/v1".to_string()),
                    model_id.to_string(),
                    temp,
                    max_tokens,
                ));
            }
        }

        // 5. Fallback: Mock provider for demo/testing
        ProviderInstance::Mock(MockProvider::new(model_id.to_string()))
    }

    pub async fn stream_chat(
        &self,
        messages: &[ChatMessage],
        system: &str,
        tools: &[ToolDefinition],
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamEvent>> + Send>>> {
        match self {
            ProviderInstance::Anthropic(p) => p.stream_chat(messages, system, tools).await,
            ProviderInstance::OpenAi(p) => p.stream_chat(messages, system, tools).await,
            ProviderInstance::Gemini(p) => p.stream_chat(messages, system, tools).await,
            ProviderInstance::Ollama(p) => p.stream_chat(messages, system, tools).await,
            ProviderInstance::Mock(p) => p.stream_chat(messages, system, tools).await,
        }
    }
}
