# CLI Usage

The AMP server provides several commands for correlation, testing, benchmarking, and data management.

## Overview

```bash
amp-server <COMMAND> [OPTIONS]
```

## Commands

- `correlate` — Run full address-to-zone correlation
- `test` — Visual testing mode with browser automation
- `benchmark` — Performance testing of algorithms
- `check-updates` — Check if data needs updating

---

## `correlate` Command

Run correlation on all addresses using specified algorithm and distance threshold.

### Usage

```bash
cargo run --release -p amp_server -- correlate [OPTIONS]
```

### Options

#### `--algorithm` / `-a`
Choose correlation algorithm (default: `kdtree`)

```bash
cargo run -- correlate --algorithm rtree
```

Available algorithms:
- `distance-based` — O(n×m) brute-force, small datasets
- `raycasting` — Geometric ray intersection method
- `overlapping-chunks` — Grid-based spatial chunking
- `rtree` — R-tree spatial index, general purpose
- `kdtree` — KD-tree spatial index, recommended
- `grid` — Fixed grid with nearest neighbor

#### `--cutoff` / `-c`
Distance threshold in meters (default: `50`)

```bash
# Only include matches within 75 meters
cargo run -- correlate --cutoff 75
```

Affects which addresses are included in results. Only matches within cutoff distance counted.

### Output

Displays:
- Total addresses processed
- Number of matches found
- Matches exceeding cutoff threshold (if any)
- 10 addresses with largest distances
- Execution time and memory usage

**Example:**
```
🚀 Loading data from ArcGIS API...
   Addresses: 147,832
   Miljö zones: 8,247
   Parkering zones: 3,156

🔐 Running KDTree correlation...
   ✓ Finished in 3.42s

📊 Results:
   Total matches: 98,436 (66.6% of addresses)
   All within 50m threshold
   
   Addresses with largest distances:
   - Sophiahemmet 1: 49.8m (Miljö)
   - Universitetsplåtsen 1: 48.3m (Miljö)
   ...
```

### Examples

```bash
# Default: KD-Tree, 50m threshold
cargo run --release -- correlate

# R-Tree with 100m threshold
cargo run -- correlate --algorithm rtree --cutoff 100

# Compare algorithms
cargo run -- correlate --algorithm kdtree
cargo run -- correlate --algorithm rtree

# Conservative (fewer matches)
cargo run -- correlate --cutoff 25

# Permissive (more matches)
cargo run -- correlate --cutoff 100
```

---

## `test` Command

Visual testing mode opens browser windows for manual verification against StadsAtlas.

### Usage

```bash
cargo run --release -p amp_server -- test [OPTIONS]
```

### Options

#### `--algorithm` / `-a`
Choose algorithm (default: `kdtree`)

```bash
cargo run -- test --algorithm rtree
```

#### `--cutoff` / `-c`
Distance threshold in meters (default: `50`)

```bash
# Only include addresses within 100 meters of zones
cargo run -- test --cutoff 100
```

#### `--windows` / `-w`
Number of browser windows to open (default: `10`)

```bash
# Open 20 windows
cargo run -- test --windows 20

# Single window for quick test
cargo run -- test --windows 1
```

### What Happens

For each selected address:
1. Opens 2-tab browser window
2. **Tab 1:** Official Malmö StadsAtlas map
   - Shows address location with blue pin
   - You manually enable "Miljöparkering" checkbox
   - You enter address in search bar to see regulations
3. **Tab 2:** Correlation result details
   - Shows matched zone and distance
   - Compare with Tab 1 StadsAtlas display

### Examples

```bash
# Default test (10 windows, KD-Tree, 50m)
cargo run --release -- test

# Quick test (5 windows)
cargo run -- test --windows 5

# Compare algorithms
cargo run -- test --algorithm kdtree --windows 10
cargo run -- test --algorithm rtree --windows 10

# Validate distance thresholds
cargo run -- test --cutoff 25 --windows 5
cargo run -- test --cutoff 50 --windows 5
cargo run -- test --cutoff 100 --windows 5

# Large-scale test
cargo run -- test --algorithm kdtree --cutoff 50 --windows 50
```

### Interpreting Results

For each opened window:

**Good Match** ✅
- StadsAtlas shows same zone as Tab 2
- Distance seems reasonable (e.g., 15.3m)
- Zone regulations visible and correct

**Poor Match** ⚠️
- StadsAtlas shows different zone
- Distance at cutoff boundary (e.g., 49.8m)
- Zone regulations don't match visible features

**No Match** ❌
- Tab 2 shows "No matches found"
- Address is outside all zones or beyond cutoff

