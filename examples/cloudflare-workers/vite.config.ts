import { cloudflare } from "@cloudflare/vite-plugin";
import { defineConfig } from "vite";
import reylo from "reylo/vite";

export default defineConfig({
  plugins: [reylo(), cloudflare()],
});
