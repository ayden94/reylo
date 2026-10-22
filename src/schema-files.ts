// The type check without a database: `schema` in `.reylo.yaml` names SQL files (a schema dump, a
// directory of migrations), which Reylo loads into a database of its own, in process, and analyzes
// against as it would against the project's: Postgres on PGlite, SQLite in memory through
// node:sqlite. Nothing connects to `direct_url`. MySQL has no database that runs in process, so a
// MySQL project reads a server (wiki/roadmap.md, wiki/project-config.md section 9).
//
// The files are read as the databases' own tools write them: a `pg_dump --schema-only` (its
// `\restrict` lines, which no SQL loader reads, are dropped, and every setting it changes is reset
// after each file), SQLite's `.schema` (its `sqlite_sequence` table, which SQLite makes itself, is
// dropped), and migrations of the common tools, in the order each tool applies them (`ordered`): a
// down or undo file is skipped, and a file is cut where dbmate's, goose's, sql-migrate's,
// node-pg-migrate's or tern's down section starts.
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { basename, dirname, join, relative, sep } from "node:path";
import { globSync, isDynamicPattern } from "tinyglobby";
import { loadDriver, type Described } from "./describe.ts";
import type { Dialect } from "./project.ts";

/** A schema the project's `schema` names, or why it cannot be read. */
export type SchemaFiles = { files: string[]; error?: undefined } | { files: string[]; error: string };

/**
 * The files `schema` names, in order: a path where the list names it, and a glob's files in the
 * order their migration tool applies them (`ordered`); a file named twice is taken once, where it
 * first appears. A down or undo migration is never applied.
 */
export function schemaFiles(root: string, patterns: readonly string[]): SchemaFiles {
  const files: string[] = [];
  for (const pattern of patterns) {
    if (!isDynamicPattern(pattern)) {
      const file = join(root, pattern);
      if (!existsSync(file)) return { files, error: `"schema" in .reylo.yaml names ${pattern}, which does not exist` };
      if (!files.includes(file)) files.push(file);
      continue;
    }
    const found = globSync(pattern, { cwd: root, absolute: true, onlyFiles: true, ignore: ["**/node_modules/**"] }).filter((f) => !/\.down\.sql$/i.test(f) && !FLYWAY_UNDO.test(basename(f)));
    if (!found.length) return { files, error: `"schema" in .reylo.yaml names ${pattern}, which matches no file` };
    const order = ordered(root, found);
    if ("error" in order) return { files, error: order.error };
    for (const file of order.files) if (!files.includes(file)) files.push(file);
  }
  return { files };
}

// How the migration tools order their files, each run on Postgres 18 on 2026-10-22 with files
// numbered 1, 2 and 10. With versions of one width (the timestamps or zero-padded numbers the tools
// write) every tool applies them in the order of their names. With versions of several widths they
// disagree: golang-migrate, goose, sqlx and node-pg-migrate go by number (1, 2, 10), dbmate and
// Atlas by name (10, 1, 2), Supabase and Prisma by the version's text (1, 10, 2); Flyway goes by
// version, `V1_1` after `V1`, its repeatable migrations last. So a glob's files are in name order
// unless their numbers say otherwise, and then in the order of the tool the files show, or refused.
const FLYWAY_VERSIONED = /^V(\d+(?:[._]\d+)*)__.*\.sql$/i;
const FLYWAY_REPEATABLE = /^R__.*\.sql$/i;
const FLYWAY_UNDO = /^U\d+(?:[._]\d+)*__.*\.sql$/i;
const BY_NUMBER = /^[ \t]*--[ \t]*(?:\+goose[ \t]+up|up[ \t]+migration)\b/im;
const BY_NAME = /^[ \t]*--[ \t]*migrate:up\b/im;

