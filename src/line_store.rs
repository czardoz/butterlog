#[derive(Debug)]
pub struct LineStore {
    pub lines: Vec<String>,
    pub normalized: Vec<String>,
    pub line_lengths: Vec<usize>,
}

impl LineStore {
    pub fn new(lines: Vec<String>) -> Self {
        let normalized = lines.iter().map(|line| line.to_lowercase()).collect();
        let line_lengths = lines.iter().map(|line| line.len()).collect();
        Self {
            lines,
            normalized,
            line_lengths,
        }
    }

    pub fn get(&self, idx: usize) -> (&str, &str) {
        (&self.lines[idx], &self.normalized[idx])
    }

    pub fn append_lines(&mut self, new_lines: Vec<String>) -> std::ops::Range<usize> {
        let start = self.lines.len();
        for line in new_lines {
            self.line_lengths.push(line.len());
            self.normalized.push(line.to_lowercase());
            self.lines.push(line);
        }
        let end = self.lines.len();
        start..end
    }
}
