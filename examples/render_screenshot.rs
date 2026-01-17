use std::fs;
use std::path::Path;

use butterlog::{
    apply_search, InputMode, LineStore, LoadStatus, Partition, SearchState, SearchTerm, render_rows,
};
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::style::{Color, Modifier};
use ratatui::Terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = Path::new("assets/screenshot.svg");
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    let line_store = LineStore::new(sample_lines());
    let mut partitions = sample_partitions();

    let term = SearchTerm::new("timeout");
    let rows = apply_search(Some(&term), &mut partitions, &line_store, 0);
    let selected = rows
        .iter()
        .position(|row| row.text.to_lowercase().contains("timeout"))
        .unwrap_or(0);
    let rows = apply_search(Some(&term), &mut partitions, &line_store, selected);

    let mut search = SearchState::new();
    search.mode = InputMode::Search;
    search.buffer = "timeout".to_string();
    search.term = Some(term.clone());

    let width = 90;
    let height = 16;
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        let load_status = LoadStatus::partial();
        render_rows(&rows, frame, 0, 0, &search, &load_status);
    })?;

    let buffer = terminal.backend().buffer();
    buffer_to_svg(buffer, output, 9, 18, 14)?;

    Ok(())
}

fn sample_lines() -> Vec<String> {
    vec![
        "2024-05-01 10:10:01 INFO Boot sequence started".to_string(),
        "2024-05-01 10:10:02 WARN Request timeout after 2000ms".to_string(),
        "2024-05-01 10:10:03 INFO Loaded module: network".to_string(),
        "2024-05-01 10:10:04 ERROR Timeout while connecting to db".to_string(),
        "2024-05-01 10:10:05 INFO Listening on 0.0.0.0:8080".to_string(),
        "2024-05-01 10:10:06 WARN Retry attempt 1".to_string(),
    ]
}

fn sample_partitions() -> Vec<Partition> {
    let mut info = Partition::new("INFO".to_string(), vec![0, 2, 4], 0, 4);
    info.expanded = true;
    info.children = vec![
        leaf_partition("INFO Boot", vec![0], 1),
        leaf_partition("INFO Loaded", vec![2], 1),
        leaf_partition("INFO Listening", vec![4], 1),
    ];

    let mut warn = Partition::new("WARN".to_string(), vec![1, 5], 0, 4);
    warn.expanded = true;
    warn.children = vec![
        leaf_partition("WARN Request", vec![1], 1),
        leaf_partition("WARN Retry", vec![5], 1),
    ];

    let mut error = Partition::new("ERROR".to_string(), vec![3], 0, 5);
    error.expanded = false;

    vec![info, warn, error]
}

fn leaf_partition(prefix: &str, line_indices: Vec<usize>, depth: usize) -> Partition {
    let prefix_len = prefix.chars().count();
    let mut partition = Partition::new(prefix.to_string(), line_indices, depth, prefix_len);
    partition.expanded = true;
    partition
}

fn buffer_to_svg(
    buffer: &Buffer,
    output: &Path,
    cell_width: u32,
    cell_height: u32,
    font_size: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let width = buffer.area.width as u32;
    let height = buffer.area.height as u32;
    let svg_width = width * cell_width;
    let svg_height = height * cell_height;
    let default_bg = "#0b0f14";
    let default_fg = "#e5e7eb";

    let mut svg = String::new();
    svg.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
        svg_width, svg_height, svg_width, svg_height
    ));
    svg.push_str(&format!(
        "  <rect width=\"100%\" height=\"100%\" fill=\"{}\"/>\n",
        default_bg
    ));
    svg.push_str(&format!(
        "  <style>text {{ font-family: monospace; font-size: {}px; dominant-baseline: hanging; }}</style>\n",
        font_size
    ));

    for y in 0..height {
        for x in 0..width {
            let cell = buffer.get(x as u16, y as u16);
            let symbol = cell.symbol();
            if let Some(bg) = cell.style().bg {
                let bg_hex = color_to_hex(bg, default_bg);
                let x_px = x * cell_width;
                let y_px = y * cell_height;
                svg.push_str(&format!(
                    "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"/>\n",
                    x_px, y_px, cell_width, cell_height, bg_hex
                ));
            }

            if symbol == " " {
                continue;
            }

            let fg_hex = match cell.style().fg {
                Some(fg) => color_to_hex(fg, default_fg),
                None => default_fg.to_string(),
            };
            let weight = if cell.style().add_modifier.contains(Modifier::BOLD) {
                "bold"
            } else {
                "normal"
            };
            let x_px = x * cell_width + 1;
            let y_px = y * cell_height + 2;
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" fill=\"{}\" font-weight=\"{}\">{}</text>\n",
                x_px,
                y_px,
                fg_hex,
                weight,
                escape_xml(symbol)
            ));
        }
    }

    svg.push_str("</svg>\n");
    fs::write(output, svg)?;
    Ok(())
}

fn escape_xml(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

fn color_to_hex(color: Color, default: &str) -> String {
    match color {
        Color::Reset => default.to_string(),
        Color::Black => "#000000".to_string(),
        Color::Red => "#ef4444".to_string(),
        Color::Green => "#22c55e".to_string(),
        Color::Yellow => "#facc15".to_string(),
        Color::Blue => "#3b82f6".to_string(),
        Color::Magenta => "#d946ef".to_string(),
        Color::Cyan => "#06b6d4".to_string(),
        Color::Gray => "#9ca3af".to_string(),
        Color::DarkGray => "#4b5563".to_string(),
        Color::LightRed => "#f87171".to_string(),
        Color::LightGreen => "#4ade80".to_string(),
        Color::LightYellow => "#fde047".to_string(),
        Color::LightBlue => "#60a5fa".to_string(),
        Color::LightMagenta => "#e879f9".to_string(),
        Color::LightCyan => "#67e8f9".to_string(),
        Color::White => "#f9fafb".to_string(),
        Color::Rgb(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b),
        _ => default.to_string(),
    }
}
