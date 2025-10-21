mod model;
mod service;
mod util;

use crate::service::git_service::GitService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = GitService::test_call().await?;
    println!("API Response: {}", result);
    Ok(())
}