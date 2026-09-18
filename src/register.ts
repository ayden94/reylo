// One preload for every runtime: `node --import lando/register`,
// `nub --import lando/register`, `bun --preload lando/register` (or bunfig.toml).
// Bun exposes Bun.plugin; Node and nub expose module.registerHooks.
import { readFileSync } from "node:fs";

function moduleSource(sql: string, path: string): string {
  return [
    `export const sql = ${JSON.stringify(sql)};`,
    `export const probe = ${JSON.stringify({ runtime: typeof Bun === "undefined" ? "node-registerHooks" : "bun-plugin", path })};`,
    `export default { sql };`,
  ].join("\n");
}

declare const Bun: undefined | { plugin(p: { name: string; setup(build: { onLoad(o: { filter: RegExp }, cb: (args: { path: string }) => { loader: string; contents: string }): void }): void }): void };

if (typeof Bun !== "undefined") {
  Bun.plugin({
    name: "lando-sql",
    setup(build) {
      build.onLoad({ filter: /\.sql$/ }, (args) => ({ loader: "js", contents: moduleSource(readFileSync(args.path, "utf8"), args.path) }));
    },
  });
} else {
  const { registerHooks } = await import("node:module");
  const { fileURLToPath } = await import("node:url");
  registerHooks({
    load(url, context, nextLoad) {
      if (!url.endsWith(".sql")) return nextLoad(url, context);
      const path = fileURLToPath(url);
      return { format: "module", source: moduleSource(readFileSync(path, "utf8"), path), shortCircuit: true };
    },
  });
}
