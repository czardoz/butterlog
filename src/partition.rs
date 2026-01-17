use std::collections::HashMap;

use crate::prefix_of;

#[derive(Debug, Clone)]
pub struct Partition {
    pub prefix: String,
    pub prefix_len: usize,
    pub line_indices: Vec<usize>,
    pub line_count: usize,
    pub children: Vec<Partition>,
    pub child_index: HashMap<String, usize>,
    pub depth: usize,
    pub expanded: bool,
    pub matches_self: bool,
    pub matches_descendants: bool,
}

impl Partition {
    pub fn new(prefix: String, line_indices: Vec<usize>, depth: usize, prefix_len: usize) -> Self {
        let line_count = line_indices.len();
        Self {
            prefix,
            prefix_len,
            line_indices,
            line_count,
            children: Vec::new(),
            child_index: HashMap::new(),
            depth,
            expanded: false,
            matches_self: false,
            matches_descendants: false,
        }
    }

    pub fn line_count(&self) -> usize {
        self.line_count
    }

    pub fn rebuild_child_index(&mut self) {
        self.child_index.clear();
        for (idx, child) in self.children.iter().enumerate() {
            self.child_index.insert(child.prefix.clone(), idx);
        }
    }

    pub fn insert_line(&mut self, line_idx: usize, lines: &[String], target_size: usize) {
        self.line_indices.push(line_idx);
        self.line_count += 1;

        if !self.children.is_empty() {
            let child_prefix_len = self
                .children
                .first()
                .map(|child| child.prefix_len)
                .unwrap_or(self.prefix_len + 1);
            let child_prefix = prefix_of(&lines[line_idx], child_prefix_len);
            if let Some(&child_idx) = self.child_index.get(&child_prefix) {
                self.children[child_idx].insert_line(line_idx, lines, target_size);
            } else {
                let new_child =
                    Partition::new(child_prefix.clone(), vec![line_idx], self.depth + 1, child_prefix_len);
                self.children.push(new_child);
                self.child_index
                    .insert(child_prefix, self.children.len() - 1);
            }
            return;
        }

        if self.line_count > target_size {
            split_partition(self, lines, target_size);
        }
    }
}

#[derive(Debug, Default)]
pub struct PartitionIndex {
    pub top_index: HashMap<String, usize>,
}

impl PartitionIndex {
    pub fn from_partitions(partitions: &[Partition]) -> Self {
        let mut top_index = HashMap::new();
        for (idx, partition) in partitions.iter().enumerate() {
            top_index.insert(partition.prefix.clone(), idx);
        }
        Self { top_index }
    }
}

pub fn build_top_level_partitions(
    groups: Vec<crate::Group>,
    depth: usize,
    prefix_len: usize,
) -> Vec<Partition> {
    groups
        .into_iter()
        .map(|group| Partition::new(group.prefix, group.line_indices, depth, prefix_len))
        .collect()
}

pub fn insert_top_level(
    partitions: &mut Vec<Partition>,
    index: &mut PartitionIndex,
    line_idx: usize,
    line: &str,
    prefix_len: usize,
) {
    let prefix = prefix_of(line, prefix_len);
    if let Some(&idx) = index.top_index.get(&prefix) {
        let partition = &mut partitions[idx];
        partition.line_indices.push(line_idx);
        partition.line_count += 1;
    } else {
        let new_idx = partitions.len();
        partitions.push(Partition::new(prefix.clone(), vec![line_idx], 0, prefix_len));
        index.top_index.insert(prefix, new_idx);
    }
}

pub fn split_partition(partition: &mut Partition, lines: &[String], target_size: usize) {
    if partition.line_count() <= target_size {
        return;
    }

    let max_len = partition
        .line_indices
        .iter()
        .map(|&idx| lines[idx].chars().count())
        .max()
        .unwrap_or(0);

    let mut candidate_len = partition.prefix_len + 1;
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
        .map(|(prefix, indices)| {
            Partition::new(prefix, indices, partition.depth + 1, candidate_len)
        })
        .collect();
    partition.rebuild_child_index();

    for child in &mut partition.children {
        split_partition(child, lines, target_size);
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
