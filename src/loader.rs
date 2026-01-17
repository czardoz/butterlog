use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::AppResult;

#[derive(Debug)]
pub struct LoadBatch {
    pub lines: Vec<String>,
    pub is_complete: bool,
}

pub struct LineLoader {
    reader: io::BufReader<File>,
    is_complete: bool,
}

impl LineLoader {
    pub fn open(path: &Path) -> AppResult<Self> {
        let file = File::open(path)?;
        Ok(Self {
            reader: io::BufReader::new(file),
            is_complete: false,
        })
    }

    pub fn load_next(&mut self, batch_size: usize) -> AppResult<LoadBatch> {
        if self.is_complete {
            return Ok(LoadBatch {
                lines: Vec::new(),
                is_complete: true,
            });
        }

        let mut lines = Vec::new();
        for _ in 0..batch_size {
            let mut buf = String::new();
            let bytes = self.reader.read_line(&mut buf)?;
            if bytes == 0 {
                self.is_complete = true;
                break;
            }
            if buf.ends_with('\n') {
                buf.pop();
                if buf.ends_with('\r') {
                    buf.pop();
                }
            }
            lines.push(buf);
        }

        Ok(LoadBatch {
            lines,
            is_complete: self.is_complete,
        })
    }
}
