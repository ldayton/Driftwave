# Developer Guide

## Building the library

### Prerequisites

All platforms:
- [Rustup](https://rustup.rs)

For WASM support:
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

### Build

```bash
# Build all workspace members (core, fmod, tools)
cargo build

# Build specific crates
cargo build -p driftwave-core    # Core traits and types
cargo build -p driftwave-fmod    # FMOD audio implementation

# Build WASM module for web
cd src-web && wasm-pack build --target web --out-dir pkg

# Build release mode
cargo build --release
```

## Project Structure

- `src-core/` - Core traits and shared types (Player trait, etc.)
- `src-fmod/` - FMOD audio implementation for desktop
- `src-web/` - Web Audio API implementation (WASM, not in workspace)
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