/** `files` (one glob's) in the order their tool applies them, or why that cannot be told. */
function ordered(root: string, files: string[]): { files: string[] } | { error: string } {
  const names = files.map((f) => basename(f));
  if (names.every((n) => FLYWAY_VERSIONED.test(n) || FLYWAY_REPEATABLE.test(n))) {
    const version = (n: string) => FLYWAY_VERSIONED.exec(n)?.[1]?.split(/[._]/).map(BigInt);
    return {
      files: [...files].sort((a, b) => {
        const x = version(basename(a));
        const y = version(basename(b));
        if (!x || !y) return x ? -1 : y ? 1 : byName(a, b);
        for (let i = 0; i < Math.max(x.length, y.length); i++) {
          const d = (x[i] ?? 0n) - (y[i] ?? 0n);
          if (d !== 0n) return d < 0n ? -1 : 1;
        }
        return byName(a, b);
      }),
    };
  }
  const byPath = [...files].sort((a, b) => byName(relative(root, a), relative(root, b)));
  // The version is the leading number of the first part of the path that starts with one: the
  // file's name, or Prisma's directory.
  const version = (f: string) => relative(root, f).split(sep).map((part) => /^\d+/.exec(part)?.[0]).find((v) => v !== undefined);
  const versions = byPath.map(version);
  if (versions.some((v) => v === undefined) || new Set(versions.map((v) => v!.length)).size === 1) return { files: byPath };
  const numeric = [...byPath].sort((a, b) => {
    const d = BigInt(version(a)!) - BigInt(version(b)!);
    return d === 0n ? 0 : d < 0n ? -1 : 1;
  });
  if (numeric.every((f, i) => f === byPath[i])) return { files: byPath };
  const texts = files.map((f) => readFileSync(f, "utf8"));
  const dir = dirname(files[0]!);
  if (files.some((f) => /\.up\.sql$/i.test(f)) || texts.some((t) => BY_NUMBER.test(t))) return { files: numeric };
  if (texts.some((t) => BY_NAME.test(t)) || existsSync(join(dir, "atlas.sum"))) return { files: byPath };
  if (files.some((f) => /(^|[\\/])supabase[\\/]migrations[\\/]/.test(f)) || existsSync(join(dir, "..", "migration_lock.toml")) || existsSync(join(dir, "migration_lock.toml"))) {
    return { files: [...byPath].sort((a, b) => byName(version(a)!, version(b)!)) };
  }
  const i = numeric.findIndex((f, i) => f !== byPath[i]);
  const [first, second] = [byPath[i]!, numeric[i]!].map((f) => relative(root, f));
  return { error: `"schema" in .reylo.yaml: ${first} comes before ${second} by name and after it by number, and migration tools disagree on which runs first. Name the files with versions of one width (timestamps, or zero-padded numbers), or list them in .reylo.yaml in the order they run` };
}

/** Byte order, as a tool that sorts file names does. */
const byName = (a: string, b: string) => (a < b ? -1 : a > b ? 1 : 0);

/** Where a migration's down section starts: dbmate, goose, sql-migrate, node-pg-migrate, tern. */
const DOWN = /^[ \t]*--[ \t]*(?:migrate:down|\+goose[ \t]+down|\+migrate[ \t]+down|down[ \t]+migration)\b.*$|^[ \t]*----[ \t]*create above[ \t]*\/[ \t]*drop below[ \t]*----/im;

/**
 * A file's text as the database is given it. A line is blanked rather than removed, so that a
 * position the database reports is still the file's.
 */
export function schemaText(text: string, dialect: Dialect): string {
  const down = DOWN.exec(text);
  let out = down ? text.slice(0, down.index) : text;
  if (dialect === "postgres") out = out.replace(/^\\(?:un)?restrict\b.*$/gm, "");
  if (dialect === "sqlite") out = out.replace(/^[ \t]*create[ \t]+table[ \t]+(?:if[ \t]+not[ \t]+exists[ \t]+)?["'`[]?sqlite_sequence["'`\]]?[ \t]*\([^;]*\);?/gim, "");
  return out;
}

/**
 * The fingerprint of what an analysis reads: each file's name and text, and the database that
 * loads them (PGlite's version, or the SQLite of this Node), since another version may type a
 * statement another way.
 */
export function schemaFingerprint(root: string, files: readonly string[], dialect: Dialect): string {
  const hash = createHash("sha256").update(JSON.stringify([dialect, dialect === "sqlite" ? process.versions.sqlite : pgliteVersion(root)]));
  for (const file of files) hash.update(`\0${relative(root, file)}\0`).update(readFileSync(file));
  return hash.digest("hex").slice(0, 16);
}

function pgliteVersion(root: string): string | null {
  const path = resolveFrom(root, "@electric-sql/pglite/package.json");
  if (!path) return null;
  try {
    return JSON.parse(readFileSync(path, "utf8")).version ?? null;
  } catch {
    return null;
  }
}

function resolveFrom(root: string, name: string): string | undefined {
  try {
    return createRequire(join(root, "package.json")).resolve(name);
  } catch {
    return undefined;
  }
}

/** What the engine asks of the loaded schema, as of a database: describe a statement, run a catalog query. */
export interface SchemaDatabase {
  describe(sql: string): Promise<Described>;
  values(sql: string, params?: unknown[]): Promise<unknown[][]>;
  close(): Promise<void>;
}

/** Why the schema cannot be loaded, already naming the file and the place in it. */
export class SchemaError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "SchemaError";
  }
}

/** The files, loaded into a database of their own. */
export async function openSchemaDatabase(dialect: Dialect, root: string, files: readonly string[]): Promise<SchemaDatabase> {
  const texts = files.map((file) => ({ file, text: schemaText(readFileSync(file, "utf8"), dialect) }));
  const at = (file: string, text: string, position: number | undefined) => `${relative(root, file)}${position ? `:${place(text, position)}` : ""}`;
  if (dialect === "sqlite") return openSqlite(texts, at);
  if (dialect === "postgres") return openPglite(root, texts, at);
  throw new SchemaError(`"schema" in .reylo.yaml loads a Postgres or SQLite schema, and MySQL needs its server: remove schema, and set DATABASE_URL or direct_url to a MySQL 8.4 server with the same schema`);
}

type Located = (file: string, text: string, position: number | undefined) => string;

/** 1-based line and column of a 1-based character position, as Postgres reports one. */
function place(text: string, position: number): string {
  const lines = text.slice(0, position - 1).split("\n");
  return `${lines.length}:${lines.at(-1)!.length + 1}`;
}

async function openSqlite(texts: { file: string; text: string }[], at: Located): Promise<SchemaDatabase> {
  const { DatabaseSync } = (await import("node:sqlite")) as typeof import("node:sqlite");
  const db = new DatabaseSync(":memory:");
  for (const { file, text } of texts) {
    try {
      db.exec(text);
    } catch (e) {
      db.close();
      throw new SchemaError(`Cannot load the schema: ${at(file, text, undefined)}: ${(e as Error).message}`);
    }
  }
  return {
    describe: () => Promise.reject(new Error("SQLite is described inside the engine")),
    async values(sql, params = []) {
      const stmt = db.prepare(sql);
      const names = stmt.columns().map((c) => c.name);
      return (stmt.all(...(params as never[])) as Record<string, unknown>[]).map((row) => names.map((n) => row[n]));
    },
    async close() {
      db.close();
    },
  };
}

/** PGlite's contrib modules, by the name `create extension` gives them; `vector` is a package of its own. */
const CONTRIB = new Set(["amcheck", "auto_explain", "bloom", "btree_gin", "btree_gist", "citext", "cube", "dict_int", "dict_xsyn", "earthdistance", "file_fdw", "fuzzystrmatch", "hstore", "intarray", "isn", "lo", "ltree", "moddatetime", "pageinspect", "pg_buffercache", "pg_freespacemap", "pg_stat_statements", "pg_surgery", "pg_trgm", "pg_visibility", "pg_walinspect", "pgcrypto", "seg", "tablefunc", "tcn", "tsm_system_rows", "tsm_system_time", "unaccent", "uuid-ossp"]);

