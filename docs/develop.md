# Developer Guide

## Setup

### All platforms

- Rust
- Node

### Windows extra setup

Install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) or Visual Studio 2022.
During installation, select "Desktop development with C++" workload.

### Mac extra setup

Install Xcode from the App Store.

### Linux extra setup

See `.github/workflows/build-linux` for Ubuntu 24.04 instructions.

You may also need to set this for `npm run dev` to work:

```bash
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:$PKG_CONFIG_PATH
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
