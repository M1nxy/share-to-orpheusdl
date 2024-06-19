use chrono::Utc;
use std::{
  fs,
  sync::{Arc, Mutex},
  time::Duration,
};

use tokio::{process::Command, time::sleep};

use super::config;

pub struct Queue {
  pub items: Arc<Mutex<Vec<String>>>,
  timeout: Duration,
  token: String,
  orpheusdl_path: String,
  orpheusdl_args: Vec<String>,
  debug: bool,
}

impl Queue {
  pub fn new(config: config::Config) -> Self {
    Self {
      items: Arc::new(Mutex::new(Vec::new())),
      timeout: Duration::from_secs(config.timeout.into()),
      token: config.token,
      orpheusdl_path: config.orpheusdl_path,
      orpheusdl_args: config.orpheusdl_args,
      debug: config.debug,
    }
  }

  pub fn append(&self, url: String) {
    info!("Adding task to queue {}", url);
    let mut items = self.items.lock().unwrap();
    items.push(url);
  }

  pub async fn process_queue(&self) -> Result<(), std::io::Error> {
    loop {
      let current_item = {
        let mut items = self.items.lock().unwrap();
        items.pop()
      };

      if let Some(url) = current_item {
        let now = Utc::now().format("%Y%m%dT%H%M%S");
        let cmd = Command::new("python")
          .current_dir(&self.orpheusdl_path)
          .arg("orpheus.py")
          .arg(&url)
          .args(&self.orpheusdl_args)
          .output()
          .await;

        match cmd {
          Ok(output) => {
            //DownloadManager::notify("Failed to download:", &url).expect("Failed to notify");
            if output.status.success() {
              info!("Processed task {}", url);
              if self.debug {
                let stdout = config::Config::get_folder()
                  .join("logs")
                  .join(format!("{}.debug.log", now));
                fs::write(stdout, output.stdout)?;
              }
            } else {
              error!("Failed to process task {}", url);
              let stderr = config::Config::get_folder()
                .join("logs")
                .join(format!("{}.error.log", now));
              fs::write(stderr, output.stderr)?;
            }
          }
          Err(err) => {
            error!("Failed to run orpheusdl {:?}", err);
          }
        }
      } else {
        sleep(self.timeout).await;
      }
    }
  }
}
