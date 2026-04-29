use anyhow::Result;
use chroma::debug::append_debug_line;
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
              append_debug_line("config", format!("Config reload error: {}", e));
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

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;
  use std::time::{SystemTime, UNIX_EPOCH};

  fn unique_test_path(name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos();

    std::env::temp_dir().join(format!("chroma-{name}-{timestamp}.toml"))
  }

  #[test]
  fn test_handle_config_change_sends_loaded_params() {
    let path = unique_test_path("valid-config");
    let expected = ShaderParams {
      frequency: 12.5,
      brightness: 0.8,
      ..ShaderParams::default()
    };
    let config_text = toml::to_string(&expected).unwrap();
    let (sender, receiver) = flume::bounded(1);

    fs::write(&path, config_text).unwrap();

    ConfigWatcher::handle_config_change(&path, &sender).unwrap();

    let received = receiver.try_recv().expect("expected config to be sent");
    assert_eq!(received.frequency, expected.frequency);
    assert_eq!(received.brightness, expected.brightness);

    let _ = fs::remove_file(path);
  }

  #[test]
  fn test_handle_config_change_ignores_invalid_toml() {
    let path = unique_test_path("invalid-config");
    let (sender, receiver) = flume::bounded(1);

    fs::write(&path, "not = [valid").unwrap();

    ConfigWatcher::handle_config_change(&path, &sender).unwrap();

    assert!(receiver.try_recv().is_err());

    let _ = fs::remove_file(path);
  }

  #[test]
  fn test_handle_config_change_ignores_missing_file() {
    let path = unique_test_path("missing-config");
    let (sender, receiver) = flume::bounded(1);

    ConfigWatcher::handle_config_change(&path, &sender).unwrap();

    assert!(receiver.try_recv().is_err());
  }

  #[test]
  fn test_handle_config_change_does_not_error_when_channel_is_full() {
    let path = unique_test_path("full-channel-config");
    let params = ShaderParams {
      frequency: 9.5,
      ..ShaderParams::default()
    };
    let config_text = toml::to_string(&params).unwrap();
    let (sender, receiver) = flume::bounded(1);

    fs::write(&path, config_text).unwrap();
    sender.try_send(ShaderParams::default()).unwrap();

    ConfigWatcher::handle_config_change(&path, &sender).unwrap();

    let received = receiver
      .try_recv()
      .expect("expected existing config to remain");
    assert_eq!(received.frequency, ShaderParams::default().frequency);

    let _ = fs::remove_file(path);
  }
}
