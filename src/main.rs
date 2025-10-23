mod model;
mod service;
mod util;

use service::git_service::GitService;

// Test for git_service
#[tokio::main]
async fn main() {
    // Creates the GitService
    let service = GitService::new();
    println!("GitService created.");

    let target_language = ["Rust", "Java", "C", "C++"];
    for language in target_language {
        let repos = service.fetch_top_repos(language).await;
        println!("Language: {}, Got {} repos.", language, repos.len());

        let forks = service.fetch_forks(&repos[0].ownerLogin, &repos[0].name).await;
        println!("Forks: {}", forks.len());

        let commits = service.fetch_commits(&repos[0].ownerLogin, &repos[0].name).await;
        println!("Commits: {}", commits.len());

        let issues = service.fetch_issues(&repos[0].ownerLogin, &repos[0].name).await;
        println!("Issues: {}", issues.len());

    }

}