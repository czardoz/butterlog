use crate::{flatten_partitions, BuildOutput, LoadState, LoadStatus, PartitionPlan, UiState, VisibleRow};

#[derive(Debug)]
pub struct AppModel {
    pub line_store: crate::LineStore,
    pub partitions: Vec<crate::Partition>,
    pub rows: Vec<VisibleRow>,
    pub ui: UiState,
    pub load_status: LoadStatus,
    pub load_state: LoadState,
    pub plan: PartitionPlan,
}

impl AppModel {
    pub fn new(output: BuildOutput, load_status: LoadStatus) -> Self {
        let rows = flatten_partitions(&output.partitions, &output.line_store, None, 0);
        let ui = UiState::new();
        Self {
            line_store: output.line_store,
            partitions: output.partitions,
            rows,
            ui,
            load_status,
            load_state: output.load_state,
            plan: output.plan,
        }
    }
}
