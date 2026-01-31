use crate::indexer::FileEntry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub modified: u64,
    pub score: i32,
}

impl From<FileEntry> for SearchResult {
    fn from(entry: FileEntry) -> Self {
        SearchResult {
            path: entry.path,
            name: entry.name,
            extension: entry.extension,
            size: entry.size,
            modified: entry.modified,
            score: 0,
        }
    }
}

pub fn fuzzy_match(query: &str, target: &str) -> Option<i32> {
    let query = query.to_lowercase();
    let target = target.to_lowercase();
    
    if target.contains(&query) {
        // Exact substring match gets high score
        let position = target.find(&query).unwrap();
        return Some(1000 - position as i32);
    }

    let mut query_chars = query.chars().peekable();
    let mut target_chars = target.chars();
    let mut score = 0;
    let mut consecutive = 0;
    let mut position = 0;

    while let Some(&qc) = query_chars.peek() {
        let mut found = false;
        
        while let Some(tc) = target_chars.next() {
            position += 1;
            if qc == tc {
                query_chars.next();
                consecutive += 1;
                score += consecutive * 10; // Bonus for consecutive matches
                found = true;
                break;
            } else {
                consecutive = 0;
            }
        }

        if !found {
            return None; // Character not found
        }
    }

    // Bonus for matching early in the string
    score -= position;
    
    Some(score)
}

pub fn search_files(query: &str, entries: &HashMap<String, FileEntry>) -> Vec<SearchResult> {
    if query.is_empty() {
        return Vec::new();
    }

    let mut results: Vec<SearchResult> = entries
        .values()
        .filter_map(|entry| {
            fuzzy_match(query, &entry.name).map(|score| {
                let mut result: SearchResult = entry.clone().into();
                result.score = score;
                result
            })
        })
        .collect();

    // Sort by score (highest first)
    results.sort_by(|a, b| b.score.cmp(&a.score));

    // Return top 50 results
    results.truncate(50);
    results
}
