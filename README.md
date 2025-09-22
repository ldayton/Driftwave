# Driftwave [![Web Build](https://github.com/ldayton/Driftwave/actions/workflows/build-web.yml/badge.svg)](https://github.com/ldayton/Driftwave/actions/workflows/build-web.yml) [![Windows Build](https://github.com/ldayton/Driftwave/actions/workflows/build-windows.yml/badge.svg)](https://github.com/ldayton/Driftwave/actions/workflows/build-windows.yml) [![Mac Build](https://github.com/ldayton/Driftwave/actions/workflows/build-mac.yml/badge.svg)](https://github.com/ldayton/Driftwave/actions/workflows/build-mac.yml) [![Linux Build](https://github.com/ldayton/Driftwave/actions/workflows/build-linux.yml/badge.svg)](https://github.com/ldayton/Driftwave/actions/workflows/build-linux.yml) 

A waveform visualization engine built with research-grade accuracy and developer-grade portability

## Why Driftwave?

Most waveform tools compromise on either precision or portability. Driftwave doesn’t. It delivers research-grade accuracy for engineers, scientists, and developers in a form that can be consumed in any environment.

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
