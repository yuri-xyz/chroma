use std::{
  path::{Path, PathBuf},
  sync::{Arc, Mutex, MutexGuard, PoisonError},
};

use anyhow::Result;
use chroma::{debug::append_debug_line, params::ShaderParams};
use flume::{Receiver, Sender, TrySendError};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

/// Turns the config file into fully layered params (preset, file, CLI overrides).
pub type ConfigLoader = Arc<dyn Fn(&Path) -> Result<ShaderParams> + Send + Sync>;

pub struct ConfigWatcher {
  _watcher: RecommendedWatcher,
  receiver: Receiver<ShaderParams>,
  reload_gate: Arc<Mutex<()>>,
}

impl ConfigWatcher {
  pub fn new<P: AsRef<Path>>(config_path: P, loader: ConfigLoader) -> Result<Self> {
    let config_path = config_path.as_ref().to_path_buf();
    let (sender, receiver) = flume::bounded(1);
    let reload_gate = Arc::new(Mutex::new(()));

    let watcher = Self::create_watcher(
      config_path,
      loader,
      sender,
      receiver.clone(),
      Arc::clone(&reload_gate),
    )?;

    Ok(Self {
      _watcher: watcher,
      receiver,
      reload_gate,
    })
  }

  /// Hold off config reloads until the guard drops. A reload is loaded and
  /// queued as one step under this gate, so a caller that changes what reloads
  /// layer over can discard the queued one knowing no stale reload is in flight.
  pub fn pause_reloads(&self) -> MutexGuard<'_, ()> {
    Self::lock_gate(&self.reload_gate)
  }

  // The gate guards no data, so a panic while it was held leaves nothing to distrust.
  fn lock_gate(reload_gate: &Mutex<()>) -> MutexGuard<'_, ()> {
    reload_gate.lock().unwrap_or_else(PoisonError::into_inner)
  }

  fn create_watcher(
    config_path: PathBuf,
    loader: ConfigLoader,
    sender: Sender<ShaderParams>,
    receiver: Receiver<ShaderParams>,
    reload_gate: Arc<Mutex<()>>,
  ) -> Result<RecommendedWatcher> {
    let watch_path = Self::watch_path_for(&config_path);
    let config_path = Arc::new(config_path);

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
      if let Ok(event) = res {
        if Self::event_affects_config(&event, &config_path) {
          Self::handle_config_change(&config_path, &loader, &sender, &receiver, &reload_gate);
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
    loader: &ConfigLoader,
    sender: &Sender<ShaderParams>,
    receiver: &Receiver<ShaderParams>,
    reload_gate: &Mutex<()>,
  ) {
    let _reload_gate = Self::lock_gate(reload_gate);

    // Invalid or half-written configs keep the current params running.
    let params = match loader(config_path) {
      Ok(params) => params,
      Err(error) => {
        append_debug_line("config", format!("Config reload skipped: {error:#}"));
        return;
      }
    };

    match sender.try_send(params) {
      Ok(()) => {}
      Err(TrySendError::Full(params)) => {
        let _ = receiver.try_recv();
        let _ = sender.try_send(params);
      }
      Err(TrySendError::Disconnected(_)) => {}
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

  fn file_loader() -> ConfigLoader {
    Arc::new(|path: &Path| ShaderParams::load_from_file(path))
  }

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

    ConfigWatcher::handle_config_change(&path, &file_loader(), &sender, &receiver, &Mutex::new(()));

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

    ConfigWatcher::handle_config_change(&path, &file_loader(), &sender, &receiver, &Mutex::new(()));

    assert!(receiver.try_recv().is_err());

    let _ = fs::remove_file(path);
  }

  #[test]
  fn test_handle_config_change_ignores_missing_file() {
    let path = unique_test_path("missing-config");
    let (sender, receiver) = flume::bounded(1);

    ConfigWatcher::handle_config_change(&path, &file_loader(), &sender, &receiver, &Mutex::new(()));

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

    ConfigWatcher::handle_config_change(&path, &file_loader(), &sender, &receiver, &Mutex::new(()));

    let received = receiver
      .try_recv()
      .expect("expected newest config to replace pending config");
    assert_eq!(received.frequency, params.frequency);

    let _ = fs::remove_file(path);
  }

  #[test]
  fn test_handle_config_change_uses_layered_loader() {
    let path = unique_test_path("layered-config");
    let base = ShaderParams {
      frequency: 15.0,
      bass_influence: 0.9,
      ..ShaderParams::default()
    };
    let loader: ConfigLoader = Arc::new(move |path: &Path| {
      let mut params = ShaderParams::load_from_file_over(path, &base)?;
      params.bass_influence = 0.25;
      Ok(params)
    });
    let (sender, receiver) = flume::bounded(1);

    fs::write(&path, "brightness = 0.9\n").unwrap();

    ConfigWatcher::handle_config_change(&path, &loader, &sender, &receiver, &Mutex::new(()));

    let received = receiver.try_recv().expect("expected config to be sent");
    assert_eq!(received.brightness, 0.9);
    assert_eq!(received.frequency, 15.0);
    assert_eq!(received.bass_influence, 0.25);

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

  #[test]
  fn test_handle_config_change_waits_while_reloads_are_paused() {
    let path = unique_test_path("paused-config");
    let (sender, receiver) = flume::bounded(1);
    let reload_gate = Arc::new(Mutex::new(()));

    fs::write(&path, "frequency = 12.0\n").unwrap();

    let paused = ConfigWatcher::lock_gate(&reload_gate);
    let reload = std::thread::spawn({
      let path = path.clone();
      let receiver = receiver.clone();
      let reload_gate = Arc::clone(&reload_gate);

      move || {
        ConfigWatcher::handle_config_change(&path, &file_loader(), &sender, &receiver, &reload_gate)
      }
    });

    // Nothing may be loaded or queued until the gate is released.
    std::thread::sleep(std::time::Duration::from_millis(50));
    assert!(receiver.try_recv().is_err());

    drop(paused);
    reload.join().unwrap();

    assert_eq!(receiver.try_recv().unwrap().frequency, 12.0);

    let _ = fs::remove_file(path);
  }
}
