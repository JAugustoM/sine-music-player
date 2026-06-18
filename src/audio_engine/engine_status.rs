use super::MusicData;
use super::enums::EngineState;

use std::time::Duration;

#[derive(Clone)]
pub struct EngineStatus {
    pub current_music: Option<MusicData>,
    pub timestamp: Option<Duration>,
    pub state: EngineState,
    pub current_track: u32,
    pub music_queue: Vec<MusicData>,
}

impl EngineStatus {
    pub fn new() -> Self {
        let music_queue: Vec<MusicData> = Vec::new();
        EngineStatus {
            current_music: None,
            timestamp: None,
            state: EngineState::New,
            current_track: 0,
            music_queue,
        }
    }
}
