mod audio_engine;

use audio_engine::AudioEngine;
use audio_engine::enums::*;

use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rfd::FileDialog;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    let mut handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

    handle.log_on_drop(false);

    let (command_sx, command_rx) = mpsc::channel();
    let (status_sx, status_rx) = mpsc::channel();

    AudioEngine::new(&handle, command_rx, status_sx).start();

    let tx = command_sx.clone();
    main_window.on_play_clicked(move || {
        tx.send(PlayerCommands::Play).unwrap();
    });

    let tx = command_sx.clone();
    main_window.on_pause_clicked(move || {
        tx.send(PlayerCommands::Pause).unwrap();
    });

    let tx = command_sx.clone();
    main_window.on_clear_clicked(move || {
        tx.send(PlayerCommands::Clear).unwrap();
    });

    let tx = command_sx.clone();
    main_window.on_file_picker(move || {
        let file = FileDialog::new()
            .add_filter("Music files", &["opus", "mp3"])
            .pick_file();

        if let Some(file) = file {
            tx.send(PlayerCommands::Load(file)).unwrap();
        };
    });

    let app = main_window.as_weak();

    thread::spawn(move || {
        loop {
            let mut latest_status = None;

            while let Ok(status) = status_rx.try_recv() {
                latest_status = Some(status);
            }

            if let Some(status) = latest_status {
                let app_clone = app.clone();

                let state_str = match status.state {
                    EngineState::Empty => "No queue",
                    EngineState::New => "No music added",
                    EngineState::Paused => "Paused",
                    EngineState::Playing => "Playing",
                };

                let time_str = if let Some(t) = status.timestamp {
                    let secs = t.as_secs();
                    format!("{}:{:02}", secs / 60, secs % 60)
                } else {
                    "00:00".to_string()
                };

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_clone.upgrade() {
                        app.set_current_timestamp(time_str.into());
                        app.set_current_state(state_str.into());
                    }
                });
            }

            thread::sleep(Duration::from_millis(50));
        }
    });

    main_window.run()
}
