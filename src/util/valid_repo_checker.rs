use crate::model::repo::Repo;
use std::path::Path;
use std::fs;

pub struct ValidRepoChecker;

impl ValidRepoChecker {
    //Build files
    const RUST_BUILD_FILES: &'static [&'static str] = &["Cargo.toml", "Cargo.lock"];
    const JAVA_BUILD_FILES: &'static [&'static str] = &["pom.xml", "build.gradle", "build.gradle.kts", "build.xml"];
    const C_BUILD_FILES: &'static [&'static str] = &["Makefile", "CMakeLists.txt", "configure", "configure.ac"];
    const CPP_BUILD_FILES: &'static [&'static str] = &["Makefile", "CMakeLists.txt", "configure", "configure.ac"];
    
    //Source file extensions
    const RUST_EXTENSIONS: &'static [&'static str] = &[".rs"];
    const JAVA_EXTENSIONS: &'static [&'static str] = &[".java"];
    const C_EXTENSIONS: &'static [&'static str] = &[".c", ".h"];
    const CPP_EXTENSIONS: &'static [&'static str] = &[".cpp", ".cc", ".cxx", ".hpp", ".h"];

    pub fn valid_repo(repo: &Repo, expected_language: &str) -> bool {
        //Check if languages match
        if repo.language == expected_language {
            return true;
        } else {
            return false;
        }
    }

    //Check if cloned repo has build files and source code files
    pub fn both_build_and_source(repo_path: &str, language: &str) -> bool {
        let has_build = Self::build_files_check(repo_path, language);
        let has_source = Self::source_files_check(repo_path, language);
        
        println!("Build files: {}, Source files: {}", has_build, has_source);
        if has_build == true && has_source == true {
            return true;
        } else {
            return false;
        }
    }
    // Check for build files
    fn build_files_check(repo_path: &str, language: &str) -> bool {
        let build_files = match language {
            "Rust" => Self::RUST_BUILD_FILES,
            "Java" => Self::JAVA_BUILD_FILES,
            "C" => Self::C_BUILD_FILES,
            "C++" => Self::CPP_BUILD_FILES,
            _ => &["Makefile"],
        };

        // Loop through each build file and check if it exists
        let mut found_build_file = false;
        for file in build_files.iter() {
            let full_path = Path::new(repo_path).join(file);
            if full_path.exists() == true {
                found_build_file = true;
                break;
            }
        }
        return found_build_file;
    }

    // Check for source code files
    fn source_files_check(repo_path: &str, language: &str) -> bool {
        let extensions = match language {
            "Rust" => Self::RUST_EXTENSIONS,
            "Java" => Self::JAVA_EXTENSIONS,
            "C" => Self::C_EXTENSIONS,
            "C++" => Self::CPP_EXTENSIONS,
            _ => &[],
        };

        // Count how many source files we found
        let file_count = Self::file_extension_counter(repo_path, extensions);
        
        // Check if we found at least one
        let has_files = file_count > 0;
        
        return has_files;
    }

    // Count files with given extensions
    fn file_extension_counter(dir: &str, extensions: &[&str]) -> usize {
        let mut count = 0;
        
        // Read the directory
        if let Ok(entries) = fs::read_dir(dir) {
            // Loop through each entry in the directory
            for entry_result in entries {
                // Check if the entry is valid
                if let Ok(entry) = entry_result {
                    // Get the path of the entry
                    let path = entry.path();
                    
                    // If the path is a directory, recursively count the files
                    if path.is_dir() {
                        count += Self::file_extension_counter(path.to_str().unwrap(), extensions);
                    // If it's a file, check the extension
                    } else if let Some(ext) = path.extension() {
                        // Check if the extension matches any in the list
                        let extension_string = format!(".{}", ext.to_str().unwrap());
                        if extensions.contains(&extension_string.as_str()) {
                            count += 1;
                        }
                    }
                    // Return if we found at least one file
                    if count > 0 {
                        return count;
                    }
                }
            }
        }
        
        return count;
    }
}