const IDENT = String.raw`("(?:[^"]|"")+"|[A-Za-z_][\w$]*)`;
const name = (raw: string) => (raw.startsWith('"') ? raw.slice(1, -1).replace(/""/g, '"') : raw.toLowerCase());

/** The extensions the files create. */
function extensionsOf(texts: { text: string }[]): Set<string> {
  const out = new Set<string>();
  const re = new RegExp(String.raw`\bcreate\s+extension\s+(?:if\s+not\s+exists\s+)?("(?:[^"]|"")+"|[\w-]+)`, "gi");
  for (const { text } of texts) for (const m of text.matchAll(re)) out.add(name(m[1]!));
  return out;
}

/**
 * The roles the files hand something to without creating them: the owner a `pg_dump` names
 * (`alter table … owner to app`), the grantees of a grant. PGlite has only `postgres`, so each is
 * made before the files load, a superuser, so that a file that acts as it (`set role app`) may do
 * what it does. A role the files create themselves is left to them.
 */
function rolesOf(texts: { text: string }[]): Set<string> {
  const used = new Set<string>();
  const created = new Set<string>();
  const list = String.raw`${IDENT}(?:\s*,\s*${IDENT})*`;
  const uses = [
    new RegExp(String.raw`\bowner\s+to\s+${IDENT}`, "gi"),
    new RegExp(String.raw`\bauthorization\s+${IDENT}`, "gi"),
    new RegExp(String.raw`\bfor\s+(?:role|user)\s+(${list})`, "gi"),
    new RegExp(String.raw`\bgrant\s[^;]*?\bto\s+(${list})`, "gi"),
    new RegExp(String.raw`\brevoke\s[^;]*?\bfrom\s+(${list})`, "gi"),
  ];
  for (const { text } of texts) {
    for (const re of uses) for (const m of text.matchAll(re)) for (const r of m[1]!.matchAll(new RegExp(IDENT, "g"))) used.add(name(r[1]!));
    for (const m of text.matchAll(new RegExp(String.raw`\bcreate\s+(?:role|user|group)\s+(?:if\s+not\s+exists\s+)?${IDENT}`, "gi"))) created.add(name(m[1]!));
  }
  const builtin = new Set(["public", "current_user", "session_user", "current_role", "default", "postgres"]);
  return new Set([...used].filter((r) => !created.has(r) && !builtin.has(r) && !r.startsWith("pg_")));
}

const quoted = (ident: string) => `"${ident.replace(/"/g, '""')}"`;

async function openPglite(root: string, texts: { file: string; text: string }[], at: Located): Promise<SchemaDatabase> {
  const require = createRequire(join(root, "package.json"));
  const path = resolveFrom(root, "@electric-sql/pglite");
  if (!path) throw new SchemaError(`"schema" in .reylo.yaml loads a Postgres schema into PGlite, which is not installed: npm install -D @electric-sql/pglite`);
  const { PGlite } = loadDriver(require, path) as { PGlite: { create(options?: object): Promise<any> } };

  const extensions: Record<string, unknown> = {};
  for (const ext of extensionsOf(texts)) {
    if (CONTRIB.has(ext)) {
      const module = ext.replace(/-/g, "_");
      const found = resolveFrom(root, `@electric-sql/pglite/contrib/${module}`);
      if (found) extensions[module] = loadDriver(require, found)[module];
    } else if (ext === "vector") {
      const found = resolveFrom(root, "@electric-sql/pglite-pgvector");
      if (!found) throw new SchemaError(`Cannot load the schema: it creates the extension vector, which PGlite loads from @electric-sql/pglite-pgvector: npm install -D @electric-sql/pglite-pgvector`);
      extensions.vector = loadDriver(require, found).vector;
    }
  }

  const roles = rolesOf(texts);
  // Loads the files into a new database. `split` runs the file at that index a statement at a
  // time, to find the one that fails where the server names no place in the file.
  const load = async (split?: number): Promise<{ db: any; failed?: { index: number; error: any; start: number | undefined } }> => {
    const db = await PGlite.create({ extensions });
    try {
      for (const role of roles) await db.exec(`create role ${quoted(role)} superuser`);
      for (const [index, { text }] of texts.entries()) {
        for (const { start, end } of index === split ? statements(text) : [{ start: 0, end: text.length }]) {
          try {
            await db.exec(text.slice(start, end));
          } catch (error) {
            return { db, failed: { index, error, start: index === split ? start : undefined } };
          }
        }
        // A dump sets `search_path` to nothing, and a file may set anything: the next file, and
        // every statement the engine describes, starts from the defaults.
        await db.exec("reset all; set session authorization default");
      }
      return { db };
    } catch (e) {
      await db.close().catch(() => {});
      throw e;
    }
  };

  for (let attempt = 0; ; attempt++) {
    const { db, failed } = await load();
    if (!failed) return pgliteDatabase(db);
    await db.close().catch(() => {});
    const { file, text } = texts[failed.index]!;
    const message: string = failed.error?.message ?? String(failed.error);
    // A role the scan missed: the database is made again with it, since the failed file may
    // have done part of its work.
    const missing = /^role "(.+)" does not exist$/.exec(message)?.[1];
    if (missing && !roles.has(missing) && attempt < 64) {
      roles.add(missing);
      continue;
    }
    let position = Number(failed.error?.position) || undefined;
    if (!position) {
      const again = await load(failed.index).catch(() => undefined);
      await again?.db.close().catch(() => {});
      const statement = again?.failed;
      if (statement?.index === failed.index && statement.start !== undefined && statement.error?.message === message) {
        position = statement.start + (Number(statement.error?.position) || firstToken(text, statement.start) - statement.start + 1);
      }
    }
    const unavailable = /^extension "(.+)" is not available$/.exec(message)?.[1];
    const hint = unavailable ? ` (PGlite, which Reylo loads the schema into, has no ${unavailable}: type against a database that has it, with DATABASE_URL or direct_url in place of schema)` : "";
    throw new SchemaError(`Cannot load the schema: ${at(file, text, position)}: ${message}${hint}`);
  }
}

/** Where the first token at or after `from` is: past whitespace and comments. */
function firstToken(text: string, from: number): number {
  let i = from;
  for (;;) {
    while (i < text.length && /\s/.test(text[i]!)) i++;
    if (text.startsWith("--", i)) i = text.includes("\n", i) ? text.indexOf("\n", i) : text.length;
    else if (text.startsWith("/*", i)) i = text.includes("*/", i) ? text.indexOf("*/", i) + 2 : text.length;
    else return i;
  }
}

/**
 * Where each statement of a Postgres script starts and ends: at a semicolon outside a string, a
 * quoted identifier, a dollar-quoted body, a comment, and a `begin atomic … end` body. Used only to
 * place an error the server gave no position for, so a statement it splits wrongly costs only the
 * place named.
 */
