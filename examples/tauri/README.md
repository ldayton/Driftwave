# Tauri Desktop App Example

A desktop application using Tauri and the Driftwave audio library.

## Prerequisites

- [Rustup](https://rustup.rs)
- [Node.js](https://nodejs.org) (v18 or later)

## Running the App

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Project Structure

- `src/` - Frontend HTML/JS
- `src-tauri/` - Rust backend using driftwave-fmod
- `src-tauri/tauri.conf.json` - Tauri configuration

## Features

- FMOD audio playback via driftwave-fmod
- File selection dialog
- Play/stop audio controls