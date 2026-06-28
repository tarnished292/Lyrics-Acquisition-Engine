use std::{fs, path::PathBuf};


const EXT: [&str; 3] = ["mp3", "flac", "opus"];
pub fn scan_music(dir: &PathBuf) -> Vec<PathBuf> {
    let music = fs::read_dir(&dir).unwrap();
    let mut songs = Vec::new();

    for file in music {
        let entry = file.unwrap();
        let path = entry.path();

        if path.is_dir() {
            songs.append(&mut scan_music(&path));
            continue;
        } else if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if EXT.contains(&extension) {
                songs.push(path);
            }
        }
    }
     songs
}
