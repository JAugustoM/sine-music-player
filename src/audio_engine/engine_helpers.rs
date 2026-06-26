use super::AudioEngine;
use super::Music;
use super::engine_enums::*;

impl AudioEngine {
    pub fn current_track(&self) -> &usize {
        &self.status.current_track
    }

    pub fn state(&self) -> &EngineState {
        &self.status.state
    }

    pub fn repeat(&self) -> &RepeatMode {
        &self.status.repeat
    }

    pub fn shuffle(&self) -> &ShuffleMode {
        &self.status.shuffle
    }

    pub fn playlist(&self) -> &Vec<Music> {
        &self.status.playlist
    }

    pub fn playlist_order(&self) -> &Vec<usize> {
        &self.status.playlist_order
    }
}
