// What every runtime hook emits for a .sql file: a module that builds the query
// object from the text and the mapper's cache when it matches.
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { lexPlaceholders } from "./lex.ts";

type CacheEntry = { hash: string; params: string[]; cardinality: string; dialect?: string; text?: string; order?: string[]; convert?: Record<string, string>; dates?: string; bigint?: string };

/** The mapper's cache that covers a `.sql` file: the nearest `.porg/queries.json` at or above it. A bundler adapter watches it, so that a module built before the mapper rewrote the statement is built again. */
export function cacheFile(path: string): string | undefined {
  for (let dir = dirname(path); ; dir = dirname(dir)) {
    const cache = join(dir, ".porg", "queries.json");
    if (existsSync(cache)) return cache;
    if (dirname(dir) === dir) return undefined;
  }
}

function cachedEntry(path: string, sql: string): CacheEntry | undefined {
  const cache = cacheFile(path);
  if (!cache) return undefined;
  let entries: Record<string, CacheEntry>;
  try {
    entries = JSON.parse(readFileSync(cache, "utf8"));
  } catch {
    return undefined;
  }
  const entry = entries[relative(dirname(dirname(cache)), path)];
  return entry && entry.hash === sha1(sql) ? entry : undefined;
}

function sha1(text: string): string {
  const { createHash } = process.getBuiltinModule("node:crypto") as typeof import("node:crypto");
  return createHash("sha1").update(text).digest("hex");
}

export function sqlModuleSource(sql: string, path: string): string {
  const cached = cachedEntry(path, sql);
  const dialect = (cached?.dialect ?? "postgres") as "postgres" | "mysql" | "sqlite";
  const params = cached?.params ?? lexPlaceholders(sql, dialect).params;
  const cardinality = cached?.cardinality ?? "many";
  const rewritten = cached?.text && cached.order ? `, text: ${JSON.stringify(cached.text)}, order: ${JSON.stringify(cached.order)}` : "";
  const converted = cached?.convert ? `, convert: ${JSON.stringify(cached.convert)}, dates: ${JSON.stringify(cached.dates ?? "string")}, bigint: ${JSON.stringify(cached.bigint ?? "string")}` : "";
  return [
    `import { defineQuery } from "porg";`,
    `export const sql = ${JSON.stringify(sql)};`,
    `export const params = ${JSON.stringify(params)};`,
    `export default defineQuery({ dialect: ${JSON.stringify(dialect)}, sql, params, cardinality: ${JSON.stringify(cardinality)}${rewritten}${converted} });`,
  ].join("\n");
}
