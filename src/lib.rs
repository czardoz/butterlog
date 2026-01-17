mod error;
mod estimate;
mod file_size;
mod grouping;
mod heuristics;
mod line_sample;
mod line_store;
mod load_status;
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
    build_partitions_from_file, build_partitions_from_file_default, DEFAULT_SCREEN_HEIGHT,
    SAMPLE_LINE_LIMIT,
};
pub use estimate::estimate_total_lines;
pub use file_size::file_size_bytes;
pub use grouping::{group_by_prefix, Group};
pub use heuristics::{initial_prefix_len, target_partition_size};
pub use line_sample::{average_line_len, read_first_n_lines, LineSample};
pub use line_store::LineStore;
pub use load_status::LoadStatus;
pub use partition::{build_top_level_partitions, split_partition, Partition};
pub use prefix::prefix_of;
pub use search::{mark_search_matches, SearchTerm};
pub use ui::{max_row_width, render_rows};
pub use view::{flatten_partitions, toggle_expanded, RowKind, RowPath, VisibleRow};
pub use ui_state::{apply_search, handle_key_normal, InputMode, SearchState, UiState};
