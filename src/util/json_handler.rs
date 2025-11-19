use crate::model::repo::Repo;
use crate::model::commit::Commit;
use crate::model::issue::Issue;

pub struct JsonHandler;

// Test for git_service
impl JsonHandler {
    pub async fn extract_items_array(json_string: &str) -> Result<String, String> {
        println!("JsonHandler Received:");
        
        let start_items = json_string.find("\"items\":[");
        if let Some(start_items) = start_items {
            let mut bracket_index = 0;
            let mut end = start_items + 9;

            while end < json_string.len() {
                if json_string.as_bytes()[end] == b'[' {
                    bracket_index += 1;
                } else if json_string.as_bytes()[end] == b']' {
                    if bracket_index == 0 {
                        break;
                    }
                    bracket_index -= 1;
                }
                end += 1;
            }
        
        // Extract the items array
        let items_json = &json_string[start_items + 9..end];

        Ok(items_json.to_string())
        } else {
            Err("No items found".to_string())
        }
    }
    
    pub async fn json_extract(array_json: &str) -> Vec<String> {
        let mut items = Vec::new();
        let mut bracket_index = 0;
        let mut end = 0;
        let mut start = None;
        
        while end < array_json.len() {
            let byte = array_json.as_bytes()[end];
            if byte == b'{' {
                if bracket_index == 0 {
                    start = Some(end);
                }
                bracket_index += 1;
            } else if byte == b'}' {
                bracket_index -= 1;
                if bracket_index == 0 {
                    if let Some(start_index) = start {
                        let obj = &array_json[start_index..=end];
                        items.push(obj.to_string());
                        start = None;
                    }
                }
            }
            end += 1;
        }

        items
    }

    pub async fn parse_repos_object(items_json: &str) -> Result<Vec<Repo>, String> {
        let objects = Self::json_extract(items_json).await;
        let mut repos: Vec<Repo> = Vec::new();
        
        for repo_json in objects {
            let name = Self::parse_data(&repo_json, "name").await
                .unwrap_or_default();
            let owner_login = Self::parse_data(&repo_json, "login").await
                .unwrap_or_default(); // owner is at root level for forks
            let html_url = Self::parse_data(&repo_json, "html_url").await
                .unwrap_or_default();
            let forks_count = Self::parse_data(&repo_json, "forks_count").await
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or(0);
            let stars_count = Self::parse_data(&repo_json, "stargazers_count").await
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or(0);
            let language = Self::parse_data(&repo_json, "language").await
                .unwrap_or_default();
            let open_issues_count = Self::parse_data(&repo_json, "open_issues_count").await
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or(0);
            let issues: Vec<Issue> = Vec::new();
            let repo = Repo::new(
                name, 
                owner_login, 
                html_url,
                forks_count, 
                stars_count,
                language, 
                open_issues_count,
                issues
                );
            
            repos.push(repo);
        }
        
        Ok(repos)
    }

    pub async fn parse_issues_result(items_json: &str) -> Result<Vec<Issue>, String> {
        let objects = Self::json_extract(items_json).await;
        let mut issues: Vec<Issue> = Vec::new();
        
        for issue_json in objects {
            let id = Self::parse_data(&items_json, "id").await
                .unwrap_or_default();
            let title = Self::parse_data(&items_json, "title").await
                .unwrap_or_default();
            let description = Self::parse_data(&items_json, "body").await; // optional
            let state = Self::parse_data(&items_json, "state").await
                .unwrap_or_default();
            let created_at = Self::parse_data(&items_json, "created_at").await
                .unwrap_or_default();
            let updated_at = Self::parse_data(&items_json, "updated_at").await
                .unwrap_or_default();
        
            let issue = Issue::new(
                id,
                title, 
                description, 
                state,
                created_at, 
                updated_at
                );
            
            issues.push(issue);
        }
        
        Ok(issues)
    }

    pub async fn parse_data(json: &str, key: &str) -> Option<String> {
        // Formats key to search for data field in the JSON
        let pattern = format!("\"{}\":", key);
        // Checks if the data field exists within the JSON
        if let Some(start) = json.find(&pattern) {
            // Calculates start index of data field (to right after the colon)
            let mut start_index = start + pattern.len();
    
            // Counts spaces and quotes after colon to find actual data start
            while start_index < json.len() && (json.as_bytes()[start_index] == b' ' || json.as_bytes()[start_index] == b'"') {
                start_index += 1;
            }
            // Finds end index of data and maps it
            let end_index = json[start_index..].find(|c| c == ',' || c == '}' || c == '"')
                .map(|c| start_index + c)
                .unwrap_or(json.len());
            // Returns data field as a string
            return Some(json[start_index..end_index].trim().to_string());
        }
        None
    
    }

    pub async fn parse_commits_result(items_json: &str) -> Result<Vec<Commit>, String> {
        let objects = Self::json_extract(items_json).await;
        let mut commits = Vec::new();

        for commit_json in objects {
            let sha = Self::parse_data(&commit_json, "sha").await
                .unwrap_or_default();
            
            let commit = Commit::new(sha);
            commits.push(commit);
        }

        Ok(commits)
    }

    pub async fn parse_commit_files(json_string: &str) -> Result<Vec<String>, String> {
        let start_files = json_string.find("\"files\":[");
        if let Some(start_files) = start_files {
            let mut bracket_index = 0;
            let mut end = start_files + 9;
    
            while end < json_string.len() {
                if json_string.as_bytes()[end] == b'[' {
                    bracket_index += 1;
                } else if json_string.as_bytes()[end] == b']' {
                    if bracket_index == 0 {
                        break;
                    }
                    bracket_index -= 1;
                }
                end += 1;
            }
        
            let files_array_json = &json_string[start_files + 9..end];
            let file_objects = Self::json_extract(files_array_json).await;
            
            let mut filenames: Vec<String> = Vec::new();
            
            // Grabs file name from each file object
            for file_obj_str in file_objects {
                if let Some(filename) = Self::parse_data(&file_obj_str, "filename").await {
                    filenames.push(filename);
                }
            }
            
            Ok(filenames)
    
        } else {
            Err("No files array found".to_string())
        }
    }
}