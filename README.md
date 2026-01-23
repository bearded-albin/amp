# AMP

**Address-to-Miljozone Parking** — Geospatial correlation library matching addresses to environmental parking zones in Malmö, Sweden.

[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](LICENSE)
[![Rust 2024](https://img.shields.io/badge/rust-2024%2B-orange)](https://www.rust-lang.org/)

## Overview

AMP correlates street addresses with parking restriction zones using geospatial algorithms. It provides a Rust library, CLI tool, and mobile apps for checking parking restrictions without internet access.

**Key Features:**
- Six correlation algorithms (distance-based, raycasting, spatial indexing)
- Dual dataset support (miljödata + parkering zones)
- CLI with benchmarking and data update checks
- Android and iOS apps built with Dioxus

## Quick Start

### CLI

```bash
# Build and run
cargo build --release -p amp_server
./target/release/amp-server correlate --algorithm rtree

# Benchmark algorithms
./target/release/amp-server benchmark --sample-size 500

# Check for data updates
./target/release/amp-server check-updates
```

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

## Project Structure

```
amp/
├── core/          # Correlation algorithms and data structures
├── server/        # CLI tool with benchmarking
├── android/       # Android app (Dioxus)
├── ios/           # iOS app (Dioxus)
└── docs/          # Architecture and algorithm documentation
```

## Documentation

- **[Architecture](docs/architecture.md)** — System design and data flow
- **[Algorithms](docs/algorithms.md)** — Correlation algorithms explained
- **[CLI Usage](docs/cli-usage.md)** — Command-line interface guide
- **[API Integration](docs/api-integration.md)** — ArcGIS data fetching
- **[Testing](docs/testing.md)** — Test strategy and validation

**Module Documentation:**
- [core/](core/) — Core library
- [server/](server/) — CLI tool
- [android/](android/) — Android app
- [ios/](ios/) — iOS app

## Building

### Prerequisites
- Rust 1.70+ ([rustup](https://rustup.rs))
- For mobile: Dioxus CLI (`cargo install dioxus-cli`)

### Build Commands

```bash
# Library and server
cargo build --release -p amp_core
cargo build --release -p amp_server

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

See module `Cargo.toml` files for complete lists.

## License

GPL-3.0 — See [LICENSE](LICENSE) for details.

## Contact

**Albin Sjögren**  
[albin@sjoegren.se](mailto:albin@sjoegren.se)  
Malmö, Sweden
