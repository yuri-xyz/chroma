//! Downward gravity for the shader UV plane, with mouse input that can fight
//! the fall but never cancel it completely.

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

/// Max fraction of gravity acceleration the mouse may cancel (must stay < 1).
const MOUSE_FIGHT_CAP: f32 = 0.75;
/// UV units / s² per unit of the `gravity` parameter.
const GRAVITY_ACCEL_SCALE: f32 = 0.45;
/// Horizontal pull toward the mouse cursor (scaled by gravity).
const HORIZONTAL_PULL: f32 = 0.55;
/// Velocity damping per second (exponential decay coefficient).
const DAMPING: f32 = 2.4;
/// Soft speed clamp in UV units / s.
const MAX_SPEED: f32 = 1.25;
/// How long after last mouse event the cursor still counts as active.
const MOUSE_ACTIVE_SECONDS: f32 = 1.25;

#[derive(Debug, Clone)]
pub struct GravityState {
  pub offset: [f32; 2],
  pub velocity: [f32; 2],
  pub mouse: [f32; 2],
  pub mouse_active: bool,
  pub mouse_pressed: bool,
  mouse_active_remaining: f32,
}

impl Default for GravityState {
  fn default() -> Self {
    Self {
      offset: [0.0, 0.0],
      velocity: [0.0, 0.0],
      mouse: [0.5, 0.5],
      mouse_active: false,
      mouse_pressed: false,
      mouse_active_remaining: 0.0,
    }
  }
}

impl GravityState {
  pub fn set_mouse_position(&mut self, x: f32, y: f32) {
    self.mouse = [x.clamp(0.0, 1.0), y.clamp(0.0, 1.0)];
    self.mouse_active = true;
    self.mouse_active_remaining = MOUSE_ACTIVE_SECONDS;
  }

  pub fn set_mouse_pressed(&mut self, pressed: bool) {
    self.mouse_pressed = pressed;
    if pressed {
      self.mouse_active = true;
      self.mouse_active_remaining = MOUSE_ACTIVE_SECONDS;
    }
  }

  /// Effective mouse influence sent to the shader (0 = idle).
  pub fn shader_mouse_influence(&self, mouse_fight: f32) -> f32 {
    if !self.mouse_active {
      return 0.0;
    }

    let base = mouse_fight.clamp(0.0, 1.0) * MOUSE_FIGHT_CAP;
    if self.mouse_pressed {
      base
    } else {
      base * 0.65
    }
  }

  /// Integrate gravity and mouse forces for one frame.
  ///
  /// Mouse vertical lift is hard-capped so net downward acceleration never
  /// drops below `gravity_accel * (1 - MOUSE_FIGHT_CAP)`.
  pub fn update(&mut self, gravity: f32, mouse_fight: f32, delta_time: f32) {
    let dt = delta_time.clamp(0.0, 0.1);

    if self.mouse_active_remaining > 0.0 {
      self.mouse_active_remaining = (self.mouse_active_remaining - dt).max(0.0);
      if self.mouse_active_remaining <= 0.0 && !self.mouse_pressed {
        self.mouse_active = false;
      }
    }

    let gravity = gravity.max(0.0);
    if gravity <= 0.0001 {
      // Ease residual motion back to rest when gravity is off.
      self.velocity[0] *= (-DAMPING * dt).exp();
      self.velocity[1] *= (-DAMPING * dt).exp();
      self.offset[0] += self.velocity[0] * dt;
      self.offset[1] += self.velocity[1] * dt;
      return;
    }

    let mut accel_x = 0.0;
    let accel_y = net_vertical_accel(
      gravity,
      mouse_fight,
      self.mouse_active,
      self.mouse_pressed,
    );

    if self.mouse_active {
      let press_boost = if self.mouse_pressed { 1.0 } else { 0.65 };
      // Horizontal tug toward the cursor; strength scales with gravity.
      accel_x += (self.mouse[0] - 0.5) * HORIZONTAL_PULL * gravity * press_boost;
    }

    self.velocity[0] += accel_x * dt;
    self.velocity[1] += accel_y * dt;

    let damp = (-DAMPING * dt).exp();
    self.velocity[0] *= damp;
    self.velocity[1] *= damp;

    let speed = (self.velocity[0] * self.velocity[0] + self.velocity[1] * self.velocity[1]).sqrt();
    if speed > MAX_SPEED {
      let scale = MAX_SPEED / speed;
      self.velocity[0] *= scale;
      self.velocity[1] *= scale;
    }

    self.offset[0] += self.velocity[0] * dt;
    self.offset[1] += self.velocity[1] * dt;
  }
}

