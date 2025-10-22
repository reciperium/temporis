# Start the slint-viewer with auto-reload enabled
view:
    slint-viewer  --auto-reload ui/main.slint

# Build the binary
build__bin:
    nix build .#temporis-bin

# Build .desktop icon
build__desktop-file:
    nix build .#temporis-desktop-file -o nix-built-assets

# Build the website
build__site:
    nix build --show-trace .#site -o dist

# Install the application for debug
fresh-install:
    nix profile remove temporis-desktop || true
    nix profile install --show-trace .#temporis-desktop

# Update all dependencies
update: flake__update cargo__update

# Update nix dependencies
flake__update:
    nix flake update

# Update cargo dependencies
cargo__update:
    cargo update

# Extract translations
translations__extract:
    find ui/ -name \*.slint | xargs slint-tr-extractor -o po/en/temporis.po


## Just for record, this is not actually used by CI
bundle_deb:
    cargo packager --formats deb

# Attempt at creating an AppImage bundle
bundle__appimage:
    nix bundle --bundler github:ralismark/nix-appimage .#temporis

# Attempt at patching ELF files from binary
# bundle__exe:
#     nix-build -E 'with import <nixpkgs> {};
#         let
#             bundleRepo = fetchFromGitHub {
#             owner = "3noch";
#             repo = "nix-bundle-exe";
#             # Use a specific commit hash for reproducibility
#             rev = "7f61fe91a73119e1e39d821f0d215d6186d01363";
#             # The sha256 hash ensures the fetched code is what you expect.
#             # See the explanation below on how to calculate this.
#             sha256 = "sha256-NKdglYdwN4M7/UOZ8Ml3fuJT1MYXAb6aU4ccU920BjM=";
#             };
#         in
#         callPackage bundleRepo {} /nix/store/049mnwsjv5ba5plbnzbxf2w1nl256yn2-temporis-0.1.0'
