// A webpack-style loader, for webpack 5, Rspack and Next.js with Turbopack, which take loaders
// (not plugins) for a file type:
//   webpack / Rspack:  module.rules: [{ test: /\.sql$/, use: "motherload/webpack-loader" }]
//   Turbopack:         turbopack: { rules: { "*.sql": { loaders: ["motherload/webpack-loader"], as: "*.js" } } }
import { sqlModule } from "./loader.ts";

type LoaderContext = {
  resourcePath: string;
  addDependency(file: string): void;
  async(): (error: Error | null, code?: string) => void;
};

export default function porgSqlLoader(this: LoaderContext, source: string): void {
  const done = this.async();
  sqlModule(this.resourcePath, source).then(
    ({ code, entry }) => {
      // A new analysis of the statement (the type check's, after a migration) rebuilds the module.
      this.addDependency(entry);
      done(null, code);
    },
    (error: Error) => done(error),
  );
}
