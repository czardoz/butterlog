use butterlog::LineLoader;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn loads_batches_and_sets_complete_flag() {
    let mut file = NamedTempFile::new().expect("temp file");
    writeln!(file, "first").unwrap();
    writeln!(file, "second").unwrap();
    writeln!(file, "third").unwrap();
    file.flush().unwrap();

    let mut loader = LineLoader::open(file.path()).expect("loader");

    let batch1 = loader.load_next(2).expect("batch1");
    assert_eq!(batch1.lines, vec!["first".to_string(), "second".to_string()]);
    assert!(!batch1.is_complete);

    let batch2 = loader.load_next(2).expect("batch2");
    assert_eq!(batch2.lines, vec!["third".to_string()]);
    assert!(batch2.is_complete);
}
