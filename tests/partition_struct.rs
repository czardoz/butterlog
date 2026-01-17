use butterlog::Partition;

#[test]
fn partition_defaults_are_stable() {
    let partition = Partition::new("ERR".to_string(), vec![1, 2], 0, 3);

    assert_eq!(partition.prefix, "ERR");
    assert_eq!(partition.line_indices, vec![1, 2]);
    assert_eq!(partition.prefix_len, 3);
    assert_eq!(partition.line_count, 2);
    assert!(partition.child_index.is_empty());
    assert!(partition.children.is_empty());
    assert!(!partition.expanded);
    assert!(!partition.matches_self);
    assert!(!partition.matches_descendants);
    assert_eq!(partition.depth, 0);
    assert_eq!(partition.line_count(), 2);
}
