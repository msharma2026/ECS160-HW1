use redis::{AsyncCommands, Client, aio::MultiplexedConnection};
use crate::model::repo::Repo;
use crate::model::issue::Issue;

pub struct RedisService {
    // Async connection
    connection: MultiplexedConnection,
}

impl RedisService {
    // Creates a new RedisService
    pub async fn new() -> Self {
        let client = Client::open("redis://127.0.0.1/")
            .expect("Failed to open Redis client");
        let connection = client.get_multiplexed_tokio_connection()
            .await
            .expect("Failed to get async Redis connection");
        
        Self { connection }
    }

    pub async fn save_repo(&mut self, repo: &Repo) {
        let repo_key = format!("reponame:{}", repo.name);
        let owner_key = format!("owner:{}", repo.ownerLogin);
        let forks_count_str = repo.forksCount.to_string();
        let open_issues_str = repo.openIssuesCount.to_string();
        let stars_count_str = repo.starsCount.to_string();

        let issue_ids = repo.issues.iter()
            .filter(|i| i.id != 0)
            .map(|i| format!("iss-{}", i.id))
            .collect::<Vec<String>>()
            .join(",");

        let _: () = self.connection.hset_multiple(
            repo_key,
            &[
                ("htmlUrl", &repo.htmlUrl),
                ("forksCount", &forks_count_str),
                ("starsCount", &stars_count_str),
                ("language", &repo.language),
                ("openIssuesCount", &open_issues_str),
                ("owner", &owner_key),
                ("issues", &issue_ids)
            ]
        )
        .await
        .unwrap();

        for issue in &repo.issues {
            self.save_issue(issue).await;
        }
    }

    pub async fn save_issue(&mut self, issue: &Issue) {
        let issue_key = format!("issue:{}", issue.id);
        let description = issue.description.clone().unwrap_or_default();

        let _: () = self.connection.hset_multiple(
            issue_key,
            &[
                ("createdAt", &issue.created_at),
                ("description", &description)
            ]
        )
        .await
        .unwrap();
    }
}