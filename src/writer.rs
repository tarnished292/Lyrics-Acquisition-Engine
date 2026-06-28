use std::fs;


use crate::metadata::SongMetadata;

pub fn write_lrc(song: &SongMetadata, lyrics: &str) {
    let path = &song.path.with_extension("lrc");
    fs::write(&path, lyrics);
    println!("Song Path: {:?}", path);
}