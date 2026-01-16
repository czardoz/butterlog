use butterlog::{flatten_partitions, Partition, RowPath};

#[test]
fn flattens_only_expanded_children() {
    let mut root = Partition::new("ROOT".to_string(), vec![0], 0);
    root.expanded = true;
    root.children.push(Partition::new("A".to_string(), vec![1], 1));
    root.children.push(Partition::new("B".to_string(), vec![2], 1));

    let mut other = Partition::new("OTHER".to_string(), vec![3], 0);
    other.expanded = false;

    let rows = flatten_partitions(&[root, other]);

    let prefixes: Vec<&str> = rows.iter().map(|r| r.prefix.as_str()).collect();
    assert_eq!(prefixes, vec!["ROOT", "A", "B", "OTHER"]);

    assert_eq!(rows[0].path, RowPath(vec![0]));
    assert_eq!(rows[1].path, RowPath(vec![0, 0]));
    assert_eq!(rows[2].path, RowPath(vec![0, 1]));
    assert_eq!(rows[3].path, RowPath(vec![1]));
}
