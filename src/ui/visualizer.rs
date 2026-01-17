use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Span,
    widgets::{Block, Borders, Widget},
};

use crate::visualizer::SpectrumData;
use super::theme::Theme;

/// Different visualization modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VisualizationMode {
    #[default]
    Spirograph,
    Pulse,
    Wave,
    Bounce,
    Starfield,
    Heart,
    Spiral,
    Rain,
}

impl VisualizationMode {
    pub fn next(self) -> Self {
        match self {
            Self::Spirograph => Self::Pulse,
            Self::Pulse => Self::Wave,
            Self::Wave => Self::Bounce,
            Self::Bounce => Self::Starfield,
            Self::Starfield => Self::Heart,
            Self::Heart => Self::Spiral,
            Self::Spiral => Self::Rain,
            Self::Rain => Self::Spirograph,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Spirograph => "Spirograph",
            Self::Pulse => "Pulse",
            Self::Wave => "Wave",
            Self::Bounce => "Bounce",
            Self::Starfield => "Stars",
            Self::Heart => "Heart",
            Self::Spiral => "Spiral",
            Self::Rain => "Rain",
        }
    }
}

// Characters for spirograph drawing
const SPIRO_CHARS: &[char] = &['·', '•', '○', '●', '◉', '★', '✦', '✧'];

pub struct Visualizer<'a> {
    spectrum: &'a SpectrumData,
    is_playing: bool,
    is_paused: bool,
    mode: VisualizationMode,
    frame: u64,
    theme: &'a Theme,
}

impl<'a> Visualizer<'a> {
    pub fn new(
        spectrum: &'a SpectrumData,
        is_playing: bool,
        is_paused: bool,
        mode: VisualizationMode,
        frame: u64,
        theme: &'a Theme,
    ) -> Self {
        Self {
            spectrum,
            is_playing,
            is_paused,
            mode,
            frame,
            theme,
        }
    }

    fn energy(&self) -> f32 {
        (self.spectrum.rms * 0.5 + self.spectrum.peak * 0.5).clamp(0.0, 1.0)
    }

    fn render_spirograph(&self, area: Rect, buf: &mut Buffer) {
        let energy = self.energy();
        let time = self.frame as f32 * 0.03;

        let cx = area.x as f32 + area.width as f32 / 2.0;
        let cy = area.y as f32 + area.height as f32 / 2.0;

        // Scale based on area size
        let scale_x = area.width as f32 / 2.5;
        let scale_y = area.height as f32 / 2.5;

        // Spirograph parameters that change with energy
        // R = fixed circle radius, r = rolling circle radius, d = pen distance from center
        let configs = [
            // (R, r, d, color, rotation_speed)
            (5.0, 3.0, 2.5, self.theme.accent, 1.0),
            (7.0, 2.0, 1.5, self.theme.primary, -0.7),
            (6.0, 4.0, 3.0, self.theme.secondary, 0.5),
        ];

        for (big_r, small_r, pen_d, base_color, rot_speed) in configs {
            // Adjust parameters based on energy
            let r_ratio = big_r / small_r;
            let d = pen_d * (0.5 + energy * 0.8);

            // Draw the spirograph pattern
            // More points when energy is higher for denser pattern
            let num_points = 200 + (energy * 300.0) as usize;

            for i in 0..num_points {
                let t = (i as f32 / num_points as f32) * std::f32::consts::PI * 2.0 * r_ratio * 3.0;
                let animated_t = t + time * rot_speed * (1.0 + energy);

                // Hypotrochoid equations
                let diff = big_r - small_r;
                let x = diff * animated_t.cos() + d * (diff * animated_t / small_r).cos();
                let y = diff * animated_t.sin() - d * (diff * animated_t / small_r).sin();

                // Scale and center
                let px = (cx + x * scale_x * 0.15) as u16;
                let py = (cy + y * scale_y * 0.3) as u16;

                if px >= area.x && px < area.x + area.width && py >= area.y && py < area.y + area.height {
                    // Vary character and color based on position in pattern
                    let intensity = (i as f32 / num_points as f32 + energy) % 1.0;
                    let char_idx = ((intensity * (SPIRO_CHARS.len() - 1) as f32) as usize).min(SPIRO_CHARS.len() - 1);

                    let color = if intensity > 0.8 {
                        self.theme.highlight
                    } else if intensity > 0.5 {
                        base_color
                    } else {
                        self.theme.muted
                    };

                    if let Some(cell) = buf.cell_mut((px, py)) {
                        cell.set_char(SPIRO_CHARS[char_idx]).set_style(Style::default().fg(color));
                    }
                }
            }
        }

        // Center decoration
        let center_char = if energy > 0.6 { '◉' } else if energy > 0.3 { '●' } else { '○' };
        let center_x = cx as u16;
        let center_y = cy as u16;
        if center_x >= area.x && center_x < area.x + area.width && center_y >= area.y && center_y < area.y + area.height {
            if let Some(cell) = buf.cell_mut((center_x, center_y)) {
                cell.set_char(center_char).set_style(Style::default().fg(self.theme.highlight));
            }
        }
    }

    fn render_pulse(&self, area: Rect, buf: &mut Buffer) {
        let cx = area.x + area.width / 2;
        let cy = area.y + area.height / 2;
        let energy = self.energy();

        // Speed varies dramatically with energy - slow when quiet, fast when loud
        let speed = 0.05 + energy * 0.35;
        let time = self.frame as f32 * speed;

        // More rings when energy is higher
        let num_rings = 3 + (energy * 4.0) as usize;

        // Multiple expanding rings
        for ring in 0..num_rings {
            let ring_spacing = 1.0 + energy * 0.5;
            let phase = (time + ring as f32 * ring_spacing) % 8.0;

            // Rings expand faster and further with more energy
            let max_expansion = 2.0 + energy * 4.0;
            let radius = phase * max_expansion;
            let intensity = (1.0 - phase / 8.0) * (0.2 + energy * 0.8);

            if intensity < 0.05 {
                continue;
            }

            let colors = [self.theme.accent, self.theme.primary, self.theme.secondary, self.theme.highlight];
            let style = Style::default().fg(colors[ring % colors.len()]);

            // Draw the ring using a circle approximation
            let steps = 48;
            for i in 0..steps {
                let angle = (i as f32 / steps as f32) * std::f32::consts::PI * 2.0;
                let x = cx as f32 + angle.cos() * radius * 2.0; // *2 for terminal aspect ratio
                let y = cy as f32 + angle.sin() * radius;

                let xi = x as u16;
                let yi = y as u16;

                if xi >= area.x && xi < area.x + area.width && yi >= area.y && yi < area.y + area.height {
                    let char = if intensity > 0.7 { '█' } else if intensity > 0.5 { '●' } else if intensity > 0.3 { '○' } else { '·' };
                    if let Some(cell) = buf.cell_mut((xi, yi)) {
                        cell.set_char(char).set_style(style);
                    }
                }
            }
        }

    }

