import dotsql from "dotsql/vite";
export default { input: "main.ts", external: ["dotsql", "postgres"], plugins: [dotsql()], output: { dir: "dist/rolldown", format: "es", entryFileNames: "main.js" } };
