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

        let mut total_forks = 0;
        let mut total_commits = 0;
        let mut total_issues = 0;

        // Loop through all 10 repos
        for repo in &repos {
            println!("\nChecking repo: {}/{}", repo.ownerLogin, repo.name);
            
            let forks = service.fetch_forks(&repo.ownerLogin, &repo.name).await;
            println!("  Forks: {}", forks.len());
            total_forks += forks.len();

            let commits = service.fetch_commits(&repo.ownerLogin, &repo.name).await;
            println!("  Commits: {}", commits.len());
            total_commits += commits.len();

            let issues = service.fetch_issues(&repo.ownerLogin, &repo.name).await;
            println!("  Issues: {}", issues.len());
            total_issues += issues.len();
        }

        // Print totals for this language
        println!("\n--- Totals for {} ---", language);
        println!("Total Forks: {}", total_forks);
        println!("Total Commits: {}", total_commits);
        println!("Total Issues: {}", total_issues);
    }

}