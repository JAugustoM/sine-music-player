use std::time::Duration;

#[derive(Clone)]
pub struct MusicData {
    title: String,
    album: String,
    album_artist: String,
    length: Duration,
}
