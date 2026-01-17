use anyhow::Result;
use ratatui::widgets::ListState;

use crate::api::{AudioQuality, Channel, Song, SomaFmClient};
use crate::artwork::ImageCache;
use crate::input::Action;
use crate::player::MpvController;
use crate::storage::{ConfigStore, FavoritesStore};
use crate::ui::{ArtworkState, Theme, ThemeType, VisualizationMode};
use crate::visualizer::{SpectrumAnalyzer, SpectrumData};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    FavoritesThenListeners,
    Alphabetical,
    ListenersOnly,
}

impl SortMode {
    pub fn next(self) -> Self {
        match self {
            Self::FavoritesThenListeners => Self::Alphabetical,
            Self::Alphabetical => Self::ListenersOnly,
            Self::ListenersOnly => Self::FavoritesThenListeners,
        }
    }
}

pub struct App {
    pub channels: Vec<Channel>,
    pub sorted_indices: Vec<usize>,
    pub list_state: ListState,
    pub current_channel: Option<usize>,
    pub current_song: Option<Song>,
    pub song_history: Vec<Song>,
    pub stream_title: Option<String>,
    pub config: ConfigStore,
    pub favorites: FavoritesStore,
    pub sort_mode: SortMode,
    pub show_help: bool,
    pub show_visualizer: bool,
    pub show_artwork: bool,
    pub show_history: bool,
    pub audio_quality: AudioQuality,
    pub player: MpvController,
    pub api_client: SomaFmClient,
    pub should_quit: bool,
    pub last_volume: u8,
    pub is_muted: bool,
    pub artwork_state: ArtworkState,
    pub image_cache: ImageCache,
    pub spectrum_analyzer: SpectrumAnalyzer,
    pub spectrum_data: SpectrumData,
    pub visualization_mode: VisualizationMode,
    pub frame: u64,
    pub theme_type: ThemeType,
    pub theme: Theme,
}

impl App {
    pub fn new() -> Self {
        let config = ConfigStore::default();
        let theme_type = config.theme_type();
        let theme = Theme::from_type(theme_type);
        let visualization_mode = config.visualization_mode();

        Self {
            channels: Vec::new(),
            sorted_indices: Vec::new(),
            list_state: ListState::default(),
            current_channel: None,
            current_song: None,
            song_history: Vec::new(),
            stream_title: None,
            config,
            favorites: FavoritesStore::default(),
            sort_mode: SortMode::FavoritesThenListeners,
            show_help: false,
            show_visualizer: true,
            show_artwork: true,
            show_history: true,
            audio_quality: AudioQuality::default(),
            player: MpvController::new(),
            api_client: SomaFmClient::new(),
            should_quit: false,
            last_volume: 80,
            is_muted: false,
            artwork_state: ArtworkState::new(),
            image_cache: ImageCache::default(),
            spectrum_analyzer: SpectrumAnalyzer::new(),
            spectrum_data: SpectrumData::default(),
            visualization_mode,
            frame: 0,
            theme_type,
            theme,
        }
    }

    pub fn cycle_theme(&mut self) {
        self.theme_type = self.theme_type.next();
        self.theme = Theme::from_type(self.theme_type);
        // Save theme preference
        self.config.set_theme(self.theme_type);
        let _ = self.config.save();
    }

    pub async fn init(&mut self) -> Result<()> {
        self.channels = self.api_client.get_channels().await?;
        self.update_sorted_indices();
        if !self.sorted_indices.is_empty() {
            self.list_state.select(Some(0));
        }
        Ok(())
    }

    fn update_sorted_indices(&mut self) {
        let favorites = self.favorites.favorites();
        let mut indices: Vec<usize> = (0..self.channels.len()).collect();

        match self.sort_mode {
            SortMode::FavoritesThenListeners => {
                indices.sort_by(|&a, &b| {
                    let a_fav = favorites.contains(&self.channels[a].id);
                    let b_fav = favorites.contains(&self.channels[b].id);

                    match (a_fav, b_fav) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => self.channels[b].listeners.cmp(&self.channels[a].listeners),
                    }
                });
            }
            SortMode::Alphabetical => {
                indices.sort_by(|&a, &b| {
                    self.channels[a].title.cmp(&self.channels[b].title)
                });
            }
            SortMode::ListenersOnly => {
                indices.sort_by(|&a, &b| {
                    self.channels[b].listeners.cmp(&self.channels[a].listeners)
                });
            }
        }

