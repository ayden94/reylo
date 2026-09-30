import porg from "porg/vite";
export default { input: "main.mjs", external: ["porg", "postgres"], plugins: [porg()], output: { dir: "dist/rollup", format: "es", entryFileNames: "main.js" } };
