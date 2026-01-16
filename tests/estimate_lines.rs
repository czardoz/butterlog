use butterlog::estimate_total_lines;

#[test]
fn estimates_total_lines_with_floor_division() {
    assert_eq!(estimate_total_lines(10, 6), 1);
}

#[test]
fn estimates_total_lines_for_simple_case() {
    assert_eq!(estimate_total_lines(100, 10), 10);
}

#[test]
fn zero_average_length_returns_zero() {
    assert_eq!(estimate_total_lines(100, 0), 0);
}
