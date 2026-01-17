use butterlog::{LineLoader, LoadConfig, LoadState};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn load_config_defaults() {
    let config = LoadConfig::default();
    assert_eq!(config.batch_size, 5000);
    assert_eq!(config.near_end_threshold, 20);
}

#[test]
fn load_state_marks_complete_on_eof() {
    let mut file = NamedTempFile::new().expect("temp file");
    writeln!(file, "one").unwrap();
    writeln!(file, "two").unwrap();
    writeln!(file, "three").unwrap();
    file.flush().unwrap();

    let loader = LineLoader::open(file.path()).expect("loader");
    let config = LoadConfig {
        batch_size: 2,
        near_end_threshold: 20,
    };
    let mut state = LoadState::new(loader, config);

    let batch1 = state.load_more().expect("batch1");
    assert!(!batch1.is_complete);
    assert!(!state.is_complete);

    let batch2 = state.load_more().expect("batch2");
    assert!(batch2.is_complete);
    assert!(state.is_complete);
}
