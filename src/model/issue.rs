pub struct Issue {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Issue {
    pub fn new(id: u64, title: String, description: Option<String>, state: String, created_at: String, updated_at: String) -> Self {
        Self { id, title, description, state, created_at, updated_at }
    }
}