#![allow(dead_code)]

use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeType {
    Synthwave,
    Ocean,
    Forest,
    Sunset,
    Monochrome,
    #[default]
    Cyberpunk,
}

impl ThemeType {
    pub fn next(self) -> Self {
        match self {
            Self::Synthwave => Self::Ocean,
            Self::Ocean => Self::Forest,
            Self::Forest => Self::Sunset,
            Self::Sunset => Self::Monochrome,
            Self::Monochrome => Self::Cyberpunk,
            Self::Cyberpunk => Self::Synthwave,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Synthwave => "Synthwave",
            Self::Ocean => "Ocean",
            Self::Forest => "Forest",
            Self::Sunset => "Sunset",
            Self::Monochrome => "Mono",
            Self::Cyberpunk => "Cyberpunk",
        }
    }
}

#[derive(Clone)]
pub struct Theme {
    pub name: &'static str,
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub muted: Color,
    pub highlight: Color,
    pub success: Color,
    pub warning: Color,
}

impl Theme {
    pub fn from_type(theme_type: ThemeType) -> Self {
        match theme_type {
            ThemeType::Synthwave => Self::synthwave(),
            ThemeType::Ocean => Self::ocean(),
            ThemeType::Forest => Self::forest(),
            ThemeType::Sunset => Self::sunset(),
            ThemeType::Monochrome => Self::monochrome(),
            ThemeType::Cyberpunk => Self::cyberpunk(),
        }
    }

    fn synthwave() -> Self {
        Self {
            name: "Synthwave",
            background: Color::Rgb(20, 12, 28),
            foreground: Color::Rgb(255, 230, 250),
            primary: Color::Rgb(255, 0, 128),      // Hot pink
            secondary: Color::Rgb(0, 255, 255),    // Cyan
            accent: Color::Rgb(255, 100, 200),     // Light pink
            muted: Color::Rgb(120, 80, 140),
            highlight: Color::Rgb(255, 220, 0),    // Yellow
            success: Color::Rgb(0, 255, 180),
            warning: Color::Rgb(255, 180, 0),
        }
    }

    fn ocean() -> Self {
        Self {
            name: "Ocean",
            background: Color::Rgb(10, 25, 47),
            foreground: Color::Rgb(200, 220, 240),
            primary: Color::Rgb(100, 180, 255),    // Sky blue
            secondary: Color::Rgb(0, 200, 180),    // Teal
            accent: Color::Rgb(150, 220, 255),     // Light blue
            muted: Color::Rgb(70, 100, 130),
            highlight: Color::Rgb(255, 200, 100),  // Sandy
            success: Color::Rgb(80, 220, 150),
            warning: Color::Rgb(255, 180, 80),
        }
    }

    fn forest() -> Self {
        Self {
            name: "Forest",
            background: Color::Rgb(15, 25, 15),
            foreground: Color::Rgb(220, 235, 210),
            primary: Color::Rgb(120, 200, 80),     // Leaf green
            secondary: Color::Rgb(180, 140, 80),   // Wood brown
            accent: Color::Rgb(200, 230, 150),     // Light green
            muted: Color::Rgb(80, 100, 70),
            highlight: Color::Rgb(255, 200, 80),   // Sunlight
            success: Color::Rgb(100, 220, 100),
            warning: Color::Rgb(220, 180, 60),
        }
    }

    fn sunset() -> Self {
        Self {
            name: "Sunset",
            background: Color::Rgb(30, 15, 25),
            foreground: Color::Rgb(255, 240, 230),
            primary: Color::Rgb(255, 100, 50),     // Orange
            secondary: Color::Rgb(255, 180, 100),  // Light orange
            accent: Color::Rgb(255, 80, 120),      // Pink-red
            muted: Color::Rgb(140, 90, 100),
            highlight: Color::Rgb(255, 220, 100),  // Yellow
            success: Color::Rgb(150, 230, 120),
            warning: Color::Rgb(255, 200, 80),
        }
    }

    fn monochrome() -> Self {
        Self {
            name: "Mono",
            background: Color::Rgb(15, 15, 15),
            foreground: Color::Rgb(220, 220, 220),
            primary: Color::Rgb(255, 255, 255),    // White
            secondary: Color::Rgb(180, 180, 180),  // Light gray
            accent: Color::Rgb(200, 200, 200),     // Gray
            muted: Color::Rgb(100, 100, 100),
            highlight: Color::Rgb(255, 255, 255),  // White
            success: Color::Rgb(180, 255, 180),
            warning: Color::Rgb(255, 220, 150),
        }
    }

    fn cyberpunk() -> Self {
        Self {
            name: "Cyberpunk",
            background: Color::Rgb(10, 10, 20),
            foreground: Color::Rgb(0, 255, 65),    // Matrix green
            primary: Color::Rgb(0, 255, 65),       // Neon green
            secondary: Color::Rgb(255, 0, 100),    // Neon pink
            accent: Color::Rgb(0, 200, 255),       // Neon blue
            muted: Color::Rgb(0, 100, 40),
            highlight: Color::Rgb(255, 255, 0),    // Yellow
            success: Color::Rgb(0, 255, 100),
            warning: Color::Rgb(255, 150, 0),
        }
    }

    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    pub fn normal_style(&self) -> Style {
        Style::default().fg(self.foreground)
    }

    pub fn muted_style(&self) -> Style {
        Style::default().fg(self.muted)
    }

    pub fn highlight_style(&self) -> Style {
        Style::default()
            .fg(self.background)
            .bg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    pub fn favorite_style(&self) -> Style {
        Style::default().fg(self.highlight)
    }

    pub fn playing_style(&self) -> Style {
        Style::default()
            .fg(self.success)
            .add_modifier(Modifier::BOLD)
    }

    pub fn paused_style(&self) -> Style {
        Style::default()
            .fg(self.warning)
            .add_modifier(Modifier::BOLD)
    }

    pub fn border_style(&self) -> Style {
        Style::default().fg(self.muted)
    }

    pub fn active_border_style(&self) -> Style {
        Style::default().fg(self.primary)
    }

    /// Get spectrum bar colors for visualizer (low to high)
    pub fn spectrum_colors(&self) -> [Color; 4] {
        [self.secondary, self.primary, self.accent, self.highlight]
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::cyberpunk()
    }
}

// Legacy static theme for components that haven't been updated yet
pub static THEME: std::sync::LazyLock<Theme> = std::sync::LazyLock::new(Theme::default);
