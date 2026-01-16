#[derive(Debug)]
pub struct LineStore {
    pub lines: Vec<String>,
    pub normalized: Vec<String>,
}

impl LineStore {
    pub fn new(lines: Vec<String>) -> Self {
        let normalized = lines.iter().map(|line| line.to_lowercase()).collect();
        Self { lines, normalized }
    }

    pub fn get(&self, idx: usize) -> (&str, &str) {
        (&self.lines[idx], &self.normalized[idx])
    }
}
