// One preload for every runtime: `node --import lando/register`,
// `nub --import lando/register`, `bun --preload lando/register` (or bunfig.toml).
// Bun exposes Bun.plugin; Node and nub expose module.registerHooks.
import { readFileSync } from "node:fs";
import { sqlModuleSource } from "./loader.ts";

declare const Bun: undefined | { plugin(p: { name: string; setup(build: { onLoad(o: { filter: RegExp }, cb: (args: { path: string }) => { loader: string; contents: string }): void }): void }): void };

if (typeof Bun !== "undefined") {
  Bun.plugin({
    name: "lando-sql",
    setup(build) {
      build.onLoad({ filter: /\.sql$/ }, (args) => ({ loader: "js", contents: sqlModuleSource(readFileSync(args.path, "utf8"), args.path) }));
    },
  });
} else {
  const { registerHooks } = await import("node:module");
  const { fileURLToPath } = await import("node:url");
  registerHooks({
    load(url, context, nextLoad) {
      if (!url.endsWith(".sql")) return nextLoad(url, context);
      const path = fileURLToPath(url);
      return { format: "module", source: sqlModuleSource(readFileSync(path, "utf8"), path), shortCircuit: true };
    },
  });
}
