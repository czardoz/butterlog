#[derive(Debug, Clone)]
pub struct SearchTerm {
    pub raw: String,
    pub normalized: String,
}

impl SearchTerm {
    pub fn new(raw: &str) -> Self {
        let normalized = raw.to_lowercase();
        Self {
            raw: raw.to_string(),
            normalized,
        }
    }

    pub fn matches_line(&self, normalized_line: &str) -> bool {
        normalized_line.contains(&self.normalized)
    }
}

pub fn mark_search_matches(
    partitions: &mut [crate::Partition],
    line_store: &crate::LineStore,
    term: &SearchTerm,
) {
    for partition in partitions {
        mark_partition(partition, line_store, term);
    }
}

fn mark_partition(
    partition: &mut crate::Partition,
    line_store: &crate::LineStore,
    term: &SearchTerm,
) -> bool {
    partition.matches_self = partition
        .line_indices
        .iter()
        .any(|&idx| term.matches_line(&line_store.normalized[idx]));
    partition.matches_descendants = false;

    for child in &mut partition.children {
        if mark_partition(child, line_store, term) {
            partition.matches_descendants = true;
        }
    }

    partition.matches_self || partition.matches_descendants
}
