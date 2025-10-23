pub struct Issue {
    title: String,
    body: String,
    state: String,
    createdAt: String,
    updatedAt: String,
}
impl Issue {
    pub fn new(title: String, body: String, state: String, createdAt: String, updatedAt: String) -> Self {
        Self { title, body, state, createdAt, updatedAt }
    }
}