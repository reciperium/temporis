{
  description = "A development shell for rust";
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
          ...
        }:
        let
          gitignoreSource = gitignore.lib.gitignoreSource;
          # rust
          rustChannel = "stable";
          fenix = inputs'.fenix.packages;
          toolchain = fenix.${rustChannel}.toolchain;
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
          ldLibraryPath =
            with pkgs;
            lib.makeLibraryPath [
              wayland
              libxkbcommon
              fontconfig
              libGL
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
          temporis-desktop = pkgs.makeDesktopItem {
            name = "temporis";
            exec = "temporis"; # recommended way, so users can wrap around it?
            desktopName = "Temporis";
            genericName = "Pomodoro Application";
            icon = "com.reciperium.temporis";
            comment = "Focus on intervals and take breaks to stimulate your productivity";
            categories = [ "Utility" ];
            terminal = false;
            keywords = [
              "pomodoro"
              "timer"
              "productivity"
            ];
          };
        in
        {
          packages = {
            temporis = craneLib.buildPackage {
              src = gitignoreSource ./.;
              strictDeps = true;
              buildInputs =
                with pkgs;
                [ ]
                ++ lib.optionals stdenv.isLinux [
                  makeWrapper
                ];

              # I should probably use patchelf but I don't know how
              postInstall =
                with pkgs;
                lib.optionalString stdenv.isLinux ''
                  wrapProgram $out/bin/temporis --set LD_LIBRARY_PATH "${ldLibraryPath}"

                  # install icon
                  mkdir -p "$out/share/icons/hicolor/scalable/apps"
                  install -Dm644 ui/icons/logo.svg $out/share/icons/hicolor/scalable/apps/com.reciperium.temporis.svg
                  # ln -s "ui/icons/logo.png" "$out/share/icons/hicolor/128x128/apps/com.reciperium.temporis.png"

                  # install desktop file
                  mkdir -p $out/share/applications
                  install -Dm644 ${temporis-desktop}/share/applications/*.desktop $out/share/applications/com.reciperium.temporis.desktop
                '';
              desktopItems = [ temporis-desktop ];
              meta = meta;
            };
            default = self'.packages.temporis;
          };

          devShells = {
            default = pkgs.mkShell {
              name = "dev";

              # Available packages on https://search.nixos.org/packages
              buildInputs = with pkgs; [
                just

                (fenix.combine [
                  toolchain
                  # https://doc.rust-lang.org/rustc/platform-support.html
                  # For more targets add:
                  # fenix.targets.aarch64-linux-android."${rustChannel}".rust-std
                  # fenix.targets.x86_64-linux-android."${rustChannel}".rust-std
                ])

                slint-viewer
                slint-lsp
                bacon
                jq
                cachix
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
