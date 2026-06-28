use lyrics_acquisition_engine::{fetch_lyrics, scan_music};

#[tokio::main]
async fn main() {
    let dir = dirs::audio_dir().unwrap_or_else(|| {
        let mut fallback = dirs::home_dir().unwrap();
        fallback.push("Music");
        fallback
    });

    let songs = scan_music(&dir);

    for song in songs {
        println!("Searching: {:?} - {:?}", song.artist, song.title);

        if let Some(lyrics) = fetch_lyrics(&song).await {
            println!("{}", lyrics);
        } else {
            println!("No lyrics found.\n");
        }
    }
}
