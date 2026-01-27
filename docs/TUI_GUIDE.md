# AMP Server - Complete TUI Guide

## Quick Start

### Build

```bash
cargo build --release -p amp_server
```

### Run

```bash
./target/release/amp-server
```

**No arguments. No configuration. Just run it.**

---

## User Interface Overview

### Main Window

The TUI has 6 tabs accessible by pressing `[1]` through `[6]` or using arrow keys `[←→]`.

```
┌──────── AMP Server ────────┐
│ [1] Dashboard  [2] Correlate  [3] Results  [4] Bench...  │
├───────────────────────────────┐
│                                                        │
│  Active Tab Content                                    │
│  (Rendered based on current tab)                      │
│                                                        │
├───────────────────────────────┐
│ Logs (color-coded, auto-scrolling)                   │
│ [?] Help  [t] Theme  [q] Quit                        │
└───────────────────────────────┘
```

---

## Tabs Explained

### [1] Dashboard

**Purpose:** Overview of the application state

**Shows:**
- Available algorithms (6 total)
- Current settings (cutoff distance, theme)
- Data status
- Quick statistics

**Actions:**
- View system information
- Check configuration
- Monitor status

### [2] Correlate

**Purpose:** Run address-to-zone correlation

**Shows:**
- Algorithm selection checkboxes (6 algorithms)
- Distance cutoff input
- Status of correlations
- Progress bars during operation

**Actions:**
```
1. Select algorithms:
   - [Space] on algorithm to toggle
   - [a] to select all
   - [c] to clear all

2. Adjust cutoff:
   - [+] to increase distance (meters)
   - [-] to decrease distance
   - Current: Shows in UI

3. Run correlation:
   - [Enter] to start
   - Progress bars appear
   - Live log updates
   - Results populate when done
```

**Algorithms Available:**
- Distance-Based
- Raycasting
- Overlapping Chunks
- R-Tree
- KD-Tree
- Grid

### [3] Results

**Purpose:** Display correlation results

**Shows:**
- Scrollable table of all matches
- Address information
- Zone information
- Distance details
- Dataset source

**Actions:**
```
[↑↓] - Scroll up/down
Page Up/Down - Scroll faster
Home/End - Jump to start/end
```

**Table Columns:**
| Column | Content |
|--------|----------|
| Address | Street address |
| Zone ID | Parking zone ID |
| Distance | Distance in meters |
| Algorithm | Which algorithm found it |

### [4] Benchmark

**Purpose:** Compare algorithm performance

**Shows:**
- Selection of algorithms to benchmark
- Sample size configuration
- Progress bars
- Comparative results table
- Performance statistics

**Actions:**
```
Select algorithms with [Space]
Adjust sample size with [+/-]
Press [Enter] to run benchmark
```

**Metrics Shown:**
- Execution time
- Memory used
- Matches found
- Matches per second
- Comparison bars

### [5] Updates

**Purpose:** Monitor data updates from Mälmö open data

**Shows:**
- Last check timestamp
- Data file checksums
- Available updates
- Update history

**Actions:**
```
Press [Enter] to check now
View update details
Monitor data freshness
```

### [6] Help

**Purpose:** Complete keyboard reference

**Shows:**
- All keyboard shortcuts
- Feature descriptions
- Tips and tricks
- Troubleshooting hints

**Always Available:**
- Press `[?]` anytime to see this
- It overlays on top of current view

---

## Keyboard Shortcuts

### Navigation

```
[1]                    Jump to Dashboard
[2]                    Jump to Correlate
[3]                    Jump to Results
[4]                    Jump to Benchmark
[5]                    Jump to Updates
[6]                    Jump to Help
[←] or [→]            Move between tabs
[↑] or [↓]            Scroll content up/down
PageUp / PageDown       Scroll faster
Home / End              Jump to start/end
```

### Algorithm Selection

```
[Space]                Toggle selected algorithm
[a]                    Select all algorithms
[c]                    Clear all selections
```

### Settings

