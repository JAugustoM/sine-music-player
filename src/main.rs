mod audio_engine;
mod callbacks;
mod helpers;

use audio_engine::AudioEngine;
use audio_engine::engine_enums::EngineState;
use callbacks::setup_callbacks;
use helpers::{heavy_extract, light_extract};

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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
        let mut last_track_index: Option<usize> = None;
        let mut was_empty = true;

        loop {
            let mut latest_status = None;

            while let Ok(status) = status_rx.try_recv() {
                latest_status = Some(status);
            }

            if let Some(status) = latest_status {
                if let Some(error) = &status.error {
                    println!("{error}");
                }

                let app_clone = app.clone();

                let is_empty = matches!(status.state, EngineState::Empty);
                let track_changed =
                    last_track_index != Some(status.current_track) || (was_empty && !is_empty);

                last_track_index = Some(status.current_track);
                was_empty = is_empty;

                let heavy_extract = if track_changed && !is_empty {
                    Some(heavy_extract(&status))
                } else {
                    None
                };

                let extract = light_extract(&status);

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_clone.upgrade() {
                        app.set_current_state(extract.state.into());
                        app.set_current_timestamp(extract.timestamp.into());
                        app.set_music_progress(extract.music_progress);
                        app.set_repeat_mode(extract.repeat.into());

                        if track_changed {
                            if let Some(metadata) = heavy_extract {
                                app.set_final_timestamp(metadata.final_timestamp.into());

                                app.set_music_title(metadata.music_title.into());
                                app.set_music_album(metadata.music_album.into());
                                app.set_music_album_artist(metadata.music_album_artist.into());

                                app.set_music_duration(metadata.music_duration);

                                if let Some(buffer) = metadata.cover {
                                    app.set_cover_art(slint::Image::from_rgba8(buffer));
                                } else {
                                    let empty_buffer =
                                        slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(1, 1);
                                    app.set_cover_art(slint::Image::from_rgba8(empty_buffer));
                                }
                            } else if is_empty {
                                app.set_music_title("None".into());
                                app.set_music_album("None".into());
                                app.set_music_album_artist("None".into());
                                app.set_music_duration(0.0);
                                app.set_final_timestamp("0:00".into());

                                let empty =
                                    slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(1, 1);
                                app.set_cover_art(slint::Image::from_rgba8(empty));
                            }
                        }
                    }
                });
            }

            thread::sleep(Duration::from_millis(50));
        }
    });

    main_window.run()
}
