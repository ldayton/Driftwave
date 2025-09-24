import init, { Driftwave as WasmDriftwave } from '../wasm/driftwave_web.js';

interface Metadata {
  sampleRate: number;
  channelCount: number;
  frameCount: number;
}

class Driftwave {
  private wasm: WasmDriftwave | null = null;
  private listeners: { [key: string]: Function[] } = {};

  private constructor() {}

  static async create(): Promise<Driftwave> {
    await init();
    const instance = new Driftwave();
    instance.wasm = new WasmDriftwave();
    return instance;
  }

  async load(url: string): Promise<void> {
    if (!this.wasm) return;

    try {
      await this.wasm.load_async(url);
      this.emit('ready');
    } catch (error) {
      this.emit('error', error);
      throw error;
    }
  }

  play(): void {
    if (!this.wasm) return;
    this.wasm.play();
    this.emit('play');
  }

  playFrom(startFrame: number): void {
    if (!this.wasm) return;
    this.wasm.play_from(startFrame);
    this.emit('play');
  }

  playRange(startFrame: number, endFrame: number): void {
    if (!this.wasm) return;
    this.wasm.play_range(startFrame, endFrame);
    this.emit('play');
  }

  pause(): number | null {
    if (!this.wasm) return null;
    const frame = this.wasm.pause();
    this.emit('pause', frame);
    return frame;
  }

  isPlaying(): boolean {
    if (!this.wasm) return false;
    return this.wasm.is_playing();
  }

  getMetadata(): Metadata | null {
    if (!this.wasm) return null;
    try {
      return this.wasm.get_metadata();
    } catch {
      return null;
    }
  }

  on(event: string, callback: Function): void {
    if (!this.listeners[event]) this.listeners[event] = [];
    this.listeners[event].push(callback);
  }

  off(event: string, callback?: Function): void {
    if (!this.listeners[event]) return;
    if (callback) {
      this.listeners[event] = this.listeners[event].filter(cb => cb !== callback);
    } else {
      delete this.listeners[event];
    }
  }

  private emit(event: string, ...args: any[]): void {
    if (!this.listeners[event]) return;
    this.listeners[event].forEach(callback => callback(...args));
  }
}

export default Driftwave;