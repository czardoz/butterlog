use butterlog::should_load_more;

#[test]
fn should_load_more_triggers_near_bottom() {
    assert!(should_load_more(80, 100, 20));
    assert!(!should_load_more(10, 100, 20));
}
