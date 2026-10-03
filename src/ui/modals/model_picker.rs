use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget};

use crate::config::ModelOption;
use crate::theme::Theme;
use crate::ui::modals::centered_rect;

pub struct ModelPickerState {
    pub search_query: String,
    pub selected_index: usize,
    pub models: Vec<ModelOption>,
}

impl ModelPickerState {
    pub fn new(models: Vec<ModelOption>, current_model: &str) -> Self {
        let initial_idx = models
            .iter()
            .position(|m| m.id == current_model)
            .unwrap_or(0);
        Self {
            search_query: String::new(),
            selected_index: initial_idx,
            models,
        }
    }

    pub fn filtered_models(&self) -> Vec<&ModelOption> {
        if self.search_query.trim().is_empty() {
            self.models.iter().collect()
        } else {
            let q = self.search_query.to_lowercase();
            self.models
                .iter()
                .filter(|m| {
                    m.name.to_lowercase().contains(&q)
                        || m.id.to_lowercase().contains(&q)
                        || m.provider.to_lowercase().contains(&q)
                })
                .collect()
        }
    }

    pub fn next(&mut self) {
        let count = self.filtered_models().len();
        if count > 0 {
            self.selected_index = (self.selected_index + 1) % count;
        }
    }

    pub fn previous(&mut self) {
        let count = self.filtered_models().len();
        if count > 0 {
            if self.selected_index == 0 {
                self.selected_index = count - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn selected_model(&self) -> Option<ModelOption> {
        let filtered = self.filtered_models();
        filtered.get(self.selected_index).cloned().cloned()
    }
}

pub struct ModelPickerModal<'a> {
    pub state: &'a ModelPickerState,
    pub theme: &'a Theme,
    pub active_model_id: &'a str,
}

impl<'a> Widget for ModelPickerModal<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let modal_area = centered_rect(75, 75, area);
        Clear.render(modal_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" 🤖 Select AI Model (Type to filter, ↑/↓ to navigate, Enter to select, Esc to close) ")
            .title_style(self.theme.title_style())
            .border_style(self.theme.border_style(true))
            .style(Style::default().bg(self.theme.bg));

        let inner = block.inner(modal_area);
        block.render(modal_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Search bar
                Constraint::Min(5),    // List of models
                Constraint::Length(1), // Footer info
            ])
            .split(inner);

        // Search bar
        let search_block = Block::default()
            .borders(Borders::ALL)
            .title(" 🔍 Filter Models ")
            .title_style(Style::default().fg(self.theme.secondary))
            .border_style(Style::default().fg(self.theme.border));

        let search_p = Paragraph::new(format!("  {}█", self.state.search_query))
            .style(Style::default().fg(self.theme.fg))
            .block(search_block);
        search_p.render(chunks[0], buf);

        // Model items list
        let filtered = self.state.filtered_models();
        let items: Vec<ListItem> = filtered
            .iter()
            .enumerate()
            .map(|(idx, m)| {
                let is_selected = idx == self.state.selected_index;
                let is_active = m.id == self.active_model_id;

                let active_badge = if is_active { " [ACTIVE] " } else { "          " };

                let prefix = if is_selected { " ▶ " } else { "   " };

                let line1 = Line::from(vec![
                    Span::styled(prefix, Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!("{:<22}", m.name),
                        Style::default()
                            .fg(if is_selected { self.theme.selected_fg } else { self.theme.fg })
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(" [{:<9}] ", m.provider),
                        Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("Ctx: {:<4}k  ", m.context_length / 1000),
                        Style::default().fg(self.theme.muted),
                    ),
                    Span::styled(
                        active_badge,
                        Style::default()
                            .fg(self.theme.success)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]);

                let line2 = Line::from(vec![
                    Span::raw("      "),
                    Span::styled(
                        &m.description,
                        Style::default().fg(self.theme.muted),
                    ),
                ]);

                let item_style = if is_selected {
                    Style::default().bg(self.theme.selected_bg)
                } else {
                    Style::default().bg(self.theme.bg)
                };

                ListItem::new(vec![line1, line2, Line::from("")]).style(item_style)
            })
            .collect();

        let list_widget = List::new(items);
        list_widget.render(chunks[1], buf);

        // Footer
        let footer = Paragraph::new(format!(
            " Total: {} models available | Providers: Anthropic, OpenAI, Gemini, Ollama, OpenRouter",
            self.state.models.len()
        ))
        .style(Style::default().fg(self.theme.muted));
        footer.render(chunks[2], buf);
    }
}
