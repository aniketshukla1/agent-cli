use anyhow::Result;
use std::pin::Pin;
use tokio::time::{sleep, Duration};
use tokio_stream::Stream;

use crate::llm::types::{StreamEvent, ToolDefinition};
use crate::session::ChatMessage;

#[derive(Clone)]
pub struct MockProvider {
    pub model: String,
}

impl MockProvider {
    pub fn new(model: String) -> Self {
        Self { model }
    }

    pub async fn stream_chat(
        &self,
        messages: &[ChatMessage],
        _system: &str,
        _tools: &[ToolDefinition],
    ) -> Result<Pin<Box<dyn Stream<Item = Result<StreamEvent>> + Send>>> {
        let last_prompt = messages
            .last()
            .map(|m| m.content.clone())
            .unwrap_or_else(|| "Hello".to_string());

        let model_name = self.model.clone();

        let stream = async_stream::stream! {
            let intro = format!(
                "⚡ [Running in offline/demo mode for '{}']\n\n\
                You asked: \"{}\"\n\n\
                To connect real AI models, configure your API keys:\n\
                - Anthropic: set `export ANTHROPIC_API_KEY=\"...\"` or in `~/.config/agent-cli/config.toml`\n\
                - OpenAI / OpenRouter: set `export OPENAI_API_KEY=\"...\"`\n\
                - Google Gemini: set `export GEMINI_API_KEY=\"...\"`\n\
                - Local Ollama: start Ollama (`ollama serve`) and pick an Ollama model\n\n\
                You can try these commands right now:\n\
                - `/model` - Switch models or pick Ollama\n\
                - `/theme` - Change terminal color theme\n\
                - `/skills` - Explore local skills\n\
                - `/mcp` - Connect MCP servers\n\
                - `/help` - View complete command manual",
                model_name,
                last_prompt
            );

            for word in intro.split_inclusive(' ') {
                sleep(Duration::from_millis(25)).await;
                yield Ok(StreamEvent::Token(word.to_string()));
            }

            yield Ok(StreamEvent::Done);
        };

        Ok(Box::pin(stream))
    }
}
