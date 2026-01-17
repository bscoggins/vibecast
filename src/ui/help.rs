use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use super::theme::Theme;

pub struct HelpOverlay<'a> {
    theme: &'a Theme,
}

impl<'a> HelpOverlay<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

        Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
    }
}

impl<'a> Widget for HelpOverlay<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = self.theme;
        let popup_area = Self::centered_rect(60, 70, area);

        // Clear the area behind the popup
        Clear.render(popup_area, buf);

        let block = Block::default()
            .title(Span::styled(" Keyboard Shortcuts ", theme.title_style()))
            .borders(Borders::ALL)
            .border_style(theme.active_border_style())
            .style(ratatui::style::Style::default().bg(theme.background));

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        let shortcuts = vec![
            ("Playback", vec![
                ("p / Space", "Play / Pause"),
                ("Enter", "Play selected station"),
                ("q / Esc", "Quit"),
            ]),
            ("Navigation", vec![
                ("j / Down", "Move down"),
                ("k / Up", "Move up"),
                ("g", "Go to top"),
                ("G", "Go to bottom"),
            ]),
            ("Volume", vec![
                ("+ / =", "Volume up"),
                ("- / _", "Volume down"),
                ("m", "Mute / Unmute"),
            ]),
            ("Stations", vec![
                ("f", "Toggle favorite"),
                ("s", "Cycle sort mode"),
                ("R", "Refresh stations"),
            ]),
            ("Display", vec![
                ("v", "Cycle visualization style"),
                ("V", "Show/hide visualizer"),
                ("a", "Toggle artwork"),
                ("r", "Toggle recently played"),
                ("t", "Cycle color theme"),
            ]),
            ("Audio", vec![
                ("< / ,", "Lower audio quality"),
                ("> / .", "Higher audio quality"),
                ("?", "Toggle this help"),
            ]),
        ];

        let mut lines = vec![];

        for (section, items) in shortcuts {
            // Section header
            lines.push(Line::from(Span::styled(
                section,
                theme.selected_style().add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));

            for (key, desc) in items {
                lines.push(Line::from(vec![
                    Span::styled(format!("  {:12}", key), theme.highlight_style()),
                    Span::raw("  "),
                    Span::styled(desc, theme.normal_style()),
                ]));
            }
            lines.push(Line::from(""));
        }

        lines.push(Line::from(Span::styled(
            "Press any key to close",
            theme.muted_style(),
        )));

        let paragraph = Paragraph::new(lines).alignment(Alignment::Left);
        paragraph.render(inner, buf);
    }
}
