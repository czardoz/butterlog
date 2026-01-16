pub fn target_partition_size(estimated_lines: u64) -> usize {
    if estimated_lines >= 10_000 {
        500
    } else if estimated_lines >= 1_000 {
        200
    } else if estimated_lines >= 100 {
        10
    } else {
        5
    }
}

pub fn initial_prefix_len(target_size: usize) -> usize {
    if target_size >= 10_000 {
        1
    } else if target_size >= 1_000 {
        2
    } else if target_size >= 100 {
        3
    } else {
        4
    }
}
