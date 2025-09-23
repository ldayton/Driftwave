import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait()
  ],
  server: {
    fs: {
      // Allow serving files from one level up to access wasm files
      allow: ['..']
    }
  },
  build: {
    lib: {
      entry: 'src/index.ts',
      name: 'Driftwave',
      fileName: 'driftwave'
    }
  }
});