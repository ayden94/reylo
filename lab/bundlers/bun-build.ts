import lando from "lando/esbuild";
const r = await Bun.build({ entrypoints: ["main.ts"], format: "esm", target: "bun", external: ["lando", "postgres"], plugins: [lando()], outdir: "dist/bun", naming: "main.js" });
console.log("bun build:", r.success, r.logs.length ? r.logs : "");
