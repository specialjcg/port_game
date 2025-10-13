import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';
import path from 'path';

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
    react()
  ],
  optimizeDeps: {
    exclude: ['port_game']
  },
  server: {
    fs: {
      allow: ['..']
    }
  },
  resolve: {
    alias: {
      'port_game': path.resolve(__dirname, '../pkg')
    }
  },
  build: {
    target: 'esnext',
    outDir: 'dist',
    sourcemap: true
  }
});
