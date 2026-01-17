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

    let output = build_partitions_from_file(file.path(), 1).expect("pipeline");

    assert!(!output.partitions.is_empty());
    assert_eq!(output.line_store.lines.len(), 3);
    assert!(output.plan.top_prefix_len > 0);
    assert!(output.plan.target_size > 0);
}

#[test]
fn returns_no_partitions_when_sample_is_too_small() {
    let mut file = NamedTempFile::new().expect("temp file");
    writeln!(file, "only one").unwrap();
    file.flush().unwrap();

    let output = build_partitions_from_file(file.path(), 10).expect("pipeline");

    assert!(output.partitions.is_empty());
    assert_eq!(output.line_store.lines.len(), 1);
}

#[test]
fn load_state_is_incomplete_when_more_data_exists() {
    let mut file = NamedTempFile::new().expect("temp file");
    for idx in 0..5001 {
        writeln!(file, "line {idx}").unwrap();
    }
    file.flush().unwrap();

    let output = build_partitions_from_file(file.path(), 1).expect("pipeline");

    assert!(!output.load_state.is_complete);
    assert_eq!(output.line_store.lines.len(), 5000);
}
