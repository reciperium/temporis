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