export function statements(text: string): { start: number; end: number }[] {
  const out: { start: number; end: number }[] = [];
  const word = /[A-Za-z_\u0080-\uffff][\w$\u0080-\uffff]*/y;
  const ident = (c: string | undefined) => c !== undefined && /[\w$\u0080-\uffff]/.test(c);
  let start = 0;
  // Inside a `begin atomic` body, each `case` and the body itself close with an `end`.
  let open = 0;
  let previous = "";
  for (let i = 0; i < text.length; ) {
    const c = text[i]!;
    if (c === "-" && text[i + 1] === "-") {
      const nl = text.indexOf("\n", i);
      i = nl < 0 ? text.length : nl;
    } else if (c === "/" && text[i + 1] === "*") {
      let depth = 1;
      for (i += 2; i < text.length && depth; i++) {
        if (text.startsWith("/*", i)) depth++, i++;
        else if (text.startsWith("*/", i)) depth--, i++;
      }
    } else if (c === "'" || c === '"') {
      // An E'…' string takes backslash escapes.
      const escapes = c === "'" && /^[eE]$/.test(text[i - 1] ?? "") && !ident(text[i - 2]);
      for (i++; i < text.length; i++) {
        if (escapes && text[i] === "\\") i++;
        else if (text[i] === c) {
          if (text[i + 1] === c) i++;
          else break;
        }
      }
      i++;
    } else if (c === "$" && !ident(text[i - 1])) {
      const tag = /^\$(?:[A-Za-z_][\w]*)?\$/.exec(text.slice(i, i + 64))?.[0];
      if (tag) {
        const close = text.indexOf(tag, i + tag.length);
        i = close < 0 ? text.length : close + tag.length;
      } else i++;
    } else if (/[A-Za-z_\u0080-\uffff]/.test(c) && !ident(text[i - 1])) {
      word.lastIndex = i;
      word.test(text);
      const w = text.slice(i, word.lastIndex).toLowerCase();
      if (w === "atomic" && previous === "begin") open++;
      else if (open && w === "case") open++;
      else if (open && w === "end") open--;
      previous = w;
      i = word.lastIndex;
    } else if (c === ";" && !open) {
      out.push({ start, end: i + 1 });
      start = ++i;
    } else i++;
  }
  if (text.slice(start).trim()) out.push({ start, end: text.length });
  return out;
}

// The frontend/backend protocol's Parse, Describe and Sync, sent to PGlite in one call: each call
// gets its own answer, even after the server rejects the statement, and the row description
// carries each column's table and column number, which the engine's nullability reads
// (wiki/dev-database.md).
const encoder = new TextEncoder();
function message(type: string, body: Uint8Array): Uint8Array {
  const out = new Uint8Array(5 + body.length);
  out[0] = type.charCodeAt(0);
  new DataView(out.buffer).setInt32(1, 4 + body.length);
  out.set(body, 5);
  return out;
}
const cstring = (s: string) => [...encoder.encode(s), 0];
function describeMessages(sql: string): Uint8Array {
  const parse = message("P", new Uint8Array([...cstring(""), ...cstring(sql), 0, 0]));
  const describe = message("D", new Uint8Array([83, 0]));
  const sync = message("S", new Uint8Array());
  const out = new Uint8Array(parse.length + describe.length + sync.length);
  out.set(parse, 0);
  out.set(describe, parse.length);
  out.set(sync, parse.length + describe.length);
  return out;
}

// What the drivers give for a 64-bit integer, which the engine's catalog queries read as pg and
// postgres.js return it: its text.
const INT8 = 20;

function pgliteDatabase(db: any): SchemaDatabase {
  // One statement at a time: a bundler loads several files at once.
  let turn: Promise<unknown> = Promise.resolve();
  const serial = <T>(call: () => Promise<T>): Promise<T> => {
    const next = turn.then(call, call);
    turn = next.catch(() => {});
    return next;
  };
  return {
    describe: (sql) =>
      serial(async () => {
        const { messages } = await db.execProtocol(describeMessages(sql), { throwOnError: false });
        const error = messages.find((m: any) => m.name === "error");
        if (error) throw Object.assign(new Error(error.message), { code: error.code, position: error.position, severity: error.severity });
        const out: Described = { params: [], columns: [] };
        for (const m of messages) {
          if (m.name === "parameterDescription") out.params = (m.dataTypeIDs as number[]).map((oid) => ({ oid }));
          if (m.name === "rowDescription") out.columns = (m.fields as any[]).map((f) => ({ name: f.name, oid: f.dataTypeID, table: f.tableID, attnum: f.columnID }));
        }
        return out;
      }),
    values: (sql, params = []) => serial(async () => (await db.query(sql, params, { rowMode: "array", parsers: { [INT8]: (text: string) => text } })).rows),
    close: () => db.close(),
  };
}
