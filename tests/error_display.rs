use butterlog::AppError;
use std::io;
use std::path::PathBuf;

#[test]
fn error_messages_are_stable() {
    let missing = AppError::MissingArg;
    assert_eq!(missing.to_string(), "missing log file argument");

    let not_found = AppError::PathNotFound(PathBuf::from("/no/such/file"));
    assert_eq!(not_found.to_string(), "log file not found");

    let not_file = AppError::PathNotFile(PathBuf::from("/tmp"));
    assert_eq!(not_file.to_string(), "path is not a file");

    let io_err = AppError::Io(io::Error::new(io::ErrorKind::Other, "boom"));
    assert_eq!(io_err.to_string(), "io error: boom");
}
