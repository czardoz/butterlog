use butterlog::{build_top_level_partitions, group_by_prefix, split_partition};

#[test]
fn splits_large_partitions_with_longer_prefixes() {
    let lines = vec![
        "ERR1 a".to_string(),
        "ERR1 b".to_string(),
        "ERR2 c".to_string(),
        "INFO d".to_string(),
    ];

    let groups = group_by_prefix(&lines, 3);
    let mut partitions = build_top_level_partitions(groups, 0);

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
