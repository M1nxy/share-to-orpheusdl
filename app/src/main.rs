#[macro_use]
extern crate log;
extern crate simplelog;

use clap::Parser;

use core::{config, queue, server, setup_logger};
use std::net::Ipv4Addr;

mod core;

#[derive(Parser)] // requires `derive` feature
#[command(version, about, long_about = None)]
pub struct Cli {
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
async fn main() {
  let args = Cli::parse();

  let config = config::Config::from(args);

  setup_logger();

  let queue = queue::Queue::new(config.clone());
  let server = server::new(queue.clone(), config.clone());

  let server_handle = tokio::task::spawn(async move {
    info!("Server listening on http://{}:{}", config.host, config.port);
    server
      .listen(format!("{}:{}", config.host, config.port))
      .await
  });

  let queue_handle = tokio::task::spawn(async move { queue.process_queue().await });

  loop {}
}
