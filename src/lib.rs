use std::path::PathBuf;
use walkdir::WalkDir;

pub fn search(path: &PathBuf) {
    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let entry = entry.path();
        println!("{}", entry.display())
    }
}
