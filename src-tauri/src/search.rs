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
    let query = normalize_search_text(query);
    let target = normalize_search_text(target);

    if query.is_empty() || target.is_empty() {
        return None;
    }
    
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
            score_entry(query, entry).map(|score| {
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

fn score_entry(query: &str, entry: &FileEntry) -> Option<i32> {
    let normalized_query = normalize_search_text(query);
    if normalized_query.is_empty() {
        return None;
    }

    let normalized_name = normalize_search_text(&entry.name);
    let normalized_path = normalize_search_text(&entry.path);
    let normalized_search_text = normalize_search_text(&entry.search_text);

    let mut best_score: Option<i32> = None;

    for (candidate, bonus) in [
        (&entry.name, 300),
        (&entry.search_text, 180),
        (&entry.path, 80),
    ] {
        if let Some(score) = fuzzy_match(query, candidate) {
            best_score = Some(best_score.map_or(score + bonus, |best| best.max(score + bonus)));
        }
    }

    let mut score = best_score?;

    if normalized_name == normalized_query {
        score += 400;
    } else if normalized_name.starts_with(&normalized_query) {
        score += 220;
    } else if normalized_search_text.starts_with(&normalized_query) {
        score += 100;
    } else if normalized_path.contains(&normalized_query) {
        score += 40;
    }

    if entry.source == "start_menu" {
        score += 150;
    }

    if matches!(entry.extension.to_ascii_lowercase().as_str(), "lnk" | "url" | "appref-ms") {
        score += 120;
    }

    Some(score)
}

fn normalize_search_text(value: &str) -> String {
    let mut normalized = String::with_capacity(value.len());
    let mut previous_was_separator = true;

    for ch in value.chars() {
        if ch.is_alphanumeric() {
            for lowered in ch.to_lowercase() {
                normalized.push(lowered);
            }
            previous_was_separator = false;
        } else if !previous_was_separator {
            normalized.push(' ');
            previous_was_separator = true;
        }
    }

    normalized.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::search_files;
    use crate::indexer::FileEntry;
    use std::collections::HashMap;

    #[test]
    fn search_matches_start_menu_apps_by_clean_name() {
        let mut entries = HashMap::new();
        entries.insert(
            "issues".to_string(),
            FileEntry {
                path: r"C:\Users\Test\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Issues.lnk".to_string(),
                name: "Issues".to_string(),
                extension: "lnk".to_string(),
                size: 0,
                modified: 0,
                search_text: "Issues Issues.lnk app launcher installed".to_string(),
                source: "start_menu".to_string(),
            },
        );

        let results = search_files("issues", &entries);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Issues");
    }

    #[test]
    fn search_prefers_start_menu_apps_over_path_only_matches() {
        let mut entries = HashMap::new();
        entries.insert(
            "app".to_string(),
            FileEntry {
                path: r"C:\Users\Test\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Issues.lnk".to_string(),
                name: "Issues".to_string(),
                extension: "lnk".to_string(),
                size: 0,
                modified: 0,
                search_text: "Issues Issues.lnk app launcher installed".to_string(),
                source: "start_menu".to_string(),
            },
        );
        entries.insert(
            "doc".to_string(),
            FileEntry {
                path: r"C:\Users\Test\Documents\tracking-issues.txt".to_string(),
                name: "tracking-issues.txt".to_string(),
                extension: "txt".to_string(),
                size: 0,
                modified: 0,
                search_text: "tracking issues txt".to_string(),
                source: "filesystem".to_string(),
            },
        );

        let results = search_files("issues", &entries);

        assert_eq!(results.first().map(|result| result.name.as_str()), Some("Issues"));
    }
}
