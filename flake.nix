{
  description = "Chroma - GPU-accelerated ASCII art audio visualizer for the terminal";

  inputs = {
    crane.url = "github:ipetkov/crane";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      crane,
      nixpkgs,
      rust-overlay,
    }:
    let
      inherit (nixpkgs) lib;

      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      # Everything per-system is defined once here and the flake outputs
      # below pick from it, so no helper flake is needed to fan out.
      perSystem = lib.genAttrs systems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          # mkRustBin reuses the shared nixpkgs instance instead of importing
          # a second copy with rust-overlay applied.
          rustBin = rust-overlay.lib.mkRustBin { } pkgs;

          # Keep these pins explicit. The package build uses stable Rust while
          # the dev shell uses nightly rustfmt for rustfmt.toml's unstable
          # import-grouping options.
          packageRustVersion = "1.98.1";
          devNightlyDate = "2026-09-22";

          rustToolchain = rustBin.stable.${packageRustVersion}.minimal.override {
            extensions = [ "clippy" ];
          };
          devRustToolchain = rustBin.nightly.${devNightlyDate}.default.override {
            extensions = [
              "rust-src"
              "rustfmt"
              "clippy"
            ];
          };

          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
          craneLibNightly = (crane.mkLib pkgs).overrideToolchain devRustToolchain;

          # crane's cleanCargoSource would drop the WGSL modules that build.rs
          # concatenates and the tests include, so list the build inputs
          # explicitly instead. Docs, examples, and CI files stay out so
          # editing them does not invalidate the Rust builds.
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              ./build.rs
              ./rustfmt.toml
              ./src
              ./tests
              ./benches
            ];
          };

          runtimeLibraries = [
            pkgs.vulkan-loader
            pkgs.alsa-lib
            pkgs.libpulseaudio
          ];
          runtimeLibraryPath = lib.makeLibraryPath runtimeLibraries;

          commonArgs = {
            inherit src;
            strictDeps = true;
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = runtimeLibraries;
          };

          # Dependencies are compiled once here and reused by the package,
          # test, and clippy derivations, so source edits only rebuild Chroma.
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          chroma = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
              nativeBuildInputs = commonArgs.nativeBuildInputs ++ [ pkgs.makeWrapper ];

              # The default package build stays focused on producing the
              # binary. The flake checks below expose fmt, tests, clippy, and
              # workflow linting explicitly for CI and local validation.
              doCheck = false;

              postInstall = ''
                wrapProgram "$out/bin/chroma" \
                  --prefix LD_LIBRARY_PATH : "${runtimeLibraryPath}" \
                  --set-default WGPU_BACKEND vulkan
              '';

              meta = {
                description = "Rust-based ASCII art shader audio visualizer for the terminal";
                homepage = "https://github.com/yuri-xyz/chroma";
                license = lib.licenses.gpl3Only;
                mainProgram = "chroma";
                platforms = lib.platforms.linux;
              };
            }
          );
        in
        {
          inherit pkgs chroma;

          app = {
            type = "app";
            program = lib.getExe chroma;
            meta.description = "Run Chroma with audio support";
          };

          checks = {
            package = chroma;
            test = craneLib.cargoTest (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoTestExtraArgs = "--all-targets";
              }
            );
            clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets -- -D warnings";
              }
            );
            fmt = craneLibNightly.cargoFmt { inherit src; };
            actionlint =
              pkgs.runCommand "chroma-actionlint-check" { nativeBuildInputs = [ pkgs.actionlint ]; }
                ''
                  actionlint -color ${./.github/workflows}/*.yml
                  touch "$out"
                '';
            nixfmt = pkgs.runCommand "chroma-nixfmt-check" { nativeBuildInputs = [ pkgs.nixfmt ]; } ''
              nixfmt --check ${./flake.nix}
              touch "$out"
            '';
          };

          devShell = pkgs.mkShell {
            packages = runtimeLibraries ++ [
              devRustToolchain
              pkgs.rust-analyzer
              pkgs.actionlint
              pkgs.just
              pkgs.nixfmt
              pkgs.pkg-config
              pkgs.vulkan-tools
              pkgs.pipewire
            ];

            LD_LIBRARY_PATH = runtimeLibraryPath;
            WGPU_BACKEND = "vulkan";

            # `nix develop github:yuri-xyz/chroma` has no justfile in the
            # working tree, so a missing one must not fail the shell.
            shellHook = ''
              just --list 2>/dev/null || true
            '';
          };
        }
      );

      forAllSystems = f: lib.mapAttrs (_: f) perSystem;
    in
    {
      packages = forAllSystems (s: {
        inherit (s) chroma;
        default = s.chroma;
      });

      apps = forAllSystems (s: {
        chroma = s.app;
        default = s.app;
      });

      checks = forAllSystems (s: s.checks);

      formatter = forAllSystems (s: s.pkgs.nixfmt-tree);

      devShells = forAllSystems (s: {
        default = s.devShell;
      });

      # Lets NixOS and Home Manager configurations use `pkgs.chroma` by
      # applying this overlay instead of reaching into `packages`.
      overlays.default = final: _prev: {
        inherit (self.packages.${final.stdenv.hostPlatform.system}) chroma;
      };
    };
}
