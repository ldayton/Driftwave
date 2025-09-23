# Development Guide

## Building the library

### Prerequisites

All platforms:
- [Rustup](https://rustup.rs)

For web support:
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

### Build native

```bash
# Build all workspace members (core, fmod, tools)
cargo build

# Build specific crates
cargo build -p driftwave-core    # Core traits and types
cargo build -p driftwave-fmod    # FMOD audio implementation

# Build release mode
cargo build --release
```

### Build web

```bash
# Build WASM module into js package
cd src-web
wasm-pack build --release --target web --out-dir ../js/wasm

# Build NPM package
cd ../js
npm install
npm run build

# Create publishable package
npm pack  # Creates driftwave-x.x.x.tgz
```

## Project Structure

- `src-core/` - Core traits and shared types (Player trait, etc.)
- `src-fmod/` - FMOD audio implementation for desktop
- `src-web/` - Web bindings (WASM, not in workspace)
- `js/` - JavaScript/TypeScript wrapper for NPM
- `tools/` - Build utilities and code generation
- `examples/` - Standalone example applications

## Running Examples

Examples are standalone projects with their own dependencies. See the README in each example directory for specific instructions.

## Develop

Enable git hooks for automatic code formatting:
```bash
git config core.hooksPath .githooks
```

Regenerate FMOD FFI bindings:
```bash
cargo run --bin generate_bindings
```
