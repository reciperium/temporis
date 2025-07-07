<img src="./assets/icons/logo.svg" alt="logo" width="200" height="200">

_[temporis-logo](./assets/icons/logo.svg) © 2025 by [Lara Sitruk](https://www.instagram.com/lazomicreative/) is licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)_

# Temporis

> A pomodoro timer focused on attention and well-being.

<img src="docs/images/screenshot-main.png" alt="screenshot main window" height="600">

## Installation

```sh
nix profile install github:reciperium/temporis
```

## Why?

A pomodoro application can help you switch between focus mode, where you can explore a single idea in depth, and diffusion mode,
where you can let your mind wonder around, and subconsciously explore other ideas, avoiding the [Einstellung effect](https://en.wikipedia.org/wiki/Einstellung_effect).
Furthermore, clear breaks give you space to stretch, move and hydrate.

The app tries to consolidate nice features from different pomodoro apps, into a single one, with the style of
the host operating system.

A [Reciperium](https://reciperium.com) application, [Reciperium](https://reciperium.com) builds applications for the mind and soul.

## Platforms

- Linux
  - [x] Nix
  - [ ] AppImage
- MacOS
  - [x] Nix
  - [ ] DMG

## Features

- Switch between focus work and diffusion modes
- Shortcuts
- TOML Configuration
- App style based on your OS
- Notifications reminding you to stretch and hydrate
- Visible progress
- Tick Sound notification
- WIP: Automatically set "Do not disturb" mode during focus work sessions (KDE for now using dbus)

## Shortcuts

- <kbd>S</kbd> Start / Stop (pause)
- <kbd>R</kbd> Reset
- <kbd>K</kbd> Skip
- <kbd>C</kbd> Configuration
- <kbd>B</kbd> Back

---

## Acknowledgments

- [Vadoola](https://github.com/Vadoola) for the display time equation using slint lang, as I was completely lost. [time-remaining()](https://github.com/Vadoola/Tomotroid/blob/main/ui/appwindow.slint#L88) and a lot of new nice libraries (e.g: rodio)
- [Francis](https://github.com/KDE/francis) is a pomodoro app for KDE, built using the KDE Kirigami framework, I drew a lot of inspiration from it.
- [metal_click_6.flac by qubodup](https://freesound.org/s/67610/) -- [License: Creative Commons 0](http://creativecommons.org/publicdomain/zero/1.0/)

## Resources

- [freedesktop.org/desktop-entry-spec/recognized-keys](https://specifications.freedesktop.org/desktop-entry-spec/latest/recognized-keys.html)
- [KDE kirigame in rust](https://develop.kde.org/docs/getting-started/kirigami/setup-rust/#build)
- show progress in task bar: `com.canonical.Unity.LauncherEntry`
- [Urgency levels](https://specifications.freedesktop.org/notification-spec/latest/urgency-levels.html)
- [makeDesktopItem](https://nixos.org/manual/nixpkgs/stable/#trivial-builder-makeDesktopItem) for NixOS

<p align="center">
    <img src="./docs/images/made-with-slint.png" alt="made with slint logo" width="200">
</p>
