use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget};

use crate::llm::types::ToolDefinition;
use crate::theme::Theme;
use crate::ui::modals::centered_rect;

pub struct McpModalState {
    pub selected_index: usize,
    pub server_statuses: Vec<(String, usize)>, // (server_name, tool_count)
    pub tools: Vec<ToolDefinition>,
}

impl McpModalState {
    pub fn new(server_statuses: Vec<(String, usize)>, tools: Vec<ToolDefinition>) -> Self {
        Self {
            selected_index: 0,
            server_statuses,
            tools,
        }
    }

    pub fn next(&mut self) {
        if !self.server_statuses.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.server_statuses.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.server_statuses.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.server_statuses.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }
}

pub struct McpModal<'a> {
    pub state: &'a McpModalState,
    pub theme: &'a Theme,
}

impl<'a> Widget for McpModal<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let modal_area = centered_rect(80, 80, area);
        Clear.render(modal_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" 🔌 Model Context Protocol (MCP) Dashboard (Esc to close) ")
            .title_style(self.theme.title_style())
            .border_style(self.theme.border_style(true))
            .style(Style::default().bg(self.theme.bg));

        let inner = block.inner(modal_area);
        block.render(modal_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Summary header
                Constraint::Min(6),    // Main content split
                Constraint::Length(4), // How to configure MCP
            ])
            .split(inner);

        // Header summary
        let summary_lines = vec![
            Line::from(vec![
                Span::styled("Active Servers: ", Style::default().fg(self.theme.muted)),
                Span::styled(
                    format!("{}", self.state.server_statuses.len()),
                    Style::default().fg(self.theme.success).add_modifier(Modifier::BOLD),
                ),
                Span::raw("    "),
                Span::styled("Discovered Tools: ", Style::default().fg(self.theme.muted)),
                Span::styled(
                    format!("{}", self.state.tools.len()),
                    Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from("────────────────────────────────────────────────────────────────────────"),
        ];
        Paragraph::new(summary_lines).render(chunks[0], buf);

        // Split: Servers on left, Tools on right
        let split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(chunks[1]);

        if self.state.server_statuses.is_empty() {
            let empty_servers = Paragraph::new(vec![
                Line::from(Span::styled("  No MCP servers configured yet.", Style::default().fg(self.theme.muted))),
            ]);
            empty_servers.render(split[0], buf);
        } else {
            let items: Vec<ListItem> = self
                .state
                .server_statuses
                .iter()
                .enumerate()
                .map(|(idx, (name, count))| {
                    let is_selected = idx == self.state.selected_index;
                    let prefix = if is_selected { " ▶ " } else { "   " };

                    let line = Line::from(vec![
                        Span::styled(prefix, Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
                        Span::styled(
                            name,
                            Style::default()
                                .fg(if is_selected { self.theme.selected_fg } else { self.theme.fg })
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!(" ({} tools)", count),
                            Style::default().fg(self.theme.accent),
                        ),
                    ]);

                    let style = if is_selected {
                        Style::default().bg(self.theme.selected_bg)
                    } else {
                        Style::default().bg(self.theme.bg)
                    };
                    ListItem::new(line).style(style)
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .borders(Borders::RIGHT)
                    .title(" Servers ")
                    .border_style(Style::default().fg(self.theme.border)),
            );
            list.render(split[0], buf);
        }

        // Tools list
        let tool_items: Vec<ListItem> = self
            .state
            .tools
            .iter()
            .map(|t| {
                let line1 = Line::from(vec![
                    Span::styled(" ⚡ ", Style::default().fg(self.theme.warning)),
                    Span::styled(&t.name, Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
                ]);
                let line2 = Line::from(vec![
                    Span::raw("    "),
                    Span::styled(&t.description, Style::default().fg(self.theme.muted)),
                ]);
                ListItem::new(vec![line1, line2, Line::from("")])
            })
            .collect();

        let tool_list = List::new(tool_items).block(
            Block::default()
                .borders(Borders::NONE)
                .title(" Available MCP Tools "),
        );
        tool_list.render(split[1], buf);

        // Footer instructions
        let footer_lines = vec![
            Line::from("────────────────────────────────────────────────────────────────────────"),
            Line::from(vec![
                Span::styled("💡 To add MCP servers, edit ", Style::default().fg(self.theme.muted)),
                Span::styled("~/.config/agent-cli/config.toml", Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("   [mcp_servers.my_server]\n   command = \"npx\"\n   args = [\"-y\", \"@modelcontextprotocol/server-filesystem\", \"/path\"]", Style::default().fg(self.theme.muted)),
            ]),
        ];
        Paragraph::new(footer_lines).render(chunks[2], buf);
    }
}
