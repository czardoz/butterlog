use butterlog::{initial_prefix_len, target_partition_size};

#[test]
fn target_partition_size_thresholds() {
    assert_eq!(target_partition_size(50), 5);
    assert_eq!(target_partition_size(100), 10);
    assert_eq!(target_partition_size(1_000), 200);
    assert_eq!(target_partition_size(10_000), 500);
}

#[test]
fn initial_prefix_len_thresholds() {
    assert_eq!(initial_prefix_len(50), 4);
    assert_eq!(initial_prefix_len(100), 3);
    assert_eq!(initial_prefix_len(1_000), 2);
    assert_eq!(initial_prefix_len(10_000), 1);
}
