use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};
use ratatui_image::{FilterType, Resize, StatefulImage};

use super::artwork::ArtworkState;
use super::theme::Theme;
use crate::api::{AudioQuality, Channel, Song};

pub struct NowPlaying<'a> {
    channel: Option<&'a Channel>,
    song: Option<&'a Song>,
    stream_title: Option<&'a str>,
    is_paused: bool,
    audio_quality: AudioQuality,
    show_artwork: bool,
    theme: &'a Theme,
}

impl<'a> NowPlaying<'a> {
    pub fn new(
        channel: Option<&'a Channel>,
        song: Option<&'a Song>,
        stream_title: Option<&'a str>,
        is_paused: bool,
        audio_quality: AudioQuality,
        show_artwork: bool,
        theme: &'a Theme,
    ) -> Self {
        Self {
            channel,
            song,
            stream_title,
            is_paused,
            audio_quality,
            show_artwork,
            theme,
        }
    }

    fn render_content(&self, area: Rect, buf: &mut Buffer) {
        if area.height < 3 || area.width < 15 {
            return;
        }

        let theme = self.theme;

        let chunks = Layout::vertical([
            Constraint::Length(2), // Station info + quality
            Constraint::Length(1), // Separator
            Constraint::Min(3),    // Song info
        ])
        .split(area);

        // Station info
        if let Some(channel) = self.channel {
            let status = if self.is_paused { "⏸" } else { "▶" };
            let status_style = if self.is_paused {
                theme.paused_style()
            } else {
                theme.playing_style()
            };

            let quality_label = self.audio_quality.label();
            let station_line = Line::from(vec![
                Span::styled(format!("{} ", status), status_style),
                Span::styled(&channel.title, theme.selected_style()),
                Span::styled(" ", theme.muted_style()),
                Span::styled(
                    format!("[{}]", quality_label),
                    ratatui::style::Style::default().fg(theme.accent),
                ),
            ]);
            Paragraph::new(station_line).render(chunks[0], buf);

            // Genre line
            if chunks[0].height > 1 {
                let genre_area = Rect {
                    y: chunks[0].y + 1,
                    height: 1,
                    ..chunks[0]
                };
                let genre_line = Line::from(vec![
                    Span::styled("  ", theme.muted_style()),
                    Span::styled(&channel.genre, theme.muted_style()),
                    Span::styled(" • ", theme.muted_style()),
                    Span::styled(
                        format!("{} listeners", channel.listeners),
                        theme.muted_style(),
                    ),
                ]);
                Paragraph::new(genre_line).render(genre_area, buf);
            }
        } else {
            let no_station = Line::from(Span::styled("No station selected", theme.muted_style()));
            Paragraph::new(no_station).render(chunks[0], buf);
        }

        // Song info
        let song_area = chunks[2];
        if let Some(song) = self.song {
            let mut lines = vec![];

            // Title
            lines.push(Line::from(vec![
                Span::styled("♫ ", ratatui::style::Style::default().fg(theme.accent)),
                Span::styled(
                    &song.title,
                    theme
                        .normal_style()
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
            ]));

            // Artist
            lines.push(Line::from(vec![
                Span::styled("  by ", theme.muted_style()),
                Span::styled(
                    &song.artist,
                    ratatui::style::Style::default().fg(theme.secondary),
                ),
            ]));

            // Album
            if let Some(album) = &song.album {
                if !album.is_empty() {
                    lines.push(Line::from(vec![
                        Span::styled("  from ", theme.muted_style()),
                        Span::styled(album, theme.muted_style()),
                    ]));
                }
            }

            Paragraph::new(lines).render(song_area, buf);
        } else if let Some(title) = self.stream_title {
            // Parse stream title (usually "Artist - Title")
            let lines = if let Some((artist, song_title)) = title.split_once(" - ") {
                vec![
                    Line::from(vec![
                        Span::styled("♫ ", ratatui::style::Style::default().fg(theme.accent)),
                        Span::styled(
                            song_title,
                            theme
                                .normal_style()
                                .add_modifier(ratatui::style::Modifier::BOLD),
                        ),
                    ]),
                    Line::from(vec![
                        Span::styled("  by ", theme.muted_style()),
                        Span::styled(artist, ratatui::style::Style::default().fg(theme.secondary)),
                    ]),
                ]
            } else {
                vec![Line::from(vec![
                    Span::styled("♫ ", ratatui::style::Style::default().fg(theme.accent)),
                    Span::styled(title, theme.normal_style()),
                ])]
            };
            Paragraph::new(lines).render(song_area, buf);
        } else if self.channel.is_some() {
            let waiting = Line::from(Span::styled("Loading song info...", theme.muted_style()));
            Paragraph::new(waiting).render(song_area, buf);
        }
    }
}

impl<'a> StatefulWidget for NowPlaying<'a> {
    type State = ArtworkState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let theme = self.theme;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style())
            .title(Span::styled(" Now Playing ", theme.title_style()));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 4 || inner.width < 20 {
            return;
        }

        // Check if we should show artwork
        let show_art = self.show_artwork && state.protocol.is_some() && inner.width >= 40;

        if show_art {
            // Fixed artwork size for consistent display across all stations
            // 16 chars wide x 8 chars tall = roughly square in terminal
            let art_width: u16 = 16;
            let art_height = inner.height.min(8);

            // Create artwork area and content area side by side
            let art_area = Rect {
                x: inner.x,
                y: inner.y,
                width: art_width,
                height: art_height,
            };

            // Content starts right after artwork with minimal gap
            let content_area = Rect {
                x: inner.x + art_width + 1,
                y: inner.y,
                width: inner.width.saturating_sub(art_width + 1),
                height: inner.height,
            };

            // Render artwork with high-quality scaling
            if let Some(ref mut protocol) = state.protocol {
                let image =
                    StatefulImage::default().resize(Resize::Fit(Some(FilterType::Lanczos3)));
                StatefulWidget::render(image, art_area, buf, protocol);
            }

            // Render content immediately to the right of artwork
            self.render_content(content_area, buf);
        } else {
            // Render content without artwork
            self.render_content(inner, buf);
        }
    }
}
