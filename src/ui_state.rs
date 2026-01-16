use crate::SearchTerm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Search,
}

#[derive(Debug)]
pub struct SearchState {
    pub mode: InputMode,
    pub buffer: String,
    pub term: Option<SearchTerm>,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            mode: InputMode::Normal,
            buffer: String::new(),
            term: None,
        }
    }

    pub fn enter(&mut self) {
        self.mode = InputMode::Search;
        self.buffer.clear();
    }

    pub fn handle_key(&mut self, key: crossterm::event::KeyCode) {
        if self.mode != InputMode::Search {
            return;
        }

        match key {
            crossterm::event::KeyCode::Char(ch) => self.buffer.push(ch),
            crossterm::event::KeyCode::Backspace => {
                self.buffer.pop();
            }
            crossterm::event::KeyCode::Enter => {
                if !self.buffer.is_empty() {
                    self.term = Some(SearchTerm::new(&self.buffer));
                }
                self.buffer.clear();
                self.mode = InputMode::Normal;
            }
            crossterm::event::KeyCode::Esc => {
                self.buffer.clear();
                self.mode = InputMode::Normal;
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct UiState {
    pub selected: usize,
    pub search: SearchState,
    pub should_quit: bool,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            selected: 0,
            search: SearchState::new(),
            should_quit: false,
        }
    }

    pub fn enter_search_mode(&mut self) {
        self.search.enter();
    }

    pub fn handle_search_key(&mut self, key: crossterm::event::KeyCode) {
        self.search.handle_key(key);
    }

    pub fn move_up(&mut self, max: usize) {
        if max == 0 {
            self.selected = 0;
            return;
        }
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self, max: usize) {
        if max == 0 {
            self.selected = 0;
            return;
        }
        if self.selected + 1 < max {
            self.selected += 1;
        }
    }
}

pub fn handle_key_normal(
    key: crossterm::event::KeyCode,
    rows: &[crate::VisibleRow],
    partitions: &mut [crate::Partition],
    state: &mut UiState,
) {
    if key == crossterm::event::KeyCode::Char('/') {
        state.enter_search_mode();
        return;
    }
    if key == crossterm::event::KeyCode::Char('q') {
        state.should_quit = true;
        return;
    }
    if rows.is_empty() {
        return;
    }

    match key {
        crossterm::event::KeyCode::Up => state.move_up(rows.len()),
        crossterm::event::KeyCode::Down => state.move_down(rows.len()),
        crossterm::event::KeyCode::Char('e') => {
            let idx = state.selected.min(rows.len() - 1);
            if rows[idx].kind == crate::RowKind::Partition && !rows[idx].expanded {
                crate::toggle_expanded(partitions, &rows[idx].path);
            }
        }
        crossterm::event::KeyCode::Char('c') => {
            let idx = state.selected.min(rows.len() - 1);
            if rows[idx].kind == crate::RowKind::Partition && rows[idx].expanded {
                crate::toggle_expanded(partitions, &rows[idx].path);
            }
        }
        _ => {}
    }
}

pub fn apply_search(
    term: Option<&crate::SearchTerm>,
    partitions: &mut [crate::Partition],
    line_store: &crate::LineStore,
    selected: usize,
) -> Vec<crate::VisibleRow> {
    match term {
        Some(term) => crate::mark_search_matches(partitions, line_store, term),
        None => clear_search_matches(partitions),
    }
    crate::flatten_partitions(partitions, line_store, term, selected)
}

fn clear_search_matches(partitions: &mut [crate::Partition]) {
    for partition in partitions {
        partition.matches_self = false;
        partition.matches_descendants = false;
        clear_search_matches(&mut partition.children);
    }
}
