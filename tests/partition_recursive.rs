use butterlog::{build_top_level_partitions, group_by_prefix, split_partition, Partition};

#[test]
fn splits_large_partitions_with_longer_prefixes() {
    let lines = vec![
        "ERR1 a".to_string(),
        "ERR1 b".to_string(),
        "ERR2 c".to_string(),
        "INFO d".to_string(),
    ];

    let groups = group_by_prefix(&lines, 3);
    let mut partitions = build_top_level_partitions(groups, 0, 3);

    for partition in &mut partitions {
        split_partition(partition, &lines, 3, 2);
    }

    let err_partition = &partitions[0];
    assert_eq!(err_partition.prefix, "ERR");
    assert_eq!(err_partition.children.len(), 2);
    assert_eq!(err_partition.children[0].prefix, "ERR1");
    assert_eq!(err_partition.children[0].line_indices, vec![0, 1]);
    assert_eq!(err_partition.children[1].prefix, "ERR2");
    assert_eq!(err_partition.children[1].line_indices, vec![2]);

    let info_partition = &partitions[1];
    assert!(info_partition.children.is_empty());
}

#[test]
fn splits_when_next_prefix_is_same_but_longer_differs() {
    let lines = vec![
        "ABC1x".to_string(),
        "ABC1y".to_string(),
        "ABC1z".to_string(),
    ];

    let mut partitions = vec![Partition::new("ABC".to_string(), vec![0, 1, 2], 0, 3)];

    split_partition(&mut partitions[0], &lines, 3, 1);

    let children = &partitions[0].children;
    assert_eq!(children.len(), 3);
    assert_eq!(children[0].prefix, "ABC1x");
    assert_eq!(children[1].prefix, "ABC1y");
    assert_eq!(children[2].prefix, "ABC1z");
}
