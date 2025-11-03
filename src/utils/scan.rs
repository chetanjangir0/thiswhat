use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub fn scan(path: &PathBuf) {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if let Some("html") = path.extension().and_then(|s| s.to_str()) {
            println!("html detected")
        }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
