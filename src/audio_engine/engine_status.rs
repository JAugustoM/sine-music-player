use super::Music;
use super::engine_enums::*;

use std::time::Duration;

#[derive(Clone)]
pub struct EngineStatus {
    pub timestamp: Option<Duration>,
    pub state: EngineState,
    pub current_track: usize,
    pub playlist: Vec<Music>,
    pub playlist_order: Vec<usize>,
    pub repeat: RepeatMode,
    pub shuffle: ShuffleMode,
    pub error: Option<String>,
}

impl EngineStatus {
    pub fn new() -> Self {
        let playlist: Vec<Music> = Vec::new();
        let playlist_order: Vec<usize> = Vec::new();
        EngineStatus {
            timestamp: None,
            state: EngineState::Empty,
            current_track: 0,
            playlist,
            playlist_order,
            repeat: RepeatMode::Off,
            shuffle: ShuffleMode::Off,
            error: None,
        }
    }
}
