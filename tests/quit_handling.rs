use butterlog::{flatten_partitions, handle_key_normal, Partition, UiState};
use crossterm::event::KeyCode;

#[test]
fn q_sets_should_quit() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0)];
    let rows = flatten_partitions(&partitions);
    let mut state = UiState::new();

    handle_key_normal(KeyCode::Char('q'), &rows, &mut partitions, &mut state);

    assert!(state.should_quit);
}
