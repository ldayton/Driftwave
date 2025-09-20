# Driftwave

Audio waveform rendering

## Project goals

- Sample-accurate and pixel-accurate, appropriate for scientific use
- Ultra-smooth performance, with GPU acceleration and SIMD execution
- Cross-platform desktop support via Tauri & FMOD: macOS, Windows, Linux
- Web support via WASM & Web Audio

## Setup

Install:
- Rust
- Node

### Linux-only extra setup

Install dependencies with Homebrew for Linux:
```bash
brew install webkitgtk libsoup@2
```

Set environment variables:
```bash
export PKG_CONFIG_PATH="/home/linuxbrew/.linuxbrew/Cellar/libsoup@2/2.74.3/lib/pkgconfig:/home/linuxbrew/.linuxbrew/lib/pkgconfig:/home/linuxbrew/.linuxbrew/share/pkgconfig:$PKG_CONFIG_PATH"
export LD_LIBRARY_PATH="/home/linuxbrew/.linuxbrew/lib:$LD_LIBRARY_PATH"
```

## Run

```bash
npm install
npm run dev
```

## Project structure

- `src-tauri/` - Rust backend
  - `src/core/` - Audio processing logic
  - `src/ffi/` - FMOD bindings
- `src/` - Web frontend
- `tools/` - Build utilities

## Develop

Enable git hooks for automatic code formatting:
```bash
git config core.hooksPath .githooks
```

Regenerate FMOD FFI bindings:
```bash
cd tools && cargo run --bin generate_bindings
```
