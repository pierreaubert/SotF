import { defineConfig } from "vitest/config";
import path from "path";

export default defineConfig({
  test: {
    environment: "jsdom",
    setupFiles: ["./src/tests/test-setup.ts"],
    globals: true,
    include: [
      "./src/**/*.{test,spec}.{js,ts}",
    ],
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html"],
      exclude: [
        "node_modules/",
        "./src/tests/test-setup.ts",
        "./src/**/*.d.ts",
        "./src/**/*.config.*",
        "dist/"
      ],
    },
  },
  resolve: {
    alias: {
      "@": "/src-ui",
      "@audio-player": path.resolve(__dirname, "../src-audio-player/src"),
      "@audio-capture": path.resolve(__dirname, "../src-audio-capture/src"),
      "@ui": path.resolve(__dirname, "./src"),
    },
  },
});
