use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

use crate::session::{ChatMessage, Role};
use crate::theme::Theme;
use crate::ui::markdown::render_markdown;

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub struct WelcomeOption {
    pub cmd: &'static str,
    pub icon: &'static str,
    pub label: &'static str,
    pub desc: &'static str,
}

pub const WELCOME_OPTIONS: &[WelcomeOption] = &[
    WelcomeOption {
        cmd: "/model",
        icon: "🤖",
        label: "/model",
        desc: "Choose AI model (Claude 3.7, GPT-4o, Gemini 2.5, Ollama...)",
    },
    WelcomeOption {
        cmd: "/theme",
        icon: "🎨",
        label: "/theme",
        desc: "Switch color theme (25 curated white/silver dark & light palettes)",
    },
    WelcomeOption {
        cmd: "/skills",
        icon: "🧩",
        label: "/skills",
        desc: "Inspect and trigger local Agent skills (.agent/skills)",
    },
    WelcomeOption {
        cmd: "/mcp",
        icon: "🔌",
        label: "/mcp",
        desc: "Manage connected Model Context Protocol servers",
    },
    WelcomeOption {
        cmd: "/help",
        icon: "📖",
        label: "/help",
        desc: "View full cheatsheet & keyboard shortcuts",
    },
    WelcomeOption {
        cmd: "/exit",
        icon: "🚪",
        label: "/exit",
        desc: "Exit the CLI agent gracefully",
    },
];

pub struct ChatView<'a> {
    pub messages: &'a [ChatMessage],
    pub theme: &'a Theme,
    pub is_streaming: bool,
    pub streaming_buffer: &'a str,
    pub spinner_tick: usize,
    pub scroll_offset: u16,
    pub welcome_option_index: usize,
}

