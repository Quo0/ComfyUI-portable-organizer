import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

// @ts-expect-error process
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [vue()],
  clearScreen: false,

  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: 'ws', host, port: 1421 } : undefined,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  build: {
    target: 'chrome105',
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },

  envPrefix: ['VITE_', 'TAURI_ENV_*'],
});
