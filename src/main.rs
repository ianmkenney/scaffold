mod builder;
mod git;

use builder::{find_builder_path, get_builder_path};
use git::find_git_root;
use std::{env::current_dir, process::exit};
use toml::Table;

use crate::builder::read_builder_file;

fn main() {
    let gitpath = find_git_root(current_dir().expect("Could not get current working directory"));

    let builderfile = match gitpath {
        None => get_builder_path(&(gitpath.unwrap())),
        Some(gp) => find_builder_path(current_dir().unwrap(), &gp),
    };

    let contents = if let Some(path) = builderfile {
        match read_builder_file(&path) {
            Ok(table) => table,
            Err(e) => {
                println!("Couldn't process file: {:}", e);
                exit(1);
            }
        }
    } else {
        Table::new()
    };

    println!("{:}", contents);
}
