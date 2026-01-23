# CLI Usage

The `amp-server` CLI provides correlation, benchmarking, and data verification commands.

## Installation

```bash
cargo install --path server
# Or
cargo build --release -p amp_server
./target/release/amp-server --help
```

## Commands

### correlate

Run address-to-zone correlation with specified algorithm.

```bash
amp-server correlate [OPTIONS]
```

**Options:**
- `-a, --algorithm <NAME>` — Algorithm to use (default: rtree)
  - `distance-based` — Brute-force O(n×m)
  - `raycasting` — 36-ray search
  - `overlapping-chunks` — Spatial grid with overlap
  - `rtree` — R-tree spatial index
  - `kdtree` — KD-tree spatial index
  - `grid` — Fixed-size grid

**Example:**

```bash
$ amp-server correlate --algorithm rtree

📋 Dataset Information:
   Addresses: 100,342
   Miljödata zones: 1,847
   Parkering zones: 3,256
   Max distance threshold: 50 meters

🚀 Running correlation with RTree algorithm
[████████████████████] 100342/100342 100% ✓ Completed in 2.31s

📊 Results:
   Addresses processed: 100,342
   Total matches: 87,234 (86.9%)
   ├─ Both datasets: 12,456 (12.4%)
   ├─ Miljödata only: 34,567 (34.4%)
   ├─ Parkering only: 40,211 (40.1%)
   └─ No match: 13,108 (13.1%)
   Average time per address: 23.02µs
```

**Output:**
- Match statistics by dataset
- Random sample of 10 matches
- Top 10 largest distances (threshold verification)

### benchmark

Compare performance of all six algorithms.

```bash
amp-server benchmark [OPTIONS]
```

**Options:**
- `-s, --sample-size <N>` — Number of addresses to test (default: 100)

**Example:**

```bash
$ amp-server benchmark --sample-size 500

🏁 Benchmarking all 6 algorithms with 500 samples

[Distance-Based    ] [██████████] 500/500 ✓ 2.45s
[Raycasting        ] [██████████] 500/500 ✓ 5.12s
[Overlapping Chunks] [██████████] 500/500 ✓ 1.23s
[R-Tree            ] [██████████] 500/500 ✓ 1.15s
[KD-Tree           ] [██████████] 500/500 ✓ 1.28s
[Grid              ] [██████████] 500/500 ✓ 1.31s

📊 Benchmark Results:

Algorithm            Total Time    Avg/Address    Matches
──────────────────────────────────────────────────────
Distance-Based       2.45s         4.90ms         423
Raycasting          5.12s         10.24ms        431
Overlapping Chunks  1.23s         2.46ms         423
R-Tree              1.15s         2.30ms         423
KD-Tree             1.28s         2.56ms         423
Grid                1.31s         2.62ms         423

✓ Fastest: R-Tree (1.15s)
```

### check-updates

Verify if Malmö's open data has changed.

```bash
amp-server check-updates [OPTIONS]
```

**Options:**
- `-c, --checksum-file <PATH>` — Checksum file (default: checksums.json)

**Example:**

```bash
$ amp-server check-updates

🔍 Checking for data updates...

✓ Data fetched

✓ Data has changed!
  Old checksums from: 2026-01-22T10:15:30Z
  New checksums from: 2026-01-23T10:15:30Z
✓ Checksums saved to checksums.json
```

**Checksum File Format:**

```json
{
  "miljoparkering": "a3f5e8...",
  "parkeringsavgifter": "b2d9c1...",
  "adresser": "f7e4a2...",
  "last_checked": "2026-01-23T10:15:30Z"
}
```

**Use Cases:**
- Daily cron job to monitor data changes
- CI/CD pipeline validation
- Manual verification before deployment

## Common Workflows

### Quick Test

```bash
# Fast correlation check
amp-server correlate --algorithm rtree
```

### Algorithm Comparison

```bash
# Small sample for quick comparison
amp-server benchmark --sample-size 100

# Large sample for accurate results
amp-server benchmark --sample-size 5000
```

### Production Deployment

```bash
# 1. Check for data updates
amp-server check-updates

# 2. Run correlation with best algorithm
amp-server correlate --algorithm rtree

# 3. Verify results (examine output statistics)
```

### Daily Monitoring

```bash
#!/bin/bash
# daily-check.sh

if amp-server check-updates; then
    echo "Data updated, re-running correlation"
    amp-server correlate --algorithm rtree
fi
```

## Environment Variables

None required. All data fetched from public Malmö Open Data Portal.

## Output Files

- `checksums.json` — Data verification checksums
- stdout — Correlation results (pipe to file if needed)

## Performance Tips

**For large datasets:**
```bash
# Use R-Tree or Overlapping Chunks
amp-server correlate --algorithm rtree
```

**For benchmarking:**
```bash
# Start with small sample
amp-server benchmark --sample-size 100

# Increase for production validation
amp-server benchmark --sample-size 1000
```

**For CI/CD:**
```bash
# Quick validation
amp-server benchmark --sample-size 50
```

## Troubleshooting

**"No matches found"**
- Check internet connection (ArcGIS API requires network)
- Verify Malmö Open Data Portal is accessible
- Try `check-updates` to confirm data availability

**"Slow performance"**
- Use `--algorithm rtree` instead of `distance-based`
- Reduce `--sample-size` for benchmarks
- Consider memory constraints (Grid/Chunks use more RAM)

**"Checksum file not found"**
- Normal on first run
- File created automatically by `check-updates`

## Related Documentation

- [Algorithms](algorithms.md) — Algorithm details
- [Architecture](architecture.md) — System design
- [server/README.md](../server/README.md) — Server module guide
