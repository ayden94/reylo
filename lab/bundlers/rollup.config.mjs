import reylo from "reylo/vite";
export default { input: "main.mjs", external: ["reylo", "postgres"], plugins: [reylo()], output: { dir: "dist/rollup", format: "es", entryFileNames: "main.js" } };
