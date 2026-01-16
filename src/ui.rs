use ratatui::prelude::*;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::VisibleRow;

pub fn render_rows(rows: &[VisibleRow], frame: &mut Frame<'_>) {
    let mut lines = Vec::new();

    for row in rows {
        let indent = "  ".repeat(row.depth);
        let arrow = if row.expanded { "v" } else { ">" };
        let text = format!("{indent}{arrow} {}", row.prefix);
        let style = if row.matches_self || row.matches_descendants {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        lines.push(Line::from(Span::styled(text, style)));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, frame.size());
}
