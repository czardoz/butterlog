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
