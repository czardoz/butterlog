use butterlog::{
    flatten_partitions, handle_key_normal, InputMode, LineStore, Partition, SearchTerm, UiState,
};
use crossterm::event::KeyCode;

#[test]
fn slash_enters_search_mode() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0)];
    let store = LineStore::new(vec!["a".to_string()]);
    let rows = flatten_partitions(&partitions, &store, None);
    let mut state = UiState::new();

    handle_key_normal(KeyCode::Char('/'), &rows, &mut partitions, &mut state);

    assert_eq!(state.search.mode, InputMode::Search);
}

#[test]
fn typing_and_enter_sets_term_and_exits_search_mode() {
    let mut state = UiState::new();
    state.enter_search_mode();

    state.handle_search_key(KeyCode::Char('e'));
    state.handle_search_key(KeyCode::Char('r'));
    state.handle_search_key(KeyCode::Enter);

    assert_eq!(state.search.mode, InputMode::Normal);
    assert!(state.search.buffer.is_empty());
    assert_eq!(state.search.term.as_ref().map(|t| t.raw.as_str()), Some("er"));
}

#[test]
fn esc_cancels_search_without_changing_term() {
    let mut state = UiState::new();
    state.search.term = Some(SearchTerm::new("error"));
    state.enter_search_mode();
    state.handle_search_key(KeyCode::Char('x'));

    state.handle_search_key(KeyCode::Esc);

    assert_eq!(state.search.mode, InputMode::Normal);
    assert!(state.search.buffer.is_empty());
    assert_eq!(state.search.term.as_ref().map(|t| t.raw.as_str()), Some("error"));
}

#[test]
fn backspace_removes_last_character() {
    let mut state = UiState::new();
    state.enter_search_mode();

    state.handle_search_key(KeyCode::Char('a'));
    state.handle_search_key(KeyCode::Char('b'));
    state.handle_search_key(KeyCode::Backspace);

    assert_eq!(state.search.buffer, "a");
}
