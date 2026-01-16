use crate::{flatten_partitions, LineStore, Partition, UiState, VisibleRow};

#[derive(Debug)]
pub struct AppModel {
    pub line_store: LineStore,
    pub partitions: Vec<Partition>,
    pub rows: Vec<VisibleRow>,
    pub ui: UiState,
}

impl AppModel {
    pub fn new(line_store: LineStore, partitions: Vec<Partition>) -> Self {
        let rows = flatten_partitions(&partitions, &line_store, None, 0);
        let ui = UiState::new();
        Self {
            line_store,
            partitions,
            rows,
            ui,
        }
    }
}
