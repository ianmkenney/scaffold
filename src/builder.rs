use std::fs;
use std::path::PathBuf;

use toml::Table;

pub fn get_builder_path(path: &PathBuf) -> Option<PathBuf> {
    // Check for a builder file in the current directory
    let builder_file = path.join(".builder");

    if builder_file.exists() {
        return Some(builder_file);
    }
    None
}

pub fn find_builder_path(mut current_dir: PathBuf, stop_path: &PathBuf) -> Option<PathBuf> {
    // Recursive search up until stop_path for a builder file
    match get_builder_path(&current_dir) {
        Some(path) => Some(path),
        None => {
            if current_dir == *stop_path {
                return None;
            } else {
                // move one directory up
                current_dir.pop();
                return find_builder_path(current_dir, stop_path);
            }
        }
    }
}

pub fn read_builder_file(filepath: &PathBuf) -> Result<Table, String> {
    // read contents from file in as a string and check for errors while
    // unwrapping
    let contents = match fs::read_to_string(filepath.to_str().unwrap()) {
        Ok(c) => c,
        Err(_) => {
            return Err("could not read file".to_string());
        }
    };

    // parse the TOML contents from the string and unwrap
    let parsed = match contents.parse::<Table>() {
        Ok(c) => c,
        Err(_) => {
            return Err("could not parse TOML file, check for formatting errors".to_string());
        }
    };

    Ok(parsed)
}
