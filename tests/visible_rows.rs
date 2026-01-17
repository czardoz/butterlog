use butterlog::{flatten_partitions, LineStore, Partition, RowKind, RowPath};

#[test]
fn flattens_only_expanded_children() {
    let mut root = Partition::new("ROOT".to_string(), vec![0], 0, 4);
    root.expanded = true;
    root.children
        .push(Partition::new("A".to_string(), vec![1], 1, 1));
    root.children
        .push(Partition::new("B".to_string(), vec![2], 1, 1));

    let mut other = Partition::new("OTHER".to_string(), vec![3], 0, 5);
    other.expanded = false;

    let store = LineStore::new(vec![
        "root".to_string(),
        "a".to_string(),
        "b".to_string(),
        "other".to_string(),
    ]);
    let rows = flatten_partitions(&[root, other], &store, None, 0);

    let partition_rows: Vec<_> = rows
        .iter()
        .filter(|row| row.kind == RowKind::Partition)
        .collect();

    let prefixes: Vec<&str> = partition_rows.iter().map(|r| r.text.as_str()).collect();
    assert_eq!(prefixes, vec!["ROOT", "A", "B", "OTHER"]);

    assert_eq!(partition_rows[0].path, RowPath(vec![0]));
    assert_eq!(partition_rows[1].path, RowPath(vec![0, 0]));
    assert_eq!(partition_rows[2].path, RowPath(vec![0, 1]));
    assert_eq!(partition_rows[3].path, RowPath(vec![1]));
}
