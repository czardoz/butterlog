use butterlog::group_by_prefix;

#[test]
fn groups_preserve_first_seen_order() {
    let lines = vec![
        "ERR A".to_string(),
        "INFO B".to_string(),
        "ERR C".to_string(),
        "WARN D".to_string(),
    ];

    let groups = group_by_prefix(&lines, 3);

    let prefixes: Vec<&str> = groups.iter().map(|g| g.prefix.as_str()).collect();
    assert_eq!(prefixes, vec!["ERR", "INF", "WAR"]);

    assert_eq!(groups[0].line_indices, vec![0, 2]);
    assert_eq!(groups[1].line_indices, vec![1]);
    assert_eq!(groups[2].line_indices, vec![3]);
}

#[test]
fn groups_handle_empty_lines() {
    let lines = vec!["".to_string(), "A".to_string(), "".to_string()];
    let groups = group_by_prefix(&lines, 1);

    assert_eq!(groups[0].prefix, "");
    assert_eq!(groups[0].line_indices, vec![0, 2]);
    assert_eq!(groups[1].prefix, "A");
    assert_eq!(groups[1].line_indices, vec![1]);
}
