use butterlog::build_partitions_from_file;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn builds_partitions_from_file() {
    let mut file = NamedTempFile::new().expect("temp file");
    writeln!(file, "ERR one").unwrap();
    writeln!(file, "ERR two").unwrap();
    writeln!(file, "INFO three").unwrap();
    file.flush().unwrap();

    let (store, partitions) = build_partitions_from_file(file.path(), 1).expect("pipeline");

    assert!(!partitions.is_empty());
    assert_eq!(store.lines.len(), 3);
}

#[test]
fn returns_no_partitions_when_sample_is_too_small() {
    let mut file = NamedTempFile::new().expect("temp file");
    writeln!(file, "only one").unwrap();
    file.flush().unwrap();

    let (store, partitions) = build_partitions_from_file(file.path(), 10).expect("pipeline");

    assert!(partitions.is_empty());
    assert_eq!(store.lines.len(), 1);
}
