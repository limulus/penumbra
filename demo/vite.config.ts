import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],

  // Vite's dev server needs to serve files from the parent dist folder
  server: {
    host: '0.0.0.0',
    allowedHosts: ['.local'],
    fs: {
      allow: ['..'],
    },
    // Required for SharedArrayBuffer
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Resource-Policy': 'same-origin',
    },
  },
})
