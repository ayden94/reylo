// A webpack-style loader, for webpack 5, Rspack and Next.js with Turbopack, which take loaders
// (not plugins) for a file type:
//   webpack / Rspack:  module.rules: [{ test: /\.sql$/, use: "porg/webpack-loader" }]
//   Turbopack:         turbopack: { rules: { "*.sql": { loaders: ["porg/webpack-loader"], as: "*.js" } } }
import { cacheFile, sqlModuleSource } from "./loader.ts";

export default function porgSqlLoader(this: { resourcePath: string; addDependency(file: string): void }, source: string): string {
  // The module carries the mapper's rewritten statement, so a new cache must rebuild it.
  const cache = cacheFile(this.resourcePath);
  if (cache) this.addDependency(cache);
  return sqlModuleSource(source, this.resourcePath);
}
