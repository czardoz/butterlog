#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadStatus {
    pub is_complete: bool,
}

impl LoadStatus {
    pub fn complete() -> Self {
        Self { is_complete: true }
    }

    pub fn partial() -> Self {
        Self { is_complete: false }
    }
}

impl Default for LoadStatus {
    fn default() -> Self {
        Self::complete()
    }
}
