use reqwest::{Client, header};
use crate::util::json_handler::JsonHandler;
use crate::model::repo::Repo;
use crate::model::commit::Commit;
use crate::model::issue::Issue;
use std::env;

pub struct GitService {
    client: Client,
    token: String,
}

impl GitService {
    // Creates a new GitService
    pub async fn new() -> Self {
        // Loads .env
        dotenv::dotenv().ok(); 
        
        // Reads github token from .env
        let token = env::var("GITHUB_TOKEN")
            .expect("GITHUB_TOKEN not found in .env file");

        let client = Client::builder()
            .user_agent("ecs160-hw1-rust")
            .build()
            .unwrap(); 
        
        GitService {client, token}
    }
    // Fetches the top 10 repos
    pub async fn fetch_top_repos(&self, language: &str) -> Vec<Repo> {
        // replace + with %2B to handle +
        let encoded_language = language.replace("+", "%2B");
        let request_url = format!(
            "https://api.github.com/search/repositories?q=language:{}&sort=stars&order=desc&per_page=10",
            encoded_language
        );

        let response = self.client.get(&request_url)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .bearer_auth(&self.token) 
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        // No error handling (implement later), so assume it was successful 
        let items_json = JsonHandler::extract_items_array(&response).await.unwrap();
        
        // Call the JsonHandler for parsing
        JsonHandler::parse_repos_object(&items_json).await.unwrap()
    }

    pub async fn fetch_forks(&self, owner: &str, repo: &str) -> Vec<Repo> {
        let request_url = format!(
            "https://api.github.com/repos/{}/{}/forks?per_page=10",
            owner, repo
        );

        let response = self.client.get(&request_url)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .bearer_auth(&self.token) 
            .send()
            .await
            .unwrap(); 
        
        let response_text = response.text().await.unwrap();

        JsonHandler::parse_repos_object(&response_text).await.unwrap()
    }

    pub async fn fetch_new_commits(&self, owner: &str, repo: &str) -> Vec<Commit> {
        let request_url = format!(
            "https://api.github.com/repos/{}/{}/commits?per_page=20",
            owner, repo
        );

        let response = self.client.get(&request_url)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .bearer_auth(&self.token) 
            .send()
            .await
            .unwrap(); 
        
        let response_text = response.text().await.unwrap();

        JsonHandler::parse_commits_result(&response_text).await.unwrap()
    }

    pub async fn fetch_commits(&self, owner: &str, repo: &str) -> Vec<Commit> {
        let request_url = format!(
            "https://api.github.com/repos/{}/{}/commits?per_page=50",
            owner, repo
        );

        let response = self.client.get(&request_url)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .bearer_auth(&self.token)
            .send()
            .await
            .unwrap();

        let response_text = response.text().await.unwrap();

        JsonHandler::parse_commits_result(&response_text).await.unwrap()
    }

    pub async fn fetch_issues(&self, owner: &str, repo: &str) -> Vec<Issue> {
        let request_url = format!(
            "https://api.github.com/repos/{}/{}/issues?per_page=10",
            owner, repo
        );

        let response = self.client.get(&request_url)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .bearer_auth(&self.token) 
            .send()
            .await
            .unwrap(); 
        
        let response_text = response.text().await.unwrap();

        JsonHandler::parse_issues_object(&response_text).await.unwrap()
    }

    pub async fn fetch_single_commit_details(&self, owner: &str, repo: &str, sha: &str) -> String {
        let request_url = format!(
            "https://api.github.com/repos/{}/{}/commits/{}",
            owner, repo, sha
        );
    
        let response = self.client.get(&request_url)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .bearer_auth(&self.token) 
            .send()
            .await
            .unwrap(); 
        
        response.text().await.unwrap()
    }
}