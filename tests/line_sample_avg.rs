use butterlog::{average_line_len, LineSample};

#[test]
fn average_line_length_is_floor_division() {
    let sample = LineSample {
        lines: vec!["a".to_string(), "bb".to_string()],
    };
    assert_eq!(average_line_len(&sample), 1);
}

#[test]
fn average_line_length_handles_multiple_lines() {
    let sample = LineSample {
        lines: vec!["aa".to_string(), "bbb".to_string(), "c".to_string()],
    };
    assert_eq!(average_line_len(&sample), 2);
}
