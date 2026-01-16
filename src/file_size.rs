use std::path::Path;

use crate::AppResult;

pub fn file_size_bytes(path: &Path) -> AppResult<u64> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len())
}
