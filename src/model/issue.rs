pub struct Issue {
    pub title: String,
    pub body: String,
    pub state: String,
    pub createdAt: String,
    pub updatedAt: String,
}
impl Issue {
    pub fn new(title: String, body: String, state: String, createdAt: String, updatedAt: String) -> Self {
        Self { title, body, state, createdAt, updatedAt }
    }
}