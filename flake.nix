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

  };
  outputs =
    inputs@{
      flake-parts,
      crane,
      ...
    }:
    # https://flake.parts/
    flake-parts.lib.mkFlake { inherit inputs; } {
      debug = true;
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          pkgs,
          inputs',
          self',
          ...
        }:
        let
          # rust
          rustChannel = "stable";
          fenix = inputs'.fenix.packages;
          toolchain = fenix.${rustChannel}.toolchain;
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

          # app
          temporis-desktop = pkgs.makeDesktopItem {
            name = "temporis";
            exec = "temporis"; # recommended way, so users can wrap around it?
            desktopName = "Temporis";
            genericName = "Pomodoro Application";
            icon = "org.reciperium.temporis";
            comment = "A simple yet powerful pomodoro timer with customizable intervals and breaks";
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
              src = ./.;
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
                  wrapProgram $out/bin/temporis --set LD_LIBRARY_PATH "${
                    lib.makeLibraryPath [
                      wayland
                      libxkbcommon
                      fontconfig
                    ]
                  }"

                  # install icon
                  mkdir -p "$out/share/icons/hicolor/scalable/apps"
                  install -Dm644 ui/icons/logo.svg $out/share/icons/hicolor/scalable/apps/org.reciperium.temporis.svg
                  # ln -s "ui/icons/logo.png" "$out/share/icons/hicolor/128x128/apps/org.reciperium.temporis.png"

                  # install desktop file
                  mkdir -p $out/share/applications
                  install -Dm644 ${temporis-desktop}/share/applications/*.desktop $out/share/applications/temporis.desktop
                '';
              desktopItems = [ temporis-desktop ];
            };
            default = self'.packages.temporis;
          };

          devShells.default = pkgs.mkShell {
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
                slint-viewer
                slint-lsp
                bacon
              ])
            ];

            shellHook = ''
              echo "Welcome to the rust devshell!"
            '';

            LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${
              with pkgs;
              lib.makeLibraryPath [
                wayland
                libxkbcommon
                fontconfig
              ]
            }";

            # PKG_CONFIG_PATH = "$PKG_CONFIG_PATH:${with pkgs; lib.makeLibraryPath [ dbus.dev ]}/pkgconfig/";
            # PKG_CONFIG_PATH = "$PKG_CONFIG_PATH:${with pkgs; dbus.dev}";
          };
        };
    };
}
