use std::{cell::RefCell, rc::Rc};

use notify_rust::{Hint, Notification, Urgency};
use temporis::conf::Config;

slint::slint! {
    export { ExternalSystem, AppWindow } from "ui/main.slint";
}

fn main() -> Result<(), slint::PlatformError> {
    let config = Config::new().expect("config should be loaded correctly");
    let main_window = AppWindow::new()?;
    main_window
        .global::<ExternalSystem>()
        .set_focus_duration(config.focus_duration);
    main_window
        .global::<ExternalSystem>()
        .set_long_break_duration(config.long_break_duration);
    main_window
        .global::<ExternalSystem>()
        .set_short_break_duration(config.short_break_duration);
    main_window
        .global::<ExternalSystem>()
        .set_sessions(config.sessions);

    let shared_cfg = Rc::new(RefCell::new(config));

    let cfg_clone1 = shared_cfg.clone();
    main_window
        .global::<ExternalSystem>()
        .on_save_focus(move |value: i32| {
            let mut cfg_mut = cfg_clone1.borrow_mut();
            cfg_mut.focus_duration = value;
            let r = cfg_mut.save();
            if let Err(e) = r {
                eprintln!("Error saving focus duration: {}", e);
            }
        });

    let cfg_clone2 = shared_cfg.clone();
    main_window
        .global::<ExternalSystem>()
        .on_save_short_break(move |value| {
            let mut cfg_mut = cfg_clone2.borrow_mut();
            cfg_mut.short_break_duration = value;
            let r = cfg_mut.save();
            if let Err(e) = r {
                eprintln!("Error saving focus duration: {}", e);
            }
        });

    let cfg_clone3 = shared_cfg.clone();
    main_window
        .global::<ExternalSystem>()
        .on_save_long_break(move |value| {
            let mut cfg_mut = cfg_clone3.borrow_mut();
            cfg_mut.long_break_duration = value;
            let r = cfg_mut.save();
            if let Err(e) = r {
                eprintln!("Error saving focus duration: {}", e);
            }
        });

    let cfg_clone4 = shared_cfg.clone();
    main_window
        .global::<ExternalSystem>()
        .on_save_sessions(move |value| {
            let mut cfg_mut = cfg_clone4.borrow_mut();
            cfg_mut.sessions = value;
            let r = cfg_mut.save();
            if let Err(e) = r {
                eprintln!("Error saving focus duration: {}", e);
            }
        });

    main_window.global::<ExternalSystem>().on_notify(|msg| {
        let summary = msg.summary.to_owned();
        let body = msg.body.to_owned();
        let res = Notification::new()
            .summary(&summary)
            .body(&body)
            .urgency(Urgency::Critical)
            .hint(Hint::Category("class".to_owned()))
            .appname("Temporis")
            .show();
        if let Err(e) = res {
            eprintln!("Error sending notification: {}", e);
        }
    });
    main_window.run()
}
