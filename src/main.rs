use std::env::current_dir;
use std::path::PathBuf;

fn find_git_root(mut path: PathBuf) -> Option<PathBuf> {
    // Check if current path is a valid git path
    if !get_git_path(&path).is_none() {
        return Some(path.clone());
    } else {
        // Keep popping until we find a valid path
        // or fail to pop anymore
        while path.pop() {
            if !get_git_path(&path).is_none() {
                return Some(path.clone());
            }
        }
    }
    // Not able to find a valid git path
    None
}

fn get_git_path(path: &PathBuf) -> Option<PathBuf> {
    // construct a new git path from a supplied one
    let newpath = path.join(PathBuf::from(".git")).join(PathBuf::from("HEAD"));

    match newpath.exists() {
        true => Some(newpath),
        false => None,
    }
}

fn main() {
    let gitroot = find_git_root(current_dir().unwrap());
    println!("{:?}", gitroot);
}
