use std::{error::Error, fs::read_to_string, path::Path};
use regex::Regex;

pub fn grep_regex_in_file(path: &Path, pattern: &str) -> Result<bool,Box<dyn Error>>  {
    let re = Regex::new(pattern)?;
    let content = read_to_string(path)?;

    for line in content.lines(){
        if re.is_match(line) {
            return Ok(true);
        }
    }

    Ok(false)
}
