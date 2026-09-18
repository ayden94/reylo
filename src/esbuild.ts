// The esbuild plugin, which is also a Bun plugin: `Bun.plugin` (in a `--preload` file or
// `bunfig.toml`) and `Bun.build({ plugins })` take the same `setup(build)` object, with the
// same `onLoad` filter and the same `{ contents, loader }` result.
import { readFileSync } from "node:fs";
import { sqlModuleSource } from "./loader.ts";

type Build = {
  onLoad(options: { filter: RegExp }, callback: (args: { path: string }) => { contents: string; loader: "js" }): void;
};

export default function lando() {
  return {
    name: "lando",
    setup(build: Build) {
      build.onLoad({ filter: /\.sql$/ }, (args) => ({ contents: sqlModuleSource(readFileSync(args.path, "utf8"), args.path), loader: "js" }));
    },
  };
}
