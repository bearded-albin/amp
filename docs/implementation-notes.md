# Implementation Notes

Technical details about AMP's implementation, including the StadsAtlas injection system and testing interface.

## StadsAtlas Injection System

The testing mode includes an automated script that injects addresses into StadsAtlas for visual verification.

### Architecture

The injection system is **completely embedded** in `server/src/main.rs` within the `create_tabbed_interface_page()` function.

**Structure:**
```
server/src/main.rs
├── HTML Template (entire page structure)
├── CSS Styles (UI layout, colors, responsive design)
└── JavaScript (injection logic, 420+ lines)
    ├── Initialization and state management
    ├── 5-phase execution strategy
    ├── Error handling and fallbacks
    └── Debug interface (window.ampInjection)
```

### 5-Phase Execution Strategy

**Phase 1: Click Menu Button**
- Targets known button ID: `#cmkv2kcmk000g206jepb76w85`
- Opens layer control panel
- Handles retry logic if not found

**Phase 2: Find Miljöparkering Layer**
- Searches DOM for text containing "miljö" AND "parkering"
- Case-insensitive search
- Gracefully handles different layer names

**Phase 3: Toggle Layer Visibility**
- Clicks visibility toggle button near layer name
- Waits for DOM update (300-500ms)
- Verifies layer is enabled

**Phase 4: Find Search Input**
- Tries 8 different CSS selectors in order:
  1. `input[placeholder*="Sök"]` - Swedish placeholder
  2. `input[placeholder*="Search"]` - English placeholder
  3. `input[type="search"]` - Type attribute
  4. `input[class*="search"]` - Class contains search
  5. `input[aria-label*="Sök"]` - ARIA label Swedish
  6. `.ol-search input` - OpenLayers search class
  7. `[class*="search"] input` - Parent has search class
  8. `input[name*="search"]` - Name contains search

**Phase 5: Inject Address**
- Sets input value
- Fires keyboard events (input, change, keydown Enter)
- Triggers search in StadsAtlas

### Execution Timeline

Typical execution from page load to search complete:

```
Phase 1: 100-500ms   (menu click + retry logic)
Phase 2: 50-200ms    (DOM text search)
Phase 3: 300-500ms   (button click + DOM update)
Phase 4: 100-300ms   (selector attempts)
Phase 5: 10-50ms     (injection + events)
         ──────────
Total:   600-1500ms  (typical)
Max:     15 seconds  (timeout limit)
```

### Error Handling

Each phase includes:
- Try/catch blocks for safety
- Fallback strategies
- Timeout protection (15 seconds max)
- Detailed console logging with `[AMP]` prefix

**Example: Phase 4 Fallback Chain**
```javascript
const searchSelectors = [
    'input[placeholder*="Sök"]',
    'input[placeholder*="Search"]',
    'input[type="search"]',
    // ... 5 more
];

let searchInput = null;
for (const selector of searchSelectors) {
    searchInput = document.querySelector(selector);
    if (searchInput) {
        console.log(`[AMP] ✓ Found search input: ${selector}`);
        break;
    }
}
```

### Debug Interface

While a test window is open, use browser console:

```javascript
// Get current phase (1-5)
window.ampInjection.phase()

// View page state
window.ampInjection.debug()
// Returns: {
//   phase: 5,
//   menuFound: true,
//   layerFound: true,
//   searchInput: HTMLInputElement,
//   ...
// }

// Manually restart injection from phase 1
window.ampInjection.retry()
```

### Known Button IDs

These IDs may change if StadsAtlas updates their UI:

```javascript
const BUTTON_IDS = {
  MENU: '#cmkv2kcmk000g206jepb76w85',      // Opens layer panel
  ZOOM_IN: '#cmkv2kcmj0004206jnnm2ygxd',  // Zoom in control
  ZOOM_OUT: '#cmkv2kcmj0006206js5fq58t2', // Zoom out control
  HOME: '#cmkv2kcmk000d206je8eyd0w0',     // Reset view
  GEO: '#cmkv2kcmk001f206jzya5lsbh'       // Show location
};
```