    fn render_wave(&self, area: Rect, buf: &mut Buffer) {
        let energy = self.energy();
        let time = self.frame as f32 * 0.2;
        let mid_y = area.y + area.height / 2;

        for x in area.x..area.x + area.width {
            let pos = (x - area.x) as f32 / area.width as f32;

            // Multiple overlapping waves
            let wave1 = (pos * 8.0 + time).sin();
            let wave2 = (pos * 12.0 - time * 1.3).sin() * 0.5;
            let wave3 = (pos * 4.0 + time * 0.7).cos() * 0.3;

            let combined = (wave1 + wave2 + wave3) / 1.8;
            let amplitude = (area.height as f32 / 2.0 - 1.0) * (0.2 + energy * 0.8);
            let y_offset = (combined * amplitude) as i16;

            let y = (mid_y as i16 + y_offset).clamp(area.y as i16, (area.y + area.height - 1) as i16) as u16;

            // Draw the wave point and a trail below/above
            let style = if energy > 0.5 {
                Style::default().fg(self.theme.highlight)
            } else {
                Style::default().fg(self.theme.accent)
            };

            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_char('█').set_style(style);
            }

            // Draw lighter trail
            let trail_style = Style::default().fg(self.theme.primary);
            if y > area.y {
                if let Some(cell) = buf.cell_mut((x, y - 1)) {
                    cell.set_char('▄').set_style(trail_style);
                }
            }
            if y < area.y + area.height - 1 {
                if let Some(cell) = buf.cell_mut((x, y + 1)) {
                    cell.set_char('▀').set_style(trail_style);
                }
            }
        }
    }

    fn render_bounce(&self, area: Rect, buf: &mut Buffer) {
        let energy = self.energy();
        let time = self.frame as f32 * 0.1;

        // Many bouncing items with varied characters, colors, speeds, and positions
        let balls = [
            ('●', 0.0, self.theme.accent, 1.0, 0.15),
            ('◉', 1.3, self.theme.primary, 1.2, 0.25),
            ('○', 2.6, self.theme.secondary, 0.9, 0.35),
            ('◆', 3.9, self.theme.highlight, 1.1, 0.45),
            ('★', 5.2, self.theme.accent, 0.8, 0.55),
            ('♦', 6.5, self.theme.primary, 1.3, 0.65),
            ('●', 7.8, self.theme.secondary, 1.0, 0.75),
            ('◉', 9.1, self.theme.highlight, 0.85, 0.85),
        ];

        for (char, phase_offset, color, speed_mult, x_pos) in balls {
            // Ball bounces based on energy and time
            let bounce_height = (area.height as f32 - 2.0) * (0.3 + energy * 0.7);
            let bounce = ((time * speed_mult + phase_offset).sin().abs() * bounce_height) as u16;

            // Each ball has its own horizontal position with slight wave motion
            let base_x = area.x as f32 + (area.width as f32 * x_pos);
            let x_wave = ((time * 0.5 + phase_offset).sin() * (area.width as f32 * 0.08)) as f32;
            let x = (base_x + x_wave).clamp(area.x as f32, (area.x + area.width - 1) as f32) as u16;
            let y = area.y + area.height - 1 - bounce.min(area.height - 1);

            if x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(char).set_style(Style::default().fg(color));
                }
            }

            // Shadow at bottom - smaller when ball is higher
            if area.height > 2 {
                let shadow_y = area.y + area.height - 1;
                let shadow_char = if bounce > (area.height / 2) { '.' } else { '─' };
                if x >= area.x && x < area.x + area.width {
                    if let Some(cell) = buf.cell_mut((x, shadow_y)) {
                        cell.set_char(shadow_char).set_style(Style::default().fg(self.theme.muted));
                    }
                }
            }
        }
    }

    fn render_starfield(&self, area: Rect, buf: &mut Buffer) {
        let energy = self.energy();
        let time = self.frame;
        let cx = area.width as f32 / 2.0;
        let cy = area.height as f32 / 2.0;

        // Generate lots of stars based on frame number as seed
        let num_stars = 120;
        for i in 0..num_stars {
            // Pseudo-random but deterministic star positions
            let seed = (i as u64 * 7919 + 1) % 10000;
            let angle = (seed as f32 / 10000.0) * std::f32::consts::PI * 2.0;
            let base_dist = ((seed * 3) % 10000) as f32 / 10000.0;

            // Stars move outward - slower, more gentle movement
            let speed = 0.1 + energy * 0.2;
            let dist = ((base_dist + time as f32 * speed * 0.005) % 1.0) * (cx.max(cy) * 1.5);

            let x = area.x + (cx + angle.cos() * dist * 2.0) as u16; // *2 for aspect ratio
            let y = area.y + (cy + angle.sin() * dist) as u16;

            if x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height {
                // Stars get brighter as they move outward
                let brightness = dist / (cx.max(cy) * 1.5);
                let (char, color) = if brightness > 0.7 {
                    ('★', self.theme.highlight)
                } else if brightness > 0.4 {
                    ('✦', self.theme.accent)
                } else if brightness > 0.2 {
                    ('·', self.theme.primary)
                } else {
                    ('.', self.theme.muted)
                };

                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(char).set_style(Style::default().fg(color));
                }
            }
        }
    }

    fn render_heart(&self, area: Rect, buf: &mut Buffer) {
        let energy = self.energy();
        let time = self.frame as f32 * 0.15;

        // ASCII heart pattern
        let heart = [
            " ♥♥ ♥♥ ",
            "♥♥♥♥♥♥♥",
            "♥♥♥♥♥♥♥",
            " ♥♥♥♥♥ ",
            "  ♥♥♥  ",
            "   ♥   ",
        ];

        let heart_height = heart.len() as u16;
        let heart_width = heart.iter().map(|s| s.chars().count()).max().unwrap_or(0) as u16;

        // Three hearts displayed horizontally with different colors and phase offsets
        let heart_configs = [
            (area.width / 6, self.theme.accent, 0.0),
            (area.width / 2, self.theme.primary, 1.0),
            (area.width * 5 / 6, self.theme.secondary, 2.0),
        ];

        let cy = area.y + area.height / 2;

        for (x_offset, base_color, phase_offset) in heart_configs {
            // Each heart pulses with a phase offset
            let pulse = ((time + phase_offset).sin() * 0.5 + 0.5) * energy;

            let style = if pulse > 0.6 {
                Style::default().fg(self.theme.highlight)
            } else if pulse > 0.3 {
                Style::default().fg(base_color)
            } else {
                Style::default().fg(self.theme.muted)
            };

            let cx = area.x + x_offset;
            let start_y = cy.saturating_sub(heart_height / 2);
            let start_x = cx.saturating_sub(heart_width / 2);

            for (row, line) in heart.iter().enumerate() {
                let y = start_y + row as u16;
                if y >= area.y + area.height {
                    break;
                }
                for (col, ch) in line.chars().enumerate() {
                    let x = start_x + col as u16;
                    if x >= area.x && x < area.x + area.width && y >= area.y && ch != ' ' {
                        if let Some(cell) = buf.cell_mut((x, y)) {
                            cell.set_char(ch).set_style(style);
                        }
                    }
                }
            }
        }
    }

    fn render_spiral(&self, area: Rect, buf: &mut Buffer) {
        let energy = self.energy();
        let time = self.frame as f32 * 0.1;

        let spiral_chars = ['·', '•', '○', '●', '◉'];

        // Three spirals displayed horizontally with different colors
        let spiral_configs = [
            (area.width / 6, self.theme.accent, 1.0),      // Left spiral
            (area.width / 2, self.theme.primary, -1.0),    // Center spiral (opposite direction)
            (area.width * 5 / 6, self.theme.secondary, 1.0), // Right spiral
        ];

        for (x_offset, color, direction) in spiral_configs {
            let cx = area.x as f32 + x_offset as f32;
            let cy = area.y as f32 + area.height as f32 / 2.0;
            let max_radius = (area.width as f32 / 3.0).min(area.height as f32);

            // Draw spiral arms
            for arm in 0..3 {
                let arm_offset = (arm as f32 / 3.0) * std::f32::consts::PI * 2.0;

                for i in 0..40 {
                    let t = i as f32 / 40.0;
                    let radius = t * max_radius * (0.5 + energy * 0.5);
                    let angle = t * std::f32::consts::PI * 4.0 + time * direction * (1.0 + energy) + arm_offset;

                    let x = (cx + angle.cos() * radius * 0.8) as u16;
                    let y = (cy + angle.sin() * radius * 0.4) as u16;

                    if x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height {
                        let char_idx = ((t * 4.0) as usize).min(spiral_chars.len() - 1);
                        let point_color = if t > 0.7 {
                            self.theme.highlight
                        } else {
                            color
                        };

                        if let Some(cell) = buf.cell_mut((x, y)) {
                            cell.set_char(spiral_chars[char_idx]).set_style(Style::default().fg(point_color));
                        }
                    }
                }
            }
        }
    }

    fn render_rain(&self, area: Rect, buf: &mut Buffer) {
        let energy = self.energy();
        let time = self.frame;

        // Rain characters - varied styles
        let rain_chars = ['│', '┃', '|', '¦', ':'];

        // Generate rain drops
        let num_drops = (15.0 + energy * 25.0) as usize;

        for i in 0..num_drops {
            // Pseudo-random but deterministic drop positions
            let seed = (i as u64 * 7919) % 10000;
            let x_ratio = (seed as f32) / 10000.0;
            let x = area.x + (x_ratio * area.width as f32) as u16;

            // Very slow rain - gentle falling effect
            let base_speed = 0.02 + energy * 0.04 + (seed % 100) as f32 / 2000.0;
            let y_offset = ((time as f32 * base_speed + seed as f32 * 0.02) % (area.height as f32 + 10.0)) as u16;

            if y_offset < area.height {
                let y = area.y + y_offset;

                // Longer drops at higher energy
                let drop_len = if energy > 0.6 { 4 } else if energy > 0.3 { 3 } else { 2 };

                for d in 0..drop_len {
                    let dy = y.saturating_sub(d);
                    if dy >= area.y && dy < area.y + area.height && x < area.x + area.width {
                        let char_idx = d as usize % rain_chars.len();
                        let color = if d == 0 {
                            self.theme.highlight
                        } else if d == 1 {
                            self.theme.accent
                        } else if d == 2 {
                            self.theme.primary
                        } else {
                            self.theme.muted
                        };

                        if let Some(cell) = buf.cell_mut((x, dy)) {
                            cell.set_char(rain_chars[char_idx]).set_style(Style::default().fg(color));
                        }
                    }
                }
            }
        }

        // Splashes at bottom when drops hit
        if energy > 0.2 {
            let splash_y = area.y + area.height - 1;
            let num_splashes = (energy * 8.0) as usize;
            for i in 0..num_splashes {
                let seed = (i as u64 * 13 + time / 3) % 1000;
                let x = area.x + (seed as u16 % area.width);
                if let Some(cell) = buf.cell_mut((x, splash_y)) {
                    cell.set_char('∙').set_style(Style::default().fg(self.theme.accent));
                }
            }
        }
    }
}

