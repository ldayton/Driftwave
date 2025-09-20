# 〰️ Driftwave 〰️

A waveform engine built with research-grade accuracy and developer-grade portability

## Why Driftwave?  
Most waveform visualizers are built for consumers, not researchers. Driftwave is different: a high-performance waveform engine that combines lab-grade accuracy with modern GPU-powered rendering. It’s designed for engineers, scientists, and developers who need sample-precise visualization as a foundation for serious analysis and annotation.  

## Project goals  
- **Exact Precision**: Every sample mapped to the right pixel, enabling reliable measurement and annotation.  
- **High Speed**: GPU acceleration and SIMD keep panning, zooming, and scrubbing fluid—even with hours of high-resolution audio.  
- **Cross-Platform by Design**:  
  - Native runtimes on macOS, Windows, and Linux powered by [FMOD](https://www.fmod.com/)  
  - WebAssembly builds for browsers, when Web Audio’s latency is acceptable
- **Built for Analysis**: Not just pretty graphics—an engine ready for phonetics research, bioacoustics, and industrial signal analysis with sub-millisecond precision.  

## Technology  
- **Rust core** for safety and speed  
- **WebGPU renderer** for portable GPU acceleration  
- **SIMD-optimized DSP** for efficient audio crunching  
- **Minimal JavaScript** so downstream apps choose their own UI stack  
