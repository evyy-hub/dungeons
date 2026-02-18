use std::{env, path::PathBuf};

fn config_path() -> PathBuf {
    env::current_exe().unwrap().with_file_name("config")
}

fn create_config_path(file_name: &str) -> PathBuf {
    config_path().join(file_name)
}

pub fn keys_config_path() -> PathBuf {
    create_config_path("keys.toml")
}

pub fn player_config_path() -> PathBuf {
    create_config_path("player.toml")
}
