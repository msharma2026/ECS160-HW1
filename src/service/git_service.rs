use reqwest::{Client, header};
use crate::util::json_handler::JsonHandler;
use crate::model::repo::Repo;
use std::env;

pub struct GitService {
    client: Client,
    token: String,
}

impl GitService {
    // Creates a new GitService
    pub fn new() -> Self {
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
        let request_url = format!(
            "https://api.github.com/search/repositories?q=language:{}&sort=stars&order=desc&per_page=10",
            language
        );

        let response = self.client.get(&request_url)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .bearer_auth(&self.token) 
            .send()
            .await
            .unwrap(); 

        // No error handling (implement later), so assume it was successful 
        let response_text = response.text().await.unwrap();
        
        // Call the JsonHandler for parsing
        JsonHandler::parse_search_result(&response_text).unwrap()
    }
}