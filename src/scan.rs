use crate::categorize::{Category, categorize, get_summary};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub fn scan(path: &PathBuf) {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(Result::ok)
    {
        let path = entry.path();
        let mut cat = Category::new();
        categorize(path, &mut cat);
        get_summary(&cat);
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
