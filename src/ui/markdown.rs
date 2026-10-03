use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::theme::Theme;

pub fn render_markdown<'a>(text: &'a str, theme: &'a Theme) -> Vec<Line<'a>> {
    let mut lines = Vec::new();
    let mut in_code_block = false;

    for raw_line in text.lines() {
        let trimmed = raw_line.trim();

        // Code block boundary
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            let lang = trimmed.trim_start_matches("```").trim();
            let label = if in_code_block {
                if lang.is_empty() {
                    "─── Code ───".to_string()
                } else {
                    format!("─── Code ({}) ───", lang)
                }
            } else {
                "────────────".to_string()
            };
            lines.push(Line::from(vec![
                Span::styled(
                    label,
                    Style::default().fg(theme.muted).add_modifier(Modifier::DIM),
                ),
            ]));
            continue;
        }

        if in_code_block {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {}", raw_line),
                    Style::default().fg(theme.accent).bg(theme.code_bg),
                ),
            ]));
            continue;
        }

        // Markdown Headers
        if trimmed.starts_with("# ") {
            let title = trimmed[2..].trim();
            lines.push(Line::from(vec![
                Span::styled(
                    format!("█ {}", title),
                    Style::default()
                        .fg(theme.primary)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                ),
            ]));
            continue;
        } else if trimmed.starts_with("## ") {
            let title = trimmed[3..].trim();
            lines.push(Line::from(vec![
                Span::styled(
                    format!("▶ {}", title),
                    Style::default()
                        .fg(theme.secondary)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            continue;
        } else if trimmed.starts_with("### ") {
            let title = trimmed[4..].trim();
            lines.push(Line::from(vec![
                Span::styled(
                    format!("• {}", title),
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            continue;
        }

        // Alert blocks: > [!NOTE], > [!TIP], > [!WARNING], > [!IMPORTANT]
        if trimmed.starts_with("> [!") {
            let style = if trimmed.contains("NOTE") || trimmed.contains("TIP") {
                Style::default().fg(theme.success).add_modifier(Modifier::BOLD)
            } else if trimmed.contains("WARNING") {
                Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.error).add_modifier(Modifier::BOLD)
            };
            lines.push(Line::from(vec![Span::styled(format!("  ▎ {}", trimmed), style)]));
            continue;
        }

        // Bullet points
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            let content = &trimmed[2..];
            let mut spans = vec![
                Span::styled("  • ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
            ];
            spans.extend(parse_inlines(content, theme));
            lines.push(Line::from(spans));
            continue;
        }

        // Numbered list
        if let Some(pos) = trimmed.find(". ") {
            if let Ok(_) = trimmed[..pos].parse::<u32>() {
                let num_str = &trimmed[..=pos];
                let content = &trimmed[pos + 2..];
                let mut spans = vec![
                    Span::styled(
                        format!("  {} ", num_str),
                        Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
                    ),
                ];
                spans.extend(parse_inlines(content, theme));
                lines.push(Line::from(spans));
                continue;
            }
        }

        // Regular line with inlines
        lines.push(Line::from(parse_inlines(raw_line, theme)));
    }

    if lines.is_empty() {
        lines.push(Line::from(""));
    }

    lines
}

fn parse_inlines<'a>(text: &'a str, theme: &'a Theme) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    let mut cursor = 0;
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    while cursor < len {
        // Inline code `...`
        if chars[cursor] == '`' {
            if let Some(end) = text[cursor + 1..].find('`') {
                let code_content = &text[cursor + 1..cursor + 1 + end];
                spans.push(Span::styled(
                    format!(" {} ", code_content),
                    Style::default().fg(theme.accent).bg(theme.code_bg),
                ));
                cursor += end + 2;
                continue;
            }
        }

        // Bold **...**
        if cursor + 1 < len && chars[cursor] == '*' && chars[cursor + 1] == '*' {
            if let Some(end) = text[cursor + 2..].find("**") {
                let bold_content = &text[cursor + 2..cursor + 2 + end];
                spans.push(Span::styled(
                    bold_content,
                    Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
                ));
                cursor += end + 4;
                continue;
            }
        }

        // Normal segment
        let next_spec = text[cursor..]
            .find(|c| c == '`' || c == '*')
            .unwrap_or(text.len() - cursor);

        if next_spec > 0 {
            let seg = &text[cursor..cursor + next_spec];
            spans.push(Span::styled(seg, Style::default().fg(theme.fg)));
            cursor += next_spec;
        } else {
            // Step 1 if nothing matched
            let ch = chars[cursor];
            spans.push(Span::styled(ch.to_string(), Style::default().fg(theme.fg)));
            cursor += 1;
        }
    }

    spans
}
