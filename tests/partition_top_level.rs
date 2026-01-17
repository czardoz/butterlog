use butterlog::{build_top_level_partitions, group_by_prefix};

#[test]
fn builds_top_level_partitions_from_groups() {
    let lines = vec![
        "ERR: a".to_string(),
        "ERR: b".to_string(),
        "INFO: c".to_string(),
    ];

    let groups = group_by_prefix(&lines, 3);
    let partitions = build_top_level_partitions(groups, 0, 3);

    assert_eq!(partitions.len(), 2);
    assert_eq!(partitions[0].prefix, "ERR");
    assert_eq!(partitions[0].prefix_len, 3);
    assert_eq!(partitions[0].line_indices, vec![0, 1]);
    assert_eq!(partitions[1].prefix, "INF");
    assert_eq!(partitions[1].line_indices, vec![2]);
    assert_eq!(partitions[0].depth, 0);
}
