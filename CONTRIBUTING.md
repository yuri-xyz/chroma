# Contributing to Chroma

Thanks for your interest in contributing! This guide covers best practices for writing quality Rust code and ensuring cross-platform compatibility.

## Getting Started

1. Fork the repository and clone your fork
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes following the guidelines below
4. Run tests and formatting: `cargo test && cargo fmt && cargo clippy`
5. Submit a pull request

## Rust Best Practices

### Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Error Handling

- Use `Result<T, E>` for recoverable errors, not panics
- Prefer `?` operator over `.unwrap()` in library code
- Reserve `.unwrap()` and `.expect()` for cases where failure is truly impossible
- Provide meaningful error messages with `.expect("reason")`

```rust
// Avoid
let file = File::open(path).unwrap();

// Prefer
let file = File::open(path).map_err(|e| AppError::FileOpen { path, source: e })?;
```

### Memory and Performance

- Prefer borrowing (`&T`) over cloning when possible
- Use `Cow<str>` when you might or might not need ownership
- Avoid allocations in hot paths (the render loop)
- Profile before optimizing - use `cargo flamegraph`

### Type Safety

- Use newtypes to distinguish semantically different values
- Prefer enums over boolean flags for clarity
- Use `#[must_use]` on functions where ignoring the return value is likely a bug

```rust
// Avoid
fn set_enabled(audio: bool, video: bool) { }

// Prefer
enum AudioState { Enabled, Disabled }
enum VideoState { Enabled, Disabled }
fn configure(audio: AudioState, video: VideoState) { }
```

### Documentation

- Document public APIs with `///` doc comments
- Include examples in doc comments where helpful
- Use `//` comments sparingly for non-obvious implementation details

## Cross-Platform Guidelines

Chroma targets Linux, macOS, and Windows. Follow these practices to maintain compatibility.

### File Paths

- Never hardcode path separators (`/` or `\`)
- Use `std::path::Path` and `PathBuf` for all file operations
- Use `Path::join()` to construct paths

```rust
// Avoid
let config = format!("{}/config.toml", home_dir);

// Prefer
let config = home_dir.join("config.toml");
```

### Line Endings

- Be aware that `\n` vs `\r\n` differs by platform
- Use `lines()` iterator which handles both
- When writing files, consider whether line endings matter

### Environment and Directories

- Use the `dirs` crate for standard directories (home, config, cache)
- Don't assume environment variables exist
- Handle missing `HOME`/`USERPROFILE` gracefully

```rust
// Avoid
let home = std::env::var("HOME").unwrap();

// Prefer
let home = dirs::home_dir().ok_or(AppError::NoHomeDir)?;
```

### Terminal Handling

- Use `crossterm` for terminal operations (we already do)
- Don't assume terminal capabilities - check or degrade gracefully
- Handle terminal resize events
- Clean up terminal state on exit (raw mode, cursor visibility)

### Conditional Compilation

When platform-specific code is unavoidable:

```rust
#[cfg(target_os = "linux")]
fn platform_specific() {
    // Linux implementation
}

#[cfg(target_os = "macos")]
fn platform_specific() {
    // macOS implementation
}

#[cfg(target_os = "windows")]
fn platform_specific() {
    // Windows implementation
}
```

### Audio (cpal)

- Don't assume specific audio backends
- Handle device enumeration failures gracefully
- The default device may not exist - always check

### Graphics (wgpu)

- Don't assume Vulkan - wgpu selects the best backend per platform
- Test on multiple GPU vendors if possible
- Handle adapter/device creation failures

## Testing

### Unit Tests

- Place unit tests in the same file with `#[cfg(test)]` module
- Test edge cases and error conditions
- Use descriptive test names

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_handles_missing_fields() {
        // ...
    }
}
```

### Integration Tests

- Place in `tests/` directory
- Test the public API as users would use it

### Running Tests

```bash
cargo test                        # Run all tests
cargo test --features audio       # Include audio feature tests
cargo test -- --nocapture         # Show println! output
```

## Pull Request Guidelines

- Keep PRs focused and small
- One logical change per PR
- Update documentation if behavior changes
- Add tests for new functionality
- Ensure CI passes before requesting review

## Project Structure

```
src/
├── main.rs          # Entry point
├── cli.rs           # Command-line parsing
├── app/             # Application logic
├── ascii/           # ASCII conversion
├── audio/           # Audio capture (feature-gated)
├── params/          # Configuration
├── shader/          # GPU pipeline
└── shader_common/   # WGSL shaders

shader_patterns/     # Shader pattern files
examples/            # Example configs
notes/               # Documentation
```

## Questions?

Open an issue for discussion before starting large changes. This helps ensure your contribution aligns with project goals.
