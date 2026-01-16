use butterlog::{flatten_partitions, handle_key_normal, LineStore, Partition, UiState};
use crossterm::event::KeyCode;

#[test]
fn q_sets_should_quit() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0)];
    let store = LineStore::new(vec!["a".to_string()]);
    let rows = flatten_partitions(&partitions, &store, None, 0);
    let mut state = UiState::new();

    handle_key_normal(KeyCode::Char('q'), &rows, &mut partitions, &mut state);

    assert!(state.should_quit);
}
