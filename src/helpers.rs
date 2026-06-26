use slint::{Rgba8Pixel, SharedPixelBuffer};

use crate::audio_engine::engine_enums::ShuffleMode;

use super::audio_engine::engine_enums::{EngineState, RepeatMode};
use super::audio_engine::engine_status::EngineStatus;

fn decode_cover_art(data: &[u8]) -> Option<SharedPixelBuffer<Rgba8Pixel>> {
    let dynamic_image = image::load_from_memory(data).ok()?;
    let rgba_image = dynamic_image.into_rgba8();

    Some(SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        rgba_image.as_raw(),
        rgba_image.width(),
        rgba_image.height(),
    ))
}

pub struct LightExtracted {
    pub state: String,
    pub timestamp: String,
    pub music_progress: f32,
    pub repeat: String,
    pub shuffle: bool,
}

pub struct HeavyExtracted {
    pub final_timestamp: String,
    pub music_title: String,
    pub music_album: String,
    pub music_album_artist: String,
    pub music_duration: f32,
    pub cover: Option<SharedPixelBuffer<Rgba8Pixel>>,
}

pub fn light_extract(status: &EngineStatus) -> LightExtracted {
    let (music_progress, timestamp) = status.timestamp.map_or((0.0, "0:00".to_string()), |t| {
        let secs = t.as_secs_f32();
        (
            secs,
            format!("{}:{:02}", secs as i32 / 60, secs as i32 % 60),
        )
    });

    let state = match status.state {
        EngineState::Empty => "No music on queue",
        EngineState::Paused => "Paused",
        EngineState::Playing => "Playing",
    }
    .to_string();

    let repeat = match status.repeat {
        RepeatMode::Off => "Off",
        RepeatMode::Track => "Track",
        RepeatMode::Playlist => "Playlist",
    }
    .to_string();

    let shuffle = match status.shuffle {
        ShuffleMode::Off => false,
        ShuffleMode::On => true,
    };

    LightExtracted {
        music_progress,
        timestamp,
        state,
        repeat,
        shuffle,
    }
}

pub fn heavy_extract(status: &EngineStatus) -> HeavyExtracted {
    let current_index = status.current_track;
    let current_track = status.playlist_order[current_index];

    let mut music_duration = 0.0;
    let mut music_title = String::from("");
    let mut music_album = String::from("");
    let mut music_album_artist = String::from("");
    let mut cover = None;

    if let Some(data) = status.playlist.get(current_track) {
        music_duration = data.length.as_secs_f32();
        music_title = data.title.clone();
        music_album = data.album.clone();
        music_album_artist = data.album_artist.clone();

        cover = if let Some(bytes) = &data.cover_bytes {
            decode_cover_art(bytes)
        } else {
            None
        };
    }

    let final_timestamp = format!(
        "{}:{:02}",
        music_duration as i32 / 60,
        music_duration as i32 % 60
    );

    HeavyExtracted {
        final_timestamp,
        music_title,
        music_album,
        music_album_artist,
        music_duration,
        cover,
    }
}
