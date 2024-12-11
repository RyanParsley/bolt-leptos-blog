# Leptos Blog

A blog application built with Leptos.

## Development Commands

- `cargo serve` - Start the development server
- `cargo watch` - Watch for changes and run tests/checks
- `cargo test` - Run all tests
- `cargo check-all` - Check all targets
- `cargo lint` - Run clippy lints
- `cargo release` - Build for release

## Project Structure

```
src/
├── bin/          # Binary entrypoint
├── components/   # Reusable UI components
├── pages/        # Page components
├── types/        # Type definitions
└── utils/        # Utility functions
```

## Getting Started

1. Install dependencies:
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install trunk
   cargo install cargo-watch  # Optional, for development
   ```

2. Start the development server:
   ```bash
   cargo serve
   ```