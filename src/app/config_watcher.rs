use std::{
  path::{Path, PathBuf},
  sync::Arc,
};

use anyhow::Result;
use chroma::{debug::append_debug_line, params::ShaderParams};
use flume::{Receiver, Sender, TrySendError};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

pub struct ConfigWatcher {
  _watcher: RecommendedWatcher,
  receiver: Receiver<ShaderParams>,
}

impl ConfigWatcher {
  pub fn new<P: AsRef<Path>>(config_path: P) -> Result<Self> {
    let config_path = config_path.as_ref().to_path_buf();
    let (sender, receiver) = flume::bounded(1);

    let watcher = Self::create_watcher(config_path, sender, receiver.clone())?;

    Ok(Self {
      _watcher: watcher,
      receiver,
    })
  }

  fn create_watcher(
    config_path: PathBuf,
    sender: Sender<ShaderParams>,
    receiver: Receiver<ShaderParams>,
  ) -> Result<RecommendedWatcher> {
    let watch_path = Self::watch_path_for(&config_path);
    let config_path = Arc::new(config_path);

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
      if let Ok(event) = res {
        if Self::event_affects_config(&event, &config_path) {
          if let Err(e) = Self::handle_config_change(&config_path, &sender, &receiver) {
            append_debug_line("config", format!("Config reload error: {}", e));
          }
        }
      }
    })?;

    watcher.watch(&watch_path, RecursiveMode::NonRecursive)?;

    Ok(watcher)
  }

  fn watch_path_for(config_path: &Path) -> PathBuf {
    config_path
      .parent()
      .filter(|parent| !parent.as_os_str().is_empty())
      .unwrap_or_else(|| Path::new("."))
      .to_path_buf()
  }

  fn event_affects_config(event: &Event, config_path: &Path) -> bool {
    if !matches!(
      event.kind,
      EventKind::Any | EventKind::Modify(_) | EventKind::Create(_)
    ) {
      return false;
    }

    event.paths.is_empty()
      || event
        .paths
        .iter()
        .any(|event_path| Self::path_matches_config(event_path, config_path))
  }

  fn path_matches_config(event_path: &Path, config_path: &Path) -> bool {
    event_path == config_path
      || event_path.file_name().is_some() && event_path.file_name() == config_path.file_name()
  }

  fn handle_config_change(
    config_path: &Path,
    sender: &Sender<ShaderParams>,
    receiver: &Receiver<ShaderParams>,
  ) -> Result<()> {
    match ShaderParams::load_from_file(config_path) {
      Ok(params) => {
        match sender.try_send(params) {
          Ok(()) => {}
          Err(TrySendError::Full(params)) => {
            let _ = receiver.try_recv();
            let _ = sender.try_send(params);
          }
          Err(TrySendError::Disconnected(_)) => {}
        }
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
  use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
  };

  use super::*;

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

    ConfigWatcher::handle_config_change(&path, &sender, &receiver).unwrap();

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

    ConfigWatcher::handle_config_change(&path, &sender, &receiver).unwrap();

    assert!(receiver.try_recv().is_err());

    let _ = fs::remove_file(path);
  }

  #[test]
  fn test_handle_config_change_ignores_missing_file() {
    let path = unique_test_path("missing-config");
    let (sender, receiver) = flume::bounded(1);

    ConfigWatcher::handle_config_change(&path, &sender, &receiver).unwrap();

    assert!(receiver.try_recv().is_err());
  }

  #[test]
  fn test_handle_config_change_replaces_pending_config_when_channel_is_full() {
    let path = unique_test_path("full-channel-config");
    let params = ShaderParams {
      frequency: 9.5,
      ..ShaderParams::default()
    };
    let config_text = toml::to_string(&params).unwrap();
    let (sender, receiver) = flume::bounded(1);

    fs::write(&path, config_text).unwrap();
    sender.try_send(ShaderParams::default()).unwrap();

    ConfigWatcher::handle_config_change(&path, &sender, &receiver).unwrap();

    let received = receiver
      .try_recv()
      .expect("expected newest config to replace pending config");
    assert_eq!(received.frequency, params.frequency);

    let _ = fs::remove_file(path);
  }

  #[test]
  fn test_watch_path_for_uses_parent_directory() {
    assert_eq!(
      ConfigWatcher::watch_path_for(Path::new("configs/chroma.toml")),
      PathBuf::from("configs")
    );
    assert_eq!(
      ConfigWatcher::watch_path_for(Path::new("chroma.toml")),
      PathBuf::from(".")
    );
  }

  #[test]
  fn test_event_affects_config_matches_config_file_events() {
    let config_path = PathBuf::from("/tmp/chroma/config.toml");
    let event = Event::new(EventKind::Create(notify::event::CreateKind::File))
      .add_path(PathBuf::from("/tmp/chroma/config.toml"));

    assert!(ConfigWatcher::event_affects_config(&event, &config_path));
  }

  #[test]
  fn test_event_affects_config_matches_save_by_rename_target() {
    let config_path = PathBuf::from("/tmp/chroma/config.toml");
    let event = Event::new(EventKind::Modify(notify::event::ModifyKind::Name(
      notify::event::RenameMode::Both,
    )))
    .add_path(PathBuf::from("/tmp/chroma/.config.toml.swp"))
    .add_path(PathBuf::from("/tmp/chroma/config.toml"));

    assert!(ConfigWatcher::event_affects_config(&event, &config_path));
  }

  #[test]
  fn test_event_affects_config_ignores_other_files() {
    let config_path = PathBuf::from("/tmp/chroma/config.toml");
    let event = Event::new(EventKind::Create(notify::event::CreateKind::File))
      .add_path(PathBuf::from("/tmp/chroma/other.toml"));

    assert!(!ConfigWatcher::event_affects_config(&event, &config_path));
  }

  #[test]
  fn test_event_affects_config_accepts_pathless_mutation_events() {
    let config_path = PathBuf::from("/tmp/chroma/config.toml");
    let event = Event::new(EventKind::Modify(notify::event::ModifyKind::Any));

    assert!(ConfigWatcher::event_affects_config(&event, &config_path));
  }
}
