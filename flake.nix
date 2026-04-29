{
  description = "Chroma - GPU-accelerated ASCII art audio visualizer for the terminal";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachSystem
      [
        "x86_64-linux"
        "aarch64-linux"
      ]
      (
        system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          inherit (pkgs) lib;

          rustToolchain = pkgs.rust-bin.stable.latest.default;
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };

          src = lib.cleanSourceWith {
            src = ./.;
            filter =
              path: type:
              let
                baseName = baseNameOf path;
              in
              !(
                type == "directory"
                && lib.elem baseName [
                  "target"
                  ".direnv"
                  ".devenv"
                ]
              );
          };

          runtimeLibraryPath = lib.makeLibraryPath [
            pkgs.vulkan-loader
            pkgs.alsa-lib
          ];

          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

          chroma =
            rustPlatform.buildRustPackage {
              pname = "chroma";
              inherit src;
              version = cargoToml.package.version;

              cargoLock.lockFile = ./Cargo.lock;

              nativeBuildInputs = [
                pkgs.makeWrapper
                pkgs.pkg-config
              ];

              buildInputs = [
                pkgs.vulkan-loader
                pkgs.alsa-lib
              ];

              # The integration suite exercises terminal/GPU/audio-adjacent behavior
              # that is better validated outside the pure Nix build sandbox.
              doCheck = false;

              postInstall = ''
                wrapProgram "$out/bin/chroma" \
                  --prefix LD_LIBRARY_PATH : "${runtimeLibraryPath}"
              '';

              meta = {
                description = "Rust-based ASCII art shader audio visualizer for the terminal";
                homepage = "https://github.com/yuri-xyz/chroma";
                license = lib.licenses.mit;
                mainProgram = "chroma";
                platforms = lib.platforms.linux;
              };
            };
        in
        {
          packages = {
            inherit chroma;
            default = chroma;
          };

          apps = {
            chroma =
              (flake-utils.lib.mkApp {
                drv = chroma;
              })
              // {
                meta.description = "Run Chroma with audio support";
              };
            default =
              (flake-utils.lib.mkApp {
                drv = chroma;
              })
              // {
                meta.description = "Run Chroma with audio support";
              };
          };

          devShells.default = pkgs.mkShell {
            packages = [
              rustToolchain
              pkgs.cargo
              pkgs.rust-analyzer
              pkgs.clippy
              pkgs.rustfmt
              pkgs.pkg-config
              pkgs.vulkan-loader
              pkgs.vulkan-tools
              pkgs.alsa-lib
              pkgs.pipewire
            ];

            LD_LIBRARY_PATH = runtimeLibraryPath;
            WGPU_BACKEND = "vulkan";

            shellHook = ''
              echo "Chroma dev shell"
              echo "  cargo run"
              echo "  nix run ."
              echo "  nix build"
            '';
          };
        }
      );
}
