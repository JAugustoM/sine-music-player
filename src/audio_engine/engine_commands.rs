use super::AudioEngine;
use super::Music;
use super::engine_enums::*;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Context;
use rand::seq::SliceRandom;
use rodio::Decoder;
use walkdir::{DirEntry, WalkDir};

const SUPPORTED_EXTENSIONS: &[&str] = &["opus", "mp3", "flac", "wav", "ogg", "m4a"];

fn is_music_file(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            SUPPORTED_EXTENSIONS
                .iter()
                .any(|&e| e.eq_ignore_ascii_case(ext))
        })
        .unwrap_or(false)
}

impl AudioEngine {
    pub fn add_music(&mut self, path: PathBuf) {
        let music = match Music::new(path) {
            Ok(m) => m,
            Err(e) => {
                self.status.error = Some(e.to_string());
                return;
            }
        };

        self.status.playlist.push(music);

        let playlist_len = self.playlist().len();

        self.status.playlist_order.push(playlist_len - 1);

        if *self.state() == EngineState::Empty {
            self.play_music();
        }
    }

    pub fn add_folder(&mut self, path: PathBuf) {
        let walker = WalkDir::new(path).into_iter();
        for entry in walker.filter_map(Result::ok).filter(|e| is_music_file(e)) {
            let path = PathBuf::from(entry.path());
            self.add_music(path);
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

    pub fn toggle_shuffle(&mut self) {
        match self.status.shuffle {
            ShuffleMode::Off => {
                let mut rng = rand::rng();
                let current_track = self.status.current_track;

                self.status.playlist_order.swap(current_track, 0);
                self.status.current_track = 0;
                self.status.playlist_order[1..].shuffle(&mut rng);
                self.status.shuffle = ShuffleMode::On;
            }
            ShuffleMode::On => {
                let current_track = self.status.current_track;
                let current_index = self.status.playlist_order[current_track];

                self.status.playlist_order = (0..self.playlist().len()).collect();
                self.status.current_track = current_index;
                self.status.shuffle = ShuffleMode::Off;
            }
        }
    }

    pub fn play_music(&mut self) {
        let index = self.status.current_track;
        let music_index = self.status.playlist_order[index];

        let music = &self.status.playlist[music_index];

        let source = match self.load_file(&music.path) {
            Ok(s) => s,
            Err(e) => {
                self.status.error = Some(e.to_string());
                return;
            }
        };

        self.player.clear();
        self.player.append(source);
        self.player.play();
        self.status.state = EngineState::Playing;
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
        } else if *self.repeat() == RepeatMode::Playlist {
            self.status.current_track = 0;
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
        } else if *self.repeat() == RepeatMode::Playlist {
            self.status.current_track = self.playlist().len() - 1;
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

        let source = match self.load_file(&music.path) {
            Ok(s) => s,
            Err(e) => {
                self.status.error = Some(e.to_string());
                return;
            }
        };

        let was_playing = *self.state() == EngineState::Playing;

        self.player.clear();

        self.player.append(source);
        std::thread::sleep(std::time::Duration::from_millis(50));

        if let Err(e) = self
            .player
            .try_seek(duration)
            .context("Failed to seek music")
        {
            self.status.error = Some(e.to_string());
        }

        if was_playing {
            self.player.play();
        } else {
            self.player.pause();
        }
    }

    pub fn load_file(&self, path: &PathBuf) -> anyhow::Result<Decoder<BufReader<File>>> {
        let file = File::open(&path)
            .with_context(|| format!("Failed to read file {:?}", path.file_name()))?;

        let buffer = BufReader::new(file);

        let source = Decoder::try_from(buffer)
            .with_context(|| format!("Failed to decode file {:?}", path.file_name()))?;

        Ok(source)
    }
}
