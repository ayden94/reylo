// A plain webpack-style loader for Next.js with Turbopack, which takes loaders
// (not plugins) through `turbopack.rules`:
//   turbopack: { rules: { "*.sql": { loaders: ["lando/turbopack-loader"], as: "*.js" } } }
import { sqlModuleSource } from "./loader.ts";

export default function landoSqlLoader(this: { resourcePath: string }, source: string): string {
  return sqlModuleSource(source, this.resourcePath);
}
