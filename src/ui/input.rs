use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget};

use crate::theme::Theme;

pub struct SlashCommand {
    pub name: &'static str,
    pub description: &'static str,
}

pub const SLASH_COMMANDS: &[SlashCommand] = &[
    SlashCommand {
        name: "/model",
        description: "Choose AI model (Claude, GPT-4o, Gemini, Ollama)",
    },
    SlashCommand {
        name: "/theme",
        description: "Switch color theme (15 curated dark & light palettes)",
    },
    SlashCommand {
        name: "/skills",
        description: "Inspect and trigger local Agent skills",
    },
    SlashCommand {
        name: "/mcp",
        description: "Manage connected Model Context Protocol servers",
    },
    SlashCommand {
        name: "/clear",
        description: "Clear conversation history",
    },
    SlashCommand {
        name: "/help",
        description: "Show help reference and shortcuts cheatsheet",
    },
    SlashCommand {
        name: "/exit",
        description: "Exit the CLI agent gracefully",
    },
];

pub struct InputBox<'a> {
    pub text: &'a str,
    pub theme: &'a Theme,
    pub is_focused: bool,
    pub suggestion_index: usize,
}

impl<'a> Widget for InputBox<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" 💬 Prompt (Enter to send, Shift+Enter for newline, Tab to complete) ")
            .title_style(self.theme.title_style())
            .border_style(self.theme.border_style(self.is_focused))
            .style(Style::default().bg(self.theme.bg));

        let inner = block.inner(area);
        block.render(area, buf);

        let line_count = if self.text.is_empty() {
            1
        } else {
            self.text.lines().count().max(1) as u16
        };

        // Calculate vertical padding to center the text inside inner area
        let pad_top = (inner.height.saturating_sub(line_count)) / 2;

        // Render text or placeholder
        if self.text.is_empty() {
            let mut lines = Vec::new();
            for _ in 0..pad_top {
                lines.push(Line::from(""));
            }
            lines.push(Line::from(vec![
                Span::raw(" "),
                Span::styled(
                    "Type a message, press ",
                    Style::default().fg(self.theme.muted),
                ),
                Span::styled(
                    "↑ / ↓",
                    Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " to navigate options, or ",
                    Style::default().fg(self.theme.muted),
                ),
                Span::styled(
                    "'/'",
                    Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " for slash commands...",
                    Style::default().fg(self.theme.muted),
                ),
            ]));
            let placeholder = Paragraph::new(lines);
            placeholder.render(inner, buf);
        } else {
            let mut lines = Vec::new();
            for _ in 0..pad_top {
                lines.push(Line::from(""));
            }
            for line in self.text.lines() {
                lines.push(Line::from(vec![
                    Span::raw(" "),
                    Span::styled(line, Style::default().fg(self.theme.fg)),
                ]));
            }
            if lines.is_empty() || lines.len() == pad_top as usize {
                lines.push(Line::from(vec![
                    Span::raw(" "),
                    Span::styled("█", Style::default().fg(self.theme.primary)),
                ]));
            } else if let Some(last) = lines.last_mut() {
                last.spans.push(Span::styled("█", Style::default().fg(self.theme.primary)));
            }

            let paragraph = Paragraph::new(lines);
            paragraph.render(inner, buf);
        }

        // Render slash suggestion popup above input box if text starts with '/'
        if self.text.starts_with('/') && !self.text.contains(' ') {
            let query = self.text.to_lowercase();
            let matches: Vec<&SlashCommand> = SLASH_COMMANDS
                .iter()
                .filter(|c| c.name.to_lowercase().starts_with(&query))
                .collect();

            if !matches.is_empty() {
                let count = matches.len() as u16;
                let popup_height = (count + 2).min(9);
                let popup_area = Rect {
                    x: area.x + 2,
                    y: area.y.saturating_sub(popup_height),
                    width: area.width.saturating_sub(4).min(72),
                    height: popup_height,
                };

                Clear.render(popup_area, buf);

                let popup_block = Block::default()
                    .borders(Borders::ALL)
                    .title(" ⚡ Commands (↑/↓ to navigate, Tab/Enter to choose) ")
                    .border_style(Style::default().fg(self.theme.primary))
                    .style(Style::default().bg(self.theme.bg));

                let items: Vec<ListItem> = matches
                    .iter()
                    .enumerate()
                    .map(|(idx, cmd)| {
                        let is_active = idx == self.suggestion_index % matches.len();
                        let prefix = if is_active { " ▶ " } else { "   " };
                        let line = Line::from(vec![
                            Span::styled(
                                prefix,
                                Style::default()
                                    .fg(self.theme.primary)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                format!("{:<10}", cmd.name),
                                Style::default()
                                    .fg(if is_active {
                                        self.theme.selected_fg
                                    } else {
                                        self.theme.accent
                                    })
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                format!(" {}", cmd.description),
                                Style::default().fg(self.theme.muted),
                            ),
                        ]);

                        let style = if is_active {
                            Style::default().bg(self.theme.selected_bg)
                        } else {
                            Style::default().bg(self.theme.bg)
                        };

                        ListItem::new(line).style(style)
                    })
                    .collect();

                let list = List::new(items).block(popup_block);
                list.render(popup_area, buf);
            }
        }
    }
}
