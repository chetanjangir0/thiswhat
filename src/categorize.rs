use std::path::Path;

use crate::grep::grep_regex_in_file;

pub struct Category {
    package_json: Option<PackageJson>,
}
impl Category {
    pub fn new() -> Self {
        return Self {
            package_json: None,
        };
    }
}

enum PackageJson {
    Node,
    Express,
    React,
    Svelte,
}

struct Summary {
    project_type: String,
    languages: String,
    main_entry: String,
    database: String,
    dockerfile: bool,
}

pub fn get_summary(cat: &Category) {
    match cat.package_json {
        Some(PackageJson::Node) => println!("node project detected"),
        Some(PackageJson::Express) => println!("express project detected"),
        Some(PackageJson::React) => println!("react project detected"),
        Some(PackageJson::Svelte) => println!("svelte project detected"),
        _ => println!("package.json file detected"),
    }
}

pub fn categorize(path: &Path, cat: &mut Category) {
    if path.ends_with("package.json") {
        if let Ok(true) = grep_regex_in_file(path, r#""main"\s*:\s*"[\w\-./]+\.js"\s*,"#) {
            cat.package_json = Some(PackageJson::Node);
        }
    }

    if let Some(extension) = path.extension() {
        match extension.to_string_lossy().as_ref() {
            "svelte" => cat.package_json = Some(PackageJson::Svelte),
            _ => {}
        }
    }
}
