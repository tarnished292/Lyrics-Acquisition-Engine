use lyrics_acquisition_engine::scan_music;

fn main() {

    let dir = dirs::audio_dir().unwrap_or_else(|| {
        let mut fallback = dirs::home_dir().unwrap();
        fallback.push("Music");
        fallback
    });
    
    
    let songs = scan_music(&dir);
    println!("{:?}", songs)
}
