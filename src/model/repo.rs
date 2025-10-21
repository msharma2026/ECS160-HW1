pub struct Repo {
    name: String,
    ownerLogin: String,
    htmlUrl: String,
    forksCount: u64,
    language: String,
    openIssuesCount: u64,
    forks: Vec<String>,
    recentCommits: Vec<String>,
    issues: Vec<String>,
    commitCount: u64,
}
impl Repo {
}