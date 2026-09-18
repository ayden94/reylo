import * as esbuild from "esbuild";
import lando from "lando/esbuild";
await esbuild.build({ entryPoints: ["main.ts"], bundle: true, format: "esm", platform: "node", external: ["lando", "postgres"], plugins: [lando()], outfile: "dist/esbuild/main.js" });
console.log("esbuild ok");
