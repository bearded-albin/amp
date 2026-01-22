#!/bin/bash
set -e

# Load variables explicitly (more reliable than set -a)
storePassword=$(grep "^storePassword=" keystore.properties | cut -d= -f2 | tr -d ' "')
keyPassword=$(grep "^keyPassword=" keystore.properties | cut -d= -f2 | tr -d ' "')
keyAlias=$(grep "^keyAlias=" keystore.properties | cut -d= -f2 | tr -d ' "')
storeFile=$(grep "^storeFile=" keystore.properties | cut -d= -f2 | tr -d ' "')

# Create Dioxus.toml
cat > Dioxus.toml << EOF
[bundle.android]
jks_file = "$storeFile"
jks_password = "$storePassword"
key_password = "$keyPassword"
key_alias = "$keyAlias"
EOF

# Verify it worked (should show real filename, not "null")
echo "✓ Generated jks_file: $(grep jks_file Dioxus.toml)"

# Build
dx build --android --release --bundle android --package amp-android --device HQ646M01AF

# Restore original
git checkout Dioxus.toml 2>/dev/null || true
