use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    TogglePlayPause,
    VolumeUp,
    VolumeDown,
    ToggleMute,
    ToggleFavorite,
    NextStation,
    PrevStation,
    SelectStation,
    GoToTop,
    GoToBottom,
    ToggleSortMode,
    ToggleVisualizer,
    CycleVisualization,
    ToggleArtwork,
    ToggleHistory,
    QualityUp,
    QualityDown,
    ToggleHelp,
    ToggleTheme,
    Refresh,
    CloseOverlay,
}

pub fn handle_key(key: KeyEvent, show_help: bool) -> Option<Action> {
    // If help is shown, any key closes it
    if show_help {
        return Some(Action::CloseOverlay);
    }

    match key.code {
        // Quit
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Esc => Some(Action::Quit),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Action::Quit),

        // Playback
        KeyCode::Char('p') | KeyCode::Char(' ') => Some(Action::TogglePlayPause),
        KeyCode::Enter => Some(Action::SelectStation),

        // Volume
        KeyCode::Char('+') | KeyCode::Char('=') => Some(Action::VolumeUp),
        KeyCode::Char('-') | KeyCode::Char('_') => Some(Action::VolumeDown),
        KeyCode::Char('m') => Some(Action::ToggleMute),

        // Navigation
        KeyCode::Down | KeyCode::Char('j') => Some(Action::NextStation),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::PrevStation),
        KeyCode::Char('g') => Some(Action::GoToTop),
        KeyCode::Char('G') => Some(Action::GoToBottom),

        // Actions
        KeyCode::Char('f') => Some(Action::ToggleFavorite),
        KeyCode::Char('s') => Some(Action::ToggleSortMode),
        KeyCode::Char('t') => Some(Action::ToggleTheme),
        KeyCode::Char('v') => Some(Action::CycleVisualization),
        KeyCode::Char('V') => Some(Action::ToggleVisualizer),
        KeyCode::Char('a') => Some(Action::ToggleArtwork),
        KeyCode::Char('r') => Some(Action::ToggleHistory),
        KeyCode::Char('>') | KeyCode::Char('.') => Some(Action::QualityUp),
        KeyCode::Char('<') | KeyCode::Char(',') => Some(Action::QualityDown),
        KeyCode::Char('R') => Some(Action::Refresh),
        KeyCode::Char('?') => Some(Action::ToggleHelp),

        _ => None,
    }
}
