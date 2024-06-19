use super::{config, queue::Queue};
use http_types::headers::HeaderValue;
use serde::Deserialize;
use std::net::Ipv4Addr;
use tide::{
  security::{CorsMiddleware, Origin},
  Request, Server, StatusCode,
};

#[derive(Debug, Deserialize)]
struct DownloadRequest {
  url: String,
}

#[derive(Debug, Clone)]
pub struct QueueServerState {
  host: Ipv4Addr,
  port: u16,
  token: String,
  queue: Queue,
}

pub fn new(queue: Queue, config: config::Config) -> Server<QueueServerState> {
  let mut app = tide::with_state(QueueServerState {
    host: config.host,
    port: config.port,
    token: config.token,
    queue,
  });

  app.with(
    CorsMiddleware::new()
      .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
      .allow_origin(Origin::from("*"))
      .allow_credentials(true),
  );

  app.at("/download").post(download_post);

  app
}

async fn download_post(mut req: Request<QueueServerState>) -> tide::Result {
  let DownloadRequest { url } = req.body_json().await?;
  if let Some(bearer) = req.header("Authorization") {
    if bearer.as_str() == format!("Bearer {}", req.state().token) {
      req.state().queue.append(url);
      return Ok(StatusCode::Accepted.into());
    } else {
      return Ok(StatusCode::Unauthorized.into());
    }
  } else {
    return Ok(StatusCode::Unauthorized.into());
  }
}
