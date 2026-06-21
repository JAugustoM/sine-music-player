use super::AudioEngine;
use super::Music;
use super::engine_enums::*;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use rodio::Decoder;

impl AudioEngine {
    pub fn add_music(&mut self, path: PathBuf) {
        let music = Music::new(path.clone());

        match music {
            Ok(music) => self.status.playlist.push(music),
            Err(e) => {
                println!("{e}");
                return;
            }
        }

        if *self.state() == EngineState::Empty {
            self.play_music();
        }
    }

    pub fn clear_music(&mut self) {
        self.player.clear();
        self.status.current_track = 0;
        self.status.playlist.clear();
        self.status.state = EngineState::Empty;
        self.status.timestamp = None;
    }

    pub fn toggle_reproduction(&mut self) {
        match self.status.state {
            EngineState::Empty => return,
            EngineState::Playing => {
                self.player.pause();
                self.status.state = EngineState::Paused;
            }
            EngineState::Paused => {
                self.player.play();
                self.status.state = EngineState::Playing;
            }
        }
    }

    pub fn toggle_repeat(&mut self) {
        match self.status.repeat {
            RepeatMode::Off => self.status.repeat = RepeatMode::Track,
            RepeatMode::Track => self.status.repeat = RepeatMode::Playlist,
            RepeatMode::Playlist => self.status.repeat = RepeatMode::Off,
        }
    }

    pub fn play_music(&mut self) {
        let index = self.status.current_track;

        let music = &self.status.playlist[index];

        if let Ok(source) = self.load_file(&music.path) {
            self.player.clear();
            self.player.append(source);
            self.player.play();
            self.status.state = EngineState::Playing;
        }
    }

    pub fn skip_next(&mut self) {
        let playlist = self.playlist();
        let track_id = self.current_track();

        if self.player.empty() || playlist.is_empty() {
            return;
        }

        if *track_id < playlist.len() - 1 {
            self.status.current_track += 1;
            self.play_music();
        }
    }

    pub fn skip_previous(&mut self) {
        let playlist = self.playlist();
        let track_id = self.current_track();

        if self.player.empty() || playlist.is_empty() {
            return;
        }

        if *track_id > 0 {
            self.status.current_track -= 1;
            self.play_music();
        }
    }

    pub fn seek(&mut self, duration: Duration) {
        let playlist = self.playlist();

        if playlist.is_empty() {
            return;
        }

        let index = *self.current_track();
        let music = &playlist[index];

        if let Ok(source) = self.load_file(&music.path) {
            let was_playing = *self.state() == EngineState::Playing;

            self.player.clear();

            self.player.append(source);
            std::thread::sleep(std::time::Duration::from_millis(50));

            if let Err(e) = self.player.try_seek(duration) {
                println!("Failed to seek: {e}");
            }

            if was_playing {
                self.player.play();
            } else {
                self.player.pause();
            }

            self.status.timestamp = Some(duration);
        }
    }

    pub fn load_file(&self, path: &PathBuf) -> Result<Decoder<BufReader<File>>> {
        let file = File::open(path)?;
        println!("{:?}", path.file_name());
        let buffer = BufReader::new(file);

        let source = Decoder::try_from(buffer)?;

        Ok(source)
    }
}
