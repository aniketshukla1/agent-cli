use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::theme::ThemeKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub providers: ProvidersConfig,
    #[serde(default)]
    pub mcp_servers: HashMap<String, McpServerConfig>,
    #[serde(default)]
    pub custom_models: Vec<ModelOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_theme")]
    pub theme: ThemeKind,
    #[serde(default = "default_model")]
    pub default_model: String,
    #[serde(default = "default_system_prompt")]
    pub system_prompt: String,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_true")]
    pub stream: bool,
    #[serde(default = "default_true")]
    pub require_tool_confirmation: bool,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            default_model: default_model(),
            system_prompt: default_system_prompt(),
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
            stream: default_true(),
            require_tool_confirmation: default_true(),
        }
    }
}

fn default_theme() -> ThemeKind {
    ThemeKind::CatppuccinMocha
}
fn default_model() -> String {
    "claude-3-7-sonnet".to_string()
}
fn default_system_prompt() -> String {
    "You are a helpful, precise, and proactive AI assistant operating in a terminal environment. You can call tools and execute tasks thoughtfully.".to_string()
}
fn default_max_tokens() -> u32 {
    4096
}
fn default_temperature() -> f32 {
    0.7
}
fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProvidersConfig {
    pub anthropic_api_key: Option<String>,
    pub openai_api_key: Option<String>,
    pub openai_base_url: Option<String>,
    pub gemini_api_key: Option<String>,
    pub openrouter_api_key: Option<String>,
    pub ollama_host: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelOption {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub context_length: u32,
    pub description: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            providers: ProvidersConfig {
                anthropic_api_key: None,
                openai_api_key: None,
                openai_base_url: None,
                gemini_api_key: None,
                openrouter_api_key: None,
                ollama_host: Some("http://localhost:11434".to_string()),
            },
            mcp_servers: HashMap::new(),
            custom_models: Vec::new(),
        }
    }
}

impl Config {
    pub fn config_dir() -> PathBuf {
        if let Some(proj_dirs) = directories::ProjectDirs::from("com", "agent", "agent-cli") {
            proj_dirs.config_dir().to_path_buf()
        } else {
            PathBuf::from(".config/agent-cli")
        }
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    pub fn load() -> Self {
        // Priority 1: local ./agent.toml
        if Path::new("agent.toml").exists() {
            if let Ok(content) = fs::read_to_string("agent.toml") {
                if let Ok(mut cfg) = toml::from_str::<Config>(&content) {
                    cfg.override_from_env();
                    return cfg;
                }
            }
        }

        // Priority 2: global ~/.config/agent-cli/config.toml
        let global_path = Self::config_path();
        if global_path.exists() {
            if let Ok(content) = fs::read_to_string(&global_path) {
                if let Ok(mut cfg) = toml::from_str::<Config>(&content) {
                    cfg.override_from_env();
                    return cfg;
                }
            }
        }

        let mut default_cfg = Config::default();
        default_cfg.override_from_env();
        // Save initial default configuration if directory can be created
        let _ = default_cfg.save();
        default_cfg
    }

    pub fn override_from_env(&mut self) {
        if let Ok(val) = std::env::var("ANTHROPIC_API_KEY") {
            if !val.trim().is_empty() {
                self.providers.anthropic_api_key = Some(val);
            }
        }
        if let Ok(val) = std::env::var("OPENAI_API_KEY") {
            if !val.trim().is_empty() {
                self.providers.openai_api_key = Some(val);
            }
        }
        if let Ok(val) = std::env::var("OPENAI_BASE_URL") {
            if !val.trim().is_empty() {
                self.providers.openai_base_url = Some(val);
            }
        }
        if let Ok(val) = std::env::var("GEMINI_API_KEY") {
            if !val.trim().is_empty() {
                self.providers.gemini_api_key = Some(val);
            }
        }
        if let Ok(val) = std::env::var("OPENROUTER_API_KEY") {
            if !val.trim().is_empty() {
                self.providers.openrouter_api_key = Some(val);
            }
        }
        if let Ok(val) = std::env::var("OLLAMA_HOST") {
            if !val.trim().is_empty() {
                self.providers.ollama_host = Some(val);
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        let dir = Self::config_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)
                .with_context(|| format!("Failed to create config dir {}", dir.display()))?;
        }
        let path = Self::config_path();
        let content = toml::to_string_pretty(self).context("Failed to serialize config to TOML")?;
        fs::write(&path, content)
            .with_context(|| format!("Failed to write config file to {}", path.display()))?;
        Ok(())
    }

    pub fn get_available_models(&self) -> Vec<ModelOption> {
        let mut list = vec![
            ModelOption {
                id: "claude-3-7-sonnet".to_string(),
                name: "Claude 3.7 Sonnet".to_string(),
                provider: "Anthropic".to_string(),
                context_length: 200_000,
                description: "State-of-the-art hybrid reasoning & coding model".to_string(),
            },
            ModelOption {
                id: "claude-3-5-sonnet".to_string(),
                name: "Claude 3.5 Sonnet".to_string(),
                provider: "Anthropic".to_string(),
                context_length: 200_000,
                description: "Fast, highly intelligent coding and analysis".to_string(),
            },
            ModelOption {
                id: "claude-3-5-haiku".to_string(),
                name: "Claude 3.5 Haiku".to_string(),
                provider: "Anthropic".to_string(),
                context_length: 200_000,
                description: "Lightweight, ultra-fast responses".to_string(),
            },
            ModelOption {
                id: "gpt-4o".to_string(),
                name: "GPT-4o".to_string(),
                provider: "OpenAI".to_string(),
                context_length: 128_000,
                description: "Flagship versatile multimodal OpenAI model".to_string(),
            },
            ModelOption {
                id: "gpt-4o-mini".to_string(),
                name: "GPT-4o Mini".to_string(),
                provider: "OpenAI".to_string(),
                context_length: 128_000,
                description: "Cost-effective, snappy, intelligent".to_string(),
            },
            ModelOption {
                id: "o3-mini".to_string(),
                name: "o3-mini".to_string(),
                provider: "OpenAI".to_string(),
                context_length: 200_000,
                description: "Advanced STEM and coding reasoning model".to_string(),
            },
            ModelOption {
                id: "gemini-2.5-pro".to_string(),
                name: "Gemini 2.5 Pro".to_string(),
                provider: "Gemini".to_string(),
                context_length: 1_000_000,
                description: "Massive 1M+ context reasoning powerhouse".to_string(),
            },
            ModelOption {
                id: "gemini-2.5-flash".to_string(),
                name: "Gemini 2.5 Flash".to_string(),
                provider: "Gemini".to_string(),
                context_length: 1_000_000,
                description: "Ultra-fast multimodal performance at scale".to_string(),
            },
            ModelOption {
                id: "llama3.3:latest".to_string(),
                name: "Llama 3.3 (70B)".to_string(),
                provider: "Ollama".to_string(),
                context_length: 128_000,
                description: "Top-tier open weights model on local Ollama".to_string(),
            },
            ModelOption {
                id: "qwen2.5-coder:latest".to_string(),
                name: "Qwen 2.5 Coder".to_string(),
                provider: "Ollama".to_string(),
                context_length: 32_000,
                description: "Specialized local coding and agent model".to_string(),
            },
            ModelOption {
                id: "deepseek-r1:latest".to_string(),
                name: "DeepSeek R1".to_string(),
                provider: "Ollama".to_string(),
                context_length: 64_000,
                description: "Open reasoning model with chain-of-thought".to_string(),
            },
        ];

        // Append custom models
        for m in &self.custom_models {
            list.push(m.clone());
        }

        list
    }
}
