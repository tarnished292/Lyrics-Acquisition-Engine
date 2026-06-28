use std::path::PathBuf;
use lofty::file::TaggedFileExt;
use lofty::probe::Probe;
use lofty::tag::Accessor;

pub struct SongMetadata {
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

pub fn get_metadata(path: &PathBuf) -> Option<SongMetadata> {
    let tagged_file = Probe::open(path).ok()?.read().ok()?;
    let tag = tagged_file.primary_tag().or_else(|| tagged_file.first_tag())?;

    Some(SongMetadata {
        path: path.clone(),
        title: tag.title().map(|t| t.to_string()),
        artist: tag.artist().map(|a| a.to_string()),
        album: tag.album().map(|a| a.to_string()),
    })
}