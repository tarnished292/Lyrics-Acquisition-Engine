mod scanner;
mod metadata;
mod lyrics;
mod writer;

pub use scanner::scan_music;
pub use metadata::get_metadata;
pub use lyrics::fetch_lyrics;
pub use writer::write_lrc;