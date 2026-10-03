import dotsql from "dotsql/esbuild";
const r = await Bun.build({ entrypoints: ["main.ts"], format: "esm", target: "bun", external: ["dotsql", "postgres"], plugins: [dotsql()], outdir: "dist/bun", naming: "main.js" });
console.log("bun build:", r.success, r.logs.length ? r.logs : "");
