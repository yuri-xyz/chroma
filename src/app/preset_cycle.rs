use anyhow::Result;
use chroma::{params::ShaderParams, presets::PresetCycle};

use super::App;

/// Turns a preset index into fully layered params (preset, config file, CLI overrides).
pub type PresetLoader = Box<dyn Fn(u32) -> Result<ShaderParams>>;

/// `--preset-interval` state: when to switch presets and how to build the next params.
pub struct PresetCycler {
  pub(crate) cycle: PresetCycle,
  pub(crate) loader: PresetLoader,
}

impl App {
  /// Switch to the next preset once the cycle interval has elapsed
  pub(super) fn advance_preset_cycle(&mut self, delta_time: f32) {
    let Some(cycler) = self.preset_cycler.as_mut() else {
      return;
    };
    let Some(index) = cycler.cycle.tick(delta_time) else {
      return;
    };

    let loaded = {
      // Reloads are paused across the switch: one that ran before it is queued
      // by now and dropped below, and one that runs after it sees the new preset.
      // Otherwise a reload loaded over the previous preset could arrive late and
      // bring that preset back.
      let _reloads_paused = self
        .config_watcher
        .as_ref()
        .map(|watcher| watcher.pause_reloads());
      let loaded = (cycler.loader)(index);

      // The loader just re-read the config file, so a queued reload is stale.
      if let (Ok(_), Some(watcher)) = (&loaded, &self.config_watcher) {
        let _ = watcher.try_receive_config();
      }

      loaded
    };

    match loaded {
      Ok(new_params) => {
        self.apply_reloaded_params(new_params);

        let _ = debug_logln!(self.debug_log, "Preset cycle switched to preset {index}");
      }
      Err(error) => {
        let _ = debug_logln!(
          self.debug_log,
          "Preset cycle could not load preset {index}: {error:#}"
        );
      }
    }
  }
}
