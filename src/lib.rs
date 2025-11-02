use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub fn search(path: &PathBuf) {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(Result::ok)
    {
        let entry = entry.path();
        println!("{}", entry.display())
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
