import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig({
  plugins: [sveltekit()],

  build: {
    // Tauri uses a modern WebView — no need for broad browser compat
    target: ["es2021", "chrome97", "safari15"],
    // Smaller output
    cssMinify: "lightningcss",
    rollupOptions: {
      output: {
        manualChunks: {
          // Split heavy JSON data out of the main page chunk
          // so the UI framework code parses without waiting on data
          "calendar-data": [
            "../../data/holidays/lunar-festivals.json",
            "../../data/holidays/solar-holidays.json",
            "../../data/tiet-khi.json",
            "../../data/canchi.json",
          ],
        },
      },
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
});
