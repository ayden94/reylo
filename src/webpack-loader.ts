// A webpack-style loader, for webpack 5, Rspack and Next.js with Turbopack, which take loaders
// (not plugins) for a file type:
//   webpack / Rspack:  module.rules: [{ test: /\.sql$/, use: "lando/webpack-loader" }]
//   Turbopack:         turbopack: { rules: { "*.sql": { loaders: ["lando/webpack-loader"], as: "*.js" } } }
import { sqlModuleSource } from "./loader.ts";

export default function landoSqlLoader(this: { resourcePath: string }, source: string): string {
  return sqlModuleSource(source, this.resourcePath);
}
