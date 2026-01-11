# Vibecast Build - Quick Reference Card

## 🎯 Project Overview
**Goal:** Terminal UI app to stream Soma.fm while coding  
**Language:** Go  
**Key Libraries:** bubbletea, lipgloss, bubbles  
**Audio Backend:** mpv  

---

## 📋 Feature Checklist

### Core Features (Must Have)
- [ ] Dynamic station fetching from Soma.fm API
- [ ] High quality (256k) vs standard (128k) stream selection
- [ ] mpv integration with playback control
- [ ] Interactive TUI with station list
- [ ] Now Playing view with metadata
- [ ] Keyboard shortcuts (space, n, p, s, h, l, q)
- [ ] Real-time song/track display
- [ ] ASCII clickable buttons [▶ ⏸ ⏭ ⏹]
- [ ] Mouse support (clicks work)

### Enhanced Features (Nice to Have)
- [ ] ASCII art station logos
- [ ] Volume control (↑/↓, visual slider)
- [ ] Simple ASCII visualizer
- [ ] Favorites system (★ mark)
- [ ] Playback history
- [ ] Color themes (t to toggle)
- [ ] Help screen (?)

### Polish Features (Optional)
- [ ] Desktop notifications on track change
- [ ] Status export for shell integration
- [ ] Demo mode (--demo flag)
- [ ] Terminal screenshot feature
- [ ] Starship/tmux integration examples

---

## 🏗️ Project Structure

```
vibecast/
├── main.go              # Entry point, CLI flags
├── go.mod               # Dependencies
├── api/
│   ├── somafm.go       # Soma.fm API client
│   └── image_cache.go  # Logo image caching
├── player/
│   ├── mpv.go          # mpv wrapper
│   ├── metadata.go     # Metadata parsing
│   └── volume.go       # Volume control
├── config/
│   └── config.go       # Config management
├── data/
│   ├── favorites.go    # Favorites system
│   └── history.go      # Playback history
├── ui/
│   ├── model.go        # Bubbletea model
│   ├── views.go        # View rendering
│   ├── update.go       # Update logic
│   ├── styles.go       # Lipgloss styles
│   ├── ascii_art.go    # Logo conversion
│   ├── visualizer.go   # Audio visualizer
│   ├── buttons.go      # Clickable buttons
│   ├── help.go         # Help screen
│   └── themes.go       # Color themes
└── integrations/       # Shell integration examples
```

---

## 🔑 Key Dependencies

```go
require (
    github.com/charmbracelet/bubbletea v0.25.0
    github.com/charmbracelet/lipgloss v0.9.1
    github.com/charmbracelet/bubbles v0.17.1
    github.com/qeesung/image2ascii v1.0.1  // For logos
    // Standard library for most else
)
```

---

## ⌨️ Keyboard Shortcuts Design

| Key | Action | View |
|-----|--------|------|
| Space | Play/Pause toggle | Playing |
| Enter | Play selected station | Station List |
| n | Next station | Any |
| p | Previous station | Any |
| s | Stop playback | Any |
| h | Toggle quality (high/standard) | Any |
| l | Return to station list | Playing |
| q | Quit | Any |
| ↑/↓ | Navigate list / Volume | Both |
| f | Favorite station | Station List |
| r | Random station | Any |
| t | Toggle theme | Any |
| ? | Help screen | Any |
| 1-9 | Jump to favorite | Any |

---

## 🎨 UI Layout Mockup

```
┌─────────────────────────────────────────────────────────┐
│ vibecast 🎵                          [Quality: 256k]     │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ♪ NOW PLAYING: Groove Salad                           │
│                                                          │
│     ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒                                    │
│     ▒ SOMA  FM  ▒    "The Silent Pool" - Helios        │
│     ▒  GROOVE   ▒    Downtempo / Ambient               │
│     ▒  SALAD    ▒                                       │
│     ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    Playing for: 23:45              │
│                                                          │
│     ▌▌▌▌▌▌▌▌▌▌                                         │
│     ▁▂▃▄▅▆▇█▇▆  Volume: ████████░░ 80%                │
│                                                          │
│     [▶ Play]  [⏸ Pause]  [⏭ Next]  [⏹ Stop]           │
│                                                          │
├─────────────────────────────────────────────────────────┤
│ Space: Pause • N: Next • L: List • Q: Quit • ?: Help   │
└─────────────────────────────────────────────────────────┘
```

---

## 🗂️ Config Files Location

```
~/.config/vibecast/
├── config.json          # User preferences
├── favorites.json       # Favorite stations
├── history.json         # Playback history
└── schedule.json        # Time-based schedules (optional)

~/.cache/vibecast/
└── logos/              # Cached station images
```

---

## 🚀 Build & Test Commands

```bash
# Quick test
go run main.go list
go run main.go play groovesalad

# Build
go build -o vibecast

# Install locally
go install

# Cross-compile
GOOS=darwin GOARCH=arm64 go build -o vibecast-mac-arm64
GOOS=linux GOARCH=amd64 go build -o vibecast-linux-amd64
GOOS=windows GOARCH=amd64 go build -o vibecast-windows.exe
```

---

## 💡 Development Tips

1. **Test mpv integration early** - Make sure IPC works
2. **Use dummy data first** - Test UI before API integration
3. **Terminal size matters** - Test in small windows (80x24)
4. **Cache everything** - API calls, images, metadata
5. **Handle failures gracefully** - Network issues are common
6. **Performance** - Don't block on slow operations
7. **Colors** - Test with different terminal color schemes

---

## 🎯 Phases to Build In

1. **Phase 1:** API + basic playback (no UI)
2. **Phase 2:** Simple TUI + keyboard controls
3. **Phase 3:** Mouse support + ASCII buttons
4. **Phase 4:** Metadata + logos + visualizer
5. **Phase 5:** Polish + favorites + themes
6. **Phase 6:** Documentation + distribution

---

## 📝 Notes for Soma.fm Submission

When you're ready to share with Soma.fm:
- Include clear attribution
- Link to their donation page prominently
- Explain it's free & open source
- Show screenshots/demo
- Mention it's built by a long-time listener
- Ask if they'd like to feature it on their site

---

## 🔗 Useful Resources

- Soma.fm API: https://somafm.com/channels.json
- Bubbletea docs: https://github.com/charmbracelet/bubbletea
- mpv JSON IPC: https://mpv.io/manual/master/#json-ipc
- ASCII art: https://github.com/qeesung/image2ascii

---

Good luck! You've got this! 🎵✨
