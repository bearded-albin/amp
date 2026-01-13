#!/bin/bash
# Build Android APK with Rust FFI

set -e

echo "🔨 Building AMP Android APK (Rust + Flutter)..."

# Set Rust targets for Android
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi

# Build Rust library for Android
cargo build --release \
    --manifest-path android/Cargo.toml \
    --target aarch64-linux-android

cargo build --release \
    --manifest-path android/Cargo.toml \
    --target armv7-linux-androideabi

# Copy built libraries to Flutter project
mkdir -p android/app/src/main/jniLibs/{arm64-v8a,armeabi-v7a}
cp android/target/aarch64-linux-android/release/libamp_android.so \
    android/app/src/main/jniLibs/arm64-v8a/
cp android/target/armv7-linux-androideabi/release/libamp_android.so \
    android/app/src/main/jniLibs/armeabi-v7a/

# Build APK
cd android
flutter build apk --release
cd ..

echo "✅ APK built: android/build/app/outputs/flutter-apk/app-release.apk"
