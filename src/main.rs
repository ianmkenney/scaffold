mod builder;
mod git;

use builder::{find_builder_path, get_builder_path};
use git::find_git_root;
use std::env::current_dir;

fn main() {
    let gitpath = find_git_root(current_dir().expect("Could not get current working directory"));

    let builderfile = match gitpath {
        None => get_builder_path(&(gitpath.unwrap())),
        Some(gp) => find_builder_path(current_dir().unwrap(), &gp),
    };

    match builderfile {
        Some(path) => println!("Found builder file at: {:}", path.to_str().unwrap()),
        None => println!("Could not find a valid builder file in the project"),
    };
}
