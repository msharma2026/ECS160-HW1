use crate::model::repo::Repo;
use std::collections::HashMap;

#[test]
fn test_total_stars() {
    let repos = vec![
        Repo::new("repo1".to_string(), "owner".to_string(), "url".to_string(), 17, 150, "Rust".to_string(), 8),
        Repo::new("repo2".to_string(), "owner".to_string(), "url".to_string(), 23, 320, "Rust".to_string(), 14),
    ];
    
    let total: u64 = repos.iter().map(|r| r.starsCount).sum();
    assert_eq!(total, 470);
}

#[test]
fn test_total_forks() {
    let repos = vec![
        Repo::new("repo1".to_string(), "owner".to_string(), "url".to_string(), 17, 150, "Rust".to_string(), 8),
        Repo::new("repo2".to_string(), "owner".to_string(), "url".to_string(), 23, 320, "Rust".to_string(), 14),
    ];
    
    let total: u64 = repos.iter().map(|r| r.forksCount).sum();
    assert_eq!(total, 40);
}

#[test]
fn test_total_open_issues() {
    let repos = vec![
        Repo::new("repo1".to_string(), "owner".to_string(), "url".to_string(), 17, 150, "Rust".to_string(), 8),
        Repo::new("repo2".to_string(), "owner".to_string(), "url".to_string(), 23, 320, "Rust".to_string(), 14),
    ];
    
    let total: u64 = repos.iter().map(|r| r.openIssuesCount).sum();
    assert_eq!(total, 22);
}

#[test]
fn test_tracked_modified_files() {
    let mut file_modification_counts: HashMap<String, u64> = HashMap::new();
    
    *file_modification_counts.entry("main.rs".to_string()).or_insert(0) += 1;
    *file_modification_counts.entry("file_mod.rs".to_string()).or_insert(0) += 1;
    *file_modification_counts.entry("main.rs".to_string()).or_insert(0) += 1;
    
    assert_eq!(file_modification_counts.get("main.rs"), Some(&2));
    assert_eq!(file_modification_counts.get("file_mod.rs"), Some(&1));
}

#[test]
fn test_top_3_most_modified_files() {
    let mut file_modification_counts: HashMap<String, u64> = HashMap::new();
    file_modification_counts.insert("file1.rs".to_string(), 42);
    file_modification_counts.insert("file2.rs".to_string(), 18);
    file_modification_counts.insert("file3.rs".to_string(), 31);
    file_modification_counts.insert("file4.rs".to_string(), 7);
    
    let mut sorted: Vec<_> = file_modification_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    let top_3: Vec<_> = sorted.iter().take(3).collect();
    assert_eq!(top_3.len(), 3);
    assert_eq!(*top_3[0].1, 42);
    assert_eq!(*top_3[1].1, 31);
    assert_eq!(*top_3[2].1, 18);
}

#[test]
fn test_top_3_with_less_than_3_modified_files() {
    let mut file_modification_counts: HashMap<String, u64> = HashMap::new();
    file_modification_counts.insert("file1.rs".to_string(), 25);
    
    let mut sorted: Vec<_> = file_modification_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    let top_3: Vec<_> = sorted.iter().take(3).collect();
    assert_eq!(top_3.len(), 1);
}

#[test]
fn test_total_commits() {
    let commit_total: u64 = 12 + 9 + 6;
    assert_eq!(commit_total, 27);
}
