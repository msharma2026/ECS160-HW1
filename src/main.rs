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

    let repos = service.fetch_top_repos("Rust").await;

    // If successful, should print "Got 0 repos"
    println!("Got {} repos.", repos.len());
}