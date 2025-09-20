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

## Package

### Mac (app/dmg)

```bash
npm run tauri build
```

### Linux

Install build dependencies with Homebrew for Linux:

```bash
brew install webkitgtk libsoup@2
```

Tell Tauri where to find things:

```bash
export PKG_CONFIG_PATH="/home/linuxbrew/.linuxbrew/Cellar/libsoup@2/2.74.3/lib/pkgconfig:/home/linuxbrew/.linuxbrew/lib/pkgconfig:/home/linuxbrew/.linuxbrew/share/pkgconfig:$PKG_CONFIG_PATH"
ln -sf /home/linuxbrew/.linuxbrew/lib/pkgconfig/javascriptcoregtk-4.1.pc /home/linuxbrew/.linuxbrew/lib/pkgconfig/javascriptcoregtk-4.0.pc
ln -sf /home/linuxbrew/.linuxbrew/lib/pkgconfig/webkit2gtk-4.1.pc /home/linuxbrew/.linuxbrew/lib/pkgconfig/webkit2gtk-4.0.pc
```

Now build like normal:

```bash
npm run tauri build
```

