<h1 align="center">
	<img src="https://raw.githubusercontent.com/ldayton/Driftwave/main/assets/logo.png" width="150" alt="Logo"/><br/>
	Driftwave
</h1>

## Audio waveform visualization that's fast, accurate, and portable

Driftwave provides:

- 📊 **Waveform visualization** ready for GPU rendering
- 🌐 **WebAssembly support** for browser deployment
- 🔊 **Audio playback** with multiple backend support
- 🔧 **Language bindings** for Python, Javascript, Java, and more

## Why Driftwave?

Most waveform tools compromise on either precision or portability. Driftwave doesn’t. It delivers research-grade accuracy for engineers, scientists, and developers in a form that can be consumed in any environment.

## Project goals

- **🎯 Exact Precision**: Every sample mapped to the right pixel, enabling reliable measurement and annotation.
- **🚀 High Speed**: GPU acceleration and SIMD keep panning, zooming, and scrubbing fluid—even with hours of high-resolution audio.
- **Cross-Platform by Design**:
  - 🌐 WebAssembly bundles for browsers, for web audio
  - 🖥️ Native runtimes on macOS, Windows, and Linux
  - 🔊 Default playback with CPAL, with FMOD and PortAudio bindings also provided
- **🔍 Built for Analysis**: Not just pretty graphics—an engine ready for phonetics research, bioacoustics, and industrial signal analysis with sample-level precision.

## Technology

- **Rust core** for safety and speed
- **WebGPU renderer** for portable GPU acceleration
- **SIMD-optimized peak/RMS detection** for efficient audio crunching
- **Configurable DSP** so you can focus on the signal, not the noise
- **Phase-locked loop** motion stabilization (optional)
- **Advanced latency estimation** in desktop versions
- **Javascript bindings** compatible with any UI framework

## Quick Links

- [GitHub Repository](https://github.com/ldayton/Driftwave)
- [API Documentation](https://docs.rs/driftwave-core)
- [Examples](https://github.com/ldayton/Driftwave/tree/main/examples)