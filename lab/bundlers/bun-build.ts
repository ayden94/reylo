import porg from "porg/esbuild";
const r = await Bun.build({ entrypoints: ["main.ts"], format: "esm", target: "bun", external: ["porg", "postgres"], plugins: [porg()], outdir: "dist/bun", naming: "main.js" });
console.log("bun build:", r.success, r.logs.length ? r.logs : "");
