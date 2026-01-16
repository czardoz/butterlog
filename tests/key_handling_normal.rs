use butterlog::{flatten_partitions, handle_key_normal, Partition, UiState};
use crossterm::event::KeyCode;

#[test]
fn down_moves_selection() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0)];
    partitions.push(Partition::new("B".to_string(), vec![1], 0));
    let rows = flatten_partitions(&partitions);
    let mut state = UiState::new();

    handle_key_normal(KeyCode::Down, &rows, &mut partitions, &mut state);

    assert_eq!(state.selected, 1);
}

#[test]
fn e_expands_selected_row() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0)];
    let rows = flatten_partitions(&partitions);
    let mut state = UiState::new();

    handle_key_normal(KeyCode::Char('e'), &rows, &mut partitions, &mut state);

    assert!(partitions[0].expanded);
}

#[test]
fn c_collapses_selected_row() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0)];
    partitions[0].expanded = true;
    let rows = flatten_partitions(&partitions);
    let mut state = UiState::new();

    handle_key_normal(KeyCode::Char('c'), &rows, &mut partitions, &mut state);

    assert!(!partitions[0].expanded);
}
