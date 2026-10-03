use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget};

use crate::theme::Theme;
use crate::ui::modals::centered_rect;

pub struct HelpModal<'a> {
    pub theme: &'a Theme,
}

impl<'a> Widget for HelpModal<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let modal_area = centered_rect(80, 80, area);
        Clear.render(modal_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" 📖 Help & Command Reference (Press Esc to close) ")
            .title_style(self.theme.title_style())
            .border_style(self.theme.border_style(true))
            .style(Style::default().bg(self.theme.bg));

        let inner = block.inner(modal_area);
        block.render(modal_area, buf);

        let lines = vec![
            Line::from(vec![
                Span::styled("✨ SLASH COMMANDS", Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("  /model           ", Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("Open interactive model picker (Anthropic, OpenAI, Gemini, Ollama)", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(vec![
                Span::styled("  /theme           ", Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("Switch color theme (Catppuccin, Tokyo Night, Nord, Gruvbox, Cyberpunk...)", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(vec![
                Span::styled("  /skills          ", Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("Inspect and trigger loaded Agent skills (.agent/skills/)", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(vec![
                Span::styled("  /mcp             ", Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("View Model Context Protocol servers and connected tools", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(vec![
                Span::styled("  /clear           ", Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("Clear current conversation history", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(vec![
                Span::styled("  /help            ", Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("Show this help reference modal", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(vec![
                Span::styled("  /exit, /quit     ", Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("Exit the CLI agent gracefully", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("⌨️ KEYBOARD SHORTCUTS", Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("  Enter            ", Style::default().fg(self.theme.secondary).add_modifier(Modifier::BOLD)),
                Span::styled("Send message / Confirm modal selection", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(vec![
                Span::styled("  Shift+Enter      ", Style::default().fg(self.theme.secondary).add_modifier(Modifier::BOLD)),
                Span::styled("Insert newline in input box", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(vec![
                Span::styled("  PageUp / PageDn  ", Style::default().fg(self.theme.secondary).add_modifier(Modifier::BOLD)),
                Span::styled("Scroll chat message history", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(vec![
                Span::styled("  Ctrl + P         ", Style::default().fg(self.theme.secondary).add_modifier(Modifier::BOLD)),
                Span::styled("Quick-open Model Picker", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(vec![
                Span::styled("  Ctrl + T         ", Style::default().fg(self.theme.secondary).add_modifier(Modifier::BOLD)),
                Span::styled("Quick-open Theme Picker", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(vec![
                Span::styled("  Ctrl + C         ", Style::default().fg(self.theme.secondary).add_modifier(Modifier::BOLD)),
                Span::styled("Cancel active streaming, or double-press to exit", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(vec![
                Span::styled("  Esc              ", Style::default().fg(self.theme.secondary).add_modifier(Modifier::BOLD)),
                Span::styled("Close modal / Clear input", Style::default().fg(self.theme.fg)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("⚙️ CONFIGURATION & API KEYS", Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("  Config file:     ", Style::default().fg(self.theme.muted)),
                Span::styled("~/.config/agent-cli/config.toml (or ./agent.toml)", Style::default().fg(self.theme.warning)),
            ]),
            Line::from(vec![
                Span::styled("  Environment:     ", Style::default().fg(self.theme.muted)),
                Span::styled("ANTHROPIC_API_KEY, OPENAI_API_KEY, GEMINI_API_KEY, OLLAMA_HOST", Style::default().fg(self.theme.fg)),
            ]),
        ];

        let paragraph = Paragraph::new(lines).style(Style::default().bg(self.theme.bg));
        paragraph.render(inner, buf);
    }
}
