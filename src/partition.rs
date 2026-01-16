use std::collections::HashMap;

use crate::prefix_of;

#[derive(Debug, Clone)]
pub struct Partition {
    pub prefix: String,
    pub line_indices: Vec<usize>,
    pub children: Vec<Partition>,
    pub depth: usize,
    pub expanded: bool,
    pub matches_self: bool,
    pub matches_descendants: bool,
}

impl Partition {
    pub fn new(prefix: String, line_indices: Vec<usize>, depth: usize) -> Self {
        Self {
            prefix,
            line_indices,
            children: Vec::new(),
            depth,
            expanded: false,
            matches_self: false,
            matches_descendants: false,
        }
    }

    pub fn line_count(&self) -> usize {
        self.line_indices.len()
    }
}

pub fn build_top_level_partitions(groups: Vec<crate::Group>, depth: usize) -> Vec<Partition> {
    groups
        .into_iter()
        .map(|group| Partition::new(group.prefix, group.line_indices, depth))
        .collect()
}

pub fn split_partition(
    partition: &mut Partition,
    lines: &[String],
    prefix_len: usize,
    target_size: usize,
) {
    if partition.line_count() <= target_size {
        return;
    }

    let max_len = partition
        .line_indices
        .iter()
        .map(|&idx| lines[idx].chars().count())
        .max()
        .unwrap_or(0);

    let mut candidate_len = prefix_len + 1;
    let mut groups = Vec::new();
    while candidate_len <= max_len {
        groups = group_indices_by_prefix(&partition.line_indices, lines, candidate_len);
        if groups.len() > 1 {
            break;
        }
        candidate_len += 1;
    }

    if groups.len() <= 1 {
        return;
    }

    partition.children = groups
        .into_iter()
        .map(|(prefix, indices)| Partition::new(prefix, indices, partition.depth + 1))
        .collect();

    for child in &mut partition.children {
        split_partition(child, lines, candidate_len, target_size);
    }
}

fn group_indices_by_prefix(
    indices: &[usize],
    lines: &[String],
    prefix_len: usize,
) -> Vec<(String, Vec<usize>)> {
    let mut groups: Vec<(String, Vec<usize>)> = Vec::new();
    let mut index_map: HashMap<String, usize> = HashMap::new();

    for &line_idx in indices {
        let prefix = prefix_of(&lines[line_idx], prefix_len);
        if let Some(&group_idx) = index_map.get(&prefix) {
            groups[group_idx].1.push(line_idx);
        } else {
            let group_idx = groups.len();
            groups.push((prefix.clone(), vec![line_idx]));
            index_map.insert(prefix, group_idx);
        }
    }

    groups
}
