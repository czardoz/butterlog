use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use crossterm::event::{self, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use butterlog::{
    apply_search, build_partitions_from_file, build_partitions_from_file_default, handle_key_normal,
    max_row_width, AppError, AppModel, InputMode, SearchTerm,
};

#[derive(Parser, Debug)]
#[command(name = "butterlog", about = "Visualize and scan huge log files")]
struct Args {
    /// Path to the log file
    log_file: Option<String>,
    /// Run the pipeline without launching the TUI
    #[arg(long)]
    no_ui: bool,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let args = Args::parse();
    let log_file = args.log_file.ok_or(AppError::MissingArg)?;
    let path = PathBuf::from(log_file);
    validate_path(&path)?;

    if args.no_ui {
        run_no_ui(&path)
    } else {
        run_ui(&path)
    }
}

fn validate_path(path: &PathBuf) -> Result<(), AppError> {
    match std::fs::metadata(path) {
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(AppError::PathNotFile(path.clone()));
            }
        }
        Err(_) => {
            return Err(AppError::PathNotFound(path.clone()));
        }
    }
    Ok(())
}

fn run_ui(path: &PathBuf) -> Result<(), AppError> {
    let (_, screen_height) = crossterm::terminal::size()?;
    let (line_store, partitions) = build_partitions_from_file(path, screen_height)?;
    let mut model = AppModel::new(line_store, partitions);

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let _guard = TerminalGuard;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let mut rows = refresh_rows(
            model.ui.search.term.as_ref(),
            &mut model.partitions,
            &model.line_store,
            model.ui.selected,
        );

        let term_size = terminal.size()?;
        let list_height = term_size.height.saturating_sub(1);
        if model.ui.ensure_visible(rows.len(), list_height) {
            rows = refresh_rows(
                model.ui.search.term.as_ref(),
                &mut model.partitions,
                &model.line_store,
                model.ui.selected,
            );
        }
        model.rows = rows;

        let term_width = term_size.width as usize;
        let max_width = max_row_width(&model.rows);
        let max_scroll = max_width
            .saturating_sub(term_width)
            .min(u16::MAX as usize) as u16;
        model.ui.clamp_horizontal(max_scroll);

        terminal.draw(|frame| {
            butterlog::render_rows(
                &model.rows,
                frame,
                model.ui.vertical_offset,
                model.ui.horizontal_offset,
                &model.ui.search,
            );
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if model.ui.search.mode == InputMode::Search {
                    model.ui.handle_search_key(key.code);
                } else {
                    handle_key_normal(key.code, &model.rows, &mut model.partitions, &mut model.ui);
                }
            }
        }

        model.ui.clamp_horizontal(max_scroll);
        model.ui.ensure_visible(model.rows.len(), list_height);

        if model.ui.should_quit {
            break;
        }
    }

    Ok(())
}

fn run_no_ui(path: &PathBuf) -> Result<(), AppError> {
    let (_store, partitions) = build_partitions_from_file_default(path)?;
    println!("partitions: {}", partitions.len());
    Ok(())
}

fn refresh_rows(
    term: Option<&SearchTerm>,
    partitions: &mut [butterlog::Partition],
    line_store: &butterlog::LineStore,
    selected: usize,
) -> Vec<butterlog::VisibleRow> {
    match term {
        Some(term) => apply_search(Some(term), partitions, line_store, selected),
        None => apply_search(None, partitions, line_store, selected),
    }
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let mut stdout = std::io::stdout();
        let _ = stdout.execute(LeaveAlternateScreen);
    }
}
