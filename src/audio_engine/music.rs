use std::{path::PathBuf, time::Duration};

use anyhow::{Context};
use lofty::picture::PictureType;
use lofty::prelude::*;
use lofty::probe::Probe;

#[derive(Clone)]
pub struct Music {
    pub path: PathBuf,
    pub title: String,
    pub album: String,
    pub album_artist: String,
    pub length: Duration,
    pub cover_bytes: Option<Vec<u8>>,
}

impl Music {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let probe = Probe::open(&path).context("Could not open audio file for probing")?;

        let tag_file =  probe.read().context("Could not read audio file properties")?;
        let properties = tag_file.properties();
        let length = properties.duration();

        let default_title = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut title = default_title.clone();
        let mut album = String::from("Unknown Album");
        let mut album_artist = String::from("Unknown Artist");
        let mut cover_bytes = None;

        if let Some(tags) = tag_file.primary_tag().or_else(|| tag_file.first_tag()) {
            
            // Extract text data safely without unwrap()
            if let Some(t) = tags.title() { title = t.to_string(); }
            if let Some(al) = tags.album() { album = al.to_string(); }
            
            if let Some(aa) = tags.get_string(ItemKey::AlbumArtist) {
                album_artist = aa.to_string();
            }

            for pic in tags.pictures() {
                if pic.pic_type() == PictureType::CoverFront {
                    cover_bytes = Some(pic.data().to_vec());
                    break;
                }
            }
            
            if cover_bytes.is_none() {
                if let Some(pic) = tags.pictures().first() {
                    cover_bytes = Some(pic.data().to_vec());
                }
            }
        }

        Ok(Music {
            path,
            title,
            album,
            album_artist,
            length,
            cover_bytes
        })
    }
}
