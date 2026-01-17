use butterlog::{toggle_expanded, Partition, RowPath};

#[test]
fn toggles_expanded_state() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0, 1)];

    toggle_expanded(&mut partitions, &RowPath(vec![0]));
    assert!(partitions[0].expanded);

    toggle_expanded(&mut partitions, &RowPath(vec![0]));
    assert!(!partitions[0].expanded);
}

#[test]
fn invalid_path_does_nothing() {
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0, 1)];
    toggle_expanded(&mut partitions, &RowPath(vec![1]));
    assert!(!partitions[0].expanded);
}
