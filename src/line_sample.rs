use std::io::{self, BufRead};
use std::path::Path;

use crate::AppResult;

#[derive(Debug)]
pub struct LineSample {
    pub lines: Vec<String>,
}

pub fn read_first_n_lines(path: &Path, n: usize) -> AppResult<LineSample> {
    let file = std::fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        lines.push(line);
        if lines.len() >= n {
            break;
        }
    }

    Ok(LineSample { lines })
}
