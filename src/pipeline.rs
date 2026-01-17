use std::path::Path;

use crate::{
    average_line_len, build_top_level_partitions, estimate_total_lines, file_size_bytes,
    group_by_prefix, split_partition, AppResult, Group, LineLoader, LineSample, LineStore,
    LoadConfig, LoadState, Partition,
};

pub const DEFAULT_SCREEN_HEIGHT: u16 = 24;
pub const SAMPLE_LINE_LIMIT: usize = 5000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartitionPlan {
    pub top_prefix_len: usize,
    pub target_size: usize,
}

impl PartitionPlan {
    pub fn from_sample(lines: &[String], estimated_lines: u64, screen_height: u16) -> Self {
        let target_partitions = target_partition_count(lines.len(), screen_height);
        let top_prefix_len = choose_prefix_len(lines, target_partitions);
        let target_size = target_partition_size_for_screen(estimated_lines, target_partitions);
        Self {
            top_prefix_len,
            target_size,
        }
    }
}

#[derive(Debug)]
pub struct BuildOutput {
    pub line_store: LineStore,
    pub partitions: Vec<Partition>,
    pub load_state: LoadState,
    pub plan: PartitionPlan,
}

pub fn build_partitions_from_file(
    path: &Path,
    screen_height: u16,
) -> AppResult<BuildOutput> {
    let loader = LineLoader::open(path)?;
    let mut load_state = LoadState::new(loader, LoadConfig::default());
    let batch = load_state.load_more()?;
    let line_store = LineStore::new(batch.lines);

    let sample = LineSample {
        lines: line_store.lines.clone(),
    };
    let avg_len = average_line_len(&sample);
    let file_size = file_size_bytes(path)?;
    let estimated_lines = estimate_total_lines(file_size, avg_len);
    let sample_count = line_store.lines.len();
    let raw_target = (screen_height as usize).saturating_mul(2).max(1);
    let plan = PartitionPlan::from_sample(&line_store.lines, estimated_lines, screen_height);

    if sample_count < raw_target {
        return Ok(BuildOutput {
            line_store,
            partitions: Vec::new(),
            load_state,
            plan,
        });
    }

    let target_partitions = target_partition_count(sample_count, screen_height);
    let mut groups = group_by_prefix(&line_store.lines, plan.top_prefix_len);
    if groups.len() < target_partitions {
        groups = split_groups_to_target(groups, target_partitions);
    }
    let mut partitions = build_top_level_partitions(groups, 0, plan.top_prefix_len);

    for partition in &mut partitions {
        split_partition(partition, &line_store.lines, plan.target_size);
    }

    Ok(BuildOutput {
        line_store,
        partitions,
        load_state,
        plan,
    })
}

pub fn build_partitions_from_file_default(
    path: &Path,
) -> AppResult<BuildOutput> {
    build_partitions_from_file(path, DEFAULT_SCREEN_HEIGHT)
}

fn target_partition_count(sample_count: usize, screen_height: u16) -> usize {
    let target = (screen_height as usize).saturating_mul(2).max(1);
    target.min(sample_count.max(1)).max(1)
}

fn target_partition_size_for_screen(estimated_lines: u64, target_partitions: usize) -> usize {
    let target = target_partitions.max(1) as u64;
    let size = (estimated_lines + target - 1) / target;
    size.max(1) as usize
}

fn choose_prefix_len(lines: &[String], target_partitions: usize) -> usize {
    let max_len = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(1);
    let mut chosen = 1;
    for len in 1..=max_len {
        chosen = len;
        let groups = group_by_prefix(lines, len);
        if groups.len() >= target_partitions {
            break;
        }
    }
    chosen
}

fn split_groups_to_target(mut groups: Vec<Group>, target_partitions: usize) -> Vec<Group> {
    let mut target = target_partitions.max(1);
    let total_lines: usize = groups.iter().map(|g| g.line_indices.len()).sum();
    if total_lines == 0 {
        return groups;
    }
    target = target.min(total_lines);

    while groups.len() < target {
        let mut max_idx = None;
        let mut max_len = 0;
        for (idx, group) in groups.iter().enumerate() {
            if group.line_indices.len() > max_len {
                max_len = group.line_indices.len();
                max_idx = Some(idx);
            }
        }

        let Some(idx) = max_idx else {
            break;
        };
        if max_len <= 1 {
            break;
        }

        let group = groups.remove(idx);
        let mid = group.line_indices.len() / 2;
        let (left, right) = group.line_indices.split_at(mid);

        let left_prefix = format!("{}#1", group.prefix);
        let right_prefix = format!("{}#2", group.prefix);

        groups.insert(
            idx,
            Group {
                prefix: right_prefix,
                line_indices: right.to_vec(),
            },
        );
        groups.insert(
            idx,
            Group {
                prefix: left_prefix,
                line_indices: left.to_vec(),
            },
        );
    }

    groups
}
