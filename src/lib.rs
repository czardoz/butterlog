mod error;
mod estimate;
mod file_size;
mod grouping;
mod heuristics;
mod line_sample;
mod line_store;
mod loader;
mod load_status;
mod load_state;
mod partition;
mod prefix;
mod search;
mod app;
mod pipeline;
mod ui;
mod view;
mod ui_state;

pub use error::{AppError, AppResult};
pub use app::AppModel;
pub use pipeline::{
    build_partitions_from_file, build_partitions_from_file_default, BuildOutput,
    DEFAULT_SCREEN_HEIGHT, PartitionPlan, SAMPLE_LINE_LIMIT,
};
pub use estimate::estimate_total_lines;
pub use file_size::file_size_bytes;
pub use grouping::{group_by_prefix, merge_small_groups, Group};
pub use heuristics::{initial_prefix_len, target_partition_size};
pub use line_sample::{average_line_len, read_first_n_lines, LineSample};
pub use line_store::LineStore;
pub use loader::{LineLoader, LoadBatch};
pub use load_status::LoadStatus;
pub use load_state::{should_load_more, LoadConfig, LoadState};
pub use partition::{
    build_top_level_partitions, insert_top_level, split_partition, Partition, PartitionIndex,
};
pub use prefix::prefix_of;
pub use search::{mark_search_matches, SearchTerm};
pub use ui::{max_row_width, render_rows};
pub use view::{flatten_partitions, toggle_expanded, RowKind, RowPath, VisibleRow};
pub use ui_state::{apply_search, handle_key_normal, InputMode, SearchState, UiState};
