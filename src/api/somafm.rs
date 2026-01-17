#![allow(dead_code)]

use anyhow::Result;
use reqwest::Client;

use super::types::{Channel, ChannelsResponse, Song, SongsResponse};

pub struct SomaFmClient {
    client: Client,
    base_url: String,
}

impl SomaFmClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.somafm.com".to_string(),
        }
    }

    pub async fn get_channels(&self) -> Result<Vec<Channel>> {
        let url = format!("{}/channels.json", self.base_url);
        let resp: ChannelsResponse = self.client.get(&url).send().await?.json().await?;
        Ok(resp.channels)
    }

    pub async fn get_songs(&self, channel_id: &str) -> Result<Vec<Song>> {
        let url = format!("https://somafm.com/songs/{}.json", channel_id);
        let resp: SongsResponse = self.client.get(&url).send().await?.json().await?;
        Ok(resp.songs)
    }

    pub async fn get_current_song(&self, channel_id: &str) -> Result<Option<Song>> {
        let songs = self.get_songs(channel_id).await?;
        Ok(songs.into_iter().next())
    }
}

impl Default for SomaFmClient {
    fn default() -> Self {
        Self::new()
    }
}
