import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

// @ts-expect-error process — глобал Node
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [vue()],

  // Иначе Vite затирает вывод компилятора Rust, и ошибки сборки не увидеть.
  clearScreen: false,

  server: {
    // Порт фиксирован: на него смотрит devUrl в tauri.conf.json.
    // strictPort обязателен — молчаливый переезд на 1421 оставил бы
    // окно приложения смотреть в пустоту.
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: 'ws', host, port: 1421 } : undefined,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },

  // WebView2 вечнозелёный, поэтому транспиляция до древних браузеров
  // только раздувает бандл.
  build: {
    target: 'chrome105',
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },

  envPrefix: ['VITE_', 'TAURI_ENV_*'],
});
