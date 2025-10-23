pub struct Commit {
    pub sha: String,
    pub message: String,
}

impl Commit {
    pub fn new(sha: String, message: String) -> Self {
        Self { sha, message }
    }
}