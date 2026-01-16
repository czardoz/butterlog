use crate::Partition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowPath(pub Vec<usize>);

#[derive(Debug, Clone)]
pub struct VisibleRow {
    pub path: RowPath,
    pub depth: usize,
    pub prefix: String,
    pub line_count: usize,
    pub expanded: bool,
    pub matches_self: bool,
    pub matches_descendants: bool,
}

pub fn flatten_partitions(partitions: &[Partition]) -> Vec<VisibleRow> {
    let mut rows = Vec::new();
    for (idx, partition) in partitions.iter().enumerate() {
        let path = RowPath(vec![idx]);
        flatten_partition(partition, path, &mut rows);
    }
    rows
}

fn flatten_partition(partition: &Partition, path: RowPath, rows: &mut Vec<VisibleRow>) {
    rows.push(VisibleRow {
        path: path.clone(),
        depth: partition.depth,
        prefix: partition.prefix.clone(),
        line_count: partition.line_count(),
        expanded: partition.expanded,
        matches_self: partition.matches_self,
        matches_descendants: partition.matches_descendants,
    });

    if partition.expanded {
        for (idx, child) in partition.children.iter().enumerate() {
            let mut child_path = path.0.clone();
            child_path.push(idx);
            flatten_partition(child, RowPath(child_path), rows);
        }
    }
}

pub fn toggle_expanded(partitions: &mut [Partition], path: &RowPath) {
    toggle_at_path(partitions, &path.0);
}

fn toggle_at_path(partitions: &mut [Partition], path: &[usize]) -> bool {
    let Some((&idx, rest)) = path.split_first() else {
        return false;
    };
    if idx >= partitions.len() {
        return false;
    }
    if rest.is_empty() {
        partitions[idx].expanded = !partitions[idx].expanded;
        return true;
    }
    toggle_at_path(&mut partitions[idx].children, rest)
}
