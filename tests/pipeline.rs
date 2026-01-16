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

    let (store, partitions) = build_partitions_from_file(file.path(), 10).expect("pipeline");

    assert!(!partitions.is_empty());
    assert_eq!(store.lines.len(), 3);
}
