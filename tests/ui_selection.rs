use butterlog::UiState;

#[test]
fn selection_moves_within_bounds() {
    let mut state = UiState::new();

    state.move_up(3);
    assert_eq!(state.selected, 0);

    state.move_down(3);
    assert_eq!(state.selected, 1);

    state.selected = 2;
    state.move_down(3);
    assert_eq!(state.selected, 2);
}

#[test]
fn selection_handles_empty_rows() {
    let mut state = UiState::new();
    state.selected = 5;
    state.move_down(0);
    assert_eq!(state.selected, 0);
}

#[test]
fn ensure_visible_scrolls_to_selected_row() {
    let mut state = UiState::new();
    state.selected = 5;

    state.ensure_visible(10, 3);

    assert_eq!(state.vertical_offset, 3);
}

#[test]
fn ensure_visible_clamps_selection_when_rows_shrink() {
    let mut state = UiState::new();
    state.selected = 4;

    state.ensure_visible(2, 3);

    assert_eq!(state.selected, 1);
    assert_eq!(state.vertical_offset, 0);
}
