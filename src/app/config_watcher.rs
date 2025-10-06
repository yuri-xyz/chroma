use anyhow::Result;
use chroma::params::ShaderParams;
use flume::{Receiver, Sender};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct ConfigWatcher {
  _watcher: RecommendedWatcher,
  receiver: Receiver<ShaderParams>,
}

impl ConfigWatcher {
  pub fn new<P: AsRef<Path>>(config_path: P) -> Result<Self> {
    let config_path = config_path.as_ref().to_path_buf();
    let (sender, receiver) = flume::bounded(1);

    let watcher = Self::create_watcher(config_path, sender)?;

    Ok(Self {
      _watcher: watcher,
      receiver,
    })
  }

  fn create_watcher(
    config_path: PathBuf,
    sender: Sender<ShaderParams>,
  ) -> Result<RecommendedWatcher> {
    let watch_path = config_path.clone();
    let config_path = Arc::new(config_path);

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
      if let Ok(event) = res {
        match event.kind {
          EventKind::Modify(_) | EventKind::Create(_) => {
            if let Err(e) = Self::handle_config_change(&config_path, &sender) {
              eprintln!("Config reload error: {}", e);
            }
          }
          _ => {}
        }
      }
    })?;

    watcher.watch(&watch_path, RecursiveMode::NonRecursive)?;

    Ok(watcher)
  }

  fn handle_config_change(config_path: &Path, sender: &Sender<ShaderParams>) -> Result<()> {
    match ShaderParams::load_from_file(config_path) {
      Ok(params) => {
        let _ = sender.try_send(params);
        Ok(())
      }
      Err(_) => Ok(()),
    }
  }

  pub fn try_receive_config(&self) -> Option<ShaderParams> {
    self.receiver.try_recv().ok()
  }
}
