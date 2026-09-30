import * as esbuild from "esbuild";
import porg from "porg/esbuild";
await esbuild.build({ entryPoints: ["main.ts"], bundle: true, format: "esm", platform: "node", external: ["porg", "postgres"], plugins: [porg()], outfile: "dist/esbuild/main.js" });
console.log("esbuild ok");
