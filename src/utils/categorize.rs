struct Category {
    package_json: PackageJson,
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

pub fn get_summary() {

} 
