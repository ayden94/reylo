// What every runtime hook emits for a .sql file: a module that builds the query
// object from the text, the mapper's cache when it matches, or the directives.
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { lexPlaceholders } from "./lex.ts";

type CacheEntry = { hash: string; params: string[]; cardinality: string; dialect?: string; text?: string; order?: string[] };

function cachedEntry(path: string, sql: string): CacheEntry | undefined {
  for (let dir = dirname(path); ; dir = dirname(dir)) {
    const cache = join(dir, ".lando", "queries.json");
    if (existsSync(cache)) {
      const entry = (JSON.parse(readFileSync(cache, "utf8")) as Record<string, CacheEntry>)[relative(dir, path)];
      return entry && entry.hash === sha1(sql) ? entry : undefined;
    }
    if (dirname(dir) === dir) return undefined;
  }
}

function sha1(text: string): string {
  const { createHash } = process.getBuiltinModule("node:crypto") as typeof import("node:crypto");
  return createHash("sha1").update(text).digest("hex");
}

export function sqlModuleSource(sql: string, path: string): string {
  const cached = cachedEntry(path, sql);
  const params = cached?.params ?? lexPlaceholders(sql).params;
  const cardinality = cached?.cardinality ?? /^\s*--\s*@returns\s+(\S+)/m.exec(sql)?.[1] ?? "many";
  const dialect = cached?.dialect ?? /^\s*--\s*@dialect\s+(\S+)/m.exec(sql)?.[1] ?? "postgres";
  const rewritten = cached?.text && cached.order ? `, text: ${JSON.stringify(cached.text)}, order: ${JSON.stringify(cached.order)}` : "";
  return [
    `import { defineQuery } from "lando";`,
    `export const sql = ${JSON.stringify(sql)};`,
    `export const params = ${JSON.stringify(params)};`,
    `export default defineQuery({ dialect: ${JSON.stringify(dialect)}, sql, params, cardinality: ${JSON.stringify(cardinality)}${rewritten} });`,
  ].join("\n");
}