```
[+]                    Increase cutoff distance (+5m)
[-]                    Decrease cutoff distance (-5m)
[t]                    Toggle theme:
                       - Light
                       - Dark
                       - Auto (follow system)
```

### Operation

```
[Enter]                Run current operation:
                       - Correlate (in Correlate tab)
                       - Benchmark (in Benchmark tab)
                       - Check updates (in Updates tab)
```

### Help & Information

```
[?]                    Show this help overlay
[Tab]                  Show next help page
[Shift+Tab]            Show previous help page
```

### Exit

```
[q]                    Quit application
[Ctrl+C]               Quit application (force)
```

---

## Workflow Examples

### Example 1: Quick Correlation

```
1. Launch: ./target/release/amp-server
2. Already on Correlate tab [2]
3. Press [a] to select all algorithms
4. Press [Enter] to run
5. Watch progress bars in live log
6. Press [3] to see results when done
7. Press [q] to quit
```

### Example 2: Compare Algorithms

```
1. Launch: ./target/release/amp-server
2. Press [4] to go to Benchmark
3. Press [a] to select all algorithms
4. Adjust sample size with [+-]
5. Press [Enter] to benchmark
6. Watch progress for each algorithm
7. View comparison table when done
8. Press [q] to quit
```

### Example 3: Test Single Algorithm

```
1. Launch: ./target/release/amp-server
2. Press [2] for Correlate tab
3. Press [c] to clear all
4. Press [Space] on KD-Tree only
5. Adjust cutoff with [+-] to 75m
6. Press [Enter] to run
7. Press [3] to see results
8. Press [q] to quit
```

### Example 4: Check for Updates

```
1. Launch: ./target/release/amp-server
2. Press [5] to go to Updates
3. View last check time
4. Press [Enter] to check now
5. Watch progress
6. View update status when done
7. Press [q] to quit
```

---

## Interpreting Results

### Correlation Results Table

Columns shown:

```
Address
  - The street address from the dataset
  - Example: "Stortorget 1, Malmö"

Zone ID
  - Unique identifier for parking zone
  - Example: "zone_12345"

Distance
  - Meters from address to zone boundary
  - 0m = inside zone
  - Positive = outside (but within cutoff)

Algorithm
  - Which algorithm found this match
  - Example: "KD-Tree"
```

### Benchmark Results

```
Algorithm          Time        Memory      Matches     Rate
┌─────────────────┬──────────┬──────────┬─────────┬──────────┐
│ Distance-Based  │ 45.2s    │ 45 MB    │ 1,250   │ 27.6/s   │
│ Raycasting      │ 52.1s    │ 52 MB    │ 1,248   │ 23.9/s   │
│ R-Tree          │ 38.5s    │ 48 MB    │ 1,251   │ 32.5/s   │
│ KD-Tree         │ 31.2s    │ 42 MB    │ 1,250   │ 40.1/s   │
│ Grid            │ 41.3s    │ 50 MB    │ 1,249   │ 30.3/s   │
│ Overlapping     │ 39.8s    │ 46 MB    │ 1,251   │ 31.4/s   │
└─────────────────┴──────────┴──────────┴─────────┴──────────┘
```

---

## Live Log Panel

### Color Coding

```
[INFO]    - Blue       - General information
[PROG]    - Green      - Progress updates
[WARN]    - Yellow     - Warnings
[ERROR]   - Red        - Errors
[DEBUG]   - Cyan       - Debug messages
```

### What to Watch For

```
✓ [PROG] Correlation started
  └─ Running KD-Tree...
  └─ Running R-Tree...
  └─ Processing results...
✓ [PROG] Correlation complete: 1,250 matches in 31.2s

✗ [ERROR] Failed to load data
  └─ Check internet connection
  └─ Retry with [Enter]
```

---

## Theme Selection

### Light Theme
- White/cream background
- Dark text
- Best for: Bright environments, printing

### Dark Theme
- Dark background
- Light text
- Best for: Dark environments, eye strain reduction

### Auto Theme
- Follows system setting
- Changes automatically with OS theme
- Best for: Most users

