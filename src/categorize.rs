use std::path::Path;

pub struct Category {
    package_json: PackageJson,
}
impl Category {
    pub fn new() -> Self {
        let package_json = PackageJson {
            exist: false,
            node: false,
            express: false,
            react: false,
            svelte: false,
        };
        return Self {
            package_json: package_json,
        };
    }
}

struct PackageJson {
    exist: bool,
    node: bool,
    express: bool,
    react: bool,
    svelte: bool,
}

struct Summary {
    project_type: String,
    languages: String,
    main_entry: String,
    database: String,
    dockerfile: bool,
}

pub fn get_summary(cat: &Category) {
    if cat.package_json.exist{
        println!("package.json file detected")
    }
    if cat.package_json.svelte {
        println!("svelte file detected")
    }
}

pub fn categorize(path: &Path, cat: &mut Category) {
    if path.ends_with("package.json") {
        cat.package_json.exist = true
    }

    if let Some(extension) = path.extension() {
        match extension.to_string_lossy().as_ref() {
            "svelte" => cat.package_json.svelte = true,
            _ => {},
        }
    }
}
