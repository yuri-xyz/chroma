{
  description = "Term Shaders - Nix flake (packages + dev shell)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Recent Rust (matches Rust 2024 / >=1.82 requirement)
        rustToolchain = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        commonNativeBuildInputs = with pkgs; [
          pkg-config
        ];

        commonBuildInputs = with pkgs; [
          vulkan-loader
          alsa-lib
          pipewire
        ];

        src = ./.;
      in
      rec {
        packages = {
          # Default (visuals only)
          term-shaders = rustPlatform.buildRustPackage {
            pname = "term-shaders";
            version = "unstable-${self.shortRev or "dirty"}";
            inherit src;

            # Use Cargo.lock; Nix will ask to update cargoSha256 on first build
            cargoLock = { lockFile = ./Cargo.lock; };
            cargoSha256 = pkgs.lib.fakeSha256;

            nativeBuildInputs = commonNativeBuildInputs;
            buildInputs = commonBuildInputs;

            # Integration tests may require a display/audio; disable in CI by default
            doCheck = false;

            meta = with pkgs.lib; {
              description = "Terminal shader visualizer rendering GPU-computed ASCII art";
              homepage = "https://github.com/yuri-xyz/chroma";
              license = licenses.agpl3Plus;
              mainProgram = "term-shaders";
              platforms = platforms.linux;
            };
          };

          # Audio-enabled variant
          term-shaders-audio = packages.term-shaders.overrideAttrs (_: {
            pname = "term-shaders-audio";
            cargoBuildFlags = [ "--features" "audio" ];
            cargoTestFlags = [ "--features" "audio" ];
          });
        };

        defaultPackage = packages.term-shaders;

        apps.default = {
          type = "app";
          program = "${packages.term-shaders}/bin/term-shaders";
        };

        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            pkgs.cargo
            pkgs.rust-analyzer
            pkgs.pkg-config
            pkgs.vulkan-loader
            pkgs.vulkan-tools
            pkgs.alsa-lib
            pkgs.pipewire
          ];

          # Helpful at runtime for wgpu
          shellHook = ''
            export WGPU_BACKEND=vulkan
            echo "Dev shell ready. Build with: cargo build --release [--features audio]"
          '';
        };
      }
    );
}
