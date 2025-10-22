use notify_rust::Notification;

use crate::platform_interfaces::notifications::{
    NotificationError, NotificationIntegration, OsMessage,
};

pub struct MacOsNotificationIntegration;

impl NotificationIntegration for MacOsNotificationIntegration {
    fn send(&self, message: &OsMessage) -> Result<(), NotificationError> {
        notify_rust::set_application("com.reciperium.temporis");
        let res = Notification::new()
            .summary(&message.summary)
            .body(&message.body)
            .appname("Temporis")
            .show();

        res.map_err(|err| NotificationError::SendFailed {
            reason: err.to_string(),
        })
        .map(|_| ())
    }
}
