mod git;

use git::find_git_root;
use std::env::current_dir;

fn main() {
    let gitroot = find_git_root(current_dir().unwrap());
    println!("{:?}", gitroot);
}
