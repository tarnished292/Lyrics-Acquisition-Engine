use std::path::PathBuf;

use id3::{Tag, TagLike};

pub struct SongMetadata {
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

pub fn get_metadata(path: &PathBuf) -> Option<SongMetadata>{
    let Ok(tag) = Tag::read_from_path(&path) else {
        return None;
    };

    Some(SongMetadata {
        path: path.clone(),
        title: tag.title().map(|t| t.to_string()),
        artist: tag.artist().map(|a| a.to_string()),
        album: tag.album().map(|a| a.to_string()),
    })
}
