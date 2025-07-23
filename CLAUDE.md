# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

bevy-settings is a Bevy game engine plugin that provides persistent settings storage across game launches. It automatically saves settings to TOML files in OS-appropriate config directories.

## Development Commands

```bash
# Build
cargo build

# Run tests (tests run serially due to filesystem access)
cargo test --verbose

# Format code
cargo fmt

# Lint (must pass for CI)
cargo clippy --all-targets --all-features -- -D warnings

# Run examples
cargo run --example basic
cargo run --example multiple
```

## Architecture

The library uses a generic plugin pattern centered around:

- `SettingsPlugin<S>` - Generic plugin that manages loading/saving for any type implementing `Settingable`
- `Settingable` trait - Requires `Resource + Clone + Serialize + Deserialize + Default`
- Event-driven persistence via `PersistSettings` and `PersistSetting<S>` events
- All code is in `src/lib.rs` (single file library)

## Important Notes

1. **Tests must use `#[serial_test::serial]`** - Tests access the filesystem and must run one at a time
2. **Known limitations**: TOML crate issues with large numbers, tuple structs don't work (use named fields)
3. **Platform deps**: Linux requires `libasound2-dev` and `libudev-dev` for Bevy
4. **Settings are Bevy Resources** - Access them like any other Resource in your systems

## Usage Pattern

```rust
// Define settings
#[derive(Resource, Default, Serialize, Deserialize, Clone)]
#[serde(crate = "bevy_settings::serde")]
struct Settings {
    volume: f32,
}

// Add plugin
app.add_plugins(SettingsPlugin::<Settings>::new("Company", "Game"));

// Save settings
writer.send(PersistSettings);
```