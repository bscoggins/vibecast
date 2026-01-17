#![allow(dead_code)]

use anyhow::Result;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct FavoritesStore {
    path: PathBuf,
    favorites: HashSet<String>,
}

impl FavoritesStore {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        let favorites = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashSet::new()
        };

        Ok(Self { path, favorites })
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = directories::ProjectDirs::from("com", "vibecast", "vibecast")
            .map(|dirs| dirs.config_dir().to_path_buf())
            .or_else(|| directories::BaseDirs::new().map(|d| d.config_dir().join("vibecast")))
            .unwrap_or_else(|| PathBuf::from(".").join("vibecast"));

        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("favorites.json"))
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(&self.favorites)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }

    pub fn toggle(&mut self, station_id: &str) -> bool {
        if self.favorites.contains(station_id) {
            self.favorites.remove(station_id);
            false
        } else {
            self.favorites.insert(station_id.to_string());
            true
        }
    }

    pub fn is_favorite(&self, station_id: &str) -> bool {
        self.favorites.contains(station_id)
    }

    pub fn favorites(&self) -> &HashSet<String> {
        &self.favorites
    }
}

impl Default for FavoritesStore {
    fn default() -> Self {
        Self::load().unwrap_or_else(|_| Self {
            path: PathBuf::from("favorites.json"),
            favorites: HashSet::new(),
        })
    }
}
