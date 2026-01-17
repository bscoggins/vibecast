#![allow(dead_code)]

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct ChannelsResponse {
    pub channels: Vec<Channel>,
}

fn deserialize_string_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

fn deserialize_optional_string_to_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) if !s.is_empty() => s.parse().map(Some).map_err(serde::de::Error::custom),
        _ => Ok(None),
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Channel {
    pub id: String,
    pub title: String,
    pub description: String,
    pub genre: String,
    pub dj: String,
    pub djmail: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u32")]
    pub listeners: u32,
    pub image: String,
    pub largeimage: String,
    pub xlimage: Option<String>,
    #[serde(rename = "lastPlaying")]
    pub last_playing: String,
    pub playlists: Vec<Playlist>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Playlist {
    pub url: String,
    pub format: String,
    pub quality: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SongsResponse {
    pub songs: Vec<Song>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    #[serde(rename = "albumArt")]
    pub album_art: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_string_to_u64")]
    pub date: Option<u64>,
}

/// Audio quality levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioQuality {
    #[default]
    Highest,
    High,
    Low,
}

impl AudioQuality {
    /// Increase quality (Low -> High -> Highest)
    pub fn higher(self) -> Self {
        match self {
            Self::Low => Self::High,
            Self::High => Self::Highest,
            Self::Highest => Self::Highest, // Already at max
        }
    }

    /// Decrease quality (Highest -> High -> Low)
    pub fn lower(self) -> Self {
        match self {
            Self::Highest => Self::High,
            Self::High => Self::Low,
            Self::Low => Self::Low, // Already at min
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Highest => "HQ",
            Self::High => "MQ",
            Self::Low => "LQ",
        }
    }

    fn quality_str(self) -> &'static str {
        match self {
            Self::Highest => "highest",
            Self::High => "high",
            Self::Low => "low",
        }
    }
}

impl Channel {
    /// Get stream URL for specified quality
    pub fn stream_url(&self, quality: AudioQuality) -> String {
        let quality_str = quality.quality_str();

        // Try AAC first at requested quality
        if let Some(playlist) = self.playlists.iter().find(|p| {
            p.quality == quality_str && p.format == "aac"
        }) {
            return playlist.url.clone();
        }

        // Try MP3 at requested quality
        if let Some(playlist) = self.playlists.iter().find(|p| {
            p.quality == quality_str && p.format == "mp3"
        }) {
            return playlist.url.clone();
        }

        // Try any format at requested quality
        if let Some(playlist) = self.playlists.iter().find(|p| p.quality == quality_str) {
            return playlist.url.clone();
        }

        // Fall back to best available
        self.best_stream_url()
    }

    /// Get the best quality stream URL from playlists
    /// Prefers: highest quality AAC > highest quality MP3 > any available
    pub fn best_stream_url(&self) -> String {
        // First try to find highest quality AAC
        if let Some(playlist) = self.playlists.iter().find(|p| {
            p.quality == "highest" && p.format == "aac"
        }) {
            return playlist.url.clone();
        }

        // Then try highest quality MP3
        if let Some(playlist) = self.playlists.iter().find(|p| {
            p.quality == "highest" && p.format == "mp3"
        }) {
            return playlist.url.clone();
        }

        // Then any highest quality
        if let Some(playlist) = self.playlists.iter().find(|p| p.quality == "highest") {
            return playlist.url.clone();
        }

        // Fall back to first available
        self.playlists.first()
            .map(|p| p.url.clone())
            .unwrap_or_else(|| format!("https://ice.somafm.com/{}", self.id))
    }

    pub fn format_listeners(&self) -> String {
        if self.listeners >= 1000 {
            format!("{:.1}k", self.listeners as f64 / 1000.0)
        } else {
            self.listeners.to_string()
        }
    }
}
