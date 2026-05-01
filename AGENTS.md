# Chroma Agent and Contributor Guide

Chroma is a Rust terminal audio visualizer. It renders GPU-generated WGSL shader output as colored ASCII and can react to system audio. The source code is the authority for behavior; treat docs and examples as helpful context that may lag behind implementation.

`CONTRIBUTING.md` and `CLAUDE.md` are symlinks to this file. Keep this file as the single source of truth for contributor and agent guidance.

## First Places To Check

- `src/main.rs` and `src/app/` for application startup, the render loop, input handling, config reload, status bar behavior, and audio-reactive updates
- `src/params/` for user-facing parameters, enum mappings, defaults, clamping, serialization, and CLI/config conversion behavior
- `src/shader_common/` and `src/shader_patterns/` for WGSL uniforms, effects, pattern dispatch, and shader pattern implementations
- `src/audio/` for PulseAudio/PipeWire and CPAL capture, device selection, buffering, and FFT analysis
- `src/ascii/` and `src/render/` for RGBA-to-ASCII conversion and terminal frame construction
- `notes/` and `README.md` for user-facing docs that should be updated when behavior changes

## Current Facts

- The default target frame rate is `60` FPS (`DEFAULT_FPS` in `src/constants.rs`) and can be overridden with `--fps`.
- Audio support is built in. Linux uses PulseAudio/PipeWire monitor capture through libpulse when available, then falls back to CPAL.
- Linux runtime/package dependencies include Vulkan loader support, ALSA, and PulseAudio libraries.
- Stream mode is selected with `--stream WIDTHxHEIGHT`; it disables terminal setup, status bar, and interactive input, and emits full frames to stdout.
- Built-in presets are numbered `0..24` and live in `src/presets/`.
- `CLAUDE.md` and `CONTRIBUTING.md` should remain symlinks to `AGENTS.md`.

## Runtime Output Rule

Do not add `println!`, `eprintln!`, `dbg!`, or direct writes to stdout/stderr in runtime paths that can execute while the visualizer is rendering. Stdout is reserved for rendered frames and explicit list/stream commands. Extra output can corrupt the terminal UI and reduce FPS.

Use file-backed logging instead:

- `debug_logln!(debug_log, ...)` when an app-owned `DebugLog` is available
- `append_debug_line(component, message)` from library or background-thread code that cannot access `DebugLog`
- `CHROMA_TRACE_FRAMES=1` only when intentionally enabling per-frame debug logs guarded by `frame_logging_enabled()`

User-facing terminal output is acceptable only for commands that exit before rendering starts, such as `--list-audio-devices`, `--list-patterns`, `--list-color-modes`, and `--list-palettes`.

## Validation Commands

Prefer the Nix dev shell because it supplies the native audio libraries and the nightly rustfmt required by this repo's rustfmt options:

```bash
nix --extra-experimental-features 'nix-command flakes' develop -c cargo fmt --check
nix --extra-experimental-features 'nix-command flakes' develop -c cargo test
nix --extra-experimental-features 'nix-command flakes' develop -c cargo clippy --all-targets -- -D warnings
nix --extra-experimental-features 'nix-command flakes' develop -c actionlint -color
```

Useful focused checks:

```bash
nix --extra-experimental-features 'nix-command flakes' develop -c cargo test --test shader_pattern_position_test
nix --extra-experimental-features 'nix-command flakes' develop -c cargo test --test render_test test_shader_produces_non_zero_output -- --ignored --exact
```

Running `cargo test` outside the dev shell may fail to link Linux PulseAudio libraries even when the Rust code is correct.

## Implementation Guidelines

- Keep changes scoped to the behavior being fixed. Avoid unrelated refactors, formatting churn, or doc rewrites unless they directly support the change.
- Prefer existing local patterns over new abstractions. This project values direct, simple code in hot paths.
- Preserve terminal rendering performance. Avoid per-cell allocations, excessive string formatting, blocking I/O, or unguarded logging inside frame rendering.
- Keep parameter changes synchronized across Rust params, CLI/config handling, `ShaderUniforms`, WGSL `Uniforms`, docs, and tests.
- Keep pattern enum ordering synchronized with WGSL dispatch IDs. If `PatternType` changes, inspect `src/shader_common/main.wgsl` and tests that assert numeric IDs.
- Keep the CLI list commands (`--list-patterns`, `--list-color-modes`, `--list-palettes`, `--list-audio-devices`) accurate when enums or audio discovery behavior changes.
- Use structured filesystem APIs such as `Path` and `PathBuf`; do not hardcode platform path separators.
- Use `Result` and contextual errors for recoverable failures. Avoid `unwrap()` in runtime/library code unless failure is genuinely impossible.
- Add tests for behavioral changes, especially config reloads, audio analysis, enum mappings, shader coordinate transforms, and terminal-frame semantics.

## Shader And Rendering Notes

- Built-in shaders are concatenated by `build.rs` into `compiled_shader.wgsl`; update `build.rs` when adding a WGSL module.
- `src/shader_common/uniforms.wgsl` and `src/shader/uniforms.rs` must stay layout-compatible.
- The shader entry point writes RGBA float output that is read back and converted to ASCII on the CPU.
- Some patterns are tiled and can scale from the origin; localized patterns such as globe/sphere effects need center-based coordinate handling.
- GPU render tests in `tests/render_test.rs` are ignored by default because they require hardware/driver availability. Run targeted ignored tests when touching shader compilation or pipeline code.

## Audio Notes

- Linux prefers PulseAudio/PipeWire monitor capture through `src/audio/pulse_capture.rs`, with CPAL fallback where appropriate.
- Device enumeration and capture can fail on valid systems. Handle those failures gracefully and log details to the debug log.
- Audio callbacks and drains should remain bounded. Respect existing ring-buffer limits and logging throttles.
- Avoid treating isolated sample spikes as sustained audio. Check both peak and RMS behavior when changing silence/activity logic.
- Keep audio fixture tests deterministic; extend `tests/support/audio_fixtures.rs` instead of duplicating ad hoc signal generation.

## Config And Input Notes

- Live config reload watches the config file's parent directory so editor save-by-rename flows can work. Preserve replacement of stale pending reload messages.
- Runtime state such as time, resolution, audio flags, and transient beat state may need to survive config reloads. Check `prepare_reloaded_params`.
- Keyboard input should update `ShaderParams` and any dependent helper state together, such as palette changes that also update `AsciiConverter`.

## Documentation Expectations

Update user-facing docs when behavior, dependencies, commands, or defaults change:

- `README.md` for installation, dependencies, and high-level behavior
- `notes/USAGE.md` for CLI usage and runtime behavior
- `notes/AUDIO_SETUP.md` for audio dependencies and capture behavior
- `notes/ARCHITECTURE.md` for render-loop or pipeline-level changes
- this file for contributor/agent rules that should stay prominent

Prefer exact command names and current defaults over broad claims. Avoid long architecture snapshots that duplicate source files and go stale quickly.

## Pull Request Standard

- One logical change per PR when possible
- Tests or focused verification for new behavior and regressions
- Docs updated when user-visible behavior changes
- `cargo fmt --check`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, and `actionlint` passing before review
- No generated artifacts, local build outputs, logs, or cache directories in commits

## Commit Hygiene

- Inspect `git status` and `git diff` before staging.
- Do not include `/target`, `/result`, `/result-*`, logs, editor backup files, or other generated artifacts.
- Keep `Cargo.lock` when dependency changes are intentional.
- Preserve symlinks for `CLAUDE.md` and `CONTRIBUTING.md` unless intentionally changing this documentation layout.
