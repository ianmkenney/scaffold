use std::path::PathBuf;

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
