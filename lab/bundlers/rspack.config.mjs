import { fileURLToPath } from "node:url";
export default {
  mode: "development", target: "node", devtool: false,
  entry: "./main.mjs",
  experiments: { outputModule: true },
  output: { path: fileURLToPath(new URL("./dist/rspack", import.meta.url)), filename: "main.js", module: true, chunkFormat: "module" },
  externalsType: "module", externals: { dotsql: "dotsql", postgres: "postgres" },
  module: { rules: [{ test: /\.sql$/, use: "dotsql/webpack-loader" }] },
};
