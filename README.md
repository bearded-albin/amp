```
                                .         .                          
         .8.                   ,8.       ,8.          8 888888888o   
        .888.                 ,888.     ,888.         8 8888    `88. 
       :88888.               .`8888.   .`8888.        8 8888     `88 
      . `88888.             ,8.`8888. ,8.`8888.       8 8888     ,88 
     .8. `88888.           ,8'8.`8888,8^8.`8888.      8 8888.   ,88' 
    .8`8. `88888.         ,8' `8.`8888' `8.`8888.     8 888888888P'  
   .8' `8. `88888.       ,8'   `8.`88'   `8.`8888.    8 8888         
  .8'   `8. `88888.     ,8'     `8.`'     `8.`8888.   8 8888         
 .888888888. `88888.   ,8'       `8        `8.`8888.  8 8888         
.8'       `8. `88888. ,8'         `         `8.`8888. 8 8888

```

# AMP

**Address-to-Miljozone Parking** — Geospatial correlation library matching addresses to environmental parking zones in Malmö, Sweden.

[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![Rust 2024](https://img.shields.io/badge/rust-2024%2B-orange)](https://www.rust-lang.org/)

## Overview

AMP correlates street addresses with parking restriction zones using geospatial algorithms. It provides a Rust library, CLI tool, and mobile apps for checking parking restrictions without internet access.

**Key Features:**
- Six correlation algorithms (distance-based, raycasting, spatial indexing, grid-based)
- Dual dataset support (miljödata + parkering zones)
- CLI with testing mode, benchmarking, and data update checks
- Android and iOS apps built with Dioxus
- Visual testing interface with StadsAtlas integration
- Cross-platform (Windows, macOS, Linux)

## Quick Start

### CLI - Testing Mode

Visually verify correlation accuracy by comparing results against official StadsAtlas:

```bash
# Open 10 browser windows with random addresses
cargo run --release -- test

# Custom algorithm and distance threshold
cargo run -- test --algorithm rtree --cutoff 100 --windows 15
```

**What happens:**
- Opens side-by-side browser windows
- Left: Official Malmö StadsAtlas map
- Right: Correlation results for each address
- Manually verify zone matches

See [docs/testing.md](docs/testing.md) for detailed testing guide.

### CLI - Correlation

```bash
# Run correlation with KD-Tree algorithm (default)
cargo build --release -p amp_server
./target/release/amp-server correlate

# Custom algorithm and distance threshold
./target/release/amp-server correlate --algorithm rtree --cutoff 75

# Benchmark all algorithms
./target/release/amp-server benchmark --sample-size 500

# Check if data needs updating
./target/release/amp-server check-updates
```

See [docs/cli-usage.md](docs/cli-usage.md) for complete CLI reference.

### Library

```rust
use amp_core::api::api_miljo_only;
use amp_core::correlation_algorithms::{RTreeSpatialAlgo, CorrelationAlgo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (addresses, zones) = api_miljo_only()?;
    let algo = RTreeSpatialAlgo::new(&zones);
    
    for addr in addresses.iter().take(10) {
        if let Some((idx, dist)) = algo.correlate(addr, &zones) {
            println!("{}: {:.2}m to zone {}", addr.adress, dist, idx);
        }
    }
    Ok(())
}
```

See [core/README.md](core/README.md) for library documentation.

## Project Structure

```
amp/
├── README.md              # This file
├── docs/                  # Documentation
│   ├── architecture.md    # System design
│   ├── algorithms.md      # Algorithm details
│   ├── api-integration.md # Data fetching
│   ├── cli-usage.md       # Command-line reference
│   ├── testing.md         # Testing strategies
│   └── implementation-notes.md  # Technical details
├── core/                  # Rust library crate
│   ├── README.md          # Library guide
│   └── src/
│       ├── lib.rs
│       ├── api.rs
│       ├── structs.rs
│       ├── correlation_algorithms/
│       ├── benchmark.rs
│       ├── checksum.rs
│       └── correlation_tests.rs
├── server/                # CLI tool crate
│   ├── README.md          # Server guide
│   └── src/
│       ├── main.rs
│       └── assets/        # UI templates
├── android/               # Android app (Dioxus)
├── ios/                   # iOS app (Dioxus)
└── build.sh              # Build script
```

## Documentation

### Getting Started
- **[Quick Start](#quick-start)** — Run correlation or testing mode
- **[CLI Usage](docs/cli-usage.md)** — Complete command reference
- **[Testing Guide](docs/testing.md)** — Visual testing with StadsAtlas

### Architecture & Design
- **[Architecture](docs/architecture.md)** — System design and data flow
- **[Algorithms](docs/algorithms.md)** — How each algorithm works
- **[API Integration](docs/api-integration.md)** — ArcGIS data fetching
- **[Implementation Notes](docs/implementation-notes.md)** — Technical details

### Module Documentation
- **[Core Library](core/README.md)** — Library API and usage
- **[Server/CLI](server/README.md)** — CLI tool guide

## Building

### Prerequisites
- Rust 1.70+ ([rustup](https://rustup.rs))
- For mobile: Dioxus CLI (`cargo install dioxus-cli`)

### Build Commands

```bash
# Core library
cargo build --release -p amp_core

# CLI server
cargo build --release -p amp_server

# Run tests
cargo test --release

# Android
cd android && dx build --release

# iOS
cd ios && dx build --release
```

## Dependencies

Core dependencies:
- `rust_decimal` — High-precision coordinates
- `rayon` — Parallel processing
- `tokio` — Async runtime
- `reqwest` — HTTP client
- `rstar` — R-tree spatial indexing
- `kiddo` — KD-tree spatial indexing
- `dioxus` — UI framework (mobile)

See `Cargo.toml` files for complete dependency lists.

## Testing

### Visual Testing Mode

```bash
# Test with default settings (10 windows, KD-Tree, 50m threshold)
cargo run --release -- test

# Test with custom parameters
cargo run -- test --algorithm rtree --cutoff 100 --windows 20

# Test with conservative threshold
cargo run -- test --cutoff 25 --windows 5
```

See [docs/testing.md](docs/testing.md) for detailed testing procedures.

### Unit Tests

```bash
# Run all tests
cargo test --release

# Run specific algorithm tests
cargo test --lib correlation_algorithms::rtree_spatial

# Run benchmark tests
cargo bench
```

## Data Sources

AMP fetches parking zone data from official Malmö Open Data:

- **Miljöparkering** — Environmental parking restrictions
- **Parkeringsavgifter** — Parking fee zones
- **Adresser** — Address coordinates

Data is verified using checksums to detect updates. See [docs/api-integration.md](docs/api-integration.md) for details.

## License

GPL-3.0 — See [LICENSE](LICENSE) for details.

## Contact

**Albin Sjögren**  
[albin@sjoegren.se](mailto:albin@sjoegren.se)  
Malmö, Sweden

## Related Documentation

For detailed information, see:
- [Architecture Overview](docs/architecture.md)
- [Algorithm Comparison](docs/algorithms.md)
- [API Integration Details](docs/api-integration.md)
- [CLI Command Reference](docs/cli-usage.md)
- [Testing Strategies](docs/testing.md)
- [Implementation Notes](docs/implementation-notes.md)
- [Core Library Guide](core/README.md)
- [Server Tool Guide](server/README.md)
