import reylo from "reylo/esbuild";
const r = await Bun.build({ entrypoints: ["main.ts"], format: "esm", target: "bun", external: ["reylo", "postgres"], plugins: [reylo()], outdir: "dist/bun", naming: "main.js" });
console.log("bun build:", r.success, r.logs.length ? r.logs : "");
