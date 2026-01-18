use butterlog::{merge_small_groups, Group};

fn group(prefix: &str, start: usize, count: usize) -> Group {
    Group {
        prefix: prefix.to_string(),
        line_indices: (start..start + count).collect(),
    }
}

#[test]
fn merges_small_groups_into_previous_without_chaining() {
    let groups = vec![
        group("A", 0, 100),
        group("B", 100, 10),
        group("C", 110, 10),
        group("D", 120, 100),
        group("E", 220, 10),
    ];

    let merged = merge_small_groups(groups, 80);

    assert_eq!(merged.len(), 3);
    assert_eq!(merged[0].prefix, "A");
    assert_eq!(merged[0].line_indices.len(), 110);
    assert_eq!(merged[1].prefix, "C");
    assert_eq!(merged[1].line_indices.len(), 10);
    assert_eq!(merged[2].prefix, "D");
    assert_eq!(merged[2].line_indices.len(), 110);
}
