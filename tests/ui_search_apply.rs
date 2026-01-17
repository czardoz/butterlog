use butterlog::{apply_search, LineStore, Partition, SearchTerm};

#[test]
fn applies_search_and_updates_rows() {
    let mut partitions = vec![
        Partition::new("ERR".to_string(), vec![0], 0, 3),
        Partition::new("OK".to_string(), vec![1], 0, 2),
    ];

    let store = LineStore::new(vec!["error here".to_string(), "all good".to_string()]);
    let term = SearchTerm::new("error");

    let rows = apply_search(Some(&term), &mut partitions, &store, 0);

    assert!(partitions[0].matches_self);
    assert!(!partitions[1].matches_self);
    assert!(rows[0].matches_self);
}

#[test]
fn clears_search_when_term_is_none() {
    let mut partitions = vec![Partition::new("ERR".to_string(), vec![0], 0, 3)];
    partitions[0].matches_self = true;
    partitions[0].matches_descendants = true;

    let store = LineStore::new(vec!["error".to_string()]);

    let rows = apply_search(None, &mut partitions, &store, 0);

    assert!(!partitions[0].matches_self);
    assert!(!partitions[0].matches_descendants);
    assert!(!rows[0].matches_self);
}
