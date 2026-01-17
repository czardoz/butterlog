use butterlog::LineStore;

#[test]
fn line_store_preserves_and_normalizes() {
    let store = LineStore::new(vec!["Error".to_string(), "Info".to_string()]);

    assert_eq!(store.lines, vec!["Error".to_string(), "Info".to_string()]);
    assert_eq!(store.normalized, vec!["error".to_string(), "info".to_string()]);

    let (original, normalized) = store.get(0);
    assert_eq!(original, "Error");
    assert_eq!(normalized, "error");
}

#[test]
fn append_lines_extends_store() {
    let mut store = LineStore::new(vec!["One".to_string()]);
    let range = store.append_lines(vec!["Two".to_string(), "Three".to_string()]);

    assert_eq!(range, 1..3);
    assert_eq!(
        store.lines,
        vec!["One".to_string(), "Two".to_string(), "Three".to_string()]
    );
    assert_eq!(
        store.normalized,
        vec!["one".to_string(), "two".to_string(), "three".to_string()]
    );
    assert_eq!(store.line_lengths, vec![3, 3, 5]);
}
