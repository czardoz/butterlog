use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppError {
    MissingArg,
    PathNotFound(PathBuf),
    PathNotFile(PathBuf),
    Io(io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MissingArg => write!(f, "missing log file argument"),
            AppError::PathNotFound(_) => write!(f, "log file not found"),
            AppError::PathNotFile(_) => write!(f, "path is not a file"),
            AppError::Io(err) => write!(f, "io error: {err}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

pub type AppResult<T> = Result<T, AppError>;
