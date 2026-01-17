mod api;
mod app;
mod artwork;
mod input;
mod player;
mod storage;
mod ui;
mod visualizer;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    Terminal,
};
use std::io;
use std::time::{Duration, Instant};

use app::App;
use input::handle_key;
use ui::{Header, HelpOverlay, NowPlaying, SongHistory, StationList, StatusBar, Visualizer, init_picker};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize image picker before entering TUI to avoid escape sequence leaks
    init_picker();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    // Initialize app
    app.init().await?;

    let tick_rate = Duration::from_millis(16); // ~60fps for smooth visualizer
    let metadata_interval = Duration::from_secs(10);
    let mut last_tick = Instant::now();
    let mut last_metadata = Instant::now();

    loop {
        // Draw UI
        terminal.draw(|f| {
            let area = f.area();
            let theme = &app.theme;

            // Main layout
            let chunks = Layout::vertical([
                Constraint::Length(3),  // Header
                Constraint::Min(10),    // Main content
                Constraint::Length(1),  // Status bar
            ])
            .split(area);

            // Header
            let station_name = app.current_channel().map(|c| c.title.as_str());
            let header = Header::new(station_name, theme);
            f.render_widget(header, chunks[0]);

            // Main content - split horizontally
            let content_chunks = Layout::horizontal([
                Constraint::Percentage(35),  // Station list
                Constraint::Percentage(65),  // Right panel
            ])
            .split(chunks[1]);

            // Station list - render sorted channels
            let sorted_channels: Vec<_> = app.sorted_channels().into_iter().cloned().collect();
            let current_station_id = app.current_channel().map(|c| c.id.as_str());
            let station_list = StationList::new(
                &sorted_channels,
                app.favorites.favorites(),
                current_station_id,
                true,
                theme,
            );
            let mut list_state = app.list_state.clone();
            f.render_stateful_widget(station_list, content_chunks[0], &mut list_state);

            // Right panel - split vertically for now playing, history, and visualizer
            let show_history = app.show_history && !app.song_history.is_empty();
            let right_chunks = Layout::vertical([
                Constraint::Min(8),                                     // Now playing
                Constraint::Length(if show_history { 8 } else { 0 }),   // Song history
                Constraint::Length(if app.show_visualizer { 12 } else { 0 }),  // Visualizer (doubled)
            ])
            .split(content_chunks[1]);

            // Now playing
            let current_channel = app.current_channel().cloned();
            let current_song = app.current_song.clone();
            let stream_title = app.stream_title.clone();
            let is_paused = app.player.state.paused;

            let now_playing = NowPlaying::new(
                current_channel.as_ref(),
                current_song.as_ref(),
                stream_title.as_deref(),
                is_paused,
                app.audio_quality,
                app.show_artwork,
                theme,
            );
            f.render_stateful_widget(
                now_playing,
                right_chunks[0],
                &mut app.artwork_state,
            );

            // Song history panel
            if show_history {
                let song_history = SongHistory::new(&app.song_history, theme);
                f.render_widget(song_history, right_chunks[1]);
            }

            // Visualizer
            if app.show_visualizer {
                let visualizer = Visualizer::new(
                    &app.spectrum_data,
                    app.player.state.playing,
                    app.player.state.paused,
                    app.visualization_mode,
                    app.frame,
                    theme,
                );
                f.render_widget(visualizer, right_chunks[2]);
            }

            // Status bar
            let status_bar = StatusBar::new(
                app.player.state.playing,
                app.player.state.paused,
                if app.is_muted { 0 } else { app.player.state.volume },
                app.theme.name,
                theme,
            );
            f.render_widget(status_bar, chunks[2]);

            // Help overlay
            if app.show_help {
                f.render_widget(HelpOverlay::new(theme), area);
            }
        })?;

        // Handle events
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let Some(action) = handle_key(key, app.show_help) {
                        app.handle_action(action).await?;
                    }
                }
            }
        }

        // Tick - update visualizer spectrum
        if last_tick.elapsed() >= tick_rate {
            app.update_spectrum().await;
            last_tick = Instant::now();
        }

        // Update metadata periodically
        if last_metadata.elapsed() >= metadata_interval {
            let _ = app.update_metadata().await;
            last_metadata = Instant::now();
        }

        // Check if should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
