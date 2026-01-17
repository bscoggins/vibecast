#![allow(dead_code)]

use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

pub struct ImageCache {
    cache_dir: PathBuf,
}

impl ImageCache {
    pub fn new() -> Result<Self> {
        let cache_dir = directories::ProjectDirs::from("com", "vibecast", "vibecast")
            .map(|dirs| dirs.cache_dir().to_path_buf())
            .or_else(|| {
                directories::BaseDirs::new().map(|d| d.cache_dir().join("vibecast"))
            })
            .unwrap_or_else(|| PathBuf::from(".").join(".vibecast-cache"))
            .join("artwork");

        std::fs::create_dir_all(&cache_dir)?;
        Ok(Self { cache_dir })
    }

    fn cache_path(&self, station_id: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.png", station_id))
    }

    pub async fn get_or_fetch(&self, url: &str, station_id: &str) -> Result<Vec<u8>> {
        let cache_path = self.cache_path(station_id);

        // Check if cached
        if cache_path.exists() {
            return Ok(fs::read(&cache_path).await?);
        }

        // Fetch from network
        let response = reqwest::get(url).await?.error_for_status()?;
        let bytes = response.bytes().await?.to_vec();

        // Cache it
        let _ = fs::write(&cache_path, &bytes).await;

        Ok(bytes)
    }

    pub fn get_cached(&self, station_id: &str) -> Option<Vec<u8>> {
        let cache_path = self.cache_path(station_id);
        if cache_path.exists() {
            std::fs::read(&cache_path).ok()
        } else {
            None
        }
    }
}

impl Default for ImageCache {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            cache_dir: PathBuf::from(".vibecast-cache").join("artwork"),
        })
    }
}
