pub struct Commit {
    pub sha: String,
}

impl Commit {
    pub fn new(sha: String) -> Self {
        Self { sha }
    }
}