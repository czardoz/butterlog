use butterlog::{LineStore, Partition};

#[test]
fn insert_splits_leaf_partition_and_populates_index() {
    let lines = vec!["AA".to_string(), "AB".to_string(), "AC".to_string()];
    let store = LineStore::new(lines.clone());
    let mut partition = Partition::new("A".to_string(), vec![0], 0, 1);

    partition.insert_line(1, &store, 2, None);
    partition.insert_line(2, &store, 2, None);

    assert!(!partition.children.is_empty());
    assert!(!partition.child_index.is_empty());
}
