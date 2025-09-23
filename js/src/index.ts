import init, { Driftwave as WasmDriftwave } from 'driftwave-web';

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

  load(url: string): void {
    if (!this.wasm) return;

    fetch(url)
      .then(response => response.arrayBuffer())
      .then(arrayBuffer => {
        if (!this.wasm) return;
        return this.wasm.decode_audio_data(arrayBuffer);
      })
      .then(audioBuffer => {
        if (!this.wasm) return;
        this.wasm.set_buffer(audioBuffer);
        this.emit('ready');
      });
  }

  play(): void {
    if (!this.wasm) return;
    this.wasm.play();
  }

  on(event: string, callback: Function): void {
    if (!this.listeners[event]) this.listeners[event] = [];
    this.listeners[event].push(callback);
  }

  private emit(event: string): void {
    if (!this.listeners[event]) return;
    this.listeners[event].forEach(callback => callback());
  }
}

export default Driftwave;