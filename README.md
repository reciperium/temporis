<img src="./ui/icons/logo.svg" alt="logo" width="150" height="150">

# Temporis

A pomodoro timer focused on productivity and attention.

_[temporis-logo](https://github.com/reciperium/temporis/blob/main/ui/icons/logo.svg) © 2025 by [Lara Sitruk](https://www.instagram.com/lazomicreative/) is licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)_

## Installation

```sh
nix profile install github:reciperium/temporis
```

## Features

- Switch between focus work and diffusion modes
- Notifications reminding you to stretch and hydrate
- WIP: Configuration
- WIP: Automatically set "Do not disturb" mode during focus work sessions (KDE for now using dbus)

---

## Thanks

- [Vadoola](https://github.com/Vadoola) for the display time equation using slint lang, as I was completely lost. [time-remaining()](https://github.com/Vadoola/Tomotroid/blob/main/ui/appwindow.slint#L88)

## Resources

- [freedesktop.org/desktop-entry-spec/recognized-keys](https://specifications.freedesktop.org/desktop-entry-spec/latest/recognized-keys.html)
- [KDE kirigame in rust](https://develop.kde.org/docs/getting-started/kirigami/setup-rust/#build)
- show progress in task bar: `com.canonical.Unity.LauncherEntry`
- [Urgency levels](https://specifications.freedesktop.org/notification-spec/latest/urgency-levels.html)
- [makeDesktopItem](https://nixos.org/manual/nixpkgs/stable/#trivial-builder-makeDesktopItem) for NixOS
