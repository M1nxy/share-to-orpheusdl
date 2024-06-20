use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;

use super::config::Config;

pub fn setup(debug_enabled: bool) {
  let config_folder = Config::get_folder();

  let log_config = simplelog::ConfigBuilder::new()
    .add_filter_allow_str("orpheusdl")
    .set_time_offset_to_local()
    .unwrap()
    .build();

  let logging_level = if debug_enabled {
    LevelFilter::Info
  } else {
    LevelFilter::Debug
  };

  if let Ok(log_file) = File::create(config_folder.join("current.log")) {
    CombinedLogger::init(vec![
      TermLogger::new(
        logging_level,
        log_config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
      ),
      WriteLogger::new(logging_level, log_config, log_file),
    ])
    .expect("Failed to setup logger");
  } else {
    panic!("Failed to create log file in {:?}", config_folder)
  }
}
