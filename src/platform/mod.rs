#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod platform;
pub use platform::*;

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod platform;
pub use platform::*;
