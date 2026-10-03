use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};

use crate::theme::Theme;
use crate::ui::modals::centered_rect;

pub enum ConfirmAction {
    ToolExecution {
        tool_name: String,
        arguments: String,
    },
    ExitApp,
}

pub struct ConfirmModal<'a> {
    pub action: &'a ConfirmAction,
    pub theme: &'a Theme,
}

impl<'a> Widget for ConfirmModal<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (width, height) = match self.action {
            ConfirmAction::ToolExecution { .. } => (70, 45),
            ConfirmAction::ExitApp => (50, 25),
        };

        let modal_area = centered_rect(width, height, area);
        Clear.render(modal_area, buf);

        let (title, border_color) = match self.action {
            ConfirmAction::ToolExecution { .. } => (" ⚠️ Tool Execution Confirmation ", self.theme.warning),
            ConfirmAction::ExitApp => (" 🚪 Exit Confirmation ", self.theme.primary),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_style(Style::default().fg(border_color).add_modifier(Modifier::BOLD))
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(self.theme.bg));

        let inner = block.inner(modal_area);
        block.render(modal_area, buf);

        match self.action {
            ConfirmAction::ToolExecution { tool_name, arguments } => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(2), // Prompt
                        Constraint::Min(4),    // Arguments box
                        Constraint::Length(3), // Actions
                    ])
                    .split(inner);

                let prompt = Paragraph::new(vec![
                    Line::from(vec![
                        Span::styled("Agent requested to execute tool: ", Style::default().fg(self.theme.fg)),
                        Span::styled(tool_name, Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
                    ]),
                ]);
                prompt.render(chunks[0], buf);

                let args_block = Block::default()
                    .borders(Borders::ALL)
                    .title(" Arguments ")
                    .border_style(Style::default().fg(self.theme.border));

                let args_p = Paragraph::new(arguments.as_str())
                    .style(Style::default().fg(self.theme.accent).bg(self.theme.code_bg))
                    .block(args_block);
                args_p.render(chunks[1], buf);

                let actions = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("  [Y] ", Style::default().fg(self.theme.success).add_modifier(Modifier::BOLD)),
                        Span::styled("Allow once   ", Style::default().fg(self.theme.fg)),
                        Span::styled("[N] ", Style::default().fg(self.theme.error).add_modifier(Modifier::BOLD)),
                        Span::styled("Deny   ", Style::default().fg(self.theme.fg)),
                        Span::styled("[A] ", Style::default().fg(self.theme.secondary).add_modifier(Modifier::BOLD)),
                        Span::styled("Always allow this session", Style::default().fg(self.theme.fg)),
                    ]),
                ]);
                actions.render(chunks[2], buf);
            }
            ConfirmAction::ExitApp => {
                let p = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("  Are you sure you want to exit? ", Style::default().fg(self.theme.fg).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("    [Y] ", Style::default().fg(self.theme.error).add_modifier(Modifier::BOLD)),
                        Span::styled("Yes, exit    ", Style::default().fg(self.theme.fg)),
                        Span::styled("[N / Esc] ", Style::default().fg(self.theme.success).add_modifier(Modifier::BOLD)),
                        Span::styled("Cancel and stay", Style::default().fg(self.theme.fg)),
                    ]),
                ]);
                p.render(inner, buf);
            }
        }
    }
}
