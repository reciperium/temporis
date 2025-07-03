use thiserror::Error;

/// Represents a message to be shown to the user via the system notification service.
#[derive(Debug, Clone)]
pub struct OsMessage {
    /// The summary or title of the message.
    pub summary: String,
    /// The body or content of the message.
    pub body: String,
    /// The urgency level of the message (optional, not supported on all platforms).
    pub urgency: Urgency,
}

impl OsMessage {
    /// Creates a new `Message`.
    ///
    /// # Arguments
    /// * `summary` - The summary or title of the message.
    /// * `body` - The body or content of the message.
    /// * `urgency` - The urgency level of the message (optional).
    pub fn new<S: Into<String>>(summary: S, body: S, urgency: Urgency) -> Self {
        Self {
            summary: summary.into(),
            body: body.into(),
            urgency,
        }
    }
}

/// Represents the urgency level of a message.
#[derive(Debug, Clone, Copy)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

impl Into<notify_rust::Urgency> for Urgency {
    fn into(self) -> notify_rust::Urgency {
        match self {
            Urgency::Low => notify_rust::Urgency::Low,
            Urgency::Normal => notify_rust::Urgency::Normal,
            Urgency::Critical => notify_rust::Urgency::Critical,
        }
    }
}

#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("Failed to send notification: {reason}")]
    SendFailed { reason: String },
}

/// Trait for integrating notifications with the operating system's native notification system.
///
/// This trait must be implemented for each supported operating system (e.g., Linux, macOS).
/// The implementation should use the provided `Message` struct to display notifications.
/// Note: On macOS, the urgency field should be ignored, as it is not supported.
pub trait NotificationIntegration {
    /// Sends a message to the user using the system's native notification service.
    ///
    /// # Arguments
    /// * `message` - The message to be displayed.
    ///
    /// # Errors
    /// Returns `NotificationError` if the notification could not be sent.
    fn send(&self, message: &OsMessage) -> Result<(), NotificationError>;
}
