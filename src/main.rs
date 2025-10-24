mod model;
mod service;
mod util;
#[cfg(test)]
mod tests;

// Import both services from the service module
use service::{git_service::GitService, redis_service::RedisService};
use util::valid_repo_checker::ValidRepoChecker;
use std::process::Command;
use std::collections::HashMap;
use crate::util::json_handler::JsonHandler;

const TARGET_LANGUAGES: [&str; 4] = ["Rust", "Java", "C", "C++"];

#[tokio::main]
async fn main() {
    let service = GitService::new().await;
    
    // Create the RedisService
    let mut redis_service = RedisService::new().await;
    println!("GitService created.");
    println!("RedisService connected.");

    for language in TARGET_LANGUAGES {
        let repos = service.fetch_top_repos(language).await;
        println!("Language: {}, Got {} repos.", language, repos.len());

        let total_forks: u64 = repos.iter().map(|f| f.forksCount).sum();

        let mut new_commits: u64 = 0;
        
        for repo in &repos {
            println!("Repo name: {}", &repo.name);

            // Hashmap to store modified files and modified count
            let mut file_counts: HashMap<String, u64> = HashMap::new();

            let commits_list = service.fetch_commits(&repo.ownerLogin, &repo.name).await;
            for commit in &commits_list{
                let commit_details_json = service.fetch_single_commit_details(
                    &repo.ownerLogin,
                    &repo.name,
                    &commit.sha
                ).await;

                if let Ok(files) = JsonHandler::parse_commit_files(&commit_details_json).await {
                    for filename in files {
                        *file_counts.entry(filename).or_insert(0) += 1;
                    }
                }
            }

            // Sorts Hashmap by count
            let mut sorted_files: Vec<_> = file_counts.iter().collect();
            sorted_files.sort_by(|a, b| b.1.cmp(a.1)); //b.1.cmp(a.1) is descending

            println!("Top-3 Most modified files:");
            for (i, (filename, count)) in sorted_files.iter().take(3).enumerate() {
                println!(" File name{}: {} ({} modifications)", i + 1, filename, count);
            }

            // If there are less than 3 files, mark as N/A
            for i in sorted_files.len()..3 {
                println!(" File name{}: [N/A]", i + 1);
            }

            let forks = service.fetch_forks(&repo.ownerLogin, &repo.name).await;
            for fork in forks.iter() {
                let new_commits_list = service.fetch_new_commits(&fork.ownerLogin, &fork.name).await;
                new_commits += new_commits_list.len() as u64;
            }
        }

        let issues: u64 = repos.iter().map(|i| i.openIssuesCount).sum();

        let stars: u64 = repos.iter().map(|s| s.starsCount).sum();

        // Print totals for this language (Part B Statistics)
        println!("\n Language: {}", language);
        println!("Total Forks: {}", total_forks);
        println!("New commits in forked repos: {}", new_commits);
        println!("Open issues in top-10 repos: {}", issues);
        println!("Total stars: {}", stars);

        //Part C code 
        println!("\n Finding Repo for {} ", language);
        for repo in &repos {
            if ValidRepoChecker::valid_repo(repo, language) {
                println!("Found repo: {}/{}", repo.ownerLogin, repo.name);
                
                let clone_url = format!("https://github.com/{}/{}.git", repo.ownerLogin, repo.name);
                let clone_dir = format!("cloned_repos/{}", language);

                println!("Cloning repo to: {}", clone_dir);
                
                let mut git_command = Command::new("git");
                
                git_command.arg("clone");
                git_command.arg("--depth=1");
                git_command.arg(&clone_url);
                git_command.arg(&clone_dir);
                
                let clone_result = git_command.status();
                
                if clone_result.is_err() {
                    println!("Failed to clone");
                    continue;
                }
                
                println!("Repo cloned, checking for build and source files");
                if ValidRepoChecker::both_build_and_source(&clone_dir, language) {
                    redis_service.save_repo(repo).await;
                    println!("Valid repo found");
                    break;
                } else {
                    println!("Missing build and source files, deleting and trying next repo");
                    std::fs::remove_dir_all(&clone_dir).ok();
                }
            } else {
                println!("Skipping repo: {}/{}", repo.ownerLogin, repo.name);
            }
        }
    }
}