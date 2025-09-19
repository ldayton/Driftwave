# Driftwave

## Build

```bash
cargo build
```

## Run

```bash
cargo run --bin driftwave
```

## Test

```bash
cargo test
```

## Develop

Enable git hooks for automatic code formatting:
```bash
git config core.hooksPath .githooks
```

Regenerate FMOD FFI bindings:
```bash
cargo run --bin generate_bindings
```
