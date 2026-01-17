use crate::{AppResult, LineLoader, LoadBatch};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadConfig {
    pub batch_size: usize,
    pub near_end_threshold: usize,
}

impl Default for LoadConfig {
    fn default() -> Self {
        Self {
            batch_size: 5000,
            near_end_threshold: 20,
        }
    }
}

#[derive(Debug)]
pub struct LoadState {
    loader: LineLoader,
    pub config: LoadConfig,
    pub is_complete: bool,
}

impl LoadState {
    pub fn new(loader: LineLoader, config: LoadConfig) -> Self {
        Self {
            loader,
            config,
            is_complete: false,
        }
    }

    pub fn load_more(&mut self) -> AppResult<LoadBatch> {
        let batch = self.loader.load_next(self.config.batch_size)?;
        self.is_complete = batch.is_complete;
        Ok(batch)
    }
}
