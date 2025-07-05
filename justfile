# Start the slint-viewer with auto-reload enabled
view:
    slint-viewer  --auto-reload ui/main.slint

# Build the binary
build__bin:
    nix build .#temporis

# Install the application for debug
fresh-install:
    nix profile remove temporis || true
    nix profile install --show-trace .#temporis

bundle__appimage:
    nix bundle --bundler github:ralismark/nix-appimage .#temporis

flake__update:
    nix flake update