See [testing.md](testing.md#interpreting-results) for detailed guidance.

---

## `benchmark` Command

Interactively benchmark all algorithms on sample data.

### Usage

```bash
cargo run --release -p amp_server -- benchmark [OPTIONS]
```

### Options

#### `--sample-size` / `-s`
Number of addresses to test (default: `100`)

```bash
# Benchmark on 1000 addresses
cargo run -- benchmark --sample-size 1000
```

If sample size exceeds available addresses, uses all available.

#### `--cutoff` / `-c`
Distance threshold in meters (default: `50`)

```bash
# Benchmark with 100m threshold
cargo run -- benchmark --cutoff 100
```

### Workflow

1. Prompts you to select which algorithms to benchmark
2. Runs each algorithm on same sample data
3. Displays timing and match count for each
4. Allows selecting different sample size

### Example Output

```
Benchmark Results (1000 addresses, 50m threshold):

Algorithm            Total Time    Avg/Address    Matches
───────────────────────────────────
R-Tree               1.15s         1.15ms         667
KD-Tree              0.92s         0.92ms         667
Raycasting           2.34s         2.34ms         667
Distance-Based       3.87s         3.87ms         667
Overlapping Chunks   1.42s         1.42ms         667
Grid                 4.12s         4.12ms         667

Recommendation: KD-Tree offers best balance of speed and simplicity
```

### Examples

```bash
# Default: interactive selection, 100 addresses
cargo run --release -- benchmark

# Larger sample
cargo run -- benchmark --sample-size 500

# With custom cutoff
cargo run -- benchmark --sample-size 1000 --cutoff 100
```

---

## `check-updates` Command

Verify if data has been updated by comparing checksums.

### Usage

```bash
cargo run --release -p amp_server -- check-updates
```

### What It Does

1. Fetches current checksums from ArcGIS API
2. Compares with stored checksums (if available)
3. Reports any changes

### Example Output

```
🔄 Checking for data updates...

✅ Miljöparkering: No changes
✅ Parkeringsavgifter: No changes  
✅ Adresser: No changes

All datasets current as of 2026-01-27
Last update: 2025-12-15
```

Or if updates exist:
```
⚠️  Miljöparkering: CHANGED (last updated 2025-12-25)
⚠️  Parkeringsavgifter: CHANGED (last updated 2025-12-18)
✅ Adresser: No changes

Action: Re-run correlate command to fetch latest data
```

---

## Injection System

The testing mode includes an automated StadsAtlas injection system that:

1. **Navigates to StadsAtlas** at https://stadsatlas.malmo.se/
2. **Enables the Miljöparkering layer** (environmental parking zones)
3. **Injects the test address** into the search field
4. **Displays results** for manual comparison

### How It Works

**5-Phase Execution:**

1. **Click menu button** → Opens layer control panel
2. **Find Miljöparkering layer** → Searches DOM for layer name
3. **Toggle layer visibility** → Enables parking zone display
4. **Find search input** → Tries 8 different CSS selectors
5. **Inject address** → Populates search field and triggers search

### Debug Mode

While testing window is open, browser console provides:

```javascript
// Check current phase
window.ampInjection.phase()  // Returns 1-5

// View page state
window.ampInjection.debug()  // Shows elements found

// Manually retry
window.ampInjection.retry()  // Restart injection
```

### Console Logging

All injection logs prefixed with `[AMP]` for easy filtering:

```
[AMP] Injection script initialized. Debug: window.ampInjection
[AMP] Page loaded, starting injection sequence...
[AMP] Phase 1: Clicking menu button
[AMP] ✓ Found menu button, clicking...
[AMP] Phase 2: Looking for Miljöparkering layer...
[AMP] ✓ Found layer, enabling...
[AMP] Phase 3: Toggling layer visibility
[AMP] Phase 4: Finding search input
[AMP] ✓ Found search field
[AMP] Phase 5: Injecting address
[AMP] ✓ Address injected and search triggered
```

### Fallback Strategies

If layer not found, system tries multiple fallbacks:
- 8 different CSS selectors for search input
- Automatic retry logic with exponential backoff
- 15-second timeout with phase tracking
- Graceful degradation if elements can't be found

See [implementation-notes.md](implementation-notes.md) for technical details.

---

## Quick Reference

### Most Common Commands

```bash
# Test mode (default settings)
cargo run --release -- test

# Full correlation run
cargo run --release -- correlate

# Benchmark all algorithms
cargo run --release -- benchmark

# Check for data updates
cargo run -- check-updates
```

### Command Cheat Sheet

```bash
# Test with different algorithms
cargo run -- test -a kdtree  # KD-Tree
cargo run -- test -a rtree   # R-Tree
cargo run -- test -a raycasting  # Raycasting

# Test with different cutoffs
cargo run -- test -c 25   # Conservative
cargo run -- test -c 50   # Standard
cargo run -- test -c 100  # Permissive

# Combine options
cargo run -- test -a rtree -c 100 -w 20
```

---

## Building for Release

```bash
# Optimized binary
cargo build --release -p amp_server

# Binary location
./target/release/amp-server

# Run directly
./target/release/amp-server correlate --algorithm rtree
```

## Troubleshooting

**"No matching addresses found"**
- Solution: Increase cutoff (`--cutoff 100`)
- Or try different algorithm

**Windows not opening**
- Windows: Check default browser configuration
- macOS: Grant terminal permission to control applications
- Linux: Ensure `xdg-open` is installed

**Data appears outdated**
- Run: `cargo run -- check-updates`
- If changed, re-run correlate to fetch latest

**High memory usage**
- R-Tree algorithm uses more memory than others
- Try KD-Tree instead: `--algorithm kdtree`

---

## Related Documentation

- [Testing Guide](testing.md) — Detailed testing procedures
- [Algorithms](algorithms.md) — How each algorithm works
- [API Integration](api-integration.md) — Data fetching
- [Implementation Notes](implementation-notes.md) — Technical details
- [Server README](../server/README.md) — Server deployment guide
