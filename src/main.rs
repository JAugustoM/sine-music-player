mod audio_engine;
mod callbacks;

use audio_engine::AudioEngine;
use audio_engine::engine_enums::*;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::callbacks::setup_callbacks;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    let mut handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

    handle.log_on_drop(false);

    let (command_sx, command_rx) = mpsc::channel();
    let (status_sx, status_rx) = mpsc::channel();

    AudioEngine::new(&handle, command_rx, status_sx).start();

    setup_callbacks(command_sx, &main_window);

    let app = main_window.as_weak();

    thread::spawn(move || {
        loop {
            let mut latest_status = None;

            while let Ok(status) = status_rx.try_recv() {
                latest_status = Some(status);
            }

            if let Some(status) = latest_status {
                if let Some(error) = status.error {
                    println!("{error}");
                }

                let app_clone = app.clone();

                let state_str = match status.state {
                    EngineState::Empty => "No music on queue",
                    EngineState::Paused => "Paused",
                    EngineState::Playing => "Playing",
                };

                let current_track = status.current_track;

                let (duration, song_details) = match status.playlist.get(current_track) {
                    None => {
                        (0, "".to_string())
                    }
                    Some(data) => {
                        (data.length.as_secs(), format!("{} - {} - {}", data.title.clone(), data.album.clone(), data.album_artist.clone()))
                    }
                };

                let (time_secs, time_str) = status.timestamp.map_or((0.0, "0:00".to_string()), |t| {
                    let secs = t.as_secs_f32();
                    (secs, format!("{}:{:02}", secs as i32 / 60, secs as i32 % 60))
                });

                let duration_str = format!("{}:{:02}", duration / 60, duration % 60);

                let repeat_mode = match status.repeat {
                    RepeatMode::Off => 0,
                    RepeatMode::Track => 1,
                    RepeatMode::Playlist => 2,
                };

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_clone.upgrade() {
                        app.set_current_state(state_str.into());
                        app.set_current_timestamp(time_str.into());
                        app.set_music_length(duration_str.into());

                        app.set_song_details(song_details.into());

                        app.set_music_progress(time_secs);
                        app.set_music_duration(duration as i32);

                        app.set_repeat_mode(repeat_mode);
                    }
                });
            }

            thread::sleep(Duration::from_millis(50));
        }
    });

    main_window.run()
}
