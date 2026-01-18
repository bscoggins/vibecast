# Vibecast

A beautiful terminal-based internet radio streaming application for [SomaFM](https://somafm.com), built with Rust.

![Vibecast](https://img.shields.io/badge/rust-1.75%2B-orange) ![License](https://img.shields.io/badge/license-MIT-blue)

## Features

- **Stream SomaFM Radio** - Access all SomaFM channels with multiple audio quality options
- **Beautiful TUI** - Modern terminal interface built with Ratatui
- **Album Artwork** - Display station artwork (best quality in Kitty, iTerm2, WezTerm)
- **8 Visualizations** - Music-reactive visual effects that respond to audio levels
- **6 Color Themes** - Synthwave, Ocean, Forest, Sunset, Monochrome, Cyberpunk
- **Favorites** - Mark and sort your favorite stations
- **Song History** - See recently played tracks
- **Persistent Settings** - Theme and visualization preferences are saved

## Screenshots

```
┌─────────────────────────────────────────────────────────────────────┐
│  VIBECAST                                            ♫ Groove Salad │
├─────────────────────┬───────────────────────────────────────────────┤
│   STATIONS          │  NOW PLAYING                                  │
│   ─────────         │  ┌────────┐                                   │
│   ★ Groove Salad    │  │ARTWORK │  ▶ Groove Salad [HQ]              │
│     Drone Zone      │  │        │    Ambient/Chill                  │
│     Space Station   │  └────────┘                                   │
│     Secret Agent    │  ♫ "The Plug"                                 │
│     DEF CON Radio   │    by Bonobo                                  │
│                     ├───────────────────────────────────────────────┤
│                     │  PREVIOUSLY PLAYED                            │
│                     │  • She Likes Ambient - Softwaver              │
│                     │  • Between Seasons - Sine                     │
│                     ├───────────────────────────────────────────────┤
│                     │  VISUALIZER                                   │
│                     │     ·bg bg bg·  ○bg bg○  ·bg bg bg·           │
│                     │        (spirograph effect)                    │
├─────────────────────┴───────────────────────────────────────────────┤
│  ▶ Playing │ Vol: ████████░░  80% │ Synthwave  │ [p]lay [f]av [?]   │
└─────────────────────────────────────────────────────────────────────┘
```

## Requirements

- **Rust** 1.75 or later
- **mpv** media player (for audio playback)
  - macOS: `brew install mpv`
  - Linux: `apt install mpv` or `pacman -S mpv`
  - Windows: `winget install mpv` or `choco install mpv`
- **Terminal** with Unicode support
  - For best artwork quality: Kitty, iTerm2, WezTerm, or terminals with Sixel support
  - Windows: Windows Terminal recommended for best experience
  - Basic support: Any terminal with Unicode (uses halfblock fallback)

## Installation

### Pre-built Binaries

Download the latest release from the [Releases page](https://github.com/bscoggins/vibecast/releases):

| Platform | File |
|----------|------|
| macOS Apple Silicon (M1/M2/M3) | `vibecast-macos-aarch64.tar.gz` |
| macOS Intel | `vibecast-macos-x86_64.tar.gz` |
| Linux x86_64 | `vibecast-linux-x86_64.tar.gz` |
| Linux ARM64 | `vibecast-linux-aarch64.tar.gz` |
| Windows x86_64 | `vibecast-windows-x86_64.zip` |

#### macOS / Linux

```bash
# Download and extract (example for macOS Apple Silicon)
tar -xzf vibecast-macos-aarch64.tar.gz

# Make executable and move to PATH
chmod +x vibecast-macos-aarch64
sudo mv vibecast-macos-aarch64 /usr/local/bin/vibecast

# Run
vibecast
```

#### Windows

```powershell
# Download vibecast-windows-x86_64.zip and extract
Expand-Archive vibecast-windows-x86_64.zip -DestinationPath .

# Run directly
.\vibecast-windows-x86_64.exe

# Or add to PATH and run from anywhere
vibecast
```

### From Source

```bash
# Clone the repository
git clone https://github.com/bscoggins/vibecast.git
cd vibecast

# Build release version
cargo build --release

# Run
./target/release/vibecast
```

### Development

```bash
# Run in development mode
cargo run

# Run with logging
RUST_LOG=debug cargo run
```

## Keyboard Controls

### Playback
| Key | Action |
|-----|--------|
| `p` / `Space` | Play / Pause |
| `Enter` | Play selected station |
| `q` / `Esc` | Quit |

### Navigation
| Key | Action |
|-----|--------|
| `j` / `Down` | Move down |
| `k` / `Up` | Move up |
| `g` | Go to top |
| `G` | Go to bottom |

### Volume
| Key | Action |
|-----|--------|
| `+` / `=` | Volume up |
| `-` / `_` | Volume down |
| `m` | Mute / Unmute |

### Audio Quality
| Key | Action |
|-----|--------|
| `>` / `.` | Higher quality (LQ → MQ → HQ) |
| `<` / `,` | Lower quality (HQ → MQ → LQ) |

### Display
| Key | Action |
|-----|--------|
| `v` | Cycle visualization style |
| `V` | Show/hide visualizer |
| `a` | Toggle artwork display |
| `r` | Toggle recently played panel |
| `t` | Cycle color theme |

### Stations
| Key | Action |
|-----|--------|
| `f` | Toggle favorite |
| `s` | Cycle sort mode (Favorites → Alphabetical → Listeners) |
| `R` | Refresh station list |

### Other
| Key | Action |
|-----|--------|
| `?` | Show help overlay |

## Visualizations

Vibecast includes 8 music-reactive visualizations that respond to audio energy levels:

1. **Spirograph** - Beautiful rotating hypotrochoid patterns with three overlapping designs
2. **Pulse** - Expanding concentric rings that pulse outward with the beat
3. **Wave** - Flowing multi-layered sine waves
4. **Bounce** - Eight bouncing shapes with varied colors and speeds
5. **Stars** - Starfield zooming outward from the center
6. **Heart** - Three pulsing hearts displayed horizontally
7. **Spiral** - Three rotating spiral patterns with different colors
8. **Rain** - Gentle falling rain effect with splashes

Press `v` to cycle through visualizations. Your selection is automatically saved.

## Themes

Six color themes are available:

- **Synthwave** - Neon pink and cyan retro vibes
- **Ocean** - Cool blues and teals
- **Forest** - Natural greens and earth tones
- **Sunset** - Warm oranges and purples
- **Monochrome** - Classic black and white
- **Cyberpunk** - Electric yellows and magentas

Press `t` to cycle through themes. Your selection is automatically saved.

## Audio Quality

Three quality levels are available:

| Level | Label | Description |
|-------|-------|-------------|
| Highest | HQ | Best available (usually AAC 256kbps or FLAC) |
| High | MQ | Medium quality (usually AAC 128kbps) |
| Low | LQ | Lower bandwidth (usually AAC 64kbps) |

Use `>` and `<` to adjust quality. If currently playing, the stream will automatically restart at the new quality.

## Configuration

Settings are automatically saved to:
- **macOS**: `~/Library/Application Support/com.vibecast.vibecast/config.json`
- **Linux**: `~/.config/vibecast/config.json`
- **Windows**: `%APPDATA%\vibecast\vibecast\config.json`

Saved settings include:
- Selected color theme
- Selected visualization mode

Favorites are saved to:
- **macOS**: `~/Library/Application Support/com.vibecast.vibecast/favorites.json`
- **Linux**: `~/.config/vibecast/favorites.json`
- **Windows**: `%APPDATA%\vibecast\vibecast\favorites.json`

## Project Structure

```
vibecast/
├── Cargo.toml              # Dependencies and metadata
├── src/
│   ├── main.rs             # Entry point and main loop
│   ├── app.rs              # Application state and logic
│   ├── api/
│   │   ├── mod.rs
│   │   ├── somafm.rs       # SomaFM API client
│   │   └── types.rs        # Channel, Song, AudioQuality types
│   ├── player/
│   │   ├── mod.rs
│   │   └── mpv.rs          # mpv IPC controller
│   ├── visualizer/
│   │   ├── mod.rs
│   │   └── spectrum.rs     # Audio level analysis
│   ├── artwork/
│   │   ├── mod.rs
│   │   └── cache.rs        # Image caching
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── artwork.rs      # Artwork display widget
│   │   ├── header.rs       # Header bar
│   │   ├── help.rs         # Help overlay
│   │   ├── now_playing.rs  # Now playing panel
│   │   ├── song_history.rs # Recently played panel
│   │   ├── station_list.rs # Station list widget
│   │   ├── status_bar.rs   # Bottom status bar
│   │   ├── theme.rs        # Color themes
│   │   └── visualizer.rs   # Visualizations
│   ├── input/
│   │   └── handler.rs      # Keyboard input handling
│   └── storage/
│       ├── mod.rs
│       ├── config.rs       # Settings persistence
│       └── favorites.rs    # Favorites persistence
└── README.md
```

## Technical Details

### SomaFM API

Vibecast uses the SomaFM public API:

| Endpoint | URL | Purpose |
|----------|-----|---------|
| Channels | `https://api.somafm.com/channels.json` | Station list with metadata |
| Songs | `https://somafm.com/songs/{id}.json` | Currently/recently playing |

### mpv Integration

Audio playback is handled by mpv via JSON IPC:
- **macOS/Linux**: Unix socket at `/tmp/vibecast_mpv_{pid}.sock`
- **Windows**: Named pipe at `\\.\pipe\vibecast_mpv_{pid}`
- Socket/pipe path is unique per process to allow multiple instances
- Commands sent: `loadfile`, `set_property` (volume, pause), `get_property`
- Audio stats (RMS/peak levels) are retrieved for visualization

### Platform Support

Vibecast supports **macOS**, **Linux**, and **Windows** (10 and later).

- macOS: Tested on Apple Silicon and Intel
- Linux: x86_64 and ARM64
- Windows: x86_64 (PowerShell, cmd.exe, Windows Terminal)

### Image Protocol Support

Artwork rendering quality depends on terminal support:
1. **Best**: Kitty Graphics Protocol, iTerm2 Protocol
2. **Good**: Sixel graphics
3. **Basic**: Unicode halfblock characters (universal fallback)

The `ratatui-image` crate automatically detects and uses the best available protocol.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `ratatui` | Terminal UI framework |
| `crossterm` | Cross-platform terminal manipulation |
| `ratatui-image` | Image rendering in terminal |
| `tokio` | Async runtime |
| `reqwest` | HTTP client for API calls |
| `serde` / `serde_json` | JSON serialization |
| `directories` | Platform-specific config paths |
| `image` | Image loading and processing |
| `anyhow` | Error handling |

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- [SomaFM](https://somafm.com) for providing excellent commercial-free internet radio
- [Ratatui](https://ratatui.rs) for the excellent TUI framework
- The Rust community for the amazing ecosystem

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Releasing

Releases are automated via GitHub Actions. To create a new release:

1. Update the version in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.0"  # Bump version number
   ```

2. Commit the version change:
   ```bash
   git add Cargo.toml Cargo.lock
   git commit -m "Bump version to 0.2.0"
   git push origin main
   ```

3. Create and push a version tag:
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```

The release workflow will automatically:
- Build binaries for all supported platforms (Linux x86_64/aarch64, macOS x86_64/aarch64, Windows x86_64)
- Create a GitHub release with the tag name
- Upload compressed binaries as release assets (.tar.gz for Unix, .zip for Windows)
- Generate release notes with installation instructions
