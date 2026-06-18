use super::AudioEngine;
use super::Music;
use super::enums::*;

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

    pub fn playlist(&self) -> &Vec<Music> {
        &self.status.playlist
    }
}
