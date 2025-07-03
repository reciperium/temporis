mod notifications;
mod progress;

use crate::platform_interfaces::notifications::NotificationIntegration;
use crate::platform_interfaces::progress::ProgressIntegration;

pub fn get_progress_integration() -> impl ProgressIntegration {
    progress::UnityLauncher::default()
}

pub fn get_notifications_integration() -> impl NotificationIntegration {
    notifications::LinuxNotificationIntegration
}
