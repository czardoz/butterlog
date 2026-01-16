use butterlog::{flatten_partitions, handle_key_normal, LineStore, Partition, UiState};
use crossterm::event::KeyCode;

#[test]
fn down_moves_selection() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0)];
    partitions.push(Partition::new("B".to_string(), vec![1], 0));
    let store = LineStore::new(vec!["a".to_string(), "b".to_string()]);
    let rows = flatten_partitions(&partitions, &store, None, 0);
    let mut state = UiState::new();

    handle_key_normal(KeyCode::Down, &rows, &mut partitions, &mut state);

    assert_eq!(state.selected, 1);
}

#[test]
fn e_expands_selected_row() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0)];
    let store = LineStore::new(vec!["a".to_string()]);
    let rows = flatten_partitions(&partitions, &store, None, 0);
    let mut state = UiState::new();

    handle_key_normal(KeyCode::Char('e'), &rows, &mut partitions, &mut state);

    assert!(partitions[0].expanded);
}

#[test]
fn c_collapses_selected_row() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0)];
    partitions[0].expanded = true;
    let store = LineStore::new(vec!["a".to_string()]);
    let rows = flatten_partitions(&partitions, &store, None, 0);
    let mut state = UiState::new();

    handle_key_normal(KeyCode::Char('c'), &rows, &mut partitions, &mut state);

    assert!(!partitions[0].expanded);
}

#[test]
fn right_scrolls_horizontally() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0)];
    let store = LineStore::new(vec!["a".to_string()]);
    let rows = flatten_partitions(&partitions, &store, None, 0);
    let mut state = UiState::new();

    handle_key_normal(KeyCode::Right, &rows, &mut partitions, &mut state);

    assert_eq!(state.horizontal_offset, 4);
}

#[test]
fn left_scrolls_horizontally_but_not_below_zero() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0)];
    let store = LineStore::new(vec!["a".to_string()]);
    let rows = flatten_partitions(&partitions, &store, None, 0);
    let mut state = UiState::new();

    handle_key_normal(KeyCode::Left, &rows, &mut partitions, &mut state);
    assert_eq!(state.horizontal_offset, 0);

    state.horizontal_offset = 2;
    handle_key_normal(KeyCode::Left, &rows, &mut partitions, &mut state);
    assert_eq!(state.horizontal_offset, 0);
}
