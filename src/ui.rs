use ratatui::prelude::*;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::{RowKind, VisibleRow};

fn row_display_text(row: &VisibleRow) -> String {
    let indent = "  ".repeat(row.depth);
    match row.kind {
        RowKind::Partition => {
            let arrow = if row.expanded { "v" } else { ">" };
            let mut label = row.text.clone();
            if !label.ends_with("...") {
                label.push_str("...");
            }
            format!("{indent}{arrow} {}", label)
        }
        RowKind::Line => format!("{indent}- {}", row.text),
    }
}

pub fn max_row_width(rows: &[VisibleRow]) -> usize {
    rows.iter()
        .map(|row| row_display_text(row).chars().count())
        .max()
        .unwrap_or(0)
}

pub fn render_rows(rows: &[VisibleRow], frame: &mut Frame<'_>, horizontal_offset: u16) {
    let mut lines = Vec::new();

    for row in rows {
        let text = row_display_text(row);
        let mut style = Style::default();
        let is_match = row.matches_self || row.matches_descendants;
        if is_match {
            style = style.add_modifier(Modifier::BOLD).bg(Color::Blue);
        }
        if row.is_selected {
            style = style.bg(if is_match { Color::Magenta } else { Color::DarkGray });
        }
        lines.push(Line::from(Span::styled(text, style)));
    }

    let paragraph = Paragraph::new(lines).scroll((0, horizontal_offset));
    frame.render_widget(paragraph, frame.size());
}
