// The esbuild plugin, which is also a Bun plugin: `Bun.build({ plugins })` and `Bun.plugin` take
// the same `setup(build)` object, with the same `onLoad` filter and the same `{ contents, loader }`
// result. Bun's runtime plugin answers `import` through it; `require()` needs a synchronous
// answer, which `motherload/register` gives.
import { readFileSync } from "node:fs";
import { releaseConnections } from "./analyze.ts";
import { sqlModule } from "./loader.ts";

type Build = {
  onLoad(options: { filter: RegExp }, callback: (args: { path: string }) => Promise<{ contents: string; loader: "js"; watchFiles?: string[] }>): void;
  /** esbuild's and Bun.build's; Bun's runtime plugin has none. */
  onEnd?(callback: () => Promise<void>): void;
};

export default function motherload() {
  return {
    name: "motherload",
    setup(build: Build) {
      build.onLoad({ filter: /\.sql$/ }, async (args) => {
        const { code, entry } = await sqlModule(args.path, readFileSync(args.path, "utf8"));
        // In watch mode a new analysis of the statement (the type check's, after a migration) rebuilds the module.
        return { contents: code, loader: "js", watchFiles: [entry] };
      });
      build.onEnd?.(releaseConnections);
    },
  };
}
