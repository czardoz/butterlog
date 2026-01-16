use butterlog::read_first_n_lines;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn reads_first_n_lines_in_order() {
    let mut file = NamedTempFile::new().expect("temp file");
    writeln!(file, "first").unwrap();
    writeln!(file, "second").unwrap();
    writeln!(file, "third").unwrap();
    file.flush().unwrap();

    let sample = read_first_n_lines(file.path(), 2).expect("sample");
    assert_eq!(sample.lines, vec!["first".to_string(), "second".to_string()]);
}
