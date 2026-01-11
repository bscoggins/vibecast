# VIBECAST BUILD PROMPTS FOR CLAUDE OPUS 4.5 IN OPENCODE

## PHASE 1: Foundation & API Integration

**Prompt 1A: Project Setup & Soma.fm API Integration**

```
I want to build a terminal UI application called "vibecast" in Go that streams Soma.fm radio stations. This will run alongside coding sessions in the terminal.

First phase requirements:
1. Fetch station list dynamically from Soma.fm's API (https://somafm.com/channels.json)
2. Parse and store station data including:
   - Station ID, title, description
   - Stream URLs for both high-quality (256kbps) and standard (128kbps)
   - Station image URLs
3. Create a configuration system that allows users to choose:
   - Default quality (high/standard)
   - Default station
4. Use mpv as the audio backend (check for availability, give helpful install instructions if missing)
5. Implement basic playback control functions:
   - Play station with quality selection
   - Stop playback
   - Get current playback status

Project structure:
- main.go - entry point
- api/somafm.go - API client for Soma.fm
- player/mpv.go - mpv wrapper and playback control
- config/config.go - configuration management
- go.mod - module definition

Please start by:
1. Creating the project structure
2. Implementing the Soma.fm API client that fetches and parses channels.json
3. Setting up the configuration system (save to ~/.config/vibecast/config.json)
4. Creating the mpv wrapper for basic playback

Use proper error handling and make the code idiomatic Go. Add comments for clarity.
```

---

## PHASE 2: Terminal UI with Bubbletea

**Prompt 2A: Interactive TUI with Bubbletea**

```
Now let's build the interactive terminal UI using bubbletea (charmbracelet/bubbletea).

Requirements for the TUI:
1. Use bubbletea for the interactive interface
2. Use lipgloss for styling (charmbracelet/lipgloss)
3. Use bubbles for components like list, viewport (charmbracelet/bubbles)

The UI should have these views:

MAIN VIEW (Station List):
- Show list of all Soma.fm stations (scrollable)
- Display for each station:
  - Station name (prominent)
  - Description (secondary text)
  - Current playing indicator (♪ symbol if playing)
  - Quality badge (256k or 128k)
- Highlight selected station
- Show controls at bottom:
  - Navigation hints (↑/↓ to move, Enter to play, Space to pause, n for next, q to quit)

PLAYING VIEW (Now Playing):
- Show current station name prominently
- Display current song/track info (pulled from mpv metadata if available)
- Show playback status (Playing/Paused/Stopped)
- Show elapsed time
- ASCII-style clickable buttons:
  - [▶ Play] [⏸ Pause] [⏭ Next] [⏹ Stop] [🔊 Vol]
  - Make these clickable with mouse support
- Show station logo as ASCII art (convert image URL to ASCII, use a library like jp2a or implement basic conversion)
- Quality indicator

Keyboard shortcuts:
- Space: Play/Pause toggle
- n: Next station (cycle through list)
- p: Previous station
- s: Stop playback
- h: Toggle between high/standard quality
- l: Return to station list view
- q: Quit
- ↑/↓: Volume control when in playing view
- Mouse clicks on ASCII buttons should work

Create files:
- ui/model.go - bubbletea model and state management
- ui/views.go - view rendering functions
- ui/styles.go - lipgloss styles
- ui/update.go - update function and message handling
- ui/ascii.go - ASCII art generation for logos and buttons

Implement smooth transitions between views and proper state management.
```

**Prompt 2B: Metadata & Real-time Updates**

```
Enhance the TUI to show real-time song information:

1. Parse metadata from mpv (use --input-ipc-server flag with mpv)
2. Set up IPC communication with mpv to get:
   - Current track/song title
   - Artist (if available)
   - Stream metadata
   - Playback position
3. Update the "Now Playing" view in real-time as metadata changes
4. Add a ticker to update elapsed time every second
5. Show a visualizer (simple ASCII bars) that responds to playback status

Files to modify:
- player/mpv.go - add IPC socket communication
- player/metadata.go - new file for metadata parsing
- ui/model.go - add metadata to state
- ui/views.go - display metadata in playing view

Handle cases where metadata isn't available gracefully.
```

---

## PHASE 3: Advanced Features

**Prompt 3A: ASCII Art Station Logos**

```
Add station logo display to the TUI:

1. Fetch station images from the Soma.fm API (image URLs are in channels.json)
2. Download and cache images locally (~/.cache/vibecast/logos/)
3. Convert images to ASCII art using one of these approaches:
   - Use the 'github.com/qeesung/image2ascii' library
   - Or implement a basic grayscale-to-ASCII converter
4. Display ASCII logo in the "Now Playing" view
5. Create a fallback generic music note ASCII art if image unavailable
6. Optimize ASCII size based on terminal dimensions

Files:
- ui/ascii_art.go - image to ASCII conversion
- api/image_cache.go - image downloading and caching

The logo should be displayed prominently but not take up too much space (maybe 20x10 character area).
```

**Prompt 3B: Mouse Support & Clickable Buttons**

```
Implement full mouse support for the UI:

1. Enable bubbletea mouse support
2. Make ASCII buttons actually clickable:
   - Detect mouse clicks on button coordinates
   - Highlight button on hover
   - Trigger appropriate action on click
3. Make the station list clickable (click to play station)
4. Add visual feedback for interactions (button press animations)

Files to modify:
- ui/update.go - handle mouse events
- ui/views.go - render interactive elements with hit zones
- ui/buttons.go - new file for button components

Use bubbletea's mouse events and zone tracking for hit detection.
```

