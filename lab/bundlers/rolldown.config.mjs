import motherload from "motherload/vite";
export default { input: "main.ts", external: ["motherload", "postgres"], plugins: [motherload()], output: { dir: "dist/rolldown", format: "es", entryFileNames: "main.js" } };
