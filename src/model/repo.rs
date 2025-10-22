pub struct Repo {
    pub name: String,
    pub ownerLogin: String,
    pub htmlUrl: String,
    pub forksCount: u64,
    pub language: String,
    pub openIssuesCount: u64,
    pub forks: Vec<String>,
    pub recentCommits: Vec<String>,
    pub issues: Vec<String>,
    pub commitCount: u64,
}
impl Repo {
    pub fn new(name: String, ownerLogin: String, htmlUrl: String, forksCount: u64, language: String, openIssuesCount: u64, forks: Vec<String>, recentCommits: Vec<String>, issues: Vec<String>, commitCount: u64) -> Self {
        Self { name, ownerLogin, htmlUrl, forksCount, language, openIssuesCount, forks, recentCommits, issues, commitCount }
    }
}