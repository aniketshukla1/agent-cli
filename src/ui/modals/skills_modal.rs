use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget};

use crate::skills::Skill;
use crate::theme::Theme;
use crate::ui::modals::centered_rect;

pub struct SkillsModalState {
    pub selected_index: usize,
    pub skills: Vec<Skill>,
}

impl SkillsModalState {
    pub fn new(skills: Vec<Skill>) -> Self {
        Self {
            selected_index: 0,
            skills,
        }
    }

    pub fn next(&mut self) {
        if !self.skills.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.skills.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.skills.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.skills.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn selected_skill(&self) -> Option<&Skill> {
        self.skills.get(self.selected_index)
    }
}

pub struct SkillsModal<'a> {
    pub state: &'a SkillsModalState,
    pub theme: &'a Theme,
}

impl<'a> Widget for SkillsModal<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let modal_area = centered_rect(80, 80, area);
        Clear.render(modal_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" 🧩 Discovered Agent Skills (↑/↓ to select, Enter to use, Esc to close) ")
            .title_style(self.theme.title_style())
            .border_style(self.theme.border_style(true))
            .style(Style::default().bg(self.theme.bg));

        let inner = block.inner(modal_area);
        block.render(modal_area, buf);

        if self.state.skills.is_empty() {
            let empty_msg = vec![
                Line::from(""),
                Line::from(vec![
                    Span::styled("  No skills found in workspace or config.", Style::default().fg(self.theme.muted)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  To create a skill, add a folder in ", Style::default().fg(self.theme.fg)),
                    Span::styled(".agent/skills/<skill-name>/SKILL.md", Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("  Example format:", Style::default().fg(self.theme.primary)),
                ]),
                Line::from(vec![
                    Span::styled("    ---\n    name: my-skill\n    description: Brief summary\n    ---\n    # Instructions here...", Style::default().fg(self.theme.muted)),
                ]),
            ];
            Paragraph::new(empty_msg).render(inner, buf);
            return;
        }

        let split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(inner);

        // Left pane: Skill list
        let items: Vec<ListItem> = self
            .state
            .skills
            .iter()
            .enumerate()
            .map(|(idx, s)| {
                let is_selected = idx == self.state.selected_index;
                let prefix = if is_selected { " ▶ " } else { "   " };

                let line1 = Line::from(vec![
                    Span::styled(prefix, Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        &s.metadata.name,
                        Style::default()
                            .fg(if is_selected { self.theme.selected_fg } else { self.theme.fg })
                            .add_modifier(Modifier::BOLD),
                    ),
                ]);

                let line2 = Line::from(vec![
                    Span::raw("     "),
                    Span::styled(
                        &s.metadata.description,
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

        let list_widget = List::new(items).block(
            Block::default()
                .borders(Borders::RIGHT)
                .border_style(Style::default().fg(self.theme.border)),
        );
        list_widget.render(split[0], buf);

        // Right pane: Selected skill details & instructions preview
        if let Some(skill) = self.state.selected_skill() {
            let mut preview_lines = vec![
                Line::from(vec![
                    Span::styled("Skill: ", Style::default().fg(self.theme.muted)),
                    Span::styled(&skill.metadata.name, Style::default().fg(self.theme.primary).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("Path:  ", Style::default().fg(self.theme.muted)),
                    Span::styled(skill.dir_path.display().to_string(), Style::default().fg(self.theme.accent)),
                ]),
                Line::from(vec![
                    Span::styled("Scripts: ", Style::default().fg(self.theme.muted)),
                    Span::styled(format!("{} executable scripts found", skill.scripts.len()), Style::default().fg(self.theme.secondary)),
                ]),
                Line::from("──────────────────────────────────────────"),
                Line::from(vec![
                    Span::styled("Instructions Preview:", Style::default().fg(self.theme.fg).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
            ];

            for line in skill.instructions.lines().take(20) {
                preview_lines.push(Line::from(vec![
                    Span::styled(format!("  {}", line), Style::default().fg(self.theme.fg)),
                ]));
            }

            let preview_p = Paragraph::new(preview_lines).block(Block::default());
            preview_p.render(split[1], buf);
        }
    }
}
