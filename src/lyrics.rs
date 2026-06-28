use reqwest;

use crate::metadata::SongMetadata;

pub async fn fetch_lyrics(song: &SongMetadata) -> Option<String> {
    let client = reqwest::Client::new();

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

    let body = response.text().await.ok()?;

    println!("{}", body);

    None
}