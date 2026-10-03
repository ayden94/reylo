import motherload from "motherload/esbuild";
const r = await Bun.build({ entrypoints: ["main.ts"], format: "esm", target: "bun", external: ["motherload", "postgres"], plugins: [motherload()], outdir: "dist/bun", naming: "main.js" });
console.log("bun build:", r.success, r.logs.length ? r.logs : "");
