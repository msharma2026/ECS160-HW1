pub struct Repo {
    pub name: String,
    pub ownerLogin: String,
    pub htmlUrl: String,
    pub forksCount: u64,
    pub starsCount: u64,
    pub language: String,
    pub openIssuesCount: u64,
}

impl Repo {
    pub fn new(name: String, ownerLogin: String, htmlUrl: String, forksCount: u64, starsCount: u64, language: String, openIssuesCount: u64) -> Self {
        Self { name, ownerLogin, htmlUrl, forksCount, starsCount, language, openIssuesCount }
    }
}