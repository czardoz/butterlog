use ratatui::prelude::*;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::{RowKind, VisibleRow};

pub fn render_rows(rows: &[VisibleRow], frame: &mut Frame<'_>) {
    let mut lines = Vec::new();

    for row in rows {
        let indent = "  ".repeat(row.depth);
        let text = match row.kind {
            RowKind::Partition => {
                let arrow = if row.expanded { "v" } else { ">" };
                let mut label = row.text.clone();
                if !label.ends_with("...") {
                    label.push_str("...");
                }
                format!("{indent}{arrow} {}", label)
            }
            RowKind::Line => format!("{indent}- {}", row.text),
        };
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
