mod error;
mod estimate;
mod file_size;
mod heuristics;
mod line_sample;
mod prefix;

pub use error::{AppError, AppResult};
pub use estimate::estimate_total_lines;
pub use file_size::file_size_bytes;
pub use heuristics::{initial_prefix_len, target_partition_size};
pub use line_sample::{average_line_len, read_first_n_lines, LineSample};
pub use prefix::prefix_of;
