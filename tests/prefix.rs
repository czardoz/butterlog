use butterlog::prefix_of;

#[test]
fn prefix_of_longer_than_line_returns_full_line() {
    assert_eq!(prefix_of("abc", 10), "abc");
}

#[test]
fn prefix_of_empty_line_is_empty() {
    assert_eq!(prefix_of("", 3), "");
}

#[test]
fn prefix_of_utf8_is_char_based() {
    assert_eq!(prefix_of("éclair", 1), "é");
}
