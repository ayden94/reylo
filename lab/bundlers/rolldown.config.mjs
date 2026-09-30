import porg from "porg/vite";
export default { input: "main.ts", external: ["porg", "postgres"], plugins: [porg()], output: { dir: "dist/rolldown", format: "es", entryFileNames: "main.js" } };
