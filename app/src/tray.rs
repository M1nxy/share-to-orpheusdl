#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate log;
extern crate simplelog;

use tokio::time::{sleep, Duration};

use core::{config, logger, queue, server, traymenu};

mod core;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  let config = config::Config::new();

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

  // info!("Shutting down and exiting...");
  // server_handle.abort();
  // queue_handle.abort();

  let result = traymenu::run_tray();

  loop {
    if result.is_ok() {
      info!("Shutting down and exiting...");
      server_handle.abort();
      queue_handle.abort();
    }

    if server_handle.is_finished() || queue_handle.is_finished() {
      break;
    } else {
      sleep(Duration::from_secs(1)).await
    }
  }

  Ok(())
}
