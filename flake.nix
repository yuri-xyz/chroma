{
  description = "Chroma - GPU-accelerated ASCII art audio visualizer for the terminal";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      perSystem =
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };

          inherit (pkgs) lib;

          # Keep these pins explicit. The package build uses stable Rust while
          # the dev shell uses nightly rustfmt for rustfmt.toml's unstable
          # import-grouping options.
          packageRustVersion = "1.95.0";
          devNightlyDate = "2026-04-29";

          rustToolchain = pkgs.rust-bin.stable.${packageRustVersion}.default;
          clippyToolchain = pkgs.rust-bin.stable.${packageRustVersion}.default.override {
            extensions = [ "clippy" ];
          };
          devRustToolchain = pkgs.rust-bin.nightly.${devNightlyDate}.default.override {
            extensions = [
              "rust-src"
              "rustfmt"
              "clippy"
            ];
          };
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
          clippyRustPlatform = pkgs.makeRustPlatform {
            cargo = clippyToolchain;
            rustc = clippyToolchain;
          };

          src = lib.cleanSourceWith {
            name = "chroma-source";
            src = ./.;
            filter =
              path: type:
              let
                baseName = baseNameOf path;
              in
              lib.cleanSourceFilter path type
              && !(
                type == "directory"
                && lib.elem baseName [
                  "target"
                  ".direnv"
                  ".devenv"
                ]
              );
          };

          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          version = cargoToml.package.version;

          runtimeLibraries = [
            pkgs.vulkan-loader
            pkgs.alsa-lib
            pkgs.libpulseaudio
          ];
          runtimeLibraryPath = lib.makeLibraryPath runtimeLibraries;

          commonNativeBuildInputs = [
            pkgs.pkg-config
          ];

          chroma = rustPlatform.buildRustPackage {
            pname = "chroma";
            inherit src version;

            cargoLock.lockFile = ./Cargo.lock;

            strictDeps = true;
            nativeBuildInputs = commonNativeBuildInputs ++ [
              pkgs.makeWrapper
            ];
            buildInputs = runtimeLibraries;

            # The default package build stays focused on producing the binary.
            # The flake checks below expose fmt, tests, clippy, and workflow
            # linting explicitly for CI and local validation.
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

          mkApp = drv: {
            type = "app";
            program = lib.getExe drv;
            meta.description = "Run Chroma with audio support";
          };

          mkCargoCheck =
            checkRustPlatform: name: command:
            checkRustPlatform.buildRustPackage {
              pname = "chroma-${name}";
              inherit src version;

              cargoLock.lockFile = ./Cargo.lock;

              strictDeps = true;
              nativeBuildInputs = commonNativeBuildInputs;
              buildInputs = runtimeLibraries;

              buildPhase = ''
                runHook preBuild
                ${command}
                runHook postBuild
              '';
              doCheck = false;
              installPhase = ''
                runHook preInstall
                touch "$out"
                runHook postInstall
              '';
            };
        in
        rec {
          packages = {
            inherit chroma;
            default = chroma;
          };

          apps = {
            chroma = mkApp chroma;
            default = apps.chroma;
          };

          checks = {
            package = chroma;
            test = mkCargoCheck rustPlatform "test" "cargo test --all-targets --offline --frozen";
            clippy =
              mkCargoCheck clippyRustPlatform "clippy"
                "cargo clippy --all-targets --offline --frozen -- -D warnings";
            fmt = pkgs.runCommand "chroma-fmt-check" { nativeBuildInputs = [ devRustToolchain ]; } ''
              cd ${src}
              cargo fmt --all -- --check
              touch "$out"
            '';
            actionlint =
              pkgs.runCommand "chroma-actionlint-check" { nativeBuildInputs = [ pkgs.actionlint ]; }
                ''
                  actionlint -color ${src}/.github/workflows/*.yml
                  touch "$out"
                '';
            nixfmt = pkgs.runCommand "chroma-nixfmt-check" { nativeBuildInputs = [ pkgs.nixfmt ]; } ''
              nixfmt --check ${./flake.nix}
              touch "$out"
            '';
          };

          formatter = pkgs.nixfmt;

          devShells.default = pkgs.mkShell {
            packages = [
              devRustToolchain
              pkgs.rust-analyzer
              pkgs.actionlint
              pkgs.nixfmt
              pkgs.pkg-config
              pkgs.vulkan-loader
              pkgs.vulkan-tools
              pkgs.alsa-lib
              pkgs.libpulseaudio
              pkgs.pipewire
            ];

            LD_LIBRARY_PATH = runtimeLibraryPath;
            WGPU_BACKEND = "vulkan";

            shellHook = ''
              echo "Chroma dev shell"
              echo "  cargo run"
              echo "  cargo fmt --all -- --check"
              echo "  cargo test"
              echo "  cargo clippy --all-targets -- -D warnings"
              echo "  actionlint -color"
              echo "  nix flake check"
            '';
          };
        };
    in
    {
      packages = forAllSystems (system: (perSystem system).packages);
      apps = forAllSystems (system: (perSystem system).apps);
      checks = forAllSystems (system: (perSystem system).checks);
      formatter = forAllSystems (system: (perSystem system).formatter);
      devShells = forAllSystems (system: (perSystem system).devShells);
    };
}
