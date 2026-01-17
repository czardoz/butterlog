use butterlog::Partition;

#[test]
fn insert_splits_leaf_partition_and_populates_index() {
    let lines = vec!["AA".to_string(), "AB".to_string(), "AC".to_string()];
    let mut partition = Partition::new("A".to_string(), vec![0], 0, 1);

    partition.insert_line(1, &lines, 2);
    partition.insert_line(2, &lines, 2);

    assert!(!partition.children.is_empty());
    assert!(!partition.child_index.is_empty());
}
