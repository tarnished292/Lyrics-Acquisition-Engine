use std::sync::OnceLock;

use crate::metadata::SongMetadata;
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct LyricsResponse {
    #[serde(rename = "syncedLyrics")]
    synced_lyrics: String,
}

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub async fn fetch_lyrics(song: &SongMetadata) -> Option<String> {
    // let client = CLIENT.get_or_init(|| {
    //     reqwest::Client::builder()
    //         .timeout(std::time::Duration::from_secs(5))
    //         .build()
    //         .unwrap()
    // });
    //

    let client = CLIENT.get_or_init(reqwest::Client::new);

    let response = client
        .get("https://lrclib.net/api/get")
        .query(&[
            ("track_name", song.title.as_deref()?),
            ("artist_name", song.artist.as_deref()?),
            ("album_name", song.album.as_deref().unwrap_or("")),
        ])
        .send()
        .await
        .ok()?;

    let lyrics = response.json::<LyricsResponse>().await.ok()?;

    Some(lyrics.synced_lyrics)
}
