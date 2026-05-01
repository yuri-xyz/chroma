# Chroma Agent Notes

The source code is the authority for implementation details. Check the actual Rust and WGSL files before relying on older notes.

## Runtime Output Rule

Do not add diagnostic logging with `println!`, `eprintln!`, `dbg!`, or direct writes to stdout/stderr in runtime paths that can execute while the visualizer is rendering. Stdout is reserved for rendered frames and explicit list/stream commands, so diagnostics there will corrupt the terminal UI and can reduce FPS.

Use the existing file-backed logging APIs instead:

- `debug_logln!(debug_log, ...)` when a `DebugLog` is already available.
- `append_debug_line(component, message)` from library/background code that cannot access the app-owned `DebugLog`.

User-facing CLI output is acceptable only for commands that exit before the visualizer starts, such as `--list-audio-devices`, `--list-patterns`, `--list-color-modes`, and `--list-palettes`.
