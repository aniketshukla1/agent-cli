# ⚡ Agent CLI

A high-performance, modern, and extensible CLI Agent built in **Rust** with **Ratatui**, featuring multi-provider LLM support, native Model Context Protocol (MCP) client capabilities, an extensible skills engine, interactive modals, and curated color themes.

![Theme](https://img.shields.io/badge/Theme-Catppuccin%20%7C%20Tokyo%20Night%20%7C%20Nord%20%7C%20Gruvbox%20%7C%20Cyberpunk-blueviolet)
![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

---

## 🌟 Key Features

1. **Stunning Terminal User Interface (TUI)**:
   - Built with `ratatui` and `crossterm`.
   - Rich Markdown rendering with code fences, headers, bullets, inline code, and status alerts.
   - Live token streaming with thinking spinner animations (`⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ Thinking...`).
   - Smart multiline input box with slash command suggestions popup on typing `/`.
   - Live status bar tracking tokens, active model, connected MCP servers, and hotkeys.

2. **Multi-Model Provider Support**:
   - **Anthropic**: Claude 3.7 Sonnet, Claude 3.5 Sonnet, Claude 3.5 Haiku, Opus.
   - **OpenAI**: GPT-4o, GPT-4o-mini, o3-mini, o1.
   - **Google Gemini**: Gemini 2.5 Pro, Gemini 2.5 Flash, Gemini 2.0 Flash.
   - **Local Ollama**: Llama 3.3, Qwen 2.5 Coder, DeepSeek-R1, Phi-4.
   - **OpenRouter / OpenAI-Compatible**: Groq, Together, DeepSeek, vLLM, etc.
   - Dynamic model switching at runtime via interactive `/model` modal or `Ctrl+P`.

3. **Curated Themes**:
   - **Catppuccin Mocha** (Modern pastel dark)
   - **Tokyo Night** (Vibrant neo-tokyo neon)
   - **Nord** (Arctic calm blues)
   - **Gruvbox Dark** (Warm retro tones)
   - **Cyberpunk** (High-energy cyan & magenta)
   - **Monokai** (Classic editor contrast)
   - **Minimalist Light** (Crisp daytime look)
   - Interactive theme picker (`/theme` or `Ctrl+T`) with instant preview and auto-saving to `config.toml`.

4. **Model Context Protocol (MCP) Client**:
   - Native Stdio JSON-RPC 2.0 client.
   - Connect any standard MCP server (e.g. `@modelcontextprotocol/server-filesystem`, fetch, github, database).
   - Auto-discovers tools, maps schemas to LLM definitions, and handles execution with interactive safety confirmation dialogs.
   - Inspect connected servers and tool definitions anytime via `/mcp`.

5. **Extensible Skills System**:
   - Scans `.agent/skills/`, `skills/`, and `~/.config/agent-cli/skills/`.
   - Supports standard `SKILL.md` format with YAML frontmatter, markdown instructions, and executable scripts.
   - Interactive `/skills` modal to browse, inspect, and invoke skills directly into conversation context.

6. **Safety & Tool Control**:
   - Built-in tools: shell execution (`run_command`), file reader (`read_file`), file writer (`write_file`), and directory inspector (`list_directory`).
   - Interactive tool approval modal before running commands (`[Y]es once`, `[N]o deny`, `[A]lways allow for session`).

7. **Clean Exit & Persistence**:
   - Clean shutdown with `/exit`, `/quit`, `Ctrl+C` (gracefully stops active generation first, double-press to exit), or `Esc`.
   - Conversations auto-saved to disk. Resume anytime with `--resume`.

---

## 🚀 Quick Start

### Build & Run
```bash
cargo build --release
./target/release/agent-cli
```

### CLI Flags
```bash
# Launch with a specific model
./target/release/agent-cli --model gpt-4o

# Launch with a specific theme
./target/release/agent-cli --theme tokyo_night

# Resume the previous session
./target/release/agent-cli --resume

# One-shot initial prompt
./target/release/agent-cli "Explain how Rust ownership works in 3 bullet points"
```

---

## ⌨️ Slash Commands & Shortcuts

| Command | Shortcut | Description |
|---|---|---|
| `/model` | `Ctrl + P` | Open interactive model picker |
| `/theme` | `Ctrl + T` | Open theme picker with live preview |
| `/skills` | - | Open skills manager and inspector |
| `/mcp` | - | Open MCP server dashboard and tool viewer |
| `/clear` | - | Clear conversation history |
| `/help` | - | Open help and shortcuts reference |
| `/exit` | `Ctrl + C` | Exit the CLI agent |
| `PageUp` / `PageDown` | - | Scroll chat conversation view |
| `Tab` | - | Auto-complete slash command |
| `Shift + Enter` | - | Insert newline in prompt input |

---

## ⚙️ Configuration

Configuration is automatically loaded from `./agent.toml` (workspace-local) or `~/.config/agent-cli/config.toml` (global).

```toml
[general]
theme = "catppuccin_mocha"
default_model = "claude-3-7-sonnet"
require_tool_confirmation = true
max_tokens = 4096
temperature = 0.7

[providers]
# Keys can also be set as environment variables:
# export ANTHROPIC_API_KEY="..."
# export OPENAI_API_KEY="..."
# export GEMINI_API_KEY="..."
ollama_host = "http://localhost:11434"

# Configure MCP servers:
[mcp_servers.filesystem]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-filesystem", "."]
```

---

## 🧩 Creating a Skill

Place a `SKILL.md` file inside `.agent/skills/<skill-name>/SKILL.md`:

```markdown
---
name: my-skill
description: Custom task instructions or workflow
version: 1.0.0
---

# My Custom Skill
Instructions for the AI agent...
```

Trigger it inside the agent using `/skills` or `/skill my-skill`!
