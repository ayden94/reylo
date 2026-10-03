import dotsql from "dotsql/vite";
export default { input: "main.mjs", external: ["dotsql", "postgres"], plugins: [dotsql()], output: { dir: "dist/rollup", format: "es", entryFileNames: "main.js" } };
