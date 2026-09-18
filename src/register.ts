// One preload for every runtime: `node --import lando/register`,
// `nub --import lando/register`, `bun --preload lando/register` (or bunfig.toml).
// Bun exposes Bun.plugin; Node and nub expose module.registerHooks.
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { lexPlaceholders } from "./lex.ts";

type CacheEntry = { hash: string; params: string[]; cardinality: string; dialect?: string };

// The mapper writes <projectDir>/.lando/queries.json when tsc runs; the loader
// uses it when the entry matches the file's current content, and otherwise falls
// back to what it can derive from the text alone.
function cachedEntry(path: string, sql: string): CacheEntry | undefined {
  for (let dir = dirname(path); ; dir = dirname(dir)) {
    const cache = join(dir, ".lando", "queries.json");
    if (existsSync(cache)) {
      const entry = (JSON.parse(readFileSync(cache, "utf8")) as Record<string, CacheEntry>)[relative(dir, path)];
      if (entry && entry.hash === sha1(sql)) return entry;
      return undefined;
    }
    if (dirname(dir) === dir) return undefined;
  }
}

function sha1(text: string): string {
  // Loaded lazily so the Bun branch does not pay for node:crypto at startup.
  return createHashSync(text);
}
let createHashSync = (text: string): string => {
  const { createHash } = requireCrypto();
  createHashSync = (t: string) => createHash("sha1").update(t).digest("hex");
  return createHashSync(text);
};
function requireCrypto(): { createHash(alg: string): { update(t: string): { digest(enc: "hex"): string } } } {
  return process.getBuiltinModule("node:crypto") as never;
}

function moduleSource(sql: string, path: string): string {
  const cached = cachedEntry(path, sql);
  const params = cached?.params ?? lexPlaceholders(sql).params;
  const cardinality = cached?.cardinality ?? /^\s*--\s*@returns\s+(\S+)/m.exec(sql)?.[1] ?? "many";
  return [
    `import { defineQuery } from "lando";`,
    `export const sql = ${JSON.stringify(sql)};`,
    `export const params = ${JSON.stringify(params)};`,
    `export default defineQuery({ dialect: ${JSON.stringify(cached?.dialect ?? "postgres")}, sql, params, cardinality: ${JSON.stringify(cardinality)} });`,
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
