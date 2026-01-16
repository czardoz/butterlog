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