impl<'a> Widget for ChatView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::NONE)
            .style(Style::default().bg(self.theme.bg));

        let inner = block.inner(area);
        block.render(area, buf);

        if self.messages.is_empty() && !self.is_streaming {
            let mut welcome = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        "  🚀 AGENT CLI — Next-Gen AI Terminal Agent",
                        Style::default()
                            .fg(self.theme.primary)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled(
                        "  Navigate options below with ",
                        Style::default().fg(self.theme.fg),
                    ),
                    Span::styled(
                        "↑ / ↓",
                        Style::default()
                            .fg(self.theme.accent)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(" and press ", Style::default().fg(self.theme.fg)),
                    Span::styled(
                        "Enter",
                        Style::default()
                            .fg(self.theme.secondary)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        " to launch, or type a prompt directly in the box below:",
                        Style::default().fg(self.theme.fg),
                    ),
                ]),
                Line::from(""),
            ];

            for (idx, opt) in WELCOME_OPTIONS.iter().enumerate() {
                let is_selected = idx == self.welcome_option_index % WELCOME_OPTIONS.len();
                let prefix = if is_selected { "  ▶ " } else { "    " };

                let cmd_style = if is_selected {
                    Style::default()
                        .fg(self.theme.selected_fg)
                        .bg(self.theme.selected_bg)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(self.theme.primary)
                        .add_modifier(Modifier::BOLD)
                };

                let desc_style = if is_selected {
                    Style::default()
                        .fg(self.theme.fg)
                        .bg(self.theme.selected_bg)
                } else {
                    Style::default().fg(self.theme.muted)
                };

                let enter_hint = if is_selected {
                    Span::styled(
                        "  [Enter to open] ",
                        Style::default()
                            .fg(self.theme.success)
                            .bg(self.theme.selected_bg)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw("")
                };

                welcome.push(Line::from(vec![
                    Span::styled(
                        prefix,
                        Style::default()
                            .fg(self.theme.primary)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(format!("{} ", opt.icon), Style::default()),
                    Span::styled(format!("{:<9} ", opt.label), cmd_style),
                    Span::styled(format!("─ {:<52}", opt.desc), desc_style),
                    enter_hint,
                ]));
            }

            welcome.push(Line::from(""));
            welcome.push(Line::from(vec![
                Span::styled(
                    "  💡 Tip: You can also press Ctrl+P for models, Ctrl+T for themes, or type '/' for quick commands.",
                    Style::default().fg(self.theme.muted),
                ),
            ]));

            let p = Paragraph::new(welcome);
            p.render(inner, buf);
            return;
        }

        let mut lines: Vec<Line> = Vec::new();

        for msg in self.messages {
            match msg.role {
                Role::User => {
                    lines.push(Line::from(vec![
                        Span::styled(
                            " 👤 You ",
                            Style::default()
                                .bg(self.theme.primary)
                                .fg(self.theme.bg)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("  {}", msg.timestamp.format("%H:%M:%S")),
                            Style::default().fg(self.theme.muted),
                        ),
                    ]));
                    lines.push(Line::from(""));
                    for line in msg.content.lines() {
                        lines.push(Line::from(vec![
                            Span::raw("  "),
                            Span::styled(line, Style::default().fg(self.theme.fg)),
                        ]));
                    }
                    lines.push(Line::from(""));
                }
                Role::Assistant => {
                    lines.push(Line::from(vec![
                        Span::styled(
                            " 🤖 Agent ",
                            Style::default()
                                .bg(self.theme.secondary)
                                .fg(self.theme.bg)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("  {}", msg.timestamp.format("%H:%M:%S")),
                            Style::default().fg(self.theme.muted),
                        ),
                    ]));
                    lines.push(Line::from(""));

                    let md_lines = render_markdown(&msg.content, self.theme);
                    lines.extend(md_lines);

                    // If assistant made tool calls
                    for tc in &msg.tool_calls {
                        lines.push(Line::from(""));
                        lines.push(Line::from(vec![
                            Span::styled(
                                "  ⚡ Call Tool: ",
                                Style::default()
                                    .fg(self.theme.warning)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                &tc.name,
                                Style::default()
                                    .fg(self.theme.accent)
                                    .add_modifier(Modifier::BOLD),
                            ),
                        ]));
                        lines.push(Line::from(vec![
                            Span::raw("     Args: "),
                            Span::styled(&tc.arguments, Style::default().fg(self.theme.muted)),
                        ]));
                    }

                    lines.push(Line::from(""));
                }
                Role::Tool => {
                    for tr in &msg.tool_results {
                        let (badge, badge_color) = if tr.is_error {
                            (" ✖ Tool Error ", self.theme.error)
                        } else {
                            (" ✔ Tool Result ", self.theme.success)
                        };

                        lines.push(Line::from(vec![Span::styled(
                            badge,
                            Style::default()
                                .bg(badge_color)
                                .fg(self.theme.bg)
                                .add_modifier(Modifier::BOLD),
                        )]));

                        for line in tr.content.lines().take(15) {
                            lines.push(Line::from(vec![Span::styled(
                                format!("    {}", line),
                                Style::default().fg(self.theme.muted),
                            )]));
                        }
                        lines.push(Line::from(""));
                    }
                }
                Role::System => {}
            }
        }

        // Active streaming message
        if self.is_streaming {
            let spinner = SPINNER_FRAMES[self.spinner_tick % SPINNER_FRAMES.len()];
            lines.push(Line::from(vec![Span::styled(
                format!(" 🤖 Agent {} Thinking... ", spinner),
                Style::default()
                    .bg(self.theme.warning)
                    .fg(self.theme.bg)
                    .add_modifier(Modifier::BOLD),
            )]));
            lines.push(Line::from(""));

            let md_lines = render_markdown(self.streaming_buffer, self.theme);
            lines.extend(md_lines);
            lines.push(Line::from(""));
        }

        let total_lines = lines.len() as u16;
        let view_height = inner.height;
        let max_scroll = if total_lines > view_height {
            total_lines - view_height
        } else {
            0
        };

        let current_scroll = if self.scroll_offset > 0 {
            if max_scroll >= self.scroll_offset {
                max_scroll - self.scroll_offset
            } else {
                0
            }
        } else {
            max_scroll
        };

        let p = Paragraph::new(lines).scroll((current_scroll, 0));
        p.render(inner, buf);
    }
}
