# Driftwave 〰️

A waveform visualization engine built with research-grade accuracy and developer-grade portability

## Why Driftwave?

Most waveform tools compromise on either precision or portability. Driftwave doesn’t. It delivers research-grade accuracy and GPU-powered performance in one engine—built for engineers, scientists, and developers who need sample-perfect visualization as the backbone of serious analysis and annotation.

## Project goals

- **🎯 Exact Precision**: Every sample mapped to the right pixel, enabling reliable measurement and annotation.
- **🚀 High Speed**: GPU acceleration and SIMD keep panning, zooming, and scrubbing fluid—even with hours of high-resolution audio.
- **Cross-Platform by Design**:
  - 🖥️ Native runtimes on macOS, Windows, and Linux powered by [FMOD](https://www.fmod.com/) and [Tauri](https://v2.tauri.app/), easily adapted to Electron
  - 🌐 WebAssembly bundles for browsers, when Web Audio’s limitations are acceptable
- **🔍 Built for Analysis**: Not just pretty graphics—an engine ready for phonetics research, bioacoustics, and industrial signal analysis with sample-level precision.

## Technology

- **Rust core** for safety and speed
- **WebGPU renderer** for portable GPU acceleration
- **SIMD-optimized peak detection** for efficient audio crunching
- **Configurable DSP** so you can focus on the signal, not the noise
- **Phase-locked loop** motion stabilization (optional)
- **Advanced latency estimation** in desktop versions
- **Javascript bindings** compatible with any UI framework
