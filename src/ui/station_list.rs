use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget},
};
use std::collections::HashSet;

use crate::api::Channel;
use super::theme::Theme;

pub struct StationList<'a> {
    channels: &'a [Channel],
    favorites: &'a HashSet<String>,
    current_station: Option<&'a str>,
    is_focused: bool,
    theme: &'a Theme,
}

impl<'a> StationList<'a> {
    pub fn new(
        channels: &'a [Channel],
        favorites: &'a HashSet<String>,
        current_station: Option<&'a str>,
        is_focused: bool,
        theme: &'a Theme,
    ) -> Self {
        Self {
            channels,
            favorites,
            current_station,
            is_focused,
            theme,
        }
    }
}

impl<'a> StatefulWidget for StationList<'a> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let theme = self.theme;

        let items: Vec<ListItem> = self
            .channels
            .iter()
            .map(|channel| {
                let is_favorite = self.favorites.contains(&channel.id);
                let is_playing = self.current_station == Some(&channel.id);

                let star = if is_favorite { "★ " } else { "  " };
                let playing_indicator = if is_playing { "▶ " } else { "" };

                let star_style = if is_favorite {
                    theme.favorite_style()
                } else {
                    theme.muted_style()
                };

                let title_style = if is_playing {
                    theme.playing_style()
                } else {
                    theme.normal_style()
                };

                let listeners = format!(" {}", channel.format_listeners());

                let line = Line::from(vec![
                    Span::styled(playing_indicator, theme.playing_style()),
                    Span::styled(star, star_style),
                    Span::styled(&channel.title, title_style),
                    Span::styled(listeners, theme.muted_style()),
                ]);

                ListItem::new(line)
            })
            .collect();

        let border_style = if self.is_focused {
            theme.active_border_style()
        } else {
            theme.border_style()
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(Span::styled(" Stations ", theme.title_style()));

        let list = List::new(items)
            .block(block)
            .highlight_style(theme.highlight_style())
            .highlight_symbol("│ ");

        StatefulWidget::render(list, area, buf, state);
    }
}
