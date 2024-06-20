use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::Ipv4Addr;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
  pub host: Ipv4Addr,
  pub port: u16,
  pub token: String,
  pub timeout: u8,
  pub debug: bool,
  pub orpheusdl_path: String,
  pub orpheusdl_args: Vec<String>,
}

impl Config {
  pub fn new() -> Config {
    let config_folder = Self::get_folder();
    let config_file = config_folder.join("config.toml");

    // if not exists write defaults
    if !fs::metadata(&config_file).is_ok() {
      let defaults = Config {
        host: Ipv4Addr::new(127, 0, 0, 1),
        port: 9221,
        token: "change_me".to_string(),
        timeout: 1,
        debug: false,
        orpheusdl_path: "change_me".to_string(),
        orpheusdl_args: Vec::new(),
      };
      Self::write_to_file(&defaults);
    }

    // return deserialized config file
    let file_contents: String =
      fs::read_to_string(&config_file).expect("Failed to read config file.");
    toml::from_str(&file_contents).expect("Failed to parse config file.")
  }

  pub fn get_folder() -> PathBuf {
    let base_dirs = BaseDirs::new().expect("Unable to find user folders.");
    let config_folder = base_dirs.config_dir().join(env!("CARGO_PKG_NAME"));
    if !fs::metadata(&config_folder).is_ok() {
      fs::create_dir(&config_folder).expect("Unable to create config folder.");
    }
    if !fs::metadata(&config_folder.join("logs")).is_ok() {
      fs::create_dir(&config_folder.join("logs")).expect("Unable to create logs folder.");
    }
    config_folder
  }

  pub fn write_to_file(config: &Config) {
    let config_file = Self::get_folder().join("config.toml");
    let toml = toml::to_string(&config).unwrap();
    // write serialized config to file
    if fs::write(&config_file, toml).is_err() {
      panic!("Failed to save config to {:?}", &config_file);
    }
  }
}
