use butterlog::{insert_top_level, LineStore, Partition, PartitionIndex};

#[test]
fn insert_top_level_updates_existing_partition() {
    let store = LineStore::new(vec!["A1".to_string(), "B1".to_string(), "A2".to_string()]);
    let mut partitions = vec![
        Partition::new("A".to_string(), vec![0], 0, 1),
        Partition::new("B".to_string(), vec![1], 0, 1),
    ];
    let mut index = PartitionIndex::from_partitions(&partitions);

    insert_top_level(&mut partitions, &mut index, 2, &store, 1, 10, None);

    assert_eq!(partitions[0].line_indices, vec![0, 2]);
    assert_eq!(partitions[0].line_count, 2);
}

#[test]
fn insert_top_level_appends_new_partition() {
    let store = LineStore::new(vec!["A1".to_string(), "C9".to_string()]);
    let mut partitions = vec![Partition::new("A".to_string(), vec![0], 0, 1)];
    let mut index = PartitionIndex::from_partitions(&partitions);

    insert_top_level(&mut partitions, &mut index, 1, &store, 1, 10, None);

    assert_eq!(partitions.len(), 2);
    assert_eq!(partitions[1].prefix, "C");
    assert_eq!(partitions[1].line_indices, vec![1]);
}
