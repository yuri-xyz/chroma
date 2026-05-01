# Nix Flake

Chroma's flake is the preferred way to build, run, and validate the project on Linux. It provides the native Vulkan, ALSA, and PulseAudio libraries needed by the Rust build and runtime wrapper.

## Outputs

```bash
nix build
nix run
nix develop
nix flake check
nix fmt
```

- `packages.<system>.default` builds the wrapped `chroma` binary.
- `apps.<system>.default` runs the wrapped binary through `nix run`.
- `devShells.<system>.default` provides Rust, rust-analyzer, actionlint, nixfmt, Vulkan tools, and audio libraries.
- `checks.<system>` exposes package build, Rust tests, clippy, rustfmt, actionlint, and nixfmt checks.
- `formatter.<system>` is `pkgs.nixfmt`, so `nix fmt` formats Nix files consistently.

Supported systems are `x86_64-linux` and `aarch64-linux`.

## Toolchains

The package build uses pinned stable Rust. The development shell uses a pinned nightly Rust because `rustfmt.toml` relies on unstable import-grouping options. Keep both pins explicit in `flake.nix` when updating toolchains:

- `packageRustVersion` for the stable compiler used by `nix build`
- `devNightlyDate` for nightly rustfmt, clippy, and rust-analyzer support in `nix develop`

`rust-overlay` follows the repository's `nixpkgs` input, so the lockfile has one nixpkgs revision instead of separate package and overlay revisions.

## Runtime Libraries

The package wraps `chroma` with an `LD_LIBRARY_PATH` containing:

- `vulkan-loader`
- `alsa-lib`
- `libpulseaudio`

These are loader/client libraries only. A working Vulkan driver or ICD still needs to come from the host system.

## Checks

Run the full Nix validation set with:

```bash
nix flake check
```

For faster focused checks while iterating:

```bash
nix develop -c cargo fmt --all -- --check
nix develop -c cargo test
nix develop -c cargo clippy --all-targets -- -D warnings
nix develop -c actionlint -color
nix fmt
```
