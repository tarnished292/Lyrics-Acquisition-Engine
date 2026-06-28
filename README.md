# Lyrics Acquisition Engine

A lightweight Rust library for automatically acquiring synchronized lyrics (`.lrc`) for local music libraries.

The library recursively scans a music directory, extracts song metadata, queries LRCLIB for synchronized lyrics, and writes `.lrc` files alongside the corresponding audio files.

## Features

* Recursive music library scanning
* Supports MP3, FLAC, and Opus files
* Extracts title, artist, and album metadata
* Fetches synchronized lyrics from LRCLIB
* Automatically creates `.lrc` files beside audio files
* Skips songs that already have lyrics
* Designed to be embedded into desktop music applications

## Supported Formats

* MP3
* FLAC
* Opus

## How It Works

1. Scan the provided music directory recursively.
2. Read metadata from each supported audio file.
3. Check whether a corresponding `.lrc` file already exists.
4. Request synchronized lyrics from LRCLIB.
5. Save the received lyrics as an `.lrc` file next to the audio file.

## Current Capabilities

* Recursive directory traversal
* Audio file filtering
* Metadata extraction
* Lyrics retrieval
* `.lrc` generation

## Planned Improvements

* Improved song matching heuristics
* Duration-aware matching
* Concurrent lyric downloads
* Retry and rate-limit handling
* Progress reporting
* Configurable providers
* Metadata caching

## Use Cases

Lyrics Acquisition Engine is intended for applications that maintain a local music library and need synchronized lyrics without implementing scanning, metadata extraction, or lyrics retrieval themselves.

## License

MIT
