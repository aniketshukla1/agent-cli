use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

use crate::theme::Theme;

pub struct Header<'a> {
    pub theme: &'a Theme,
    pub model_name: &'a str,
    pub mcp_count: usize,
    pub skill_count: usize,
}

impl<'a> Widget for Header<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::BOTTOM)
            .border_style(self.theme.border_style(false))
            .style(Style::default().bg(self.theme.status_bar_bg));

        let spans = vec![
            Span::styled(
                " ⚡ AGENT CLI ",
                Style::default()
                    .bg(self.theme.primary)
                    .fg(self.theme.bg)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled("Model: ", Style::default().fg(self.theme.muted)),
            Span::styled(
                format!("{} ", self.model_name),
                Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" │ "),
            Span::styled("Theme: ", Style::default().fg(self.theme.muted)),
            Span::styled(
                format!("{} ", self.theme.kind.name()),
                Style::default().fg(self.theme.secondary),
            ),
            Span::raw(" │ "),
            Span::styled(
                format!("🔌 {} MCPs ", self.mcp_count),
                Style::default().fg(self.theme.success),
            ),
            Span::raw(" │ "),
            Span::styled(
                format!("🧩 {} Skills ", self.skill_count),
                Style::default().fg(self.theme.warning),
            ),
        ];

        let paragraph = Paragraph::new(Line::from(spans)).block(block);
        paragraph.render(area, buf);
    }
}
