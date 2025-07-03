use std::{cell::RefCell, rc::Rc};

use slint::{CloseRequestResponse, set_xdg_app_id};
use temporis::{
    conf::Config,
    platform::{get_notifications_integration, get_progress_integration},
    platform_interfaces::{
        notifications::{NotificationIntegration, OsMessage, Urgency},
        progress::ProgressIntegration,
    },
};

slint::slint! {
    export { ExternalSystem, AppWindow } from "ui/main.slint";
}

fn main() -> Result<(), slint::PlatformError> {
    let config = Config::new().expect("config should be loaded correctly");
    let main_window = AppWindow::new()?;

    _ = set_xdg_app_id("com.reciperium.temporis");

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
                eprintln!("Error saving short break duration: {}", e);
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
                eprintln!("Error saving long break duration: {}", e);
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
                eprintln!("Error saving sessions amount: {}", e);
            }
        });

    let cfg_clone5 = shared_cfg.clone();
    main_window
        .global::<ExternalSystem>()
        .on_save_bypass_dnd(move |value| {
            let mut cfg_mut = cfg_clone5.borrow_mut();
            cfg_mut.bypass_dnd = value;
            let r = cfg_mut.save();
            if let Err(e) = r {
                eprintln!("Error saving bypass dnd: {}", e);
            }
        });

    main_window.global::<ExternalSystem>().on_notify(|msg| {
        let summary = msg.summary.to_owned();
        let body = msg.body.to_owned();
        let urgency = if msg.critical {
            Urgency::Critical
        } else {
            Urgency::Normal
        };
        let message = OsMessage::new(summary, body, urgency);
        let notification = get_notifications_integration();
        let res = notification.send(&message);

        if let Err(e) = res {
            eprintln!("Error sending notification: {}", e);
        }
    });
    let shared_progress_integration = Rc::new(RefCell::new(get_progress_integration()));
    let progress_integration = shared_progress_integration.clone();
    main_window
        .global::<ExternalSystem>()
        .on_update_progress(move |progress| {
            let progress: f64 = progress as f64;
            _ = progress_integration.borrow_mut().set_progress(progress);
            _ = progress_integration.borrow().emit();
        });

    let progress_integration = shared_progress_integration.clone();
    main_window
        .global::<ExternalSystem>()
        .on_stop_progress(move || {
            _ = progress_integration.borrow_mut().stop();
            _ = progress_integration.borrow().emit();
        });

    let progress_integration = shared_progress_integration.clone();
    main_window.window().on_close_requested(move || {
        _ = progress_integration.borrow_mut().stop();
        _ = progress_integration.borrow().emit();
        CloseRequestResponse::HideWindow
    });

    main_window.run()
}
