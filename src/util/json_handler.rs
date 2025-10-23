use crate::model::repo::Repo;
use crate::model::commit::Commit;
use crate::model::issue::Issue;
use crate::model::owner::Owner;

pub struct JsonHandler;

// Test for git_service
impl JsonHandler {
    pub fn extract_items_array(json_string: &str) -> Result<String, String> {
        println!("JsonHandler Received:");
        
        let start_items = json_string.find("\"items\":[");
        if let Some(start_items) = start_items {
            let mut bracket_index = 0;
            let mut end = start_items + 9; // skip "items":[

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
    
    pub fn json_extract(array_json: &str) -> Vec<String> {
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

    pub fn parse_repos_object(items_json: &str) -> Result<Vec<Repo>, String> {
        let objects = Self::json_extract(items_json);
        let mut repos: Vec<Repo> = Vec::new();
        
        for repo_json in objects {
            let name = Self::parse_data(&repo_json, "name").unwrap_or_default();
            let owner_login = Self::parse_data(&repo_json, "login").unwrap_or_default(); // owner is at root level for forks
            let html_url = Self::parse_data(&repo_json, "html_url").unwrap_or_default();
            let forks_count = Self::parse_data(&repo_json, "forks_count")
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or(0);
            let language = Self::parse_data(&repo_json, "language").unwrap_or_default();
            let open_issues_count = Self::parse_data(&repo_json, "open_issues_count")
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or(0);
            let forks_url = Self::parse_data(&repo_json, "forks_url").unwrap_or_default();
            let commits_url = Self::parse_data(&repo_json, "commits_url").unwrap_or_default();
            let issues_url = Self::parse_data(&repo_json, "issues_url").unwrap_or_default();
        
            let repo = Repo::new(
                name, 
                owner_login, 
                html_url,
                forks_count, 
                language, 
                open_issues_count, 
                forks_url,
                commits_url,
                issues_url
                );
            
            repos.push(repo);
        }
        
        Ok(repos)
    }

    pub fn parse_owners_object(items_json: &str) -> Result<Vec<Owner>, String> {
        let objects = Self::json_extract(items_json);
        let mut owners = Vec::new();

        for owner_json in objects {
            let login = Self::parse_data(&owner_json, "login").unwrap_or_default();
            let id = Self::parse_data(&owner_json, "id").unwrap_or_default().parse::<u64>().unwrap_or(0);
            let htmlUrl = Self::parse_data(&owner_json, "htmlUrl").unwrap_or_default();
            let site_admin = Self::parse_data(&owner_json, "site_admin").map(|s| s == "true").unwrap_or(false);

            let owner = Owner::new(login, id, htmlUrl, site_admin);

            owners.push(owner)
        }
        Ok(owners)
    }

    pub fn parse_issues_object(items_json: &str) -> Result<Vec<Issue>, String> {
        let objects = Self::json_extract(items_json);
        let mut issues = Vec::new();

        for issue_json in objects {
            let title = Self::parse_data(&issue_json, "title").unwrap_or_default();
            let body = Self::parse_data(&issue_json, "body").unwrap_or_default();
            let state = Self::parse_data(&issue_json, "state").unwrap_or_default();
            let created_at = Self::parse_data(&issue_json, "created_at").unwrap_or_default();
            let updated_at = Self::parse_data(&issue_json, "updated_at").unwrap_or_default();

            let issue = Issue::new(title, body, state, created_at, updated_at);

            issues.push(issue);
        }

        Ok(issues)
    }

    pub fn parse_data(json: &str, key: &str) -> Option<String> {
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
            let end_index = json[start_index..].find(|c| c == ',' || c == '}' || c == '"').map(|c| start_index + c).unwrap_or(json.len());
            // Returns data field as a string
            return Some(json[start_index..end_index].trim().to_string());
        }
        None
    
    }

    pub fn parse_commits_result(items_json: &str) -> Result<Vec<Commit>, String> {
        let objects = Self::json_extract(items_json);
        let mut commits = Vec::new();

        for commit_json in objects {
            let sha = Self::parse_data(&commit_json, "sha").unwrap_or_default();
            let message = Self::parse_data(&commit_json, "message").unwrap_or_default();
            
            let commit = Commit::new(sha, message);
            commits.push(commit);
        }

        Ok(commits)
    }
}