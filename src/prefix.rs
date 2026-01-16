pub fn prefix_of(line: &str, len: usize) -> String {
    line.chars().take(len).collect()
}
