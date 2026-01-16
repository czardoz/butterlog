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
