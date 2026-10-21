import * as esbuild from "esbuild";
import reylo from "reylo/esbuild";
await esbuild.build({ entryPoints: ["main.ts"], bundle: true, format: "esm", platform: "node", external: ["reylo", "postgres"], plugins: [reylo()], outfile: "dist/esbuild/main.js" });
console.log("esbuild ok");
