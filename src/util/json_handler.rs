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

        // Testing parser function below
        let name = Self::parse_data(json_string, "name");
        match name {
            Some(n) => println!("Extracted name: {}", n),
            None => println!("Name not found"),
        }
        let id = Self::parse_data(json_string, "id");
        match id {
            Some(i) => println!("Extracted id: {}", i),
            None => println!("ID not found"),
        }
        Ok(Vec::new())
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
}