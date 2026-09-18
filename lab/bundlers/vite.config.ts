import { defineConfig } from "vite";
import lando from "lando/vite";
export default defineConfig({
  plugins: [lando()],
  build: { ssr: "main.ts", outDir: "dist/vite", rollupOptions: { external: ["lando", "postgres"], output: { format: "es", entryFileNames: "main.js" } } },
});
