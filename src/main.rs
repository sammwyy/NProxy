use std::{env, path::PathBuf};

use crate::config::config_loader;

pub mod config;

fn get_cwd() -> PathBuf {
    env::current_dir().unwrap()
}

fn main() {
    let dir = get_cwd().join("example_config/config.toml");
    let config = config_loader::read_config_from_file(&dir).unwrap();
    println!("{}", config.servers.unwrap().len());
}
