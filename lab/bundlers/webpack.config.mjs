import { fileURLToPath } from "node:url";
export default {
  mode: "development", target: "node", devtool: false,
  entry: "./main.mjs",
  experiments: { outputModule: true, topLevelAwait: true },
  output: { path: fileURLToPath(new URL("./dist/webpack", import.meta.url)), filename: "main.js", module: true, chunkFormat: "module" },
  externalsType: "module", externals: { reylo: "reylo", postgres: "postgres" },
  module: { rules: [{ test: /\.sql$/, use: "reylo/webpack-loader" }] },
};
