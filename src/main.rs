use notify_rust::{Hint, Notification, Urgency};

slint::slint! {
    export { System, Pomodoro } from "ui/main.slint";
}

fn main() -> Result<(), slint::PlatformError> {
    let main_window = Pomodoro::new()?;
    main_window.global::<System>().set_focus_duration(10);
    main_window.global::<System>().set_long_break_duration(15);
    main_window.global::<System>().set_short_break_duration(5);

    main_window.global::<System>().on_notify(|msg| {
        let summary = msg.summary.to_owned();
        let body = msg.body.to_owned();
        _ = Notification::new()
            .summary(&summary)
            .body(&body)
            .urgency(Urgency::Critical)
            .hint(Hint::Category("class".to_owned()))
            .appname("Temporis")
            .show()
            .map_err(|e| slint::PlatformError::Other(e.to_string()));
    });
    main_window.run()
}
