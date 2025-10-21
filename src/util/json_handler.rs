use crate::model::repo::Repo;

pub struct JsonHandler;

// Test for git_service
impl JsonHandler {
    pub fn parse_search_result(json_string: &str) -> Result<Vec<Repo>, String> {
        println!("JsonHandler Received:");
        if json_string.len() > 500 {
            println!("{}...", &json_string[..500]);
        } else {
            println!("{}", json_string);
        }
        println!("End of JsonHandler");
        
        Ok(Vec::new())
    }
}