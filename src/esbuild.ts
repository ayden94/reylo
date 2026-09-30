// The esbuild plugin, which is also a Bun plugin: `Bun.plugin` (in a `--preload` file or
// `bunfig.toml`) and `Bun.build({ plugins })` take the same `setup(build)` object, with the
// same `onLoad` filter and the same `{ contents, loader }` result.
import { readFileSync } from "node:fs";
import { cacheFile, sqlModuleSource } from "./loader.ts";

type Build = {
  onLoad(options: { filter: RegExp }, callback: (args: { path: string }) => { contents: string; loader: "js"; watchFiles?: string[] }): void;
};

export default function porg() {
  return {
    name: "porg",
    setup(build: Build) {
      // The module carries the mapper's rewritten statement, so in watch mode a new cache rebuilds it.
      build.onLoad({ filter: /\.sql$/ }, (args) => {
        const cache = cacheFile(args.path);
        return { contents: sqlModuleSource(readFileSync(args.path, "utf8"), args.path), loader: "js", ...(cache ? { watchFiles: [cache] } : {}) };
      });
    },
  };
}
