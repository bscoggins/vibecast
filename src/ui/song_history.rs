#![allow(dead_code)]

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::api::Song;
use super::theme::Theme;

pub struct SongHistory<'a> {
    songs: &'a [Song],
    theme: &'a Theme,
}

impl<'a> SongHistory<'a> {
    pub fn new(songs: &'a [Song], theme: &'a Theme) -> Self {
        Self { songs, theme }
    }
}

impl<'a> Widget for SongHistory<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = self.theme;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style())
            .title(Span::styled(" Previously Played ", theme.title_style()));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 1 || inner.width < 10 {
            return;
        }

        if self.songs.is_empty() {
            let empty = Line::from(Span::styled(
                "No history available",
                theme.muted_style(),
            ));
            Paragraph::new(empty).render(inner, buf);
            return;
        }

        let mut lines = vec![];
        let max_songs = inner.height as usize;

        for (i, song) in self.songs.iter().take(max_songs).enumerate() {
            let display = if song.artist.is_empty() {
                song.title.clone()
            } else {
                format!("{} - {}", song.artist, song.title)
            };

            // Truncate if too long
            let max_len = inner.width.saturating_sub(4) as usize;
            let truncated = if display.len() > max_len {
                format!("{}...", &display[..max_len.saturating_sub(3)])
            } else {
                display
            };

            // First song slightly highlighted, rest muted
            let style = if i == 0 {
                theme.normal_style()
            } else {
                theme.muted_style()
            };

            lines.push(Line::from(vec![
                Span::styled("  ", style),
                Span::styled(truncated, style),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
