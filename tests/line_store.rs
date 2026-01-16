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
