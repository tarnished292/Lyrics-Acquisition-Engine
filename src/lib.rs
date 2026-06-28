mod scanner;
mod metadata;
mod lyrics;

pub use scanner::scan_music;
pub use metadata::get_metadata;
pub use lyrics::fetch_lyrics;