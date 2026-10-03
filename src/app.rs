use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use crate::config::Config;
use crate::llm::types::StreamEvent;
use crate::llm::ProviderInstance;
use crate::mcp::McpManager;
use crate::session::{ChatMessage, Session};
use crate::skills::SkillsManager;
use crate::theme::Theme;
use crate::tools::BuiltinTools;
use crate::ui::chat::WELCOME_OPTIONS;
use crate::ui::input::SLASH_COMMANDS;
use crate::ui::modals::confirm_modal::{ConfirmAction, ConfirmModal};
use crate::ui::modals::help_modal::HelpModal;
use crate::ui::modals::mcp_modal::{McpModal, McpModalState};
use crate::ui::modals::model_picker::{ModelPickerModal, ModelPickerState};
use crate::ui::modals::skills_modal::{SkillsModal, SkillsModalState};
use crate::ui::modals::theme_picker::{ThemePickerModal, ThemePickerState};
use crate::ui::status_bar::AppMode;
use crate::ui::{ChatView, Header, InputBox, StatusBar};

pub enum ActiveModal {
    ModelPicker(ModelPickerState),
    ThemePicker(ThemePickerState),
    Help,
    Skills(SkillsModalState),
    Mcp(McpModalState),
    Confirm(ConfirmAction),
}

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
    LlmToken(String),
    LlmToolCall {
        id: String,
        name: String,
        arguments: String,
    },
    LlmDone,
    LlmError(String),
}

pub struct App {
    pub config: Config,
    pub theme: Theme,
    pub session: Session,
    pub mcp_manager: Arc<McpManager>,
    pub skills_manager: SkillsManager,

    pub input_text: String,
    pub suggestion_index: usize,
    pub welcome_option_index: usize,
    pub scroll_offset: u16,

    pub history: Vec<String>,
    pub history_index: Option<usize>,

    pub is_streaming: bool,
    pub streaming_buffer: String,
    pub spinner_tick: usize,

    pub active_modal: Option<ActiveModal>,
    pub running: bool,

    pub always_allow_tools: bool,
    pub pending_tool_call: Option<(String, String, String)>,

    pub event_tx: Sender<AppEvent>,
}

impl App {
    pub fn new(config: Config, event_tx: Sender<AppEvent>) -> Self {
        let theme = Theme::from_kind(config.general.theme);
        let session = Session::new(&config.general.default_model);
        let mcp_manager = Arc::new(McpManager::new());
        let skills_manager = SkillsManager::load_all();

        Self {
            config,
            theme,
            session,
            mcp_manager,
            skills_manager,
            input_text: String::new(),
            suggestion_index: 0,
            welcome_option_index: 0,
            scroll_offset: 0,
            history: Vec::new(),
            history_index: None,
            is_streaming: false,
            streaming_buffer: String::new(),
            spinner_tick: 0,
            active_modal: None,
            running: true,
            always_allow_tools: false,
            pending_tool_call: None,
            event_tx,
        }
    }

    pub async fn initialize(&mut self) {
        self.mcp_manager.init_from_config(&self.config).await;
    }

    pub fn mode(&self) -> AppMode {
        if self.active_modal.is_some() {
            AppMode::Modal
        } else if self.is_streaming {
            AppMode::Streaming
        } else {
            AppMode::Insert
        }
    }

    pub fn on_tick(&mut self) {
        if self.is_streaming {
            self.spinner_tick = (self.spinner_tick + 1) % 100;
        }
    }

