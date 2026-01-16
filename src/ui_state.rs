#[derive(Debug)]
pub struct UiState {
    pub selected: usize,
}

impl UiState {
    pub fn new() -> Self {
        Self { selected: 0 }
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
    if rows.is_empty() {
        return;
    }

    match key {
        crossterm::event::KeyCode::Up => state.move_up(rows.len()),
        crossterm::event::KeyCode::Down => {
            let idx = state.selected.min(rows.len() - 1);
            if !rows[idx].expanded {
                crate::toggle_expanded(partitions, &rows[idx].path);
            } else {
                state.move_down(rows.len());
            }
        }
        _ => {}
    }
}
