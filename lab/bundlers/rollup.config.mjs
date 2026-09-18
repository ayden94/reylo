import lando from "lando/vite";
export default { input: "main.mjs", external: ["lando", "postgres"], plugins: [lando()], output: { dir: "dist/rollup", format: "es", entryFileNames: "main.js" } };
