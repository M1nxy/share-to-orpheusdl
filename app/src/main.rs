#[macro_use]
extern crate log;
extern crate simplelog;

use clap::Parser;
use std::net::Ipv4Addr;
use tokio::time::{sleep, Duration};

use core::{config, logger, queue, server};

mod core;

#[derive(Parser)] // requires `derive` feature
#[command(version, about, long_about = None)]
pub struct CliConfig {
  #[arg(long)]
  host: Option<Ipv4Addr>,

  #[arg(long)]
  port: Option<u16>,

  #[arg(long)]
  token: Option<String>,

  #[arg(long)]
  timeout: Option<u8>,

  #[arg(long = "path")]
  orpheusdl_path: Option<String>,

  #[arg(long = "debug", default_value_t = false)]
  debug: bool,

  #[arg(long = "save")]
  save_to_file: bool,

  #[arg(last = true)]
  orpheusdl_args: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let cli_config = CliConfig::parse();

  let config = config::Config::from(cli_config);

  logger::setup(config.debug);

  let queue = queue::Queue::new(config.clone());
  let server = server::new(queue.clone(), config.clone());

  let server_handle = tokio::task::spawn(async move {
    info!(
      "Starting server listening on http://{}:{}",
      config.host, config.port
    );
    let server = server
      .listen(format!("{}:{}", config.host, config.port))
      .await;
    if (server.is_err()) {
      error!(
        "Stopping server listening on http://{}:{}",
        config.host, config.port
      );
    }
  });

  let queue_handle = tokio::task::spawn(async move { queue.process_queue().await });

  tokio::signal::ctrl_c()
    .await
    .expect("failed to listen for keyboard events");

  info!("Shutting down and exiting...");

  server_handle.abort();
  queue_handle.abort();

  loop {
    if server_handle.is_finished() || queue_handle.is_finished() {
      break;
    } else {
      sleep(Duration::from_secs(1)).await
    }
  }

  Ok(())
}

impl From<CliConfig> for config::Config {
  fn from(args: CliConfig) -> Self {
    // get base config file. Also creates file if not exist.
    let base = config::Config::new();

    // replace missing args with value from base config.
    let config = Self {
      host: args.host.unwrap_or(base.host),
      port: args.port.unwrap_or(base.port),
      token: args.token.unwrap_or(base.token),
      timeout: args.timeout.unwrap_or(base.timeout),
      debug: args.debug,
      orpheusdl_path: args.orpheusdl_path.unwrap_or(base.orpheusdl_path),
      orpheusdl_args: args.orpheusdl_args,
    };

    // save if specified
    if args.save_to_file {
      Self::write_to_file(&config)
    }

    // return merged config
    config
  }
}
