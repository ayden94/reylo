import { defineConfig } from "vite";
import motherload from "motherload/vite";
export default defineConfig({
  plugins: [motherload()],
  build: { ssr: "main.ts", outDir: "dist/vite", rollupOptions: { external: ["motherload", "postgres"], output: { format: "es", entryFileNames: "main.js" } } },
});
