use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

use crate::theme::Theme;

pub enum AppMode {
    Insert,
    Modal,
    Streaming,
}

pub struct StatusBar<'a> {
    pub theme: &'a Theme,
    pub mode: AppMode,
    pub token_count: u32,
    pub tool_confirm: bool,
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::TOP)
            .border_style(self.theme.border_style(false))
            .style(Style::default().bg(self.theme.status_bar_bg));

        let (mode_text, mode_bg, mode_fg) = match self.mode {
            AppMode::Insert => (" INSERT ", self.theme.success, self.theme.bg),
            AppMode::Modal => (" MODAL ", self.theme.secondary, self.theme.bg),
            AppMode::Streaming => (" THINKING ", self.theme.warning, self.theme.bg),
        };

        let confirm_text = if self.tool_confirm {
            " 🛡️ Confirm: ON "
        } else {
            " 🛡️ Confirm: OFF "
        };

        let spans = vec![
            Span::styled(
                mode_text,
                Style::default().bg(mode_bg).fg(mode_fg).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                format!(" 🪙 Tokens: ~{} ", self.token_count),
                Style::default().fg(self.theme.status_bar_fg),
            ),
            Span::raw("│"),
            Span::styled(
                confirm_text,
                Style::default().fg(self.theme.muted),
            ),
            Span::raw("│ "),
            Span::styled("/help", Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled(" Help  ", Style::default().fg(self.theme.muted)),
            Span::styled("/model", Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled(" Model  ", Style::default().fg(self.theme.muted)),
            Span::styled("/theme", Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled(" Theme  ", Style::default().fg(self.theme.muted)),
            Span::styled("/skills", Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled(" Skills  ", Style::default().fg(self.theme.muted)),
            Span::styled("/mcp", Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
            Span::styled(" MCP  ", Style::default().fg(self.theme.muted)),
            Span::styled("/exit", Style::default().fg(self.theme.error).add_modifier(Modifier::BOLD)),
            Span::styled(" Quit", Style::default().fg(self.theme.muted)),
        ];

        let p = Paragraph::new(Line::from(spans)).block(block);
        p.render(area, buf);
    }
}
