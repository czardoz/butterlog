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
