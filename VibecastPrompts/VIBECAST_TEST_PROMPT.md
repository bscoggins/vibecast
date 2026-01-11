# Test Prompt for Claude Opus 4.5

Use this prompt first to validate that Claude Opus can handle the project well before committing to the full build.

---

## Quick Test Prompt

```
I want to test building a Go application that fetches and displays data from the Soma.fm API.

Please create a simple Go program that:

1. Fetches the station list from https://somafm.com/channels.json
2. Parses the JSON response
3. Displays a formatted list of stations with:
   - Station name
   - Description
   - Available stream qualities (look for mp3 format playlists)
4. Includes proper error handling
5. Uses only the Go standard library (no external dependencies for this test)

Project structure:
- main.go
- go.mod

After creating the code, explain:
- How the Soma.fm API works
- What data is available
- How we could extend this for the full vibecast project

This is a test to validate the approach before building the full TUI application.
```

---

## What to Look For in Claude's Response

✅ **Good Signs:**
- Creates working, runnable code
- Proper error handling
- Clear code comments
- Explains the API structure
- Suggests reasonable next steps
- Code is idiomatic Go
- Handles edge cases (network failures, parsing errors)

❌ **Red Flags:**
- Code doesn't compile
- Missing error handling
- Over-complicated for the task
- Doesn't explain the approach
- Ignores requirements

---

## Follow-up Questions to Test Understanding

After Claude provides the initial code, ask:

1. **"How would we modify this to cache the API response for 1 hour?"**
   - Tests understanding of file I/O and time-based logic

2. **"Can you show how to extract both 128kbps and 256kbps stream URLs?"**
   - Tests JSON parsing depth

3. **"How would we integrate this with mpv to actually play a stream?"**
   - Tests system command execution understanding

4. **"What would the bubbletea model look like for displaying this data?"**
   - Tests TUI framework knowledge

---

## Expected Output Example

When you run the test program, you should see something like:

```
Fetching stations from Soma.fm...
Successfully retrieved 30 stations

Available Stations:
==================

1. Groove Salad
   A nicely chilled plate of ambient/downtempo beats and grooves
   Streams: 128kbps, 256kbps

2. DEF CON Radio
   Music for Hacking. The DEF CON Year-Round Channel
   Streams: 128kbps, 256kbps

3. Drone Zone
   Served best chilled, safe with most medications
   Streams: 64kbps, 128kbps, 256kbps

[... more stations ...]
```

---

## If the Test Goes Well

Great! You can proceed with confidence to Phase 1 of the full prompts.

The test validates that Claude Opus can:
- Write working Go code
- Understand REST APIs
- Parse JSON effectively
- Think about project architecture
- Explain technical concepts

---

## If the Test Has Issues

**Don't panic!** Try these:

1. **Be more specific:**
   - "Please use the encoding/json package for parsing"
   - "Show me the exact struct definitions needed"

2. **Ask for iterations:**
   - "The code doesn't handle X case, can you fix it?"
   - "Can you add comments explaining each step?"

3. **Request explanations:**
   - "Walk me through how this code works line by line"
   - "What could go wrong with this approach?"

4. **Simplify:**
   - Start even smaller (just fetch and print raw JSON)
   - Build up incrementally

---

## Moving to Full Development

Once the test works, you'll know:
- Claude Opus can handle Go development
- The Soma.fm API structure
- How to structure future prompts
- Your feedback style that works with Claude

Then proceed to Phase 1 in the main prompts document!

---

## Pro Tips

- **Copy/paste the output** and actually run it
- **Share errors** with Claude if something breaks
- **Ask "why"** if you don't understand something
- **Iterate** - good code comes from refinement
- **Save working versions** before major changes

Happy building! 🚀
