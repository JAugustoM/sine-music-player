mod audio_engine;

use audio_engine::AudioEngine;
use audio_engine::engine_enums::*;

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
    main_window.on_previous_button(move || {
        tx.send(PlayerCommands::SkipPrevious).unwrap();
    });

    let tx = command_sx.clone();
    main_window.on_next_button(move || {
        tx.send(PlayerCommands::SkipNext).unwrap();
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

    let tx = command_sx.clone();
    main_window.on_set_timestamp(move |value| {
        let duration = Duration::from_secs_f32(value);
        tx.send(PlayerCommands::Seek(duration)).unwrap();
    });

    let tx = command_sx.clone();
    main_window.on_toggle_repeat(move || {
        tx.send(PlayerCommands::ToggleRepeat).unwrap();
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

                let mut duration = 0;
                let mut title = "".to_string();
                let mut album = "".to_string();
                let mut album_artist = "".to_string();

                if let Some(music) = status.playlist.get(current_track) {
                    duration = music.length.as_secs();
                    title = music.title.clone();
                    album = music.album.clone();
                    album_artist = music.album_artist.clone();
                }

                let timestamp = status.timestamp.map_or_else(|| 0.0, |t| t.as_secs_f32());

                let time_str = format!("{}:{:02}", timestamp as i32 / 60, timestamp as i32 % 60);
                let duration_str = format!("{}:{:02}", duration / 60, duration % 60);

                let repeat_str = match status.repeat {
                    RepeatMode::Off => "Off",
                    RepeatMode::Track => "Track",
                    RepeatMode::Playlist => "Playlist",
                };

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_clone.upgrade() {
                        app.set_current_state(state_str.into());
                        app.set_current_timestamp(time_str.into());
                        app.set_music_length(duration_str.into());

                        app.set_music_title(title.into());
                        app.set_music_album(album.into());
                        app.set_music_album_artist(album_artist.into());

                        app.set_music_progress(timestamp);
                        app.set_music_duration(duration as i32);

                        app.set_repeat_mode(repeat_str.into());
                    }
                });
            }

            thread::sleep(Duration::from_millis(50));
        }
    });

    main_window.run()
}
