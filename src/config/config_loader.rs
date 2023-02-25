use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use thiserror::Error;

use super::{config::Config, default_config::default_config};

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("{0}")]
    IO(#[from] std::io::Error),
}

pub fn merge_configs(mut first: Config, second: Config) -> Config {
    if first.servers.is_none() {
        first.servers = second.servers;
    } else if second.servers.is_some() {
        let mut first_servers = first.servers.clone().unwrap();
        let mut second_servers = second.servers.clone().unwrap();

        Vec::append(&mut first_servers, &mut second_servers);
        first.servers = Some(first_servers);
    }

    return first;
}

pub fn read_config_from_str(string: &str) -> Result<Config, ConfigError> {
    let config: Config = toml::from_str(string).unwrap();
    return Ok(config);
}

pub fn read_config_from_file(file: &PathBuf) -> Result<Config, ConfigError> {
    let root = file.parent().unwrap();

    let raw = fs::read_to_string(file)?;
    let mut config: Config = read_config_from_str(&raw)?;
    let includes = config.include.to_owned();

    if config.include.is_some() {
        let includes = includes.unwrap();

        for include in includes.iter() {
            if include.ends_with("*") {
                let dir_files = root.join(include).parent().unwrap().read_dir().unwrap();

                for dir_file in dir_files {
                    let dir_data = dir_file.unwrap();
                    let child_file = dir_data.path();
                    let child = read_config_from_file(&child_file).unwrap();
                    config = merge_configs(config, child);
                }
            } else {
                let child_file = root.join(include);
                let child = read_config_from_file(&child_file).unwrap();
                config = merge_configs(config, child);
            }
        }
    }

    return Ok(config);
}

pub fn config_to_str(config: Config) -> String {
    let raw = toml::to_string(&config).unwrap();
    return raw;
}

pub fn create_config_if_not_exist(cwd: PathBuf) {
    let file_path = cwd.join("config.toml");

    if !file_path.exists() {
        let def = default_config();
        let raw = config_to_str(def);

        let mut file = File::create(file_path).unwrap();
        file.write_all(raw.as_bytes()).unwrap();
    }
}
