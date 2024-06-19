use config::Config;
use simplelog::{ColorChoice, CombinedLogger, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;

pub mod config;
pub mod queue;
pub mod server;

pub fn setup_logger() {
  let config_folder = Config::get_folder();

  let log_config = simplelog::ConfigBuilder::new()
    .add_filter_allow_str("orpheusdl")
    .build();

  if let Ok(log_file) = File::create(config_folder.join("current.log")) {
    CombinedLogger::init(vec![
      TermLogger::new(
        LevelFilter::Info,
        log_config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
      ),
      WriteLogger::new(LevelFilter::Info, log_config, log_file),
    ])
    .expect("Failed to setup logger");
  } else {
    panic!("Failed to create log file in {:?}", config_folder)
  }
}
