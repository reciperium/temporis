## v0.7.7 (2026-02-12)

### Fix

- bump dependencies
- **site**: apparmor config

## v0.7.6 (2026-02-09)

### Fix

- **cargo**: bump the all-cargo-dependencies group across 1 directory with 2 updates

## v0.7.5 (2026-01-27)

### Fix

- **cargo**: bump the all-cargo-dependencies group with 2 updates

## v0.7.4 (2026-01-25)

### Fix

- errors in rodio
- **ci**: change if condition

## v0.7.3 (2026-01-25)

### Fix

- **cargo**: bump the all-cargo-dependencies group across 1 directory with 3 updates

## v0.7.2 (2025-12-28)

### Fix

- attempt at correct configuration
- **ci**: use macos latest runner

## v0.7.1 (2025-12-28)

### Fix

- **cargo**: bump deps
- bump flake

## v0.7.0 (2025-11-05)

### Feat

- bump dependencies

### Fix

- **ci**: attempt at properly pushing to binary cache

## v0.6.7 (2025-10-22)

### Fix

- **macos**: attempt at notifications in MacOS

## v0.6.6 (2025-10-22)

### Fix

- bump dependencies
- **macos**: attempt at fixing notifications in macos

## v0.6.5 (2025-10-12)

### Fix

- bump dependencies

## v0.6.4 (2025-09-08)

### Fix

- bump dependencies
- **ci**: fix link to DMG

## v0.6.3 (2025-08-25)

### Fix

- **ci**: correct mac bundle name

## v0.6.2 (2025-08-24)

### Fix

- attempt at mac notifications

## v0.6.1 (2025-08-16)

### Fix

- **site**: update script for appimage

## v0.6.0 (2025-08-15)

### Feat

- add bell sound at the end of an interval

### Fix

- **ui**: use criticial_notifications settings in long-break message

## v0.5.1 (2025-08-10)

### Fix

- **site**: add CNAME

## v0.5.0 (2025-08-10)

### Feat

- **nix**: add build site package
- add website

### Fix

- use cargo-packager from crates

## v0.4.1 (2025-07-31)

### Fix

- set cycles based on the config

## v0.4.0 (2025-07-31)

### Feat

- add max cycles setting, allows auto-ending

## v0.3.0 (2025-07-30)

### Feat

- **mac**: attempt at building DMG
- update dependencies

## v0.2.0 (2025-07-30)

### Feat

- first attempt at a .deb
- add more information to crate
- add translation strings
- add link to reciperium and logo license
- add Divider component
- add Link component
- add setting to enable/disable notifications
- bump dependencies
- add tick sound
- add ctrl+q exit shortcuti
- add ability to configure notification urgency
- add platform specific code
- new settings style
- add dummy macos implementation for progress
- **linux**: add progress in launcher
- bump dependencies
- add keyboard shortcuts
- add config support
- add flake package
- introduce AppWindow and basic settings support
- add new icons
- initial commit including a functional pomodoro app

### Fix

- use semver for releases
- improve about section
- packages names in justfile
- name files properly so dbus works
- format slint import
- warnings when importing modules
- use points instead of pixels in main window
- update logo
- layout size
- icon location during build
- add gitignoresource
- **mac**: import error
- minor glitch in pomodoro page
- use com.reciperium.temporis
- make some buttons smaller
- update lock
- add missing meta

### Refactor

- split the flake into bin and desktop items
- rename setting to critical_notifications
- move icons to assets folder
- structure slint code
- create global ExternalSystem and move properties to System