    pub async fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        // Global interrupt / Exit checks
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            if self.is_streaming {
                self.is_streaming = false;
                self.streaming_buffer.clear();
                return Ok(());
            } else {
                self.active_modal = Some(ActiveModal::Confirm(ConfirmAction::ExitApp));
                return Ok(());
            }
        }

        // Shortcut: Ctrl+P -> Model picker
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('p') {
            self.open_model_picker();
            return Ok(());
        }

        // Shortcut: Ctrl+T -> Theme picker
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('t') {
            self.open_theme_picker();
            return Ok(());
        }

        // Modal routing
        if let Some(modal) = &mut self.active_modal {
            match modal {
                ActiveModal::ModelPicker(state) => match key.code {
                    KeyCode::Esc => {
                        self.active_modal = None;
                    }
                    KeyCode::Up => state.previous(),
                    KeyCode::Down => state.next(),
                    KeyCode::Backspace => {
                        state.search_query.pop();
                    }
                    KeyCode::Char(c) => {
                        state.search_query.push(c);
                    }
                    KeyCode::Enter => {
                        if let Some(selected) = state.selected_model() {
                            self.session.active_model = selected.id.clone();
                            self.config.general.default_model = selected.id;
                            let _ = self.config.save();
                        }
                        self.active_modal = None;
                    }
                    _ => {}
                },
                ActiveModal::ThemePicker(state) => match key.code {
                    KeyCode::Esc => {
                        self.active_modal = None;
                    }
                    KeyCode::Up => {
                        state.previous();
                        self.theme = Theme::from_kind(state.selected_theme());
                    }
                    KeyCode::Down => {
                        state.next();
                        self.theme = Theme::from_kind(state.selected_theme());
                    }
                    KeyCode::Enter => {
                        let selected = state.selected_theme();
                        self.theme = Theme::from_kind(selected);
                        self.config.general.theme = selected;
                        let _ = self.config.save();
                        self.active_modal = None;
                    }
                    _ => {}
                },
                ActiveModal::Help => match key.code {
                    KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                        self.active_modal = None;
                    }
                    _ => {}
                },
                ActiveModal::Skills(state) => match key.code {
                    KeyCode::Esc => {
                        self.active_modal = None;
                    }
                    KeyCode::Up => state.previous(),
                    KeyCode::Down => state.next(),
                    KeyCode::Enter => {
                        if let Some(skill) = state.selected_skill() {
                            let skill_prompt = format!(
                                "Please follow the instructions from skill '{}':\n\n{}",
                                skill.metadata.name, skill.instructions
                            );
                            self.session.messages.push(ChatMessage::user(skill_prompt));
                            self.trigger_llm_stream().await;
                        }
                        self.active_modal = None;
                    }
                    _ => {}
                },
                ActiveModal::Mcp(state) => match key.code {
                    KeyCode::Esc => {
                        self.active_modal = None;
                    }
                    KeyCode::Up => state.previous(),
                    KeyCode::Down => state.next(),
                    _ => {}
                },
                ActiveModal::Confirm(action) => match action {
                    ConfirmAction::ExitApp => match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                            self.running = false;
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            self.active_modal = None;
                        }
                        _ => {}
                    },
                    ConfirmAction::ToolExecution { .. } => match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                            self.active_modal = None;
                            self.execute_pending_tool(false).await;
                        }
                        KeyCode::Char('a') | KeyCode::Char('A') => {
                            self.always_allow_tools = true;
                            self.active_modal = None;
                            self.execute_pending_tool(false).await;
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            self.active_modal = None;
                            self.execute_pending_tool(true).await;
                        }
                        _ => {}
                    },
                },
            }
            return Ok(());
        }

        // Navigation Case 1: Slash command auto-suggestions
        let is_slash_mode = self.input_text.starts_with('/') && !self.input_text.contains(' ');
        if is_slash_mode {
            let query = self.input_text.to_lowercase();
            let matches: Vec<&'static str> = SLASH_COMMANDS
                .iter()
                .map(|c| c.name)
                .filter(|n| n.to_lowercase().starts_with(&query))
                .collect();

            if !matches.is_empty() {
                match key.code {
                    KeyCode::Up => {
                        if self.suggestion_index == 0 {
                            self.suggestion_index = matches.len() - 1;
                        } else {
                            self.suggestion_index -= 1;
                        }
                        return Ok(());
                    }
                    KeyCode::Down => {
                        self.suggestion_index = (self.suggestion_index + 1) % matches.len();
                        return Ok(());
                    }
                    KeyCode::Tab => {
                        self.input_text = matches[self.suggestion_index % matches.len()].to_string() + " ";
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        let selected_cmd = matches[self.suggestion_index % matches.len()];
                        self.input_text.clear();
                        self.execute_slash_command(selected_cmd).await;
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }

        // Navigation Case 2: Welcome screen options in chat (when no messages yet and input is empty)
        if self.session.messages.is_empty() && self.input_text.is_empty() {
            let total_opts = WELCOME_OPTIONS.len();
            match key.code {
                KeyCode::Up => {
                    if self.welcome_option_index == 0 {
                        self.welcome_option_index = total_opts - 1;
                    } else {
                        self.welcome_option_index -= 1;
                    }
                    return Ok(());
                }
                KeyCode::Down | KeyCode::Tab => {
                    self.welcome_option_index = (self.welcome_option_index + 1) % total_opts;
                    return Ok(());
                }
                KeyCode::Enter => {
                    let cmd = WELCOME_OPTIONS[self.welcome_option_index % total_opts].cmd;
                    self.execute_slash_command(cmd).await;
                    return Ok(());
                }
                _ => {}
            }
        }

        // Navigation Case 3: Prompt history when input is empty or using Up/Down
        if !self.session.messages.is_empty() && self.input_text.is_empty() {
            if key.code == KeyCode::Up && !self.history.is_empty() {
                let next_idx = match self.history_index {
                    Some(idx) => idx.saturating_sub(1),
                    None => self.history.len() - 1,
                };
                self.history_index = Some(next_idx);
                if let Some(item) = self.history.get(next_idx) {
                    self.input_text = item.clone();
                }
                return Ok(());
            }
        }

        // Chat View scrolling with PageUp/PageDown
        match key.code {
            KeyCode::PageUp => {
                self.scroll_offset = self.scroll_offset.saturating_add(5);
                return Ok(());
            }
            KeyCode::PageDown => {
                self.scroll_offset = self.scroll_offset.saturating_sub(5);
                return Ok(());
            }
            _ => {}
        }

        // Input textarea handling
        match key.code {
            KeyCode::Char(c) => {
                self.input_text.push(c);
                self.suggestion_index = 0;
            }
            KeyCode::Backspace => {
                self.input_text.pop();
                self.suggestion_index = 0;
            }
            KeyCode::Esc => {
                self.input_text.clear();
                self.scroll_offset = 0;
                self.history_index = None;
            }
            KeyCode::Enter => {
                let trimmed = self.input_text.trim().to_string();
                if trimmed.is_empty() {
                    return Ok(());
                }

                // Check for Slash Commands
                if trimmed.starts_with('/') {
                    self.execute_slash_command(&trimmed).await;
                    self.input_text.clear();
                    return Ok(());
                }

                // User chat prompt
                self.history.push(trimmed.clone());
                self.history_index = None;
                self.input_text.clear();
                self.scroll_offset = 0;
                self.session.messages.push(ChatMessage::user(trimmed));
                self.trigger_llm_stream().await;
            }
            _ => {}
        }

        Ok(())
    }

    pub async fn execute_slash_command(&mut self, cmd: &str) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        match parts[0] {
            "/model" => {
                if parts.len() > 1 {
                    self.session.active_model = parts[1].to_string();
                    self.config.general.default_model = parts[1].to_string();
                    let _ = self.config.save();
                } else {
                    self.open_model_picker();
                }
            }
            "/theme" => {
                self.open_theme_picker();
            }
            "/help" => {
                self.active_modal = Some(ActiveModal::Help);
            }
            "/skills" | "/skill" => {
                let modal = SkillsModalState::new(self.skills_manager.list().to_vec());
                self.active_modal = Some(ActiveModal::Skills(modal));
            }
            "/mcp" => {
                let statuses = self.mcp_manager.get_server_statuses().await;
                let tools = self.mcp_manager.get_all_tools().await;
                let modal = McpModalState::new(statuses, tools);
                self.active_modal = Some(ActiveModal::Mcp(modal));
            }
            "/clear" => {
                self.session.messages.clear();
                self.scroll_offset = 0;
                self.welcome_option_index = 0;
            }
            "/exit" | "/quit" => {
                self.active_modal = Some(ActiveModal::Confirm(ConfirmAction::ExitApp));
            }
            _ => {
                self.session.messages.push(ChatMessage::system(format!(
                    "Unknown command '{}'. Type /help for available commands.",
                    parts[0]
                )));
            }
        }
    }

    pub fn open_model_picker(&mut self) {
        let models = self.config.get_available_models();
        let state = ModelPickerState::new(models, &self.session.active_model);
        self.active_modal = Some(ActiveModal::ModelPicker(state));
    }

    pub fn open_theme_picker(&mut self) {
        let state = ThemePickerState::new(self.theme.kind);
        self.active_modal = Some(ActiveModal::ThemePicker(state));
    }

    pub async fn trigger_llm_stream(&mut self) {
        self.is_streaming = true;
        self.streaming_buffer.clear();

        let model_id = self.session.active_model.clone();
        let provider = ProviderInstance::create(&self.config, &model_id);
        let messages = self.session.messages.clone();

        // Collect all tools (builtin + MCP)
        let mut tools = BuiltinTools::definitions();
        let mcp_tools = self.mcp_manager.get_all_tools().await;
        tools.extend(mcp_tools);

        let system_prompt = self.config.general.system_prompt.clone();
        let tx = self.event_tx.clone();

        tokio::spawn(async move {
            match provider.stream_chat(&messages, &system_prompt, &tools).await {
                Ok(mut stream) => {
                    use futures::StreamExt;
                    while let Some(event_res) = stream.next().await {
                        match event_res {
                            Ok(StreamEvent::Token(token)) => {
                                let _ = tx.send(AppEvent::LlmToken(token)).await;
                            }
                            Ok(StreamEvent::ToolCall { id, name, arguments }) => {
                                let _ = tx.send(AppEvent::LlmToolCall { id, name, arguments }).await;
                            }
                            Ok(StreamEvent::Done) => {
                                let _ = tx.send(AppEvent::LlmDone).await;
                                break;
                            }
                            Ok(StreamEvent::Thinking(_)) => {}
                            Err(e) => {
                                let _ = tx.send(AppEvent::LlmError(e.to_string())).await;
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(AppEvent::LlmError(e.to_string())).await;
                }
            }
        });
    }

    pub async fn handle_llm_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::LlmToken(tok) => {
                self.streaming_buffer.push_str(&tok);
            }
            AppEvent::LlmToolCall { id, name, arguments } => {
                self.is_streaming = false;

                let mut assistant_msg = ChatMessage::assistant(std::mem::take(&mut self.streaming_buffer));
                assistant_msg.tool_calls.push(crate::session::ToolCall {
                    id: id.clone(),
                    name: name.clone(),
                    arguments: arguments.clone(),
                });
                self.session.messages.push(assistant_msg);

                if self.config.general.require_tool_confirmation && !self.always_allow_tools {
                    self.pending_tool_call = Some((id, name.clone(), arguments.clone()));
                    self.active_modal = Some(ActiveModal::Confirm(ConfirmAction::ToolExecution {
                        tool_name: name,
                        arguments,
                    }));
                } else {
                    self.pending_tool_call = Some((id, name, arguments));
                    self.execute_pending_tool(false).await;
                }
            }
            AppEvent::LlmDone => {
                self.is_streaming = false;
                if !self.streaming_buffer.is_empty() {
                    let msg = ChatMessage::assistant(std::mem::take(&mut self.streaming_buffer));
                    self.session.messages.push(msg);
                }
                let _ = self.session.save();
            }
            AppEvent::LlmError(err_msg) => {
                self.is_streaming = false;
                self.session.messages.push(ChatMessage::system(format!("Error: {}", err_msg)));
            }
            _ => {}
        }
    }

    pub async fn execute_pending_tool(&mut self, rejected: bool) {
        if let Some((id, name, args)) = self.pending_tool_call.take() {
            if rejected {
                self.session.messages.push(ChatMessage::tool_response(
                    id,
                    "Tool execution was denied by the user.",
                    true,
                ));
            } else {
                let result = if name.contains("__") {
                    match self.mcp_manager.call_tool(&name, &args).await {
                        Ok(res) => {
                            let text = res
                                .content
                                .iter()
                                .filter_map(|c| c.text.clone())
                                .collect::<Vec<_>>()
                                .join("\n");
                            (text, res.is_error)
                        }
                        Err(e) => (format!("MCP error: {:?}", e), true),
                    }
                } else {
                    match BuiltinTools::execute(&name, &args).await {
                        Ok(output) => (output, false),
                        Err(e) => (format!("Tool execution error: {:?}", e), true),
                    }
                };

                self.session.messages.push(ChatMessage::tool_response(
                    id,
                    result.0,
                    result.1,
                ));
            }

            self.trigger_llm_stream().await;
        }
    }

    pub async fn render(&self, terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>) -> Result<()> {
        let mcp_count = self.mcp_manager.server_count().await;
        let skill_count = self.skills_manager.list().len();
        let token_count = self.session.estimate_tokens();

        // Calculate dynamic prompt box height (5 to 7 lines for generous typing area)
        let lines_count = self.input_text.lines().count().max(1) as u16;
        let input_box_height = (lines_count + 3).clamp(5, 7);

        terminal.draw(|f| {
            let size = f.area();

            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Length(2), // Header
                    ratatui::layout::Constraint::Min(5),    // Chat history
                    ratatui::layout::Constraint::Length(input_box_height), // Prompt box (expanded height)
                    ratatui::layout::Constraint::Length(1), // Status bar
                ])
                .split(size);

            // 1. Header
            let header = Header {
                theme: &self.theme,
                model_name: &self.session.active_model,
                mcp_count,
                skill_count,
            };
            f.render_widget(header, chunks[0]);

            // 2. Chat history & interactive welcome screen
            let chat = ChatView {
                messages: &self.session.messages,
                theme: &self.theme,
                is_streaming: self.is_streaming,
                streaming_buffer: &self.streaming_buffer,
                spinner_tick: self.spinner_tick,
                scroll_offset: self.scroll_offset,
                welcome_option_index: self.welcome_option_index,
            };
            f.render_widget(chat, chunks[1]);

            // 3. Input box (bigger height)
            let input_box = InputBox {
                text: &self.input_text,
                theme: &self.theme,
                is_focused: self.active_modal.is_none(),
                suggestion_index: self.suggestion_index,
            };
            f.render_widget(input_box, chunks[2]);

            // 4. Status bar
            let status_bar = StatusBar {
                theme: &self.theme,
                mode: self.mode(),
                token_count,
                tool_confirm: self.config.general.require_tool_confirmation,
            };
            f.render_widget(status_bar, chunks[3]);

            // 5. Active Modal (if any)
            if let Some(modal) = &self.active_modal {
                match modal {
                    ActiveModal::ModelPicker(state) => {
                        let widget = ModelPickerModal {
                            state,
                            theme: &self.theme,
                            active_model_id: &self.session.active_model,
                        };
                        f.render_widget(widget, size);
                    }
                    ActiveModal::ThemePicker(state) => {
                        let widget = ThemePickerModal {
                            state,
                            current_theme: &self.theme,
                        };
                        f.render_widget(widget, size);
                    }
                    ActiveModal::Help => {
                        let widget = HelpModal { theme: &self.theme };
                        f.render_widget(widget, size);
                    }
                    ActiveModal::Skills(state) => {
                        let widget = SkillsModal {
                            state,
                            theme: &self.theme,
                        };
                        f.render_widget(widget, size);
                    }
                    ActiveModal::Mcp(state) => {
                        let widget = McpModal {
                            state,
                            theme: &self.theme,
                        };
                        f.render_widget(widget, size);
                    }
                    ActiveModal::Confirm(action) => {
                        let widget = ConfirmModal {
                            action,
                            theme: &self.theme,
                        };
                        f.render_widget(widget, size);
                    }
                }
            }
        })?;

        Ok(())
    }
}
