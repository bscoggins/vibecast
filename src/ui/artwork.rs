#![allow(dead_code)]

use image::DynamicImage;
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use std::sync::OnceLock;

static PICKER: OnceLock<Option<Picker>> = OnceLock::new();

/// Initialize the image picker early, before TUI starts.
/// Image quality depends on terminal support:
/// - Best: Kitty, iTerm2, WezTerm (native image protocols)
/// - Good: Terminals with Sixel support
/// - Basic: Halfblocks fallback (uses Unicode block characters)
pub fn init_picker() {
    PICKER.get_or_init(|| {
        // Try to create a picker, catching any panics
        std::panic::catch_unwind(|| Picker::from_query_stdio().ok()).unwrap_or(None)
    });
}

fn get_picker() -> Option<&'static Picker> {
    PICKER.get().and_then(|p| p.as_ref())
}

/// Check if we have a high-quality image protocol available
pub fn has_quality_protocol() -> bool {
    // If picker exists, we have at least halfblocks
    // Higher quality protocols are auto-detected by ratatui-image
    get_picker().is_some()
}

pub struct ArtworkState {
    pub(crate) protocol: Option<StatefulProtocol>,
    current_url: Option<String>,
}

impl ArtworkState {
    pub fn new() -> Self {
        Self {
            protocol: None,
            current_url: None,
        }
    }

    pub fn set_image(&mut self, image: DynamicImage, url: &str) {
        if let Some(picker) = get_picker() {
            self.protocol = Some(picker.new_resize_protocol(image));
            self.current_url = Some(url.to_string());
        }
    }

    pub fn clear(&mut self) {
        self.protocol = None;
        self.current_url = None;
    }

    pub fn current_url(&self) -> Option<&str> {
        self.current_url.as_deref()
    }

    pub fn has_image(&self) -> bool {
        self.protocol.is_some()
    }
}

impl Default for ArtworkState {
    fn default() -> Self {
        Self::new()
    }
}
