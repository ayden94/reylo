// One preload for every runtime: `node --import lando/register`,
// `nub --import lando/register`, `bun --preload lando/register` (or bunfig.toml).
// Bun exposes Bun.plugin; Node and nub expose module.registerHooks.
import { readFileSync } from "node:fs";
import { lexPlaceholders } from "./lex.ts";

function moduleSource(sql: string): string {
  const { params } = lexPlaceholders(sql);
  return [
    `import { defineQuery } from "lando";`,
    `export const sql = ${JSON.stringify(sql)};`,
    `export default defineQuery({ dialect: "postgres", sql, params: ${JSON.stringify(params)} });`,
  ].join("\n");
}

declare const Bun: undefined | { plugin(p: { name: string; setup(build: { onLoad(o: { filter: RegExp }, cb: (args: { path: string }) => { loader: string; contents: string }): void }): void }): void };

if (typeof Bun !== "undefined") {
  Bun.plugin({
    name: "lando-sql",
    setup(build) {
      build.onLoad({ filter: /\.sql$/ }, (args) => ({ loader: "js", contents: moduleSource(readFileSync(args.path, "utf8")) }));
    },
  });
} else {
  const { registerHooks } = await import("node:module");
  const { fileURLToPath } = await import("node:url");
  registerHooks({
    load(url, context, nextLoad) {
      if (!url.endsWith(".sql")) return nextLoad(url, context);
      return { format: "module", source: moduleSource(readFileSync(fileURLToPath(url), "utf8")), shortCircuit: true };
    },
  });
}
