# Project tasks for Chroma. Run them inside the flake dev shell (`nix develop`,
# or direnv with the checked-in .envrc), which provides the pinned toolchains
# and the native audio and Vulkan libraries.

# Pass recipe arguments to commands as "$@" so quoting is preserved.
set positional-arguments

[private]
default:
    @just --list

# Run Chroma with optional arguments, e.g. `just run --preset 3`
run *args:
    cargo run --release -- "$@"

# Build an optimized binary
build:
    cargo build --release

# Format Rust and Nix sources
fmt:
    cargo fmt --all
    nix fmt

# Check formatting without changing files
fmt-check:
    cargo fmt --all -- --check
    nixfmt --check flake.nix

# Run the test suite, passing extra arguments through to cargo test
test *args:
    cargo test "$@"

# Parallel Vulkan device creation can segfault, so these run on one thread.

# Run the GPU tests that are ignored by default (needs a Vulkan driver)
test-gpu:
    cargo test --test render_test --test pattern_zoom_test -- --ignored --test-threads=1

# Lint Rust code with warnings denied
clippy:
    cargo clippy --all-targets -- -D warnings

# Lint GitHub Actions workflows
actionlint:
    actionlint -color

# Run every check required before review
check: fmt-check clippy test actionlint

# Run the benchmarks and save them as a named baseline
bench-save baseline="before":
    cargo bench -- --save-baseline {{ baseline }}

# Run the benchmarks and compare them against a saved baseline
bench-compare baseline="before":
    cargo bench -- --baseline-lenient {{ baseline }}

# Build and run every flake check in the Nix sandbox (slow)
nix-check:
    nix flake check

# Build the Nix package into ./result
nix-build:
    nix build

# Update the flake inputs in flake.lock
nix-update:
    nix flake update
