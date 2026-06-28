use lyrics_acquisition_engine::{fetch_lyrics, scan_music, write_lrc};

#[tokio::main]
async fn main() {
    let dir = dirs::audio_dir().unwrap_or_else(|| {
        let mut fallback = dirs::home_dir().unwrap();
        fallback.push("Music");
        fallback
    });

    let songs = scan_music(&dir);

    for song in songs {
        let lrc_path = song.path.with_extension("lrc");
    
       if lrc_path.exists() {
           println!("Skipping {:?}: .lrc already exists", song.title);
           continue;
       }
       
        println!("Searching: {:?} - {:?}", song.artist, song.title);

        if let Some(lyrics) = fetch_lyrics(&song).await {
            println!("{}", lyrics);
            write_lrc(&song, &lyrics);
        } else {
            println!("No lyrics found.\n");
        }
    }
}
