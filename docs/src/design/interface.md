# Waveform Renderer Interface Design

## Core philosophy

An audio waveform renderer with playback that's "stupid easy" to consume from any language or environment. This means:

- 🌐 WebAssembly support with minimal JavaScript glue
- 🚀 Simple C-compatible FFI for native bindings
- 🖼️ Pull-based viewport rendering that's configurable:
  - Any FPS
  - Geometric output for GPU-based rendering (WebGPU, OpenGL, etc)
  - Rasterized output for CPU-based rendering
  - Optional phase-locked loop smoothing 
- 🔊 Pluggable audio implementations:
  - CPAL for native
  - Web Audio for web
  - Bindings provided for FMOD
  - Bring-your-own (JUCE, PortAudio, etc)

## Architecture

```
┌─────────────────────────────────────────────┐
│              Consumer Layer                 │
├─────────────────────────────────────────────┤
│  Python │ JavaScript │ Swift │ C++ │ Java   │
├─────────────────────────────────────────────┤
│             Bindings Layer                  │
├─────────────────────────────────────────────┤
│      C FFI          │      WASM API         │
├─────────────────────────────────────────────┤
│            src-core (Rust)                  │
│  ┌────────────────────────────────────┐     │
│  │   Waveform Renderer Core           │     │
│  ├────────────────────────────────────┤     │
│  │ • Peak detection                   │     │
|. | • RMS detection                    │     │
│  │ • Audio plugin bridge              │     │
|. | • Phase-locked loop smoothing      │     │
│  │ • Viewport rendering               │     │
│  └────────────────────────────────────┘     │
└─────────────────────────────────────────────┘
```

## Javascript usage

```javascript
const driftwave = await Driftwave.create();
driftwave.load('audio.mp3');
driftwave.on('ready', () => driftwave.play());
```
