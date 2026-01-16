pub fn estimate_total_lines(file_size_bytes: u64, avg_line_len: usize) -> u64 {
    if avg_line_len == 0 {
        return 0;
    }
    file_size_bytes / avg_line_len as u64
}
