mod error;
mod file_size;
mod line_sample;

pub use error::{AppError, AppResult};
pub use file_size::file_size_bytes;
pub use line_sample::{average_line_len, read_first_n_lines, LineSample};
