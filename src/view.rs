use crate::{LineStore, Partition, SearchTerm};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowPath(pub Vec<usize>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowKind {
    Partition,
    Line,
}

#[derive(Debug, Clone)]
pub struct VisibleRow {
    pub kind: RowKind,
    pub path: RowPath,
    pub depth: usize,
    pub text: String,
    pub line_count: usize,
    pub expanded: bool,
    pub matches_self: bool,
    pub matches_descendants: bool,
    pub line_index: Option<usize>,
}

pub fn flatten_partitions(
    partitions: &[Partition],
    line_store: &LineStore,
    term: Option<&SearchTerm>,
) -> Vec<VisibleRow> {
    let mut rows = Vec::new();
    for (idx, partition) in partitions.iter().enumerate() {
        let path = RowPath(vec![idx]);
        flatten_partition(partition, &path, &mut rows, line_store, term);
    }
    rows
}

fn flatten_partition(
    partition: &Partition,
    path: &RowPath,
    rows: &mut Vec<VisibleRow>,
    line_store: &LineStore,
    term: Option<&SearchTerm>,
) {
    rows.push(VisibleRow {
        kind: RowKind::Partition,
        path: path.clone(),
        depth: partition.depth,
        text: partition.prefix.clone(),
        line_count: partition.line_count(),
        expanded: partition.expanded,
        matches_self: partition.matches_self,
        matches_descendants: partition.matches_descendants,
        line_index: None,
    });

    if partition.expanded {
        if partition.children.is_empty() {
            for &line_idx in &partition.line_indices {
                let matches = term
                    .map(|term| term.matches_line(&line_store.normalized[line_idx]))
                    .unwrap_or(false);
                rows.push(VisibleRow {
                    kind: RowKind::Line,
                    path: path.clone(),
                    depth: partition.depth + 1,
                    text: line_store.lines[line_idx].clone(),
                    line_count: 0,
                    expanded: false,
                    matches_self: matches,
                    matches_descendants: false,
                    line_index: Some(line_idx),
                });
            }
        } else {
            for (idx, child) in partition.children.iter().enumerate() {
                let mut child_path = path.0.clone();
                child_path.push(idx);
                flatten_partition(child, &RowPath(child_path), rows, line_store, term);
            }
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
