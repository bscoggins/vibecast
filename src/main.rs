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
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, watch, Mutex};
use tokio::time;

use api::{SomaFmClient, Song};
use app::App;
use artwork::ImageCache;
use image::DynamicImage;
use input::handle_key;
use player::MpvController;
use ui::{
    init_picker, Header, HelpOverlay, NowPlaying, SongHistory, StationList, StatusBar, Visualizer,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct MetadataRequest {
    channel_id: Option<String>,
    image_url: Option<String>,
    show_artwork: bool,
}

enum AppUpdate {
    Songs {
        channel_id: String,
        current_song: Option<Song>,
        history: Vec<Song>,
    },
    StreamTitle {
        channel_id: Option<String>,
        title: Option<String>,
    },
    Artwork {
        channel_id: String,
        image: DynamicImage,
        url: String,
    },
}

fn build_metadata_request(app: &App) -> MetadataRequest {
    let (channel_id, image_url) = match app.current_channel() {
        Some(channel) => {
            let image_url = channel
                .xlimage
                .as_ref()
                .unwrap_or(&channel.largeimage)
                .to_string();
            (Some(channel.id.clone()), Some(image_url))
        }
        None => (None, None),
    };

    MetadataRequest {
        channel_id,
        image_url,
        show_artwork: app.show_artwork,
    }
}

async fn metadata_worker(
    mut req_rx: watch::Receiver<MetadataRequest>,
    player: Arc<Mutex<MpvController>>,
    update_tx: mpsc::UnboundedSender<AppUpdate>,
) {
    let api_client = SomaFmClient::new();
    let image_cache = ImageCache::default();
    let mut interval = time::interval(Duration::from_secs(10));
    let mut last_artwork_url: Option<String> = None;

    loop {
        tokio::select! {
            _ = interval.tick() => {}
            changed = req_rx.changed() => {
                if changed.is_err() {
                    break;
                }
            }
        }

        let req = req_rx.borrow().clone();
        let Some(channel_id) = req.channel_id.clone() else {
            last_artwork_url = None;
            continue;
        };

        if let Ok(Ok(songs)) =
            time::timeout(Duration::from_secs(5), api_client.get_songs(&channel_id)).await
        {
            let current_song = songs.first().cloned();
            let history = songs.into_iter().skip(1).take(5).collect();
            let _ = update_tx.send(AppUpdate::Songs {
                channel_id: channel_id.clone(),
                current_song,
                history,
            });
        }

        if req.show_artwork {
            if let Some(image_url) = req.image_url.clone() {
                if last_artwork_url.as_deref() != Some(image_url.as_str()) {
                    last_artwork_url = Some(image_url.clone());
                    if let Ok(Ok(bytes)) = time::timeout(
                        Duration::from_secs(5),
                        image_cache.get_or_fetch(&image_url, &channel_id),
                    )
                    .await
                    {
                        if let Ok(img) = image::load_from_memory(&bytes) {
                            let _ = update_tx.send(AppUpdate::Artwork {
                                channel_id: channel_id.clone(),
                                image: img,
                                url: image_url,
                            });
                        }
                    }
                }
            }
        } else {
            last_artwork_url = None;
        }

        if let Ok(mut locked) = player.try_lock() {
            if locked.state.playing {
                if let Ok(Ok(Some((artist, title)))) =
                    time::timeout(Duration::from_millis(500), locked.get_metadata()).await
                {
                    if !title.is_empty() {
                        let stream_title = if artist.is_empty() {
                            title
                        } else {
                            format!("{} - {}", artist, title)
                        };
                        let _ = update_tx.send(AppUpdate::StreamTitle {
                            channel_id: Some(channel_id.clone()),
                            title: Some(stream_title),
                        });
                    }
                }
            }
        }
    }
}

async fn audio_worker(
    player: Arc<Mutex<MpvController>>,
    audio_tx: watch::Sender<Option<(f32, f32)>>,
) {
    let mut interval = time::interval(Duration::from_millis(50));

    loop {
        interval.tick().await;

        let mut locked = match player.try_lock() {
            Ok(locked) => locked,
            Err(_) => continue,
        };

        if !locked.state.playing || locked.state.paused {
            let _ = audio_tx.send(None);
            continue;
        }

        let _ = audio_tx.send(locked.get_audio_stats().await);
    }
}

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

    let initial_request = build_metadata_request(app);
    let (metadata_tx, metadata_rx) = watch::channel(initial_request.clone());
    let (update_tx, mut update_rx) = mpsc::unbounded_channel();
    let (audio_tx, mut audio_rx) = watch::channel::<Option<(f32, f32)>>(None);

    tokio::spawn(metadata_worker(metadata_rx, app.player.clone(), update_tx));
    tokio::spawn(audio_worker(app.player.clone(), audio_tx));

    let tick_rate = Duration::from_millis(16); // ~60fps for smooth visualizer
    let mut last_tick = Instant::now();
    let mut last_request = initial_request;

    loop {
        while let Ok(update) = update_rx.try_recv() {
            match update {
                AppUpdate::Songs {
                    channel_id,
                    current_song,
                    history,
                } => {
                    if app.current_channel().map(|c| c.id.as_str()) == Some(channel_id.as_str()) {
                        app.current_song = current_song;
                        app.song_history = history;
                    }
                }
                AppUpdate::StreamTitle { channel_id, title } => {
                    if channel_id.as_deref() == app.current_channel().map(|c| c.id.as_str()) {
                        if let Some(title) = title {
                            app.stream_title = Some(title);
                        }
                    }
                }
                AppUpdate::Artwork {
                    channel_id,
                    image,
                    url,
                } => {
                    if app.show_artwork
                        && app.current_channel().map(|c| c.id.as_str()) == Some(channel_id.as_str())
                    {
                        app.artwork_state.set_image(image, &url);
                    }
                }
            }
        }

        if audio_rx.has_changed().unwrap_or(false) {
            app.audio_levels = *audio_rx.borrow_and_update();
        }

        let mut list_state = app.list_state.clone();

        // Draw UI
        terminal.draw(|f| {
            let area = f.area();
            let theme = &app.theme;

            // Main layout
            let chunks = Layout::vertical([
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Main content
                Constraint::Length(1), // Status bar
            ])
            .split(area);

            // Header
            let station_name = app.current_channel().map(|c| c.title.as_str());
            let header = Header::new(station_name, theme);
            f.render_widget(header, chunks[0]);

            // Main content - split horizontally
            let content_chunks = Layout::horizontal([
                Constraint::Percentage(35), // Station list
                Constraint::Percentage(65), // Right panel
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
            f.render_stateful_widget(station_list, content_chunks[0], &mut list_state);

            // Right panel - split vertically for now playing, history, and visualizer
            let show_history = app.show_history && !app.song_history.is_empty();
            let right_chunks = Layout::vertical([
                Constraint::Min(8),                                           // Now playing
                Constraint::Length(if show_history { 8 } else { 0 }),         // Song history
                Constraint::Length(if app.show_visualizer { 12 } else { 0 }), // Visualizer (doubled)
            ])
            .split(content_chunks[1]);

            // Now playing
            let current_channel = app.current_channel().cloned();
            let current_song = app.current_song.clone();
            let stream_title = app.stream_title.clone();
            let is_paused = app.playback_state.paused;

            let now_playing = NowPlaying::new(
                current_channel.as_ref(),
                current_song.as_ref(),
                stream_title.as_deref(),
                is_paused,
                app.audio_quality,
                app.show_artwork,
                theme,
            );
            f.render_stateful_widget(now_playing, right_chunks[0], &mut app.artwork_state);

            // Song history panel
            if show_history {
                let song_history = SongHistory::new(&app.song_history, theme);
                f.render_widget(song_history, right_chunks[1]);
            }

            // Visualizer
            if app.show_visualizer {
                let visualizer = Visualizer::new(
                    &app.spectrum_data,
                    app.playback_state.playing,
                    app.playback_state.paused,
                    app.visualization_mode,
                    app.frame,
                    theme,
                );
                f.render_widget(visualizer, right_chunks[2]);
            }

            // Status bar
            let status_bar = StatusBar::new(
                app.playback_state.playing,
                app.playback_state.paused,
                if app.is_muted {
                    0
                } else {
                    app.playback_state.volume
                },
                app.theme.name,
                theme,
            );
            f.render_widget(status_bar, chunks[2]);

            // Help overlay
            if app.show_help {
                f.render_widget(HelpOverlay::new(theme), area);
            }
        })?;

        app.list_state = list_state;

        // Handle events
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let Some(action) = handle_key(key, app.show_help) {
                        app.handle_action(action).await?;
                        let next_request = build_metadata_request(app);
                        if next_request != last_request {
                            let _ = metadata_tx.send(next_request.clone());
                            last_request = next_request;
                        }
                    }
                }
            }
        }

        // Tick - update visualizer spectrum
        if last_tick.elapsed() >= tick_rate {
            app.update_spectrum().await;
            last_tick = Instant::now();
        }

        // Check if should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
