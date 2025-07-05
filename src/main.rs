use std::{cell::RefCell, io::Cursor, rc::Rc};

use rodio::{Decoder, OutputStream, Sink};
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
    export { ExternalSystem, AppWindow, Interval } from "ui/main.slint";
}

pub const TICK: &[u8] = include_bytes!("../assets/audio/tick.flac");

fn main() -> Result<(), slint::PlatformError> {
    let config = Config::new().expect("config should be loaded correctly");
    let main_window = AppWindow::new()?;

    // Initialize audio sink
    let (_audio_stream, audio_handle) = OutputStream::try_default().unwrap();
    let audio_sink = Rc::new(Sink::try_new(&audio_handle).unwrap());
    audio_sink.set_volume(0.5);

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
        .set_critical_notifications(config.critical_notifications);
    main_window
        .global::<ExternalSystem>()
        .set_sessions(config.sessions);

    main_window
        .global::<ExternalSystem>()
        .set_tick_sound(config.tick_sound);

    let shared_cfg = Rc::new(RefCell::new(config));

    let cfg_clone = shared_cfg.clone();
    main_window
        .global::<ExternalSystem>()
        .on_save_focus(move |value: i32| {
            let mut cfg_mut = cfg_clone.borrow_mut();
            cfg_mut.focus_duration = value;
            let r = cfg_mut.save();
            if let Err(e) = r {
                eprintln!("Error saving focus duration: {}", e);
            }
        });

    let cfg_clone = shared_cfg.clone();
    main_window
        .global::<ExternalSystem>()
        .on_save_short_break(move |value| {
            let mut cfg_mut = cfg_clone.borrow_mut();
            cfg_mut.short_break_duration = value;
            let r = cfg_mut.save();
            if let Err(e) = r {
                eprintln!("Error saving short break duration: {}", e);
            }
        });

    let cfg_clone = shared_cfg.clone();
    main_window
        .global::<ExternalSystem>()
        .on_save_long_break(move |value| {
            let mut cfg_mut = cfg_clone.borrow_mut();
            cfg_mut.long_break_duration = value;
            let r = cfg_mut.save();
            if let Err(e) = r {
                eprintln!("Error saving long break duration: {}", e);
            }
        });

    let cfg_clone = shared_cfg.clone();
    main_window
        .global::<ExternalSystem>()
        .on_save_sessions(move |value| {
            let mut cfg_mut = cfg_clone.borrow_mut();
            cfg_mut.sessions = value;
            let r = cfg_mut.save();
            if let Err(e) = r {
                eprintln!("Error saving sessions amount: {}", e);
            }
        });

    let cfg_clone = shared_cfg.clone();
    main_window
        .global::<ExternalSystem>()
        .on_save_critical_notifications(move |value| {
            let mut cfg_mut = cfg_clone.borrow_mut();
            cfg_mut.critical_notifications = value;
            let r = cfg_mut.save();
            if let Err(e) = r {
                eprintln!("Error saving bypass dnd: {}", e);
            }
        });
    let cfg_clone = shared_cfg.clone();
    main_window
        .global::<ExternalSystem>()
        .on_save_tick_sound(move |value| {
            let mut cfg_mut = cfg_clone.borrow_mut();
            cfg_mut.tick_sound = value;
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

    let cfg_clone = shared_cfg.clone();
    main_window.global::<ExternalSystem>().on_tick(
        move |current_interval| match current_interval {
            Interval::Focus => {
                if cfg_clone.borrow().tick_sound {
                    let source = Decoder::new(Cursor::new(TICK)).unwrap();
                    audio_sink.append(source);
                }
            }
            _ => {}
        },
    );

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
    main_window.global::<ExternalSystem>().on_exit(move || {
        _ = progress_integration.borrow_mut().stop();
        _ = progress_integration.borrow().emit();
        std::process::exit(0)
    });

    let progress_integration = shared_progress_integration.clone();
    main_window.window().on_close_requested(move || {
        _ = progress_integration.borrow_mut().stop();
        _ = progress_integration.borrow().emit();
        CloseRequestResponse::HideWindow
    });

    main_window.run()
}
