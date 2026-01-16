mod error;
mod estimate;
mod file_size;
mod grouping;
mod heuristics;
mod line_sample;
mod line_store;
mod partition;
mod prefix;

pub use error::{AppError, AppResult};
pub use estimate::estimate_total_lines;
pub use file_size::file_size_bytes;
pub use grouping::{group_by_prefix, Group};
pub use heuristics::{initial_prefix_len, target_partition_size};
pub use line_sample::{average_line_len, read_first_n_lines, LineSample};
pub use line_store::LineStore;
pub use partition::{build_top_level_partitions, Partition};
pub use prefix::prefix_of;
