use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use super::theme::Theme;

pub struct StatusBar<'a> {
    is_playing: bool,
    is_paused: bool,
    volume: u8,
    theme_name: &'a str,
    theme: &'a Theme,
}

impl<'a> StatusBar<'a> {
    pub fn new(is_playing: bool, is_paused: bool, volume: u8, theme_name: &'a str, theme: &'a Theme) -> Self {
        Self {
            is_playing,
            is_paused,
            volume,
            theme_name,
            theme,
        }
    }

    fn volume_bar(&self) -> String {
        let filled = (self.volume as usize * 10) / 100;
        let empty = 10 - filled;
        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    }
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = self.theme;

        // Fixed-width status section (11 chars: " ▶ Playing " or " ⏸ Paused  " or " ■ Stopped ")
        let (status_icon, status_text, status_style) = if !self.is_playing {
            ("■", "Stopped", theme.muted_style())
        } else if self.is_paused {
            ("⏸", "Paused ", theme.paused_style())
        } else {
            ("▶", "Playing", theme.playing_style())
        };

        let volume_bar = self.volume_bar();
        // Fixed-width volume section: "Vol: ██████████ 100%"
        let volume_percent = format!("{:>3}%", self.volume);

        // Fixed-width theme name (pad to 10 chars)
        let theme_display = format!("{:<10}", self.theme_name);

        let line = Line::from(vec![
            // Status section (fixed 11 chars)
            Span::styled(format!(" {} ", status_icon), status_style),
            Span::styled(status_text, status_style),
            Span::styled(" │ ", theme.muted_style()),
            // Volume section (fixed width: "Vol: " + 10 bar chars + " " + 4 percent chars = 20)
            Span::styled("Vol: ", theme.muted_style()),
            Span::styled(&volume_bar, theme.normal_style()),
            Span::styled(format!(" {}", volume_percent), theme.muted_style()),
            Span::styled(" │ ", theme.muted_style()),
            // Theme section (fixed 10 chars)
            Span::styled(&theme_display, theme.selected_style()),
            Span::styled(" │ ", theme.muted_style()),
            // Help hints
            Span::styled("[p]", theme.selected_style()),
            Span::styled("lay ", theme.muted_style()),
            Span::styled("[f]", theme.selected_style()),
            Span::styled("av ", theme.muted_style()),
            Span::styled("[v]", theme.selected_style()),
            Span::styled("iz ", theme.muted_style()),
            Span::styled("[?]", theme.selected_style()),
            Span::styled("help", theme.muted_style()),
        ]);

        Paragraph::new(line).render(area, buf);
    }
}
