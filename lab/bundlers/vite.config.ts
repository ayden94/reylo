import { defineConfig } from "vite";
import porg from "porg/vite";
export default defineConfig({
  plugins: [porg()],
  build: { ssr: "main.ts", outDir: "dist/vite", rollupOptions: { external: ["porg", "postgres"], output: { format: "es", entryFileNames: "main.js" } } },
});
