use std::path::Path;

use crate::{
    average_line_len, build_top_level_partitions, estimate_total_lines, file_size_bytes,
    group_by_prefix, initial_prefix_len, read_first_n_lines, split_partition, AppResult, LineStore,
    Partition,
};

pub const DEFAULT_SCREEN_HEIGHT: u16 = 24;

pub fn build_partitions_from_file(
    path: &Path,
    screen_height: u16,
) -> AppResult<(LineStore, Vec<Partition>)> {
    let sample = read_first_n_lines(path, 5000)?;
    let avg_len = average_line_len(&sample);
    let file_size = file_size_bytes(path)?;
    let estimated_lines = estimate_total_lines(file_size, avg_len);
    let target_size = target_partition_size_for_screen(estimated_lines, screen_height);
    let prefix_len = initial_prefix_len(target_size);

    let line_store = LineStore::new(sample.lines);
    let groups = group_by_prefix(&line_store.lines, prefix_len);
    let mut partitions = build_top_level_partitions(groups, 0);

    for partition in &mut partitions {
        split_partition(partition, &line_store.lines, prefix_len, target_size);
    }

    Ok((line_store, partitions))
}

pub fn build_partitions_from_file_default(
    path: &Path,
) -> AppResult<(LineStore, Vec<Partition>)> {
    build_partitions_from_file(path, DEFAULT_SCREEN_HEIGHT)
}

fn target_partition_size_for_screen(estimated_lines: u64, screen_height: u16) -> usize {
    let target_partitions = (screen_height as u64).saturating_mul(2).max(1);
    let size = (estimated_lines + target_partitions - 1) / target_partitions;
    size.max(1) as usize
}
