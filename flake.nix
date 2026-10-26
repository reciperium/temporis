{
  description = "Temporis rust flake with slint support";
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.zst";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    inputs@{
      flake-parts,
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
          lib,
          ...
        }:
        let
          nodejs = pkgs.nodejs_latest;

          # rust
          rustChannel = "stable";
          fenix = inputs'.fenix.packages;
          rustToolchain = (
            fenix.combine [
              fenix.${rustChannel}.toolchain
              # https://doc.rust-lang.org/rustc/platform-support.html
            ]
          );
          rustplatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
          cargoToml = fromTOML (builtins.readFile ./Cargo.toml);

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
            license = lib.licenses.gpl3;
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
            temporis-bin = rustplatform.buildRustPackage {
              pname = cargoToml.package.name;
              version = cargoToml.package.version;
              src = lib.fileset.toSource {
                root = ./.;
                fileset = lib.fileset.unions [
                  ./ui
                  ./Cargo.toml
                  ./Cargo.lock
                  ./assets/audio
                  ./assets/icons
                  ./assets/logos
                  ./src
                ];
              };
              strictDeps = true;
              cargoLock = {
                lockFile = ./Cargo.lock;
                allowBuiltinFetchGit = true;
              };
              nativeBuildInputs = with pkgs; [
                pkg-config
                llvmPackages.bintools
                removeReferencesTo
              ];

              buildInputs =
                with pkgs;
                [ ]
                ++ lib.optionals stdenv.hostPlatform.isLinux [
                  fontconfig
                  alsa-lib
                  makeWrapper
                ];
              meta = meta;
              env = pkgs.lib.optionalAttrs (system == "x86_64-linux") {
                RUSTFLAGS = "-C link-self-contained=-linker";
              };
              postFixup = ''
                remove-references-to -t ${rustToolchain} $out/bin/temporis
              '';
            };
            # installable for NixOS
            temporis-desktop = pkgs.stdenv.mkDerivation {
              name = "temporis-desktop";
              nativeBuildInputs = with pkgs; [ ] ++ lib.optionals stdenv.hostPlatform.isLinux [ copyDesktopItems ];
              buildInputs =
                with pkgs;
                [ ]
                ++ lib.optionals stdenv.hostPlatform.isLinux [
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
                + lib.optionalString stdenv.hostPlatform.isLinux ''
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
              nativeBuildInputs = [
                pkgs.pkg-config
              ];
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
                nodejs
                fontconfig
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