**Prompt 3C: Volume Control & Visualizer**

```
Add volume control and a simple audio visualizer:

1. Volume control:
   - Send volume commands to mpv via IPC
   - Display volume level (0-100%) with a bar
   - Keyboard: ↑/↓ to adjust, m to mute
   - Mouse: click volume slider
2. Simple visualizer:
   - Create ASCII bar visualizer (5-10 bars)
   - Animate based on playback state (simulate with random when can't get actual audio data)
   - Show different animation for different stations/genres

Files:
- ui/visualizer.go - ASCII visualizer component
- player/volume.go - volume control via mpv IPC

Keep the visualizer lightweight and fun rather than trying to be technically accurate.
```

---

## PHASE 4: Polish & Extras

**Prompt 4A: Favorites & History**

```
Add favorites and playback history:

1. Favorites system:
   - Mark stations as favorites (press 'f' in station list)
   - Show favorites at top of list with ★ symbol
   - Save to ~/.config/vibecast/favorites.json
2. Playback history:
   - Track recently played stations
   - Show in a separate view (press 'h' for history)
   - Save to ~/.config/vibecast/history.json
3. Add quick-switch keybinds:
   - Numbers 1-9 to jump to favorites

Files:
- data/favorites.go - favorites management
- data/history.go - history tracking
- ui/views.go - update to show favorites and history views
```

**Prompt 4B: Final Polish**

```
Final touches for vibecast:

1. Add a help screen (press '?'):
   - Show all keyboard shortcuts
   - Show tips and tricks
   - Show link to support Soma.fm
2. Add color themes:
   - Default, Dark, Light, Hacker (green on black), Sunset
   - Switchable with 't' key
   - Save preference to config
3. Add notification support (optional):
   - Desktop notification when track changes (use beeep or similar)
   - Make it configurable
4. Error handling and recovery:
   - Graceful handling of network issues
   - Auto-reconnect on stream failure
   - User-friendly error messages
5. Performance optimization:
   - Lazy load station images
   - Cache API responses
   - Optimize rendering
6. Add startup flags:
   - --station <name> - start playing immediately
   - --quality <high|standard> - override quality
   - --no-ui - run headless (daemon mode from original)

Files:
- ui/help.go - help screen
- ui/themes.go - theme system
- ui/notifications.go - notification support
- main.go - command-line flag handling

Create a polished, production-ready experience.
```

---

## PHASE 5: Documentation & Distribution

**Prompt 5: Final Documentation**

```
Create comprehensive documentation and distribution setup:

1. README.md with:
   - Beautiful ASCII art logo for vibecast
   - Clear installation instructions
   - Screenshots (use terminal screenshots or ASCII diagrams)
   - Full feature list
   - Keyboard shortcuts reference
   - Configuration guide
   - Troubleshooting section
   - Credits to Soma.fm with donation link

2. CONTRIBUTING.md:
   - How to contribute
   - Development setup
   - Code style guidelines

3. Build system:
   - Makefile with targets: build, install, clean, test
   - Support for cross-compilation (Mac, Linux, Windows)
   - Create release binaries

4. Add a screenshot feature:
   - Capture terminal state as text (for README)
   - Save with 's' key in help screen

5. Include a demo mode:
   - --demo flag that shows UI without requiring mpv
   - Simulates playback for screenshots

Make it look professional and welcoming for contributors.
```

---

## BONUS: Advanced Enhancements (Optional)

**Bonus Prompt A: Integration Features**

```
Add integration capabilities for power users:

1. Status export:
   - Write current status to ~/.config/vibecast/status.json
   - Include: station, song, quality, playing state
   - Update in real-time for other tools to read

2. Add shell integration scripts:
   - starship.toml integration (show 🎵 in prompt when playing)
   - tmux status bar integration
   - Include example configs in /integrations directory

3. IPC server:
   - Allow other programs to control vibecast
   - Socket at /tmp/vibecast.sock
   - Accept JSON commands

This makes vibecast scriptable and integrable with developer workflows.
```

**Bonus Prompt B: Playlist & Discovery**

```
Add discovery and playlist features:

1. Similar stations:
   - When viewing a station, show similar stations
   - Based on genre/tags from API

2. Random mode:
   - Play random stations (press 'r')
   - Smart shuffle (avoid repeating too soon)

3. Schedule mode:
   - Set different stations for different times of day
   - "Focus hours" station vs "chill evening" station
   - Configurable in ~/.config/vibecast/schedule.json

4. Stats tracking:
   - Track listening habits
   - Show stats view: total time, favorite station, etc.
```

---

## HOW TO USE THESE PROMPTS

**Recommended Approach:**

1. **Start with Phase 1** - Get the foundation solid
2. **Test it** - Make sure API fetching and basic playback works
3. **Move to Phase 2** - Build the TUI iteratively
4. **Test each feature** as it's added
5. **Phases 3-4** - Add features as desired
6. **Phase 5** - Polish before sharing with Soma.fm

**Tips for Working with Claude Opus:**

- Feed one prompt at a time
- After each phase, test the code
- Ask Claude to explain any unclear parts
- Request modifications as needed ("Can you make the colors more muted?")
- Show Claude errors if things don't work
- Ask for intermediate checkpoints ("Let's test the API client first")

**Code Review Checkpoints:**

After Phase 1: "Review the code and suggest improvements for error handling and structure"
After Phase 2: "Review the TUI code for performance and UX improvements"
After Phase 4: "Do a final code review focusing on polish and edge cases"

Good luck building vibecast! 🚀🎵