**If button ID changes:**
1. Inspect element with browser DevTools
2. Find new menu button ID
3. Update `BUTTON_IDS.MENU` in main.rs
4. Rebuild: `cargo build --release`

### Maintenance Notes

**If Layer Name Changes:**
1. Check StadsAtlas for current layer name
2. Update Phase 2 search terms in main.rs
3. Change: `"miljö" && "parkering"`
4. Rebuild: `cargo build --release`

**If Search Input HTML Changes:**
1. Inspect with DevTools
2. Find new selector (class, id, aria-label, etc.)
3. Add to `searchSelectors` array in Phase 4
4. Rebuild: `cargo build --release`

---

## Testing Interface & Map Rendering

The testing interface uses a two-tab layout with StadsAtlas map integration.

### HTML Structure

**Tab 1: StadsAtlas Integration**
- Embedded iframe pointing to https://stadsatlas.malmo.se/
- 550px fixed height, 100% width
- Uses flexbox layout for responsive design
- Inline CSS with `!important` flags for robustness

**Tab 2: Correlation Results**
- HTML page generated dynamically
- Embedded as data URL for self-contained delivery
- Shows:
  - Address and postal code
  - Dataset source (Miljödata, Parkering, or both)
  - Distance to matched zone
  - Zone information and regulations

### CSS Architecture

**Inline Critical Styles:**
```css
<style>
    html, body {
        height: 100% !important;
        width: 100% !important;
    }
    .map-section {
        height: 550px !important;
        min-height: 550px !important;
        display: flex !important;
        flex-direction: column !important;
    }
    .map-container {
        height: 100% !important;
        flex: 1 !important;
        min-height: 0 !important;  /* Critical for flex sizing */
    }
</style>
```

**Why `!important`:**
- Overrides conflicting styles in all contexts
- Works in file:// URLs, iframes, HTTP servers
- Ensures dimensions always apply
- Critical for OpenLayers/Origo rendering

### Map Rendering

**Key Requirements:**
1. **Parent has explicit dimensions** - `550px` height
2. **Child uses flex layout** - `flex: 1` for fill
3. **Flex children have `min-height: 0`** - Allows flex sizing
4. **Inline styles** - Guarantees they apply

**Rendering Pipeline:**
```
1. HTML loaded (file:// or http://)
2. CSS parsed (inline + external)
3. Map div gets dimensions: 550px × 100%
4. iframe loads: https://stadsatlas.malmo.se/
5. OpenLayers initializes with container size
6. Base tiles request: https://gis.malmo.se/arcgis/...
7. Miljöparkering layer request: https://stadsatlas.malmo.se/wms/...
8. Map tiles render in iframe
9. Layers display with proper styling
```

### JavaScript Enhancements

**Dimension Logging:**
```javascript
const style = window.getComputedStyle(container);
const width = parseInt(style.width);
const height = parseInt(style.height);
console.log(`[AMP] Map dimensions: ${width}x${height}`);
```

**Emergency Fallback:**
```javascript
if (height === 0) {
    console.log('[AMP] Emergency fix: setting height to 550px');
    container.style.height = '550px';
}
```

### Data URL Generation

Results are encoded as data URLs for self-contained delivery:

```
data:text/html;charset=utf-8,[urlencoded HTML]
```

**Advantages:**
- No temporary files created
- Self-contained in single URL
- Browser renders directly
- Automatic cleanup on window close
- Works across all platforms

**Dependencies:**
- Uses `urlencoding` crate for HTML encoding
- Supports special characters and Unicode
- Works in all modern browsers

---

## Browser Automation

### Platform-Specific Window Opening

**Windows:**
```rust
std::process::Command::new("cmd")
    .args(&["start", url1, "&&", "start", url2])
    .spawn()
```

**macOS:**
```rust
std::process::Command::new("open")
    .args(&["-n", url])
    .spawn()
```

