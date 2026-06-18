use super::Music;
use super::enums::EngineState;
use super::enums::RepeatMode;

use std::time::Duration;

#[derive(Clone)]
pub struct EngineStatus {
    pub timestamp: Option<Duration>,
    pub state: EngineState,
    pub current_track: usize,
    pub playlist: Vec<Music>,
    pub repeat: RepeatMode,
}

impl EngineStatus {
    pub fn new() -> Self {
        let playlist: Vec<Music> = Vec::new();
        EngineStatus {
            timestamp: None,
            state: EngineState::Empty,
            current_track: 0,
            playlist,
            repeat: RepeatMode::Off,
        }
    }
}
