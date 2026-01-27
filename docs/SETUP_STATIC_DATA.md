# Setting Up Static Address Data for Android App

## Overview

This guide explains how to generate and embed the pre-computed address correlations into the Android app.

## Step-by-Step Setup

### 1. Generate Correlations from Server

On your development machine with the full `amp` repository:

```bash
# Build release binary
cargo build --release

# Run correlation with kdtree algorithm and 20 clusters
# Output includes all parking zone correlations
cargo run --release correlate -c 20 -a kdtree > address_correlations.json
```

This produces a JSON file with entries like:

```json
[
  {
    "adress": "Storgatan 10",
    "gata": "Storgatan",
    "gatunummer": "10",
    "postnummer": 22100,
    "dag": 15,
    "tid": "0800-1000",
    "info": "Parkering",
    "distance": 2.5,
    "coordinates": [55.7089, 13.1978]
  },
  ...
]
```

### 2. Parse the JSON Output

Create a Python or Rust script to convert the JSON into Rust code:

**Example Python script `parse_correlations.py`:**

```python
import json
import sys

if len(sys.argv) < 2:
    print("Usage: python parse_correlations.py <json_file>")
    sys.exit(1)

with open(sys.argv[1]) as f:
    data = json.load(f)

print("let entries = vec![")
for entry in data:
    print(f'''    StaticAddressEntry {{
        adress: "{entry['adress']}".to_string(),
        gata: "{entry['gata']}".to_string(),
        gatunummer: "{entry['gatunummer']}".to_string(),
        postnummer: {entry['postnummer']},
        dag: {entry['dag']},
        tid: "{entry['tid']}".to_string(),
        info: "{entry.get('info', 'Parkering')}".to_string(),
        distance: {entry.get('distance', 0.0)},
    }},''')
print("];")
```

Run it:

```bash
python parse_correlations.py address_correlations.json > entries.rs
```

Output sample:

```rust
let entries = vec![
    StaticAddressEntry {
        adress: "Storgatan 10".to_string(),
        gata: "Storgatan".to_string(),
        gatunummer: "10".to_string(),
        postnummer: 22100,
        dag: 15,
        tid: "0800-1000".to_string(),
        info: "Parkering".to_string(),
        distance: 2.5,
    },
    StaticAddressEntry {
        adress: "Kungsgatan 5".to_string(),
        gata: "Kungsgatan".to_string(),
        gatunummer: "5".to_string(),
        postnummer: 22200,
        dag: 2,
        tid: "0900-1100".to_string(),
        info: "Parkering".to_string(),
        distance: 3.2,
    },
    // ... more entries
];
```

### 3. Update `android/src/static_data.rs`

Replace the TODO section:

```rust
pub fn get_static_addresses() -> HashMap<String, StaticAddressEntry> {
    let mut map = HashMap::new();
    
    // PASTE THE GENERATED ENTRIES HERE
    let entries = vec![
        StaticAddressEntry {
            adress: "Storgatan 10".to_string(),
            gata: "Storgatan".to_string(),
            gatunummer: "10".to_string(),
            postnummer: 22100,
            dag: 15,
            tid: "0800-1000".to_string(),
            info: "Parkering".to_string(),
            distance: 2.5,
        },
        // Add all other entries...
    ];
    
    for entry in entries {
        let key = format!("{} {}-{}", entry.gata, entry.gatunummer, entry.postnummer);
        map.insert(key, entry);
    }
    
    map
}
```

### 4. Verify the Data

Run the tests to ensure data integrity:

```bash
cd android
cargo test static_data
```

OrManually verify:

```bash
# Check compilation
cargo check

# Run app and test with known addresses
cargo run --release
```

## Validation

### Data Quality Checks

1. **No duplicate keys:**
   ```bash
   # Should return 0 duplicates
   cat address_correlations.json | jq 'map("\(.gata) \(.gatunummer)-\(.postnummer)") | group_by(.) | map(select(length > 1)) | length'
   ```

2. **Valid time formats:**
   All `tid` values should match `HHMM-HHMM` format (e.g., "0800-1000")

3. **Valid day ranges:**
   All `dag` values should be 1-31

4. **All required fields:**
   Check for missing `adress`, `gata`, `gatunummer`, `postnummer`, `dag`, `tid`

### Testing Known Addresses

After embedding, test with these actions:

1. **Test exact match:**
   ```
   Gata: "Storgatan"
   Gatunummer: "10"
   Postnummer: "22100"
   Result: ✓ Should show "Adress hittad!"
   ```

2. **Test invalid address:**
   ```
   Gata: "FakeStreet"
   Gatunummer: "999"
   Postnummer: "00000"
   Result: ✓ Should show "Adressen finns inte i systemet"
   ```

3. **Test countdown display:**
   ```
   Add valid address
   Result: ✓ Should appear in correct category with countdown timer
   ```

## Troubleshooting

### Issue: "Expected {} but found some addresses"

**Cause:** JSON parsing error in the Python script

**Solution:** Verify JSON structure:
```bash
jq '.[0]' address_correlations.json
```

Should show:
```json
{
  "adress": "...",
  "gata": "...",
  "gatunummer": "...",
  "postnummer": 0,
  "dag": 0,
  "tid": "HHMM-HHMM",
  "info": "...",
  "distance": 0.0
}
```

### Issue: Compilation errors in `static_data.rs`

**Common causes:**
- Invalid character in address string (needs escaping)
- Missing `to_string()` calls
- Type mismatch (postnummer should be u16)

**Solution:** Run `cargo check` to get specific line numbers, then fix

### Issue: Addresses not matching

**Cause:** Whitespace or case differences

**Example:**
```
Input: "Storgatan  10" (extra space)
Key: "Storgatan  10-22100"
Static: "Storgatan 10-22100"
Result: No match ✗
```

**Fix:** The `make_lookup_key()` function trims whitespace, so input should normalize correctly. Check the actual data in `static_data.rs`.

## Performance Optimization

If you have many addresses (>10,000), consider:

1. **Splitting by postal code:**
   Create separate HashMaps per postal code prefix

2. **Lazy initialization:**
   Use `lazy_static` to avoid computing HashMap on every call

3. **Binary format:**
   Instead of Rust code, embed as binary with deserialization

For now (small datasets), the simple HashMap in Rust code is fine.

## Updating Data

Whenever parking zone data changes:

1. Re-run: `cargo run --release correlate -c 20 -a kdtree > address_correlations.json`
2. Re-run Python script: `python parse_correlations.py address_correlations.json > entries.rs`
3. Update `android/src/static_data.rs` with new entries
4. Rebuild APK

## File Locations

Reference for the files involved:

```
amp/
├── android/src/
│   └── static_data.rs          ←← Update this with generated entries
├── Cargo.toml                  ←← Check for chrono dependency
├── docs/
│   └── SETUP_STATIC_DATA.md    ←← This file
├── parkeringsavgifter.json     ←← Input data
└── miljoparkeringar.json       ←← Input data
```

## Next Steps

1. ✅ Generate correlations
2. ✅ Parse JSON to Rust
3. ✅ Update static_data.rs
4. ✅ Test compilation
5. ✅ Test address matching
6. ✅ Build APK

See [ANDROID_INTEGRATION.md](./ANDROID_INTEGRATION.md) for the full integration guide.
