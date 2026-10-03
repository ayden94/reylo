import { defineConfig } from "vite";
import dotsql from "dotsql/vite";
export default defineConfig({
  plugins: [dotsql()],
  build: { ssr: "main.ts", outDir: "dist/vite", rollupOptions: { external: ["dotsql", "postgres"], output: { format: "es", entryFileNames: "main.js" } } },
});
