import reylo from "reylo/vite";
export default { input: "main.ts", external: ["reylo", "postgres"], plugins: [reylo()], output: { dir: "dist/rolldown", format: "es", entryFileNames: "main.js" } };
