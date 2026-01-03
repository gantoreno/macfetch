# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```sh
cargo build              # Debug build
cargo build --release    # Optimized release build
cargo run                # Run debug target
cargo run --release      # Run release target
cargo test               # Run all unit tests
cargo clippy             # Run linter
cargo fmt                # Format code
```

## Architecture

Macfetch is a macOS-only Neofetch alternative written in Rust. It displays system information alongside an ASCII art logo.

### Core Structure

- **`src/main.rs`**: Entry point. Calls `macfetch::render()` with the Darwin ASCII logo and a list of segment functions.
- **`src/macfetch/mod.rs`**: Contains the `render()` function that pairs ASCII logo lines with segment output lines.
- **`src/macfetch/segments/mod.rs`**: Individual segment functions (os, cpu, memory, etc.) that return `ColoredString`. Each segment fetches system data and formats it with `titled_segment()`.
- **`src/macfetch/ascii/mod.rs`**: ASCII art generation with `generate_logo()` that returns colored logo lines.
- **`src/macfetch/utils/`**: Helper modules for CLI handling, sysctl queries, host info, caching, and configuration.
- **`src/macfetch/utils/config.rs`**: TOML configuration loading. Maps segment names to functions and builds the segment list from `~/.config/macfetch/config.toml`.

### Key Patterns

- Segments are functions with signature `fn() -> ColoredString` passed as a vector to `render()`.
- System info is retrieved via `sysctl` crate, environment variables, and macOS-specific APIs (Core Graphics, Metal).
- The `cache` module provides `fallback()` for expensive operations (CPU/GPU lookups).
- CLI args are handled via `clap` in `utils/cli.rs` with derive-based parsing.

### CI Pipeline

GitHub Actions runs on every push/PR to `main`:
- `cargo check` - Compilation
- `cargo test` - Unit tests
- `cargo clippy -- -D warnings` - Linter (warnings fail the build)
- `cargo fmt --check` - Formatting

### Adding New Segments

1. Create a new function in `src/macfetch/segments/mod.rs` returning `ColoredString`
2. Use `titled_segment(name, value)` for consistent formatting
3. Register the segment in `src/macfetch/utils/config.rs`:
   - Add it to `segment_registry()` HashMap
   - Add it to `default_segments()` vector (if it should appear by default)
