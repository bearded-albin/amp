#!/bin/bash
# Build Rust server binary for Linux

set -e

echo "🔨 Building AMP Server Binary (Rust)..."

# Install Rust nightly 2024
rustup override set nightly-2026-01-12

# Build release binary
cargo build --release \
    --manifest-path server/Cargo.toml \
    --target x86_64-unknown-linux-gnu

# Create systemd service
cat > /tmp/amp-server.service <<'EOF'
[Unit]
Description=AMP Malmö Parking Alerts Server
After=network.target

[Service]
Type=simple
User=amp
WorkingDirectory=/opt/amp-server
ExecStart=/opt/amp-server/amp-server
Restart=on-failure
RestartSec=10

Environment="RUST_LOG=amp_server=debug"

NoNewPrivileges=true
PrivateTmp=true

[Install]
WantedBy=multi-user.target
EOF

echo "✅ Binary: ./server/target/x86_64-unknown-linux-gnu/release/amp-server"
echo "📋 Systemd file: /tmp/amp-server.service"

# Strip binary for smaller size
strip ./server/target/x86_64-unknown-linux-gnu/release/amp-server
echo "📦 Binary size: $(du -h ./server/target/x86_64-unknown-linux-gnu/release/amp-server)"
