use std::collections::HashMap;

use crate::prefix_of;

#[derive(Debug, Clone)]
pub struct Group {
    pub prefix: String,
    pub line_indices: Vec<usize>,
}

pub fn group_by_prefix(lines: &[String], prefix_len: usize) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    let mut index_map: HashMap<String, usize> = HashMap::new();

    for (idx, line) in lines.iter().enumerate() {
        let prefix = prefix_of(line, prefix_len);
        if let Some(&group_idx) = index_map.get(&prefix) {
            groups[group_idx].line_indices.push(idx);
        } else {
            let group_idx = groups.len();
            groups.push(Group {
                prefix: prefix.clone(),
                line_indices: vec![idx],
            });
            index_map.insert(prefix, group_idx);
        }
    }

    groups
}

pub fn merge_small_groups(mut groups: Vec<Group>, min_lines: usize) -> Vec<Group> {
    if groups.is_empty() {
        return groups;
    }

    let mut merged: Vec<Group> = Vec::with_capacity(groups.len());
    let mut prev_merged = false;

    for group in groups.drain(..) {
        if merged.is_empty() {
            merged.push(group);
            prev_merged = false;
            continue;
        }

        if group.line_indices.len() < min_lines && !prev_merged {
            let last = merged.last_mut().expect("last group");
            last.line_indices.extend(group.line_indices);
            prev_merged = true;
        } else {
            merged.push(group);
            prev_merged = false;
        }
    }

    merged
}