/// Apply a terminal mouse event to gravity interaction state.
pub fn handle_mouse_event(
  mouse: MouseEvent,
  gravity: &mut GravityState,
  terminal_size: (u16, u16),
) {
  let width = terminal_size.0.max(1) as f32;
  let height = terminal_size.1.max(1) as f32;
  let x = mouse.column as f32 / width;
  let y = mouse.row as f32 / height;

  match mouse.kind {
    MouseEventKind::Down(MouseButton::Left) => {
      gravity.set_mouse_position(x, y);
      gravity.set_mouse_pressed(true);
    }
    MouseEventKind::Up(MouseButton::Left) => {
      gravity.set_mouse_position(x, y);
      gravity.set_mouse_pressed(false);
    }
    MouseEventKind::Drag(MouseButton::Left) | MouseEventKind::Moved => {
      gravity.set_mouse_position(x, y);
    }
    _ => {}
  }
}

/// Net vertical acceleration after mouse fight.
fn net_vertical_accel(
  gravity: f32,
  mouse_fight: f32,
  mouse_active: bool,
  mouse_pressed: bool,
) -> f32 {
  let gravity = gravity.max(0.0);
  let gravity_accel = gravity * GRAVITY_ACCEL_SCALE;
  if gravity_accel <= 0.0 {
    return 0.0;
  }
  if !mouse_active {
    return gravity_accel;
  }

  let fight_strength = mouse_fight.clamp(0.0, 1.0);
  let press_boost = if mouse_pressed { 1.0 } else { 0.65 };
  let requested_fight = gravity_accel * MOUSE_FIGHT_CAP * fight_strength * press_boost;
  let min_net_accel = gravity_accel * (1.0 - MOUSE_FIGHT_CAP);
  (gravity_accel - requested_fight).max(min_net_accel)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn mouse_cannot_overpower_gravity() {
    let gravity = 1.0;
    let with_mouse = net_vertical_accel(gravity, 1.0, true, true);
    let without = net_vertical_accel(gravity, 1.0, false, false);

    assert!(without > 0.0);
    assert!(with_mouse > 0.0);
    assert!(with_mouse < without);
    assert!(
      with_mouse + f32::EPSILON >= without * (1.0 - MOUSE_FIGHT_CAP),
      "mouse fight exceeded cap: {with_mouse} vs floor {}",
      without * (1.0 - MOUSE_FIGHT_CAP)
    );
  }

  #[test]
  fn inactive_mouse_leaves_full_gravity() {
    let accel = net_vertical_accel(0.8, 1.0, false, false);
    assert!((accel - 0.8 * GRAVITY_ACCEL_SCALE).abs() < 1e-5);
  }

  #[test]
  fn update_increases_downward_offset_over_time() {
    let mut state = GravityState::default();
    for _ in 0..30 {
      state.update(1.0, 0.7, 1.0 / 60.0);
    }
    assert!(state.offset[1] > 0.0);
    assert!(state.velocity[1] > 0.0);
  }

  #[test]
  fn pressed_mouse_slows_but_does_not_reverse_fall() {
    let mut falling = GravityState::default();
    let mut fighting = GravityState::default();
    fighting.set_mouse_position(0.5, 0.2);
    fighting.set_mouse_pressed(true);

    for _ in 0..45 {
      falling.update(1.0, 1.0, 1.0 / 60.0);
      fighting.update(1.0, 1.0, 1.0 / 60.0);
    }

    assert!(fighting.offset[1] > 0.0);
    assert!(fighting.offset[1] < falling.offset[1]);
    assert!(fighting.velocity[1] > 0.0);
  }

  #[test]
  fn mouse_move_updates_normalized_position() {
    let mut gravity = GravityState::default();
    handle_mouse_event(
      MouseEvent {
        kind: MouseEventKind::Moved,
        column: 40,
        row: 12,
        modifiers: crossterm::event::KeyModifiers::NONE,
      },
      &mut gravity,
      (80, 24),
    );

    assert!(gravity.mouse_active);
    assert!((gravity.mouse[0] - 0.5).abs() < 1e-5);
    assert!((gravity.mouse[1] - 0.5).abs() < 1e-5);
  }
}
