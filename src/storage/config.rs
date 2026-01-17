#![allow(dead_code)]

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::ui::{ThemeType, VisualizationMode};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub visualization: String,
}

pub struct ConfigStore {
    path: PathBuf,
    pub config: Config,
}

impl ConfigStore {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        let config = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Config::default()
        };

        Ok(Self { path, config })
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = directories::ProjectDirs::from("com", "vibecast", "vibecast")
            .map(|dirs| dirs.config_dir().to_path_buf())
            .or_else(|| directories::BaseDirs::new().map(|d| d.config_dir().join("vibecast")))
            .unwrap_or_else(|| PathBuf::from(".").join("vibecast"));

        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("config.json"))
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(&self.config)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }

    pub fn theme_type(&self) -> ThemeType {
        match self.config.theme.as_str() {
            "Synthwave" => ThemeType::Synthwave,
            "Ocean" => ThemeType::Ocean,
            "Forest" => ThemeType::Forest,
            "Sunset" => ThemeType::Sunset,
            "Mono" => ThemeType::Monochrome,
            "Cyberpunk" => ThemeType::Cyberpunk,
            _ => ThemeType::default(),
        }
    }

    pub fn set_theme(&mut self, theme_type: ThemeType) {
        self.config.theme = match theme_type {
            ThemeType::Synthwave => "Synthwave",
            ThemeType::Ocean => "Ocean",
            ThemeType::Forest => "Forest",
            ThemeType::Sunset => "Sunset",
            ThemeType::Monochrome => "Mono",
            ThemeType::Cyberpunk => "Cyberpunk",
        }
        .to_string();
    }

    pub fn visualization_mode(&self) -> VisualizationMode {
        match self.config.visualization.as_str() {
            "Spirograph" => VisualizationMode::Spirograph,
            "Pulse" => VisualizationMode::Pulse,
            "Wave" => VisualizationMode::Wave,
            "Bounce" => VisualizationMode::Bounce,
            "Stars" => VisualizationMode::Starfield,
            "Heart" => VisualizationMode::Heart,
            "Spiral" => VisualizationMode::Spiral,
            "Rain" => VisualizationMode::Rain,
            _ => VisualizationMode::Spiral, // Default to Spiral
        }
    }

    pub fn set_visualization(&mut self, mode: VisualizationMode) {
        self.config.visualization = mode.name().to_string();
    }
}

impl Default for ConfigStore {
    fn default() -> Self {
        Self::load().unwrap_or_else(|_| Self {
            path: PathBuf::from("config.json"),
            config: Config::default(),
        })
    }
}
