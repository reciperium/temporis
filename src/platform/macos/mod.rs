mod notifications;
use crate::platform_interfaces::progress::{ProgressError, ProgressIntegration};

#[derive(Debug, Default, Clone, Copy)]
struct DummyProgress;

impl ProgressIntegration for DummyProgress {
    fn set_progress(&mut self, progress: f64) -> Result<(), ProgressError> {
        Ok(())
    }

    fn emit(&self) -> Result<(), ProgressError> {
        Ok(())
    }

    fn stop(&mut self) -> Result<(), ProgressError> {
        Ok(())
    }
}

pub fn get_progress_integration() -> impl ProgressIntegration {
    progress::UnityLauncher::default()
}

pub fn get_notifications_integration() -> impl NotificationIntegration {
    notifications::MacOsNotificationIntegration
}