**Linux:**
```rust
std::process::Command::new("xdg-open")
    .arg(url)
    .spawn()
```

### Window Delay Strategy

500ms delays between opening windows:
- Prevents system resource exhaustion
- Avoids race conditions in browser launching
- Prevents network congestion
- Prevents overwhelming user's system

```rust
for (idx, address) in addresses.iter().enumerate() {
    open_browser_windows(&address)?;
    if idx < addresses.len() - 1 {
        std::thread::sleep(Duration::from_millis(500));
    }
}
```

---

## Address Injection in StadsAtlas

The injection system handles three major challenges:

### Challenge 1: Dynamic Content

**Problem:** Search input doesn't exist on page load

**Solution:** 
- Wait for layer to be enabled first
- Search input appears only after activation
- Phase 3 waits 300-500ms for DOM update
- Phase 4 then searches for input

### Challenge 2: Lazy Loading

**Problem:** UI elements load asynchronously

**Solution:**
- Each phase has retry logic
- Waits for events before proceeding
- Timeout protection (15 seconds max)
- Phase-based tracking for debugging

### Challenge 3: Cross-Frame Communication

**Problem:** Map is in nested iframe with CORS restrictions

**Solution:**
- Injection runs in main page, not iframe
- Finds/manipulates search input in main page
- Search events trigger in iframe automatically
- No direct cross-frame communication needed

---

## Coordinate Systems

### Precision

AMP uses `rust_decimal::Decimal` for coordinate storage:
- Preserves full ArcGIS precision
- Avoids floating-point rounding errors
- Enables exact distance calculations

**Conversion Pipeline:**
```
ArcGIS JSON (string)
    ↓
  Decimal (high precision storage)
    ↓
  f64 (for algorithm calculations)
    ↓
  Distance result (rounded to 2 decimals)
```

### Malmö Coordinate Range

Typical coordinates for Malmö:
- **Latitude:** 55.60 – 55.70
- **Longitude:** 12.95 – 13.10

Both fit comfortably in 2D spatial indexes (R-tree, KD-tree).

---

## Performance Optimizations

### Algorithm Selection

**KD-Tree (Default):**
- Best for 2D point-to-line correlation
- Malmö's coordinates are well-distributed
- Excellent performance on medium datasets
- Good cache locality

**R-Tree:**
- Better for 3D or irregular distributions
- More memory overhead
- Similar performance to KD-Tree for this use case

**Distance-Based:**
- No preprocessing overhead
- Good for tiny datasets (<1000 zones)
- O(n×m) complexity becomes slow quickly

### Parallel Processing

Benchmarking uses `rayon` for parallel iteration:

```rust
let results: Vec<_> = addresses
    .par_iter()      // Parallel iterator
    .map(|addr| algo.correlate(addr, &zones))
    .collect();      // Automatic thread pool management
```

---

## Known Limitations

1. **Browser automation** requires OS-level process spawning
   - Works on Windows, macOS, Linux
   - May fail in restricted environments

2. **StadsAtlas button IDs** may change with UI updates
   - Requires manual update and rebuild
   - See "Maintenance Notes" section

3. **HTTP context for testing** recommended
   - file:// URLs work but may have limitations
   - Local HTTP server provides better compatibility

4. **Data freshness** depends on ArcGIS API updates
   - Use `check-updates` command regularly
   - Checksums detect data changes

---

## Testing Recommendations

1. **Verify map rendering:**
   ```bash
   cargo run -- test -w 1
   # Check if StadsAtlas loads properly
   ```

2. **Test injection system:**
   ```
   Open browser console (F12)
   Filter to "[AMP]" messages
   Verify all 5 phases complete
   ```

3. **Validate results:**
   - Check Tab 2 shows data
   - Compare with Tab 1 StadsAtlas
   - Verify distance seems accurate

---

## Related Documentation

- [CLI Usage](cli-usage.md) — Command reference
- [Testing Guide](testing.md) — Testing procedures
- [Architecture](architecture.md) — System design
- [Algorithms](algorithms.md) — Algorithm details
