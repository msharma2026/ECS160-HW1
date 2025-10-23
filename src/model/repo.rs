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
    pub forks_url: String,
    pub commits_url: String,
    pub issues_url: String,
}
impl Repo {
    pub fn new(name: String, ownerLogin: String, htmlUrl: String, forksCount: u64, language: String, openIssuesCount: u64, forks_url: String, commits_url: String, issues_url:String) -> Self {
        Self { name, ownerLogin, htmlUrl, forksCount, language, openIssuesCount, forks: Vec::new(), recentCommits: Vec::new(), issues: Vec::new(), commitCount: 0, forks_url, commits_url, issues_url }
    }
}