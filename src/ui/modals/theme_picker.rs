use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget};

use crate::theme::{Theme, ThemeKind};
use crate::ui::modals::centered_rect;

pub struct ThemePickerState {
    pub selected_index: usize,
    pub themes: Vec<ThemeKind>,
}

impl ThemePickerState {
    pub fn new(current_theme: ThemeKind) -> Self {
        let themes = ThemeKind::ALL.to_vec();
        let selected_index = themes
            .iter()
            .position(|&t| t == current_theme)
            .unwrap_or(0);
        Self {
            selected_index,
            themes,
        }
    }

    pub fn next(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.themes.len();
    }

    pub fn previous(&mut self) {
        if self.selected_index == 0 {
            self.selected_index = self.themes.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn selected_theme(&self) -> ThemeKind {
        self.themes[self.selected_index]
    }
}

pub struct ThemePickerModal<'a> {
    pub state: &'a ThemePickerState,
    pub current_theme: &'a Theme,
}

impl<'a> Widget for ThemePickerModal<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let modal_area = centered_rect(82, 85, area);
        Clear.render(modal_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " 🎨 Select Theme ({}/{} - ↑/↓ to preview, Enter to apply, Esc to close) ",
                self.state.selected_index + 1,
                self.state.themes.len()
            ))
            .title_style(self.current_theme.title_style())
            .border_style(self.current_theme.border_style(true))
            .style(Style::default().bg(self.current_theme.bg));

        let inner = block.inner(modal_area);
        block.render(modal_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(6),    // List of themes
                Constraint::Length(2), // Footer tip
            ])
            .split(inner);

        // Calculate visible items window based on available height (each item takes 2 lines)
        let max_visible_items = (chunks[0].height / 2).max(1) as usize;
        let total = self.state.themes.len();

        let start_idx = if total <= max_visible_items {
            0
        } else if self.state.selected_index >= max_visible_items {
            self.state.selected_index + 1 - max_visible_items
        } else {
            0
        };

        let end_idx = (start_idx + max_visible_items).min(total);

        let items: Vec<ListItem> = self.state.themes[start_idx..end_idx]
            .iter()
            .enumerate()
            .map(|(rel_idx, &kind)| {
                let actual_idx = start_idx + rel_idx;
                let is_selected = actual_idx == self.state.selected_index;
                let is_active = kind == self.current_theme.kind;
                let sample_theme = Theme::from_kind(kind);

                let prefix = if is_selected { " ▶ " } else { "   " };
                let active_tag = if is_active { " [ACTIVE] " } else { "          " };

                let cat_badge = match kind.category() {
                    "DARK / SILVER-WHITE" => Span::styled(
                        " [SILVER/WHITE] ",
                        Style::default()
                            .fg(self.current_theme.fg)
                            .bg(self.current_theme.selected_bg)
                            .add_modifier(Modifier::BOLD),
                    ),
                    "DARK PALETTES" => Span::styled(
                        " [DARK] ",
                        Style::default()
                            .fg(self.current_theme.accent)
                            .add_modifier(Modifier::BOLD),
                    ),
                    _ => Span::styled(
                        " [LIGHT] ",
                        Style::default()
                            .fg(self.current_theme.warning)
                            .add_modifier(Modifier::BOLD),
                    ),
                };

                let line1 = Line::from(vec![
                    Span::styled(
                        prefix,
                        Style::default()
                            .fg(self.current_theme.primary)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{:<20}", kind.name()),
                        Style::default()
                            .fg(if is_selected {
                                self.current_theme.selected_fg
                            } else {
                                self.current_theme.fg
                            })
                            .add_modifier(Modifier::BOLD),
                    ),
                    cat_badge,
                    Span::raw(" "),
                    // Swatches
                    Span::styled(" █", Style::default().fg(sample_theme.bg)),
                    Span::styled("█", Style::default().fg(sample_theme.fg)),
                    Span::styled("█", Style::default().fg(sample_theme.primary)),
                    Span::styled("█", Style::default().fg(sample_theme.secondary)),
                    Span::styled("█", Style::default().fg(sample_theme.accent)),
                    Span::styled("█", Style::default().fg(sample_theme.success)),
                    Span::styled("█ ", Style::default().fg(sample_theme.warning)),
                    Span::styled(
                        active_tag,
                        Style::default()
                            .fg(self.current_theme.success)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]);

                let line2 = Line::from(vec![
                    Span::raw("      "),
                    Span::styled(
                        kind.description(),
                        Style::default().fg(self.current_theme.muted),
                    ),
                ]);

                let item_style = if is_selected {
                    Style::default().bg(self.current_theme.selected_bg)
                } else {
                    Style::default().bg(self.current_theme.bg)
                };

                ListItem::new(vec![line1, line2]).style(item_style)
            })
            .collect();

        let list_widget = List::new(items);
        list_widget.render(chunks[0], buf);

        let footer = Paragraph::new(Line::from(vec![
            Span::styled(" 💡 Categories: ", Style::default().fg(self.theme_accent(&self.current_theme))),
            Span::styled("Dark with White/Silver Text", Style::default().fg(self.current_theme.fg).add_modifier(Modifier::BOLD)),
            Span::styled(" │ ", Style::default().fg(self.current_theme.muted)),
            Span::styled("Iconic Dark Palettes", Style::default().fg(self.current_theme.primary)),
            Span::styled(" │ ", Style::default().fg(self.current_theme.muted)),
            Span::styled("Light Daytime Palettes", Style::default().fg(self.current_theme.warning)),
        ]))
        .style(Style::default().fg(self.current_theme.muted));
        footer.render(chunks[1], buf);
    }
}

impl<'a> ThemePickerModal<'a> {
    fn theme_accent(&self, theme: &Theme) -> ratatui::style::Color {
        theme.accent
    }
}
