use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub modified: u64,
    #[serde(default)]
    pub search_text: String,
    #[serde(default)]
    pub source: String,
}

pub struct FileIndex {
    entries: HashMap<String, FileEntry>,
    is_indexing: bool,
    total_files: usize,
}

impl FileIndex {
    pub fn new() -> Self {
        let mut index = FileIndex {
            entries: HashMap::new(),
            is_indexing: false,
            total_files: 0,
        };
        // Try to load cache immediately
        index.load_from_disk();
        index
    }

    fn get_cache_path() -> PathBuf {
        let mut path = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("aksha");
        fs::create_dir_all(&path).ok();
        path.push("index.json");
        path
    }

    fn get_legacy_cache_path() -> PathBuf {
        let mut path = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("glasslight");
        path.push("index.json");
        path
    }

    pub fn save_to_disk(&self) {
        let path = Self::get_cache_path();
        if let Ok(json) = serde_json::to_string(&self.entries) {
            let _ = fs::write(path, json);
        }
    }

    pub fn load_from_disk(&mut self) {
        for path in [Self::get_cache_path(), Self::get_legacy_cache_path()] {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(entries) = serde_json::from_str::<HashMap<String, FileEntry>>(&content) {
                        self.entries = entries;
                        self.total_files = self.entries.len();
                        break;
                    }
                }
            }
        }
    }

    pub fn update_entries(&mut self, new_entries: HashMap<String, FileEntry>) {
        self.entries = new_entries;
        self.total_files = self.entries.len();
        self.save_to_disk();
    }

    pub fn get_entries(&self) -> &HashMap<String, FileEntry> {
        &self.entries
    }

    pub fn is_indexing(&self) -> bool {
        self.is_indexing
    }

    pub fn set_indexing(&mut self, status: bool) {
        self.is_indexing = status;
    }

    pub fn total_files(&self) -> usize {
        self.total_files
    }
}

#[derive(Clone, Copy)]
enum EntrySource {
    Filesystem,
    StartMenu,
}

impl EntrySource {
    fn as_str(self) -> &'static str {
        match self {
            Self::Filesystem => "filesystem",
            Self::StartMenu => "start_menu",
        }
    }
}

// Standalone function to scan files without holding the struct lock
pub fn scan_system_files() -> HashMap<String, FileEntry> {
    let mut entries = HashMap::new();
    
    // Standard User Directories
    let dirs_to_index = [
        dirs::document_dir(),
        dirs::desktop_dir(),
        dirs::download_dir(),
        dirs::picture_dir(),
        dirs::video_dir(),
        dirs::audio_dir(),
    ];

    for dir in dirs_to_index.iter().flatten() {
        index_directory_into(dir, &mut entries, EntrySource::Filesystem);
    }

    // Windows Start Menu Programs (User & Common)
    #[cfg(target_os = "windows")]
    {
        if let Ok(app_data) = std::env::var("APPDATA") {
            let user_start_menu = PathBuf::from(app_data).join(r"Microsoft\Windows\Start Menu\Programs");
            if user_start_menu.exists() {
                index_directory_into(&user_start_menu, &mut entries, EntrySource::StartMenu);
            }
        }

        if let Ok(program_data) = std::env::var("PROGRAMDATA") {
            let common_start_menu = PathBuf::from(program_data).join(r"Microsoft\Windows\Start Menu\Programs");
            if common_start_menu.exists() {
                index_directory_into(&common_start_menu, &mut entries, EntrySource::StartMenu);
            }
        }
    }

    entries
}

fn index_directory_into(path: &Path, entries: &mut HashMap<String, FileEntry>, source: EntrySource) {
    for entry in WalkDir::new(path)
        .follow_links(false)
        .max_depth(10)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let is_file = entry.file_type().is_file();
        let is_dir = entry.file_type().is_dir();

        if is_file || is_dir {
            if let Ok(metadata) = entry.metadata() {
                let path_str = entry.path().to_string_lossy().to_string();
                let raw_name = entry.file_name().to_string_lossy().to_string();
                let extension = if is_dir {
                    "folder".to_string()
                } else {
                    entry.path()
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_string()
                };
                let name = build_display_name(entry.path(), &raw_name, &extension, is_dir);
                let search_text = build_search_text(entry.path(), &raw_name, &name, &extension, source);

                let modified = metadata
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);

                let file_entry = FileEntry {
                    path: path_str.clone(),
                    name,
                    extension,
                    size: metadata.len(),
                    modified,
                    search_text,
                    source: source.as_str().to_string(),
                };

                entries.insert(path_str, file_entry);
            }
        }
    }
}

fn build_display_name(path: &Path, raw_name: &str, extension: &str, is_dir: bool) -> String {
    if is_dir {
        return raw_name.to_string();
    }

    if is_launcher_extension(extension) {
        return path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(raw_name)
            .to_string();
    }

    raw_name.to_string()
}

fn build_search_text(
    path: &Path,
    raw_name: &str,
    display_name: &str,
    extension: &str,
    source: EntrySource,
) -> String {
    let mut parts = vec![display_name.to_string(), raw_name.to_string()];

    if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
        if !stem.eq_ignore_ascii_case(display_name) && !stem.eq_ignore_ascii_case(raw_name) {
            parts.push(stem.to_string());
        }
    }

    if let Some(parent) = path.parent() {
        parts.push(parent.to_string_lossy().to_string());
    }

    if matches!(source, EntrySource::StartMenu) {
        parts.push("application app launcher program start menu installed".to_string());
    }

    if extension.eq_ignore_ascii_case("url") {
        parts.push("web app website shortcut".to_string());
    }

    parts.join(" ")
}

fn is_launcher_extension(extension: &str) -> bool {
    matches!(
        extension.to_ascii_lowercase().as_str(),
        "lnk" | "url" | "appref-ms" | "exe" | "msi" | "bat" | "cmd"
    )
}

#[cfg(test)]
mod tests {
    use super::{build_display_name, build_search_text, EntrySource};
    use std::path::Path;

    #[test]
    fn strips_shortcut_extension_from_display_name() {
        let path = Path::new(r"C:\Users\Test\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Issues.lnk");
        let display_name = build_display_name(path, "Issues.lnk", "lnk", false);

        assert_eq!(display_name, "Issues");
    }

    #[test]
    fn start_menu_entries_include_app_terms_in_search_text() {
        let path = Path::new(r"C:\Users\Test\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Issues.lnk");
        let search_text = build_search_text(path, "Issues.lnk", "Issues", "lnk", EntrySource::StartMenu);

        assert!(search_text.contains("installed"));
        assert!(search_text.contains("Issues"));
    }
}
