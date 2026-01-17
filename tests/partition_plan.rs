use butterlog::PartitionPlan;

#[test]
fn builds_partition_plan_from_sample() {
    let lines = vec![
        "A1".to_string(),
        "A2".to_string(),
        "B1".to_string(),
        "B2".to_string(),
    ];

    let plan = PartitionPlan::from_sample(&lines, 20, 1);

    assert_eq!(plan.top_prefix_len, 1);
    assert_eq!(plan.target_size, 10);
}
