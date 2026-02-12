{
  description = "Temporis rust flake with slint support";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
    };

    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs =
    inputs@{
      flake-parts,
      crane,
      gitignore,
      self,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
    in
    # https://flake.parts/
    flake-parts.lib.mkFlake { inherit inputs; } {
      debug = true;
      systems = systems;
      perSystem =
        {
          pkgs,
          inputs',
          self',
          system,
          ...
        }:
        let
          gitignoreSource = gitignore.lib.gitignoreSource;
          nodejs = pkgs.nodejs_latest;

          # rust
          rustChannel = "stable";
          fenix = inputs'.fenix.packages;
          rustToolchain = (
            fenix.combine [
              fenix.${rustChannel}.toolchain
              # https://doc.rust-lang.org/rustc/platform-support.html
              # For more targets add:
              # fenix.targets.aarch64-linux-android."${rustChannel}".rust-std
              # fenix.targets.wasm32-wasip1."${rustChannel}".rust-std
            ]
          );
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
          ldLibraryPath =
            with pkgs;
            lib.makeLibraryPath [
              wayland
              libxkbcommon
              fontconfig
              libGL
              libx11
              libxcursor
              libxi
              alsa-lib
              libinput
              libgbm
              freetype
              # libinput-gestures
              # glibc
            ];

          # app
          meta = {
            description = "A Pomodoro application helping you stay focused, fresh and healthy";
            longDescription = "Temporis is a Pomodoro application designed to help you seamlessly switch between focused work and periods of diffused thinking";
            homepage = "https://github.com/reciperium/temporis";
            license = with pkgs; lib.licenses.gpl3;
            platforms = systems;
            mainProgram = "temporis";
          };
          temporis-desktop-file = pkgs.makeDesktopItem {
            name = "com.reciperium.temporis";
            exec = "temporis"; # recommended way, so users can wrap around it?
            desktopName = "Temporis";
            genericName = "Pomodoro Application";
            icon = "com.reciperium.temporis";
            comment = "A pomodoro timer focused on attention and well-being";
            categories = [ "Utility" ];
            terminal = false;
            keywords = [
              "pomodoro"
              "timer"
              "productivity"
            ];
          };
          src = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [
              ./site
              ./assets/icons
              ./assets/images
            ];
          };
        in
        {
          packages = {
            temporis-bin = craneLib.buildPackage {
              # name = "temporis";
              src = gitignoreSource ./.;
              strictDeps = true;

              nativeBuildInputs = with pkgs; [
                # fontconfig
                pkg-config
                rustPlatform.bindgenHook
                llvmPackages.bintools
              ];

              buildInputs =
                with pkgs;
                [ ]
                ++ lib.optionals stdenv.isLinux [
                  fontconfig
                  alsa-lib
                  makeWrapper
                ];
              meta = meta;
              env = pkgs.lib.optionalAttrs (system == "x86_64-linux") {
                RUSTFLAGS = "-C link-self-contained=-linker";
              };
            };
            # installable for NixOS
            temporis-desktop = pkgs.stdenv.mkDerivation {
              name = "temporis-desktop";
              nativeBuildInputs = with pkgs; [ ] ++ lib.optionals stdenv.isLinux [ copyDesktopItems ];
              buildInputs =
                with pkgs;
                [ ]
                ++ lib.optionals stdenv.isLinux [
                  makeWrapper
                ];
              src = self;
              buildPhase = ''
                mkdir -p "$out/bin"
                install -Dm755 ${self'.packages.temporis-bin}/bin/temporis $out/bin/temporis
              '';
              desktopItems = [ temporis-desktop-file ];
              installPhase =
                with pkgs;
                ''
                  # install icon
                  mkdir -p "$out/share/icons/hicolor/scalable/apps"
                  install -Dm644 assets/icons/logo.svg $out/share/icons/hicolor/scalable/apps/com.reciperium.temporis.svg

                ''
                + lib.optionalString stdenv.isLinux ''
                  wrapProgram $out/bin/temporis --set LD_LIBRARY_PATH "${ldLibraryPath}"
                ''
                + ''
                  runHook postInstall
                '';
              meta = meta;
            };

            site = pkgs.buildNpmPackage {
              name = "site";
              src = src;
              sourceRoot = "${src.name}/site";
              npmDeps = pkgs.importNpmLock {
                npmRoot = ./site;
              };
              npmConfigHook = pkgs.importNpmLock.npmConfigHook;

              buildPhase = ''
                ${nodejs}/bin/npm run build --unsafe-perm=true
              '';
              installPhase = ''
                mkdir -p $out
                cp -R dist/* $out
              '';
              nodejs = nodejs;
            };

            temporis-desktop-file = temporis-desktop-file;
            default = self'.packages.temporis-desktop;
          };

          devShells = {
            default = pkgs.mkShell {
              name = "dev";

              # Available packages on https://search.nixos.org/packages
              buildInputs = with pkgs; [
                just
                rustToolchain
                alsa-lib
                slint-viewer
                slint-lsp
                bacon
                jq
                cachix
                gettext
                commitizen
                nodejs
              ];

              shellHook = ''
                echo "Welcome to the rust devshell!"
              '';

              LD_LIBRARY_PATH = "\$LD_LIBRARY_PATH:${ldLibraryPath}";

            };
          };
        };
    };
  nixConfig = {
    extra-substituters = [ "https://reciperium.cachix.org" ];
    extra-trusted-public-keys = [
      "reciperium.cachix.org-1:xAmT5McauMNqMlXkkyVzDzoDNO6G+Zo7gCAUYaPsGxQ="
    ];
  };
}
