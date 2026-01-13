# AMP Android MVP - File Structure

```
amp/
├── Cargo.toml                          # Root workspace
├── rust-toolchain.toml                 # Nightly 2026-01-12
├── rustfmt.toml
├── .gitignore
│
├── core/                               # ✅ Shared library (NO Android dependencies)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── error.rs
│       ├── models.rs
│       ├── correlation.rs
│       ├── geolocation.rs
│       └── state.rs
│
├── android/                            # ✅ Android-specific (Flutter FFI)
│   ├── Cargo.toml
│   ├── android/
│   │   ├── app/
│   │   │   ├── build.gradle
│   │   │   └── src/main/
│   │   │       ├── AndroidManifest.xml
│   │   │       └── kotlin/com/amp/MainActivity.kt
│   │   └── settings.gradle
│   ├── pubspec.yaml                    # Flutter config
│   └── src/
│       ├── lib.rs                      # FFI entry point
│       ├── ffi.rs                      # Flutter FFI bindings
│       ├── notifications.rs            # Android notifications
│       └── gps.rs                      # GPS integration
│
└── old/                                # Your previous code (reference)
    └── ...
```

## Build Targets

- **Android ARM64**: `aarch64-linux-android`
- **Android ARM32**: `armv7-linux-androideabi`
- **Linux x86_64**: `x86_64-unknown-linux-gnu`
- **Linux ARM64**: `aarch64-unknown-linux-gnu`
