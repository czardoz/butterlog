use crate::{flatten_partitions, LineStore, LoadStatus, Partition, UiState, VisibleRow};

#[derive(Debug)]
pub struct AppModel {
    pub line_store: LineStore,
    pub partitions: Vec<Partition>,
    pub rows: Vec<VisibleRow>,
    pub ui: UiState,
    pub load_status: LoadStatus,
}

impl AppModel {
    pub fn new(line_store: LineStore, partitions: Vec<Partition>, load_status: LoadStatus) -> Self {
        let rows = flatten_partitions(&partitions, &line_store, None, 0);
        let ui = UiState::new();
        Self {
            line_store,
            partitions,
            rows,
            ui,
            load_status,
        }
    }
}
