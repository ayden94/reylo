import lando from "lando/vite";
export default { input: "main.ts", external: ["lando", "postgres"], plugins: [lando()], output: { dir: "dist/rolldown", format: "es", entryFileNames: "main.js" } };
