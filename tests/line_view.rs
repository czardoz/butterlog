use butterlog::{flatten_partitions, LineStore, Partition, RowKind};

#[test]
fn expanded_leaf_partition_shows_lines() {
    let store = LineStore::new(vec!["line one".to_string(), "line two".to_string()]);
    let mut partition = Partition::new("P".to_string(), vec![0, 1], 0);
    partition.expanded = true;

    let rows = flatten_partitions(&[partition], &store, None, 0);

    let line_rows: Vec<&str> = rows
        .iter()
        .filter(|row| row.kind == RowKind::Line)
        .map(|row| row.text.as_str())
        .collect();

    assert_eq!(line_rows, vec!["line one", "line two"]);
}
