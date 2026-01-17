use crate::{
    apply_search, flatten_partitions, max_row_width, BuildOutput, LoadState, LoadStatus,
    PartitionIndex, PartitionPlan, UiState, VisibleRow,
};

#[derive(Debug)]
pub struct AppModel {
    pub line_store: crate::LineStore,
    pub partitions: Vec<crate::Partition>,
    pub partition_index: PartitionIndex,
    pub rows: Vec<VisibleRow>,
    pub rows_dirty: bool,
    pub cached_max_width: usize,
    pub ui: UiState,
    pub load_status: LoadStatus,
    pub load_state: LoadState,
    pub plan: PartitionPlan,
}

impl AppModel {
    pub fn new(output: BuildOutput, load_status: LoadStatus) -> Self {
        let rows = flatten_partitions(&output.partitions, &output.line_store, None, 0);
        let ui = UiState::new();
        let partition_index = PartitionIndex::from_partitions(&output.partitions);
        Self {
            line_store: output.line_store,
            partitions: output.partitions,
            partition_index,
            rows,
            rows_dirty: true,
            cached_max_width: 0,
            ui,
            load_status,
            load_state: output.load_state,
            plan: output.plan,
        }
    }

    pub fn mark_rows_dirty(&mut self) {
        self.rows_dirty = true;
    }

    pub fn mark_rows_clean(&mut self) {
        self.rows_dirty = false;
    }

    pub fn refresh_rows_if_dirty(&mut self) {
        if !self.rows_dirty {
            return;
        }
        let term = self.ui.search.term.as_ref();
        self.rows = apply_search(term, &mut self.partitions, &self.line_store, self.ui.selected);
        self.cached_max_width = max_row_width(&self.rows);
        self.rows_dirty = false;
    }
}