### How to Change

```
Press [t] while in TUI
Theme cycles: Light → Dark → Auto → Light
Setting is remembered between sessions
```

---

## Terminal Requirements

### Minimum Size
- Width: 60 characters
- Height: 15 lines
- Smaller = Some features hidden

### Supported Terminals
- Linux: Most terminal emulators (gnome-terminal, konsole, etc.)
- macOS: Terminal.app, iTerm2, Alacritty
- Windows: Windows Terminal, ConEmu, cmd.exe (with unicode)

### SSH Usage

```bash
# Must use -t flag for pseudo-terminal
ssh -t user@host
./amp-server
```

---

## Troubleshooting

### TUI Won't Start

**Problem:** Terminal not entering raw mode

**Solution:**
1. Try different terminal emulator
2. Check TERM variable: `echo $TERM`
3. For SSH: Use `ssh -t user@host`
4. Ensure TERM supports raw mode

### Keyboard Shortcuts Not Working

**Problem:** Keys not responding

**Solution:**
1. Check TERM variable
2. Try Alt modifier instead of regular key
3. Ensure no shell bindings conflict
4. Try different terminal

### Display Issues

**Problem:** Garbled characters, wrong colors

**Solution:**
1. Check terminal encoding (should be UTF-8)
2. Resize terminal window
3. Clear screen: `Ctrl+L`
4. Restart application

### Results Not Showing

**Problem:** Correlation ran but Results tab is empty

**Solution:**
1. Check log panel for errors
2. Ensure algorithm was selected
3. Try with fewer algorithms
4. Check memory usage

### Memory Issues

**Problem:** Application using too much memory

**Solution:**
1. Reduce sample size in Benchmark
2. Run fewer algorithms at once
3. Close other applications
4. Restart AMP

---

## Performance Tips

### Faster Results
1. Use 1-2 algorithms instead of all 6
2. Increase cutoff distance
3. Reduce sample size
4. Close other applications

### Lower Memory Usage
1. Don't run large benchmarks
2. Use Grid or Distance-Based (smaller footprint)
3. Reduce sample size
4. Monitor with system tools

### Better Responsiveness
1. Use themes that match your screen
2. Larger terminal window
3. Close unnecessary programs
4. Use faster disk (SSD)

---

## Getting Help

### Inside the Application

```
Press [?] anytime to see help overlay
Help covers:
- All keyboard shortcuts
- Feature descriptions
- Tips and tricks
- Common issues
```

### Documentation Files

- **README.md** - Project overview
- **docs/IMPLEMENTATION_COMPLETE.md** - Technical details
- **TUI_ONLY_SUMMARY.md** - Quick reference
- **This file** - Complete TUI guide

### Error Messages

Check the live log panel for detailed error information:

```
[ERROR] Could not load asset file: stadsatlas_interface.html
        → Check server/src/assets/ directory exists
        → Check file permissions
        → Run from workspace root
```

---

## Advanced Usage

### Custom Sample Sizes

In Benchmark tab:
1. Select algorithms
2. Press `[+]` or `[-]` to adjust sample size
3. Press `[Enter]` to run

### Adjusting Cutoff Distance

In Correlate tab:
1. Use `[+]` to increase cutoff
2. Use `[-]` to decrease cutoff
3. Default: 50 meters
4. Useful range: 0-500 meters

### Multiple Correlations

1. Run first correlation
2. Switch to Results tab [3]
3. Go back to Correlate [2]
4. Select different algorithms
5. Adjust cutoff if needed
6. Press [Enter] again
7. Previous results are preserved

---

## Summary

**TUI provides:**
- ✅ Interactive correlation
- ✅ Algorithm comparison
- ✅ Performance benchmarking
- ✅ Data update checking
- ✅ Real-time feedback
- ✅ Theme switching
- ✅ Keyboard navigation
- ✅ Live logging

**Just launch and explore!**

```bash
./target/release/amp-server
```

Press `[?]` to learn all shortcuts. Enjoy!
