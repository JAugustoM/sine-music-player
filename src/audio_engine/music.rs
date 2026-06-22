use std::{path::PathBuf, time::Duration};

use anyhow::{Context};
use lofty::prelude::*;
use lofty::probe::Probe;

#[derive(Clone)]
pub struct Music {
    pub path: PathBuf,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub length: Duration,
}

impl Music {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let probe = Probe::open(&path)?;
        let tag_file =  probe.read().with_context(|| format!("Failed to read metadata from {:?}", path.file_name()))?;

        let properties = tag_file.properties();
        let length = properties.duration();

        let tags = match tag_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => tag_file.first_tag().unwrap(),
        };

        let title = tags.title().as_deref().unwrap_or("None").to_string();
        let artist = tags.artist().as_deref().unwrap_or("None").to_string();
        let album = tags.album().as_deref().unwrap_or("None").to_string();

        let album_artist = tags
            .get_string(ItemKey::AlbumArtist)
            .unwrap_or("None")
            .to_string();

        Ok(Music {
            path,
            title,
            artist,
            album,
            album_artist,
            length,
        })
    }
}
