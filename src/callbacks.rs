use super::audio_engine::engine_enums::PlayerCommands;
use super::{AppWindow, MusicControl};

use std::{sync::mpsc::Sender, thread, time::Duration};

use rfd::FileDialog;
use slint::ComponentHandle;

pub fn setup_callbacks(sender: Sender<PlayerCommands>, main_window: &AppWindow) {
    let control = main_window.global::<MusicControl>();

    let tx = sender.clone();
    control.on_toggle_clicked(move || {
        tx.send(PlayerCommands::ToggleReproduction).unwrap();
    });

    // let tx = sender.clone();
    // main_window.on_clear_clicked(move || {
    //     tx.send(PlayerCommands::Clear).unwrap();
    // });

    let tx = sender.clone();
    control.on_previous_button(move || {
        tx.send(PlayerCommands::SkipPrevious).unwrap();
    });

    let tx = sender.clone();
    control.on_next_button(move || {
        tx.send(PlayerCommands::SkipNext).unwrap();
    });

    let tx = sender.clone();
    control.on_pick_music(move || {
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

    let tx = sender.clone();
    control.on_pick_folder(move || {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut dialog = FileDialog::new();
            if let Some(path) = dirs::audio_dir().or_else(|| dirs::home_dir()) {
                dialog = dialog.set_directory(path);
            }

            if let Some(folder) = dialog.pick_folder() {
                tx.send(PlayerCommands::AddFolder(folder)).unwrap();
            };
        });
    });

    let tx = sender.clone();
    control.on_set_timestamp(move |value| {
        let duration = Duration::from_secs_f32(value);
        tx.send(PlayerCommands::Seek(duration)).unwrap();
    });

    let tx = sender.clone();
    control.on_toggle_repeat(move || {
        tx.send(PlayerCommands::ToggleRepeat).unwrap();
    });
}
