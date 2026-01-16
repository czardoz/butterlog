use butterlog::file_size_bytes;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn file_size_matches_contents() {
    let mut file = NamedTempFile::new().expect("temp file");
    let content = b"abc\n";
    file.write_all(content).expect("write content");
    file.flush().expect("flush");

    let size = file_size_bytes(file.path()).expect("size");
    assert_eq!(size, content.len() as u64);
}