        self.sorted_indices = indices;
    }

    pub fn sorted_channels(&self) -> Vec<&Channel> {
        self.sorted_indices
            .iter()
            .map(|&i| &self.channels[i])
            .collect()
    }

    pub fn selected_channel(&self) -> Option<&Channel> {
        self.list_state
            .selected()
            .and_then(|i| self.sorted_indices.get(i))
            .map(|&idx| &self.channels[idx])
    }

    pub fn selected_channel_index(&self) -> Option<usize> {
        self.list_state
            .selected()
            .and_then(|i| self.sorted_indices.get(i).copied())
    }

    pub fn current_channel(&self) -> Option<&Channel> {
        self.current_channel.map(|i| &self.channels[i])
    }

    async fn load_artwork(&mut self, channel: &Channel) {
        // Prefer xlimage (extra large) for best quality, fall back to largeimage
        let image_url = channel.xlimage.as_ref().unwrap_or(&channel.largeimage);

        // Check if we already have this image loaded
        if self.artwork_state.current_url() == Some(image_url) {
            return;
        }

        // Try to load from cache or fetch
        match self.image_cache.get_or_fetch(image_url, &channel.id).await {
            Ok(bytes) => {
                if let Ok(img) = image::load_from_memory(&bytes) {
                    self.artwork_state.set_image(img, image_url);
                }
            }
            Err(_) => {
                // Failed to load, clear artwork
                self.artwork_state.clear();
            }
        }
    }

    async fn play_current_station(&mut self) -> Result<()> {
        if let Some(channel) = self.selected_channel().cloned() {
            let url = channel.stream_url(self.audio_quality);
            let idx = self.selected_channel_index();
            self.player.play(&url).await?;
            self.current_channel = idx;
            self.stream_title = None;
            self.current_song = None;
            self.song_history.clear();
            if self.show_artwork {
                self.load_artwork(&channel).await;
            }
        }
        Ok(())
    }

    pub async fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Quit => {
                self.should_quit = true;
                self.player.stop().await?;
            }
            Action::TogglePlayPause => {
                if self.player.state.playing {
                    self.player.toggle_pause().await?;
                } else {
                    self.play_current_station().await?;
                }
            }
            Action::SelectStation => {
                self.play_current_station().await?;
            }
            Action::VolumeUp => {
                if self.is_muted {
                    self.is_muted = false;
                    self.player.set_volume(self.last_volume).await?;
                } else {
                    self.player.volume_up().await?;
                }
            }
            Action::VolumeDown => {
                self.player.volume_down().await?;
                if self.player.state.volume == 0 {
                    self.is_muted = true;
                }
            }
            Action::ToggleMute => {
                if self.is_muted {
                    self.is_muted = false;
                    self.player.set_volume(self.last_volume).await?;
                } else {
                    self.last_volume = self.player.state.volume;
                    self.is_muted = true;
                    self.player.set_volume(0).await?;
                }
            }
            Action::ToggleFavorite => {
                if let Some(channel) = self.selected_channel() {
                    let id = channel.id.clone();
                    self.favorites.toggle(&id);
                    let _ = self.favorites.save();
                    self.update_sorted_indices();
                }
            }
            Action::NextStation => {
                let len = self.sorted_indices.len();
                if len > 0 {
                    let current = self.list_state.selected().unwrap_or(0);
                    self.list_state.select(Some((current + 1) % len));
                }
            }
            Action::PrevStation => {
                let len = self.sorted_indices.len();
                if len > 0 {
                    let current = self.list_state.selected().unwrap_or(0);
                    self.list_state.select(Some(current.checked_sub(1).unwrap_or(len - 1)));
                }
            }
            Action::GoToTop => {
                if !self.sorted_indices.is_empty() {
                    self.list_state.select(Some(0));
                }
            }
            Action::GoToBottom => {
                if !self.sorted_indices.is_empty() {
                    self.list_state.select(Some(self.sorted_indices.len() - 1));
                }
            }
            Action::ToggleSortMode => {
                self.sort_mode = self.sort_mode.next();
                self.update_sorted_indices();
            }
            Action::ToggleVisualizer => {
                self.show_visualizer = !self.show_visualizer;
            }
            Action::CycleVisualization => {
                self.visualization_mode = self.visualization_mode.next();
                // Save preference
                self.config.set_visualization(self.visualization_mode);
                let _ = self.config.save();
            }
            Action::ToggleArtwork => {
                self.show_artwork = !self.show_artwork;
                if self.show_artwork {
                    if let Some(channel) = self.current_channel().cloned() {
                        self.load_artwork(&channel).await;
                    }
                }
            }
            Action::ToggleHistory => {
                self.show_history = !self.show_history;
            }
            Action::QualityUp => {
                let new_quality = self.audio_quality.higher();
                if new_quality != self.audio_quality {
                    self.audio_quality = new_quality;
                    // If playing, restart with new quality
                    if self.player.state.playing {
                        if let Some(channel) = self.current_channel().cloned() {
                            let url = channel.stream_url(self.audio_quality);
                            self.player.play(&url).await?;
                        }
                    }
                }
            }
            Action::QualityDown => {
                let new_quality = self.audio_quality.lower();
                if new_quality != self.audio_quality {
                    self.audio_quality = new_quality;
                    // If playing, restart with new quality
                    if self.player.state.playing {
                        if let Some(channel) = self.current_channel().cloned() {
                            let url = channel.stream_url(self.audio_quality);
                            self.player.play(&url).await?;
                        }
                    }
                }
            }
            Action::ToggleTheme => {
                self.cycle_theme();
            }
            Action::ToggleHelp => {
                self.show_help = !self.show_help;
            }
            Action::CloseOverlay => {
                self.show_help = false;
            }
            Action::Refresh => {
                if let Ok(channels) = self.api_client.get_channels().await {
                    self.channels = channels;
                    self.update_sorted_indices();
                }
            }
        }
        Ok(())
    }

    /// Update spectrum data for visualization
    pub async fn update_spectrum(&mut self) {
        // Increment frame counter for animations
        self.frame = self.frame.wrapping_add(1);

        // Try to get real audio stats from mpv
        if let Some((rms_db, peak_db)) = self.player.get_audio_stats().await {
            self.spectrum_analyzer.update_from_levels(rms_db, peak_db).await;
        } else {
            // Fall back to animated visualization
            self.spectrum_analyzer.animate(
                self.player.state.playing,
                self.player.state.paused,
            ).await;
        }

        // Update the cached spectrum data for rendering
        self.spectrum_data = self.spectrum_analyzer.get_data().await;
    }

    pub async fn update_metadata(&mut self) -> Result<()> {
        if let Some(channel) = self.current_channel().cloned() {
            // Try to get song info from API (includes history)
            if let Ok(songs) = self.api_client.get_songs(&channel.id).await {
                if !songs.is_empty() {
                    self.current_song = Some(songs[0].clone());
                    // Store up to 5 previous songs
                    self.song_history = songs.into_iter().skip(1).take(5).collect();
                }
            }

            // Load artwork if enabled and not already loaded
            if self.show_artwork && !self.artwork_state.has_image() {
                self.load_artwork(&channel).await;
            }
        }

        // Also try to get metadata from stream
        if self.player.state.playing {
            if let Ok(Some((artist, title))) = self.player.get_metadata().await {
                if !title.is_empty() {
                    self.stream_title = Some(if artist.is_empty() {
                        title
                    } else {
                        format!("{} - {}", artist, title)
                    });
                }
            }
        }

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
