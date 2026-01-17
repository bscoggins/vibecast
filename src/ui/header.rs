use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use super::theme::Theme;

pub struct Header<'a> {
    station_name: Option<&'a str>,
    theme: &'a Theme,
}

impl<'a> Header<'a> {
    pub fn new(station_name: Option<&'a str>, theme: &'a Theme) -> Self {
        Self { station_name, theme }
    }
}

impl<'a> Widget for Header<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = self.theme;
        let title = "VIBECAST";

        let right_text = self
            .station_name
            .map(|name| format!("Now Playing: {}", name))
            .unwrap_or_default();

        // Create a gradient-like effect for the title
        let title_spans: Vec<Span> = title
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let colors = [
                    theme.primary,
                    theme.secondary,
                    theme.accent,
                ];
                let color = colors[i % colors.len()];
                Span::styled(
                    c.to_string(),
                    ratatui::style::Style::default()
                        .fg(color)
                        .add_modifier(Modifier::BOLD),
                )
            })
            .collect();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style());

        let inner = block.inner(area);
        block.render(area, buf);

        // Render title on the left
        let title_line = Line::from(title_spans);
        let title_para = Paragraph::new(title_line);
        title_para.render(
            Rect {
                x: inner.x + 1,
                y: inner.y,
                width: inner.width.saturating_sub(2),
                height: 1,
            },
            buf,
        );

        // Render station name on the right
        if !right_text.is_empty() && inner.width > 30 {
            let right_len = right_text.len() as u16;
            let right_x = inner.x + inner.width.saturating_sub(right_len + 1);
            let right_line = Line::from(Span::styled(&right_text, theme.selected_style()));
            let right_para = Paragraph::new(right_line).alignment(Alignment::Right);
            right_para.render(
                Rect {
                    x: right_x,
                    y: inner.y,
                    width: right_len,
                    height: 1,
                },
                buf,
            );
        }
    }
}
