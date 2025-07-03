use notify_rust::{Hint, Notification};

use crate::platform_interfaces::notifications::{
    NotificationError, NotificationIntegration, OsMessage,
};

pub struct LinuxNotificationIntegration;

impl NotificationIntegration for LinuxNotificationIntegration {
    fn send(&self, message: &OsMessage) -> Result<(), NotificationError> {
        let res = Notification::new()
            .summary(&message.summary)
            .body(&message.body)
            .urgency(message.urgency.into())
            .hint(Hint::Category("class".to_owned()))
            .appname("Temporis")
            .icon("com.reciperium.temporis")
            .show();

        res.map_err(|err| NotificationError::SendFailed {
            reason: err.to_string(),
        })
        .map(|_| ())
    }
}
