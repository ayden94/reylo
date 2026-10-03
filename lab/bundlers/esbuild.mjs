import * as esbuild from "esbuild";
import dotsql from "dotsql/esbuild";
await esbuild.build({ entryPoints: ["main.ts"], bundle: true, format: "esm", platform: "node", external: ["dotsql", "postgres"], plugins: [dotsql()], outfile: "dist/esbuild/main.js" });
console.log("esbuild ok");
