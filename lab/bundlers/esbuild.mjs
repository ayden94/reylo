import * as esbuild from "esbuild";
import motherload from "motherload/esbuild";
await esbuild.build({ entryPoints: ["main.ts"], bundle: true, format: "esm", platform: "node", external: ["motherload", "postgres"], plugins: [motherload()], outfile: "dist/esbuild/main.js" });
console.log("esbuild ok");
