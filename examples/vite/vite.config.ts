import { reactRouter } from "@react-router/dev/vite";
import { defineConfig } from "vite";
import reylo from "reylo/vite";

export default defineConfig({
  plugins: [reylo(), reactRouter()],
});
