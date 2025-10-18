import path from "path";
import { defineConfig } from "vite";

// Vite config for audio-capture demo development
export default defineConfig({
  // Clear screen for better debugging
  clearScreen: false,

  server: {
    port: 5174, // Different port from main app (5173)
    strictPort: false,
    host: false,
    watch: {
      ignored: ["*/src-tauri/**", "*/dist/**", "*/node_modules/**"],
    },
  },

  // Point to audio-capture directory as root
  root: path.resolve(__dirname, "."),

  build: {
    outDir: "./dist",
  },

  resolve: {
    alias: {
      "@audio-player": path.resolve(__dirname, "../src-audio-player/src"),
      "@audio-capture": path.resolve(__dirname, "../src-audio-capture/src"),
      "@ui": path.resolve(__dirname, "./src"),
    },
  },
});
