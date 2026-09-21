# Nix

Chroma's flake is the preferred way to build, run, and validate the project on Linux. It supplies the Vulkan, ALSA, and PulseAudio libraries that the Rust build and the finished binary both need, so nothing has to be installed by hand.

Supported systems are `x86_64-linux` and `aarch64-linux`.

### Everyday commands

```bash
# Run Chroma without installing it
nix run github:yuri-xyz/chroma

# Install it into your profile
nix profile add github:yuri-xyz/chroma

# From a checkout: build, run, or enter the development shell
nix build
nix run
nix develop
```

### Flake outputs

| Output | What it provides |
| --- | --- |
| `packages.<system>.default` | The wrapped `chroma` binary |
| `apps.<system>.default` | The same binary, for `nix run` |
| `devShells.<system>.default` | Rust, rust-analyzer, actionlint, nixfmt, Vulkan tools, and the audio libraries |
| `checks.<system>` | The package build, Rust tests, clippy, rustfmt, actionlint, and nixfmt |
| `formatter.<system>` | `nixfmt`, so `nix fmt` formats Nix files consistently |

### Toolchains

Two Rust toolchains are pinned in `flake.nix`, and the split is deliberate.

`packageRustVersion` is the stable compiler used by `nix build` and by the clippy check. Shipping from a pinned stable release keeps builds reproducible.

`devNightlyDate` is the nightly used inside `nix develop`. It exists because `rustfmt.toml` relies on unstable import-grouping options that stable rustfmt rejects. The same nightly provides clippy and rust-analyzer support in the shell.

The GitHub Actions workflow pins the same two versions through `RUST_STABLE_VERSION` and `RUST_NIGHTLY_VERSION` in `.github/workflows/test.yml`. When you update a pin, change it in both files. Keeping them equal means a new Rust release cannot fail CI through a freshly added lint, and that CI and `nix flake check` agree.

`rust-overlay` follows the repository's `nixpkgs` input, so the lockfile carries a single nixpkgs revision.

### Runtime libraries

The package wraps `chroma` with an `LD_LIBRARY_PATH` containing `vulkan-loader`, `alsa-lib`, and `libpulseaudio`. These are loader and client libraries only. A working Vulkan driver (ICD) still has to come from the host system, which is why Chroma can report that no GPU was found even when installed through Nix.

### Validation

`nix flake check` runs the complete set of checks and is what to run before proposing a change to the flake itself. It builds everything in the sandbox, so it is slow.

While iterating, run the underlying tools directly inside the dev shell instead:

```bash
nix develop -c cargo fmt --all -- --check
nix develop -c cargo test
nix develop -c cargo clippy --all-targets -- -D warnings
nix develop -c actionlint -color
nix fmt
```

If your Nix installation does not enable flakes by default, add `--extra-experimental-features 'nix-command flakes'` after `nix`. [Contributing](../CONTRIBUTING.md) lists the focused test commands, including the GPU tests that are skipped by default.
