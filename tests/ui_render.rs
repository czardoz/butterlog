use butterlog::{render_rows, RowKind, RowPath, VisibleRow};
use ratatui::backend::TestBackend;
use ratatui::style::Modifier;
use ratatui::Terminal;

fn row_text(buffer: &ratatui::buffer::Buffer, y: u16, width: u16) -> String {
    (0..width)
        .map(|x| buffer.get(x, y).symbol())
        .collect::<String>()
}

#[test]
fn renders_arrows_and_prefixes_with_highlight() {
    let rows = vec![
        VisibleRow {
            kind: RowKind::Partition,
            path: RowPath(vec![0]),
            depth: 0,
            text: "ERR".to_string(),
            line_count: 2,
            expanded: false,
            matches_self: false,
            matches_descendants: false,
            line_index: None,
        },
        VisibleRow {
            kind: RowKind::Partition,
            path: RowPath(vec![1]),
            depth: 1,
            text: "INFO".to_string(),
            line_count: 1,
            expanded: true,
            matches_self: true,
            matches_descendants: false,
            line_index: None,
        },
    ];

    let backend = TestBackend::new(20, 5);
    let mut terminal = Terminal::new(backend).expect("terminal");

    terminal
        .draw(|frame| {
            render_rows(&rows, frame);
        })
        .expect("draw");

    let buffer = terminal.backend().buffer();
    let line0 = row_text(buffer, 0, 20);
    let line1 = row_text(buffer, 1, 20);

    assert!(line0.contains("> ERR..."));
    assert!(line1.contains("  v INFO..."));

    let cell = buffer.get(2, 1);
    assert!(cell.style().add_modifier.contains(Modifier::BOLD));
}
