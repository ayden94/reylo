import motherload from "motherload/vite";
export default { input: "main.mjs", external: ["motherload", "postgres"], plugins: [motherload()], output: { dir: "dist/rollup", format: "es", entryFileNames: "main.js" } };
