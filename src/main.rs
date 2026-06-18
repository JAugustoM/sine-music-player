mod audio_engine;

use audio_engine::AudioEngine;
use audio_engine::enums::*;

use core::time;
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
    main_window.on_toggle_clicked(move || {
        tx.send(PlayerCommands::ToggleReproduction).unwrap();
    });

    let tx = command_sx.clone();
    main_window.on_clear_clicked(move || {
        tx.send(PlayerCommands::Clear).unwrap();
    });

    let tx = command_sx.clone();
    main_window.on_file_picker(move || {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut dialog = FileDialog::new().add_filter("Music files", &["opus", "mp3"]);
            if let Some(path) = dirs::audio_dir().or_else(|| dirs::home_dir()) {
                dialog = dialog.set_directory(path);
            }

            if let Some(file) = dialog.pick_file() {
                tx.send(PlayerCommands::Add(file)).unwrap();
            };
        });
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
                    EngineState::Empty => "No music on queue",
                    EngineState::Paused => "Paused",
                    EngineState::Playing => "Playing",
                };

                let current_track = status.current_track;
                let music = status.playlist.get(current_track);

                let timestamp = status.timestamp.map_or_else(|| 0, |t| t.as_secs());
                let duration = music.map_or_else(|| 0, |m| m.length.as_secs());

                let time_str = format!("{}:{:02}", timestamp / 60, timestamp % 60);
                let duration_str = format!("{}:{:02}", duration / 60, duration % 60);
                let music_progress: f32 = timestamp as f32 / duration as f32;

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_clone.upgrade() {
                        app.set_current_state(state_str.into());
                        app.set_current_timestamp(time_str.into());
                        app.set_music_duration(duration_str.into());
                        app.set_music_progress(music_progress);
                    }
                });
            }

            thread::sleep(Duration::from_millis(50));
        }
    });

    main_window.run()
}
