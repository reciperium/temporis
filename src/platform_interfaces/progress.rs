use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProgressError {
    #[error("Failed to emit progress {reason}")]
    EmitFailed { reason: String },
}

/// Trait for integrating progress indication with the operating system's native UI.
///
/// This trait must be implemented for each supported operating system to provide
/// platform-specific progress feedback (such as taskbar or dock progress) in the launcher.
/// The progress refers to the current state of the pomodoro current interval (focus or break),
/// as tracked by the launcher. This allows the user to see the interval progress in the system UI.
///
/// # Implementation
/// Each OS should provide its own implementation of this trait to ensure that progress
/// is communicated using the appropriate system APIs.
///
/// # Methods
/// - `set_progress`: Sets the current progress value (typically between 0.0 and 1.0).
/// - `emit`: Applies or displays the current progress state to the system UI.
/// - `stop`: Stops or clears the progress indication.
pub trait ProgressIntegration {
    /// Sets the progress value for the launcher.
    ///
    /// # Arguments
    /// * `progress` - A floating point value representing the progress (e.g., 0.0 to 1.0).
    fn set_progress(&mut self, progress: f64) -> Result<(), ProgressError>;

    /// Emits or applies the current progress state to the system UI.
    fn emit(&self) -> Result<(), ProgressError>;

    /// Stops or clears the progress indication in the system UI.
    fn stop(&mut self) -> Result<(), ProgressError>;
}
