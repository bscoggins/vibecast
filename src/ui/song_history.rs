#![allow(dead_code)]

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use super::theme::Theme;
use crate::api::Song;

pub struct SongHistory<'a> {
    songs: &'a [Song],
    theme: &'a Theme,
}

fn truncate_to_width(text: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }

    if UnicodeWidthStr::width(text) <= max_width {
        return text.to_string();
    }

    let ellipsis = "...";
    let ellipsis_width = UnicodeWidthStr::width(ellipsis);

    if max_width <= ellipsis_width {
        return ellipsis.chars().take(max_width).collect();
    }

    let target_width = max_width.saturating_sub(ellipsis_width);
    let mut width = 0;
    let mut out = String::new();

    for ch in text.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
        if width + ch_width > target_width {
            break;
        }
        width += ch_width;
        out.push(ch);
    }

    out.push_str(ellipsis);
    out
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
            let empty = Line::from(Span::styled("No history available", theme.muted_style()));
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
            let max_width = inner.width.saturating_sub(4) as usize;
            let truncated = truncate_to_width(&display, max_width);

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
