use reqwest::Client;


static API_URL: &str = "https://api.github.com";

pub struct GitService {
}
impl GitService {
    pub async fn test_call() -> Result<String, Box<dyn std::error::Error>> {
        let client = Client::new();
        let url = format!("{}/search/repositories?q=language:rust&sort=stars&order=desc&per_page=1", API_URL);

        let response = client.get(&url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }
}