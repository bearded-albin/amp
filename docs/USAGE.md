# AMP Server Usage Guide

## Installation

```bash
git clone https://github.com/resonant-jovian/amp.git
cd amp
cargo build --release --bin amp-server
```

## Quick Start

### Run Correlation

```bash
# Use default algorithm (distance-based)
amp-server correlate

# Use specific algorithm
amp-server correlate --algorithm overlapping-chunks

# Use raycasting
amp-server correlate --algorithm raycasting

# Use linear algebra
amp-server correlate --algorithm linear-algebra
```

### Benchmark All Algorithms

```bash
# Test with 100 samples (default)
amp-server benchmark

# Test with 500 samples
amp-server benchmark --sample-size 500

# Test with 1000 samples
amp-server benchmark --sample-size 1000
```

Example output:
```
Algorithm            Total Time      Avg per Address     Processed       Matches
-------------------------------------------------------------------------------------
Distance-Based       2.45s           4.90ms              500             423
Raycasting (50m)     5.12s           10.24ms             500             431
Overlapping Chunks   1.23s           2.46ms              500             423
Linear Algebra       2.31s           4.62ms              500             423

✓ Fastest: Overlapping Chunks (1.23s)
```

### Check Data Updates

```bash
# Check if Malmö's data has changed
amp-server check-updates

# Use custom checksum file
amp-server check-updates --checksum-file my_checksums.json
```

Example output:
```
Checking for data updates...
Fetching remote data...

✓ Data has changed!
  Old checksums from: 2026-01-22T10:15:30Z
  New checksums from: 2026-01-23T10:15:30Z
✓ Checksums saved to checksums.json
```

## Command Reference

### `amp-server correlate`

Run correlation between addresses and parking zones.

**Options:**
- `-a, --algorithm <ALGORITHM>`: Algorithm to use
  - `distance-based` (default): Perpendicular distance
  - `raycasting`: Ray intersection with 50m range
  - `overlapping-chunks`: Spatial grid optimization
  - `linear-algebra`: Vector projection method
- `-m, --miljo-file <FILE>`: Path to miljöparking JSON file (optional)
- `-p, --parkering-file <FILE>`: Path to parking fees JSON file (optional)
- `-a, --addresses-file <FILE>`: Path to addresses JSON file (optional)

**Examples:**
```bash
# Default algorithm
amp-server correlate

# Fastest algorithm for large datasets
amp-server correlate --algorithm overlapping-chunks

# Most spatially aware
amp-server correlate --algorithm raycasting

# Most mathematically clean
amp-server correlate --algorithm linear-algebra
```

### `amp-server benchmark`

Compare performance of all algorithms.

**Options:**
- `-m, --miljo-file <FILE>`: Path to miljöparking JSON file (optional)
- `-a, --addresses-file <FILE>`: Path to addresses JSON file (optional)
- `-s, --sample-size <N>`: Number of addresses to test (default: 100)

**Examples:**
```bash
# Quick test
amp-server benchmark --sample-size 100

# Comprehensive test
amp-server benchmark --sample-size 1000

# Full dataset test
amp-server benchmark --sample-size 99999
```

### `amp-server check-updates`

Verify if remote data sources have changed.

**Options:**
- `-c, --checksum-file <FILE>`: Checksum file path (default: checksums.json)

**Examples:**
```bash
# Default location
amp-server check-updates

# Custom location
amp-server check-updates --checksum-file /var/amp/checksums.json
```

## Data Sources

AMP uses three open data sources from Malmö stad:

1. **Miljöparkeringar** (Environmental Parking)
   - URL: https://opendata.malmo.se/@fastighets-och-gatukontoret/miljoparkering/...
   - 2,260 LineString features
   - Time-based parking restrictions

2. **Parkeringsavgifter** (Parking Fees)
   - URL: https://opendata.malmo.se/@fastighets-och-gatukontoret/parkeringsavgifter/...
   - 1,676 LineString features
   - Fee zones (Taxa A-E)

3. **Adresser** (Addresses)
   - URL: https://opendata.malmo.se/@stadsbyggnadskontoret/adresser/...
   - 100,000+ Point features
   - Full Malmö address coverage

## Workflow Example

### Daily Data Verification

Set up a cron job to check for updates:

```bash
#!/bin/bash
# check_parking_data.sh

cd /path/to/amp
amp-server check-updates

if [ $? -eq 0 ]; then
    echo "Data check completed successfully"
    # Optionally trigger re-correlation
    # amp-server correlate --algorithm overlapping-chunks
fi
```

Cron entry:
```
0 2 * * * /path/to/check_parking_data.sh
```

### Production Correlation

For production use with large datasets:

```bash
# 1. Check for updates
amp-server check-updates

# 2. Benchmark to verify performance
amp-server benchmark --sample-size 500

# 3. Run full correlation with fastest algorithm
amp-server correlate --algorithm overlapping-chunks > results.txt
```

## Performance Tips

### Algorithm Selection

| Dataset Size | Recommended Algorithm | Expected Time |
|--------------|----------------------|---------------|
| < 1,000 | Distance-Based | < 1 second |
| 1,000-10,000 | Overlapping Chunks | 2-10 seconds |
| 10,000+ | Overlapping Chunks | 10-60 seconds |

### Memory Usage

- **Distance-Based**: ~100 MB
- **Raycasting**: ~100 MB
- **Overlapping Chunks**: ~200 MB (spatial index)
- **Linear Algebra**: ~100 MB

### Optimization

For maximum performance:

```bash
# Build with optimizations
cargo build --release --bin amp-server

# Run with release binary
./target/release/amp-server correlate --algorithm overlapping-chunks

# Parallel processing (future enhancement)
# Set RAYON_NUM_THREADS environment variable
export RAYON_NUM_THREADS=8
amp-server correlate --algorithm overlapping-chunks
```

## Troubleshooting

### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build --release --bin amp-server

# Update dependencies
cargo update
```

### Runtime Errors

**"Failed to load data"**
- Verify internet connection
- Check Malmö's open data portal is accessible
- Try alternative data source URLs

**"Algorithm taking too long"**
- Use overlapping-chunks for large datasets
- Reduce sample size for benchmarking
- Ensure release build is being used

### Getting Help

```bash
# View help
amp-server --help

# View subcommand help
amp-server correlate --help
amp-server benchmark --help
amp-server check-updates --help
```

## API Integration

For programmatic use:

```rust
use amp_core::correlation_algorithms::{OverlappingChunksAlgo, CorrelationAlgo};
use amp_core::api::api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load data
    let (addresses, zones) = api().await?;
    
    // Create algorithm
    let algo = OverlappingChunksAlgo::new(&zones);
    
    // Correlate
    for address in &addresses {
        if let Some((zone_idx, distance)) = algo.correlate(address, &zones) {
            println!("Address {} matched zone {} at {:.2}m", 
                     address.adress, zone_idx, distance);
        }
    }
    
    Ok(())
}
```