impl<'a> Widget for Visualizer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = self.theme;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_style())
            .title(Span::styled(" Visualizer ", theme.title_style()));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width < 4 || inner.height < 1 {
            return;
        }

        // Render the selected visualization
        if self.is_playing && !self.is_paused {
            match self.mode {
                VisualizationMode::Spirograph => self.render_spirograph(inner, buf),
                VisualizationMode::Pulse => self.render_pulse(inner, buf),
                VisualizationMode::Wave => self.render_wave(inner, buf),
                VisualizationMode::Bounce => self.render_bounce(inner, buf),
                VisualizationMode::Starfield => self.render_starfield(inner, buf),
                VisualizationMode::Heart => self.render_heart(inner, buf),
                VisualizationMode::Spiral => self.render_spiral(inner, buf),
                VisualizationMode::Rain => self.render_rain(inner, buf),
            }
        }

        // If paused, show message
        if self.is_paused && inner.width > 10 {
            let msg = "PAUSED";
            let x = inner.x + (inner.width - msg.len() as u16) / 2;
            let y = inner.y + inner.height / 2;
            if y < inner.y + inner.height {
                for (i, c) in msg.chars().enumerate() {
                    if let Some(cell) = buf.cell_mut((x + i as u16, y)) {
                        cell.set_char(c).set_style(theme.paused_style());
                    }
                }
            }
        }

        // If not playing, show message
        if !self.is_playing && inner.width > 20 {
            let msg = "Select a station to play";
            let x = inner.x + (inner.width.saturating_sub(msg.len() as u16)) / 2;
            let y = inner.y + inner.height / 2;
            if y < inner.y + inner.height {
                for (i, c) in msg.chars().enumerate() {
                    let pos_x = x + i as u16;
                    if pos_x < inner.x + inner.width {
                        if let Some(cell) = buf.cell_mut((pos_x, y)) {
                            cell.set_char(c).set_style(theme.muted_style());
                        }
                    }
                }
            }
        }
    }
}
