import { defineConfig } from "vite";
import reylo from "reylo/vite";
export default defineConfig({
  plugins: [reylo()],
  build: { ssr: "main.ts", outDir: "dist/vite", rollupOptions: { external: ["reylo", "postgres"], output: { format: "es", entryFileNames: "main.js" } } },
});
