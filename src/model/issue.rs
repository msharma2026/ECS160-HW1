pub struct Issue {
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Issue {
    pub fn new(title: String, description: Option<String>, state: String, created_at: String, updated_at: String) -> Self {
        Self { title, description, state, created_at, updated_at }
    }
}