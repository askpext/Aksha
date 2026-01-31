use crate::indexer::FileIndex;
use crate::search::{search_files as search_impl, SearchResult};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStatus {
    pub total_files: usize,
    pub is_indexing: bool,
}

#[tauri::command]
pub fn search_files(query: String, index: State<Arc<Mutex<FileIndex>>>) -> Vec<SearchResult> {
    if let Ok(idx) = index.lock() {
        let entries = idx.get_entries();
        search_impl(&query, entries)
    } else {
        Vec::new()
    }
}

#[tauri::command]
pub fn open_file(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }


    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn show_in_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .args(["/select,", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        // Linux file managers vary, tried to use dbus or generic approach
        // simplified: just open parent dir
        if let Some(parent) = std::path::Path::new(&path).parent() {
            std::process::Command::new("xdg-open")
                .arg(parent)
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
pub fn get_index_status(index: State<Arc<Mutex<FileIndex>>>) -> IndexStatus {
    if let Ok(idx) = index.lock() {
        IndexStatus {
            total_files: idx.total_files(),
            is_indexing: idx.is_indexing(),
        }
    } else {
        IndexStatus {
            total_files: 0,
            is_indexing: false,
        }
    }
}
