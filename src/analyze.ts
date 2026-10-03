// One analysis path for every consumer of a `.sql` file — the type check, the bundler plugins,
// the runtime hook and `motherload generate` — and one cache they share: `.motherload/` beside the project
// file, one entry per statement. The cache is only for speed. An entry records the schema it was
// analyzed against; while the database answers, an entry for another schema is analyzed again,
// and while it does not, the entries are used as they are, so a build that cannot reach the
// database runs on what `motherload generate` or an earlier type check wrote.
import { existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { join } from "node:path";
import { engine, reason, AnswerCache, Unreachable, type Analysis, type Engine, type EngineIo } from "./engine.ts";
import { packageDir, resolveProject, type ResolvedProject } from "./project.ts";
import { openPostgresDescriber } from "./describe.ts";
import { openMysqlDescriber } from "./mysql-describe.ts";
import { openSqliteDescriber, sqlitePath } from "./sqlite-describe.ts";

/** What `.motherload/` holds for one statement. */
export type Entry = { version: 1; dialect: ResolvedProject["dialect"]; schema: string; analysis: Analysis };

export type Outcome =
  | { ok: true; analysis: Analysis; source: "cache" | "database"; entry: string }
  | { ok: false; message: string; code: number };

type Io = EngineIo & { close(): Promise<void> };

const ENTRY_VERSION = 1;
// How long a schema fingerprint is trusted, as long as a statement answer: a migration shows up
// in a running dev server or editor within seconds. Once the database is found unreachable, the
// session stays offline for a minute instead of paying the connect timeout per file.
const SCHEMA_TTL = 5_000;
const OFFLINE_TTL = 60_000;
// A build step's session closes its connection a moment after its last file, so that a build
// process ends when its work does: the plugins whose bundler says when a build ends close it then
// (`releaseConnections`), and a loader (webpack, Rspack, Turbopack) has no such signal. The type
// check's session keeps its connection.
const BUILD_IDLE = 250;

const sessions = new Map<string, Session>();

export class Session {
  readonly project: ResolvedProject;
  /** The project's directory, which holds `.motherload/`. */
  readonly root: string;
  readonly #engine: Engine;
  readonly #engineFingerprint: string;
  readonly #answers = new AnswerCache();
  readonly #idle: number;
  #io: Promise<Io> | undefined;
  #busy = 0;
  #timer: ReturnType<typeof setTimeout> | undefined;
  #schema: { value: string | null; at: number; error?: string; fatal?: boolean } | undefined;

  private constructor(engine: Engine, project: ResolvedProject, idle: number) {
    this.#engine = engine;
    this.project = project;
    this.root = project.root;
    this.#idle = idle;
    const ping = engine.call({ op: "ping" });
    this.#engineFingerprint = ping.status === "ok" ? ping.fingerprint : "unknown";
  }

  /** The build steps' session for the project a file or directory belongs to: one per project (and connection) and process. */
  static async for(from: string, env: Record<string, string | undefined> = process.env): Promise<Session> {
    const e = await engine();
    // From the package's directory, so that a message names a file as the project sees it.
    const project = resolveProject(e, packageDir(from), env);
    const key = JSON.stringify([project.root, project.url ?? null, project.error ?? null]);
    let session = sessions.get(key);
    if (!session) sessions.set(key, (session = new Session(e, project, BUILD_IDLE)));
    return session;
  }

  /** A session of its own, for a caller that must not share one (the type check, a test); it keeps its connection until closed. */
  static async open(from: string, env: Record<string, string | undefined> = process.env, idle = Infinity): Promise<Session> {
    const e = await engine();
    return new Session(e, resolveProject(e, from, env), idle);
  }

  /** Every build-step session this process opened, for a caller that ends them (`motherload generate`). */
  static opened(): Session[] {
    return [...sessions.values()];
  }

  lex(sql: string) {
    return this.#engine.lex(sql, this.project.dialect);
  }

  /** Where the entry for a statement lives, whether or not it exists yet. */
  entryFile(sql: string): string {
    const server = this.#server();
    const key = createHash("sha256").update(JSON.stringify([ENTRY_VERSION, this.#engineFingerprint, this.project.dialect, server, sql])).digest("hex");
    return join(this.root, ".motherload", "queries", `${key.slice(0, 32)}.json`);
  }

  /** The statement's analysis: from `.motherload/` when the entry matches the schema, else from the database. */
  async analyze(sql: string, options: { force?: boolean } = {}): Promise<Outcome> {
    clearTimeout(this.#timer);
    this.#busy++;
    try {
      return await this.#analyze(sql, options);
    } finally {
      if (--this.#busy === 0 && this.#io && Number.isFinite(this.#idle)) this.#timer = setTimeout(() => this.#release(), this.#idle).unref();
    }
  }

  async #analyze(sql: string, options: { force?: boolean }): Promise<Outcome> {
    if (this.project.error) return { ok: false, message: this.project.error, code: 11 };
    if (!this.project.url) return { ok: false, message: `No database URL: set DATABASE_URL in the environment, .env.local or .env, or "url" in .motherload.yaml.`, code: 4 };
    const file = this.entryFile(sql);
    const entry = options.force ? undefined : readEntry(file);
    const schema = await this.schema();
    if (schema.fatal) return { ok: false, message: schema.error!, code: 5 };
    if (entry && (schema.value === null || entry.schema === schema.value)) return { ok: true, analysis: entry.analysis, source: "cache", entry: file };
    if (schema.value === null) {
      return { ok: false, message: `${this.cannotConnect(schema.error)}; .motherload has no analysis of this statement, so run motherload generate where the database can be reached.`, code: 5 };
    }
    let analysis: Analysis;
    try {
      analysis = await this.#engine.analyze(sql, this.project.dialect, await this.#connection(), this.#answers);
    } catch (e) {
      // A dead connection is reopened by the next call; a server error is reported.
      if (e instanceof Unreachable) {
        this.#io = undefined;
        this.#schema = { value: null, at: Date.now(), error: reason(e) };
        return { ok: false, message: this.cannotConnect(reason(e)), code: 5 };
      }
      return { ok: false, message: (e as Error).message, code: 5 };
    }
    writeEntry(this.root, file, { version: ENTRY_VERSION, dialect: this.project.dialect, schema: schema.value, analysis });
    return { ok: true, analysis, source: "database", entry: file };
  }

  /**
   * The fingerprint of the database's schema; null when the database cannot be reached, and
   * `fatal` when no connection can be made at all (no driver installed). A server that refuses the
   * fingerprint query gets a value no entry matches, so every statement is analyzed live.
   */
  async schema(): Promise<{ value: string | null; error?: string; fatal?: boolean }> {
    const now = Date.now();
    if (this.#schema && now - this.#schema.at < (this.#schema.value === null ? OFFLINE_TTL : SCHEMA_TTL)) return this.#schema;
    let io: Io;
    try {
      io = await this.#connection();
    } catch (e) {
      this.#io = undefined;
      this.#schema = e instanceof Unreachable ? { value: null, at: now, error: reason(e) } : { value: null, at: now, error: (e as Error).message, fatal: true };
      return this.#schema;
    }
    try {
      const rows = await io.query(FINGERPRINT[this.project.dialect], []);
      const value = createHash("sha256").update(JSON.stringify(rows)).digest("hex").slice(0, 16);
      // The engine's answers describe the old schema too.
      if (this.#schema?.value && this.#schema.value !== value) this.#answers.clear();
      this.#schema = { value, at: now };
    } catch (e) {
      if (e instanceof Unreachable) {
        this.#io = undefined;
        this.#schema = { value: null, at: now, error: reason(e) };
      } else {
        this.#schema = { value: `unverified ${process.pid} ${now}`, at: now };
      }
    }
    return this.#schema;
  }

  /** Where the database was looked for, and where that came from; never the credentials. */
  cannotConnect(why?: string): string {
    const { dialect, url, root, urlSource } = this.project;
    if (dialect === "sqlite") return `Cannot open the database ${sqlitePath(url ?? "", root)} (${urlSource})${why ? `: ${why}` : ""}`;
    return `Cannot connect to the database at ${this.where()} (${urlSource})${why ? `: ${why}` : ""}`;
  }

  /** The database as a person names it: host, port and database, or the SQLite file. */
  where(): string {
    const { dialect, url, root } = this.project;
    if (dialect === "sqlite") return sqlitePath(url ?? "", root);
    try {
      const u = new URL(url ?? "");
      return `${u.hostname || u.searchParams.get("host") || "localhost"}${u.port ? `:${u.port}` : ""}${u.pathname}`;
    } catch {
      return "the database";
    }
  }

  async close(): Promise<void> {
    for (const [key, session] of sessions) if (session === this) sessions.delete(key);
    await this.release();
  }

  /** Closes the connection now, for a caller that knows no file is coming; the next analysis opens another. */
  async release(): Promise<void> {
    clearTimeout(this.#timer);
    await this.#release();
  }

  /** Closes the connection; the next analysis opens another. */
  async #release(): Promise<void> {
    const io = this.#io;
    this.#io = undefined;
    if (io) await io.then((i) => i.close()).catch(() => {});
  }

  /** The server, without credentials: entries for another database are other entries. */
  #server(): string {
    const { dialect, url, root } = this.project;
    if (dialect === "sqlite") return sqlitePath(url ?? "", root);
    try {
      const u = new URL(url ?? "");
      return `${u.protocol}//${u.username}@${u.host}${u.pathname}`;
    } catch {
      return url ?? "";
    }
  }

  #connection(): Promise<Io> {
    const { dialect, url, root } = this.project;
    this.#io ??= (async (): Promise<Io> => {
      if (dialect === "mysql") {
        const m = await openMysqlDescriber(root, url!);
        return { describe: (text) => m.describe(text), query: (text, params) => m.values(text, params), close: () => m.close() };
      }
      if (dialect === "sqlite") {
        // A file that cannot be opened is a database that cannot be reached: a build without it
        // runs on `.motherload/`.
        const s = await openSqliteDescriber(url!, root).catch((e) => Promise.reject(new Unreachable(e)));
        // The engine prepares SQLite statements inside its own SQLite; it never asks to describe.
        return { describe: () => Promise.reject(new Error("SQLite is described inside the engine")), query: (text, params) => s.values(text, params), close: () => s.close() };
      }
      const d = await openPostgresDescriber(root, url!);
      return { describe: (text) => d.describe(text), query: (text, params) => d.values(text, params), close: () => d.close() };
    })();
    const io = this.#io;
    io.catch(() => {
      if (this.#io === io) this.#io = undefined;
    });
    return io;
  }
}

/** Closes every build-step session's connection, for a build step whose build has ended: the process then ends without waiting out the idle time. */
export async function releaseConnections(): Promise<void> {
  await Promise.all([...sessions.values()].map((session) => session.release()));
}

function readEntry(file: string): Entry | undefined {
  if (!existsSync(file)) return undefined;
  try {
    const entry = JSON.parse(readFileSync(file, "utf8")) as Entry;
    return entry.version === ENTRY_VERSION ? entry : undefined;
  } catch {
    return undefined;
  }
}

// Several processes write `.motherload/` at once (an editor's type check, a dev server, a build), so an
// entry is written to a file of its own and renamed into place: a reader sees a whole entry or
// none. `.motherload/.gitignore` keeps the directory out of git, as pytest's and ruff's caches do. The
// cache is only for speed, so a directory that cannot be written (a read-only checkout) is not an
// error.
function writeEntry(root: string, file: string, entry: Entry): void {
  const dir = join(root, ".motherload");
  const tmp = `${file}.${process.pid}.${Math.random().toString(36).slice(2)}.tmp`;
  try {
    mkdirSync(join(dir, "queries"), { recursive: true });
    const ignore = join(dir, ".gitignore");
    if (!existsSync(ignore)) writeFileSync(ignore, "# motherload's cache: written by the type check, the build steps and motherload generate; never committed.\n*\n");
    writeFileSync(tmp, JSON.stringify(entry) + "\n");
    renameSync(tmp, file);
  } catch {
    rmSync(tmp, { force: true });
  }
}

/**
 * One query per dialect whose rows change when anything an analysis reads changes: relations and
 * their columns, constraints and indexes (nullability and the row count read them), enum labels,
 * domains, functions and view definitions, and the server settings that change a statement's
 * types. SQLite's whole schema is its `sqlite_schema` table.
 */
const NS = `not in ('pg_catalog', 'information_schema') and n.nspname !~ '^pg_(toast|temp)'`;
const FINGERPRINT: Record<ResolvedProject["dialect"], string> = {
  postgres: `select md5(coalesce(string_agg(x, E'\\n' order by x), '')) from (
    select 'v ' || current_setting('server_version_num') || ' ' || current_setting('search_path') as x
    union all select 'r ' || c.oid || ' ' || n.nspname || '.' || c.relname || ' ' || c.relkind from pg_class c join pg_namespace n on n.oid = c.relnamespace where n.nspname ${NS}
    union all select 'a ' || a.attrelid || ' ' || a.attnum || ' ' || a.attname || ' ' || a.atttypid || ' ' || a.attnotnull from pg_attribute a join pg_class c on c.oid = a.attrelid join pg_namespace n on n.oid = c.relnamespace where a.attnum > 0 and not a.attisdropped and n.nspname ${NS}
    union all select 'c ' || o.conrelid || ' ' || o.conname || ' ' || pg_get_constraintdef(o.oid) from pg_constraint o join pg_namespace n on n.oid = o.connamespace where n.nspname ${NS}
    union all select 'i ' || i.indexrelid || ' ' || pg_get_indexdef(i.indexrelid) from pg_index i join pg_class c on c.oid = i.indrelid join pg_namespace n on n.oid = c.relnamespace where n.nspname ${NS}
    union all select 'e ' || e.enumtypid || ' ' || e.enumsortorder || ' ' || e.enumlabel from pg_enum e
    union all select 't ' || t.oid || ' ' || t.typname || ' ' || t.typtype || ' ' || t.typbasetype || ' ' || t.typnotnull from pg_type t join pg_namespace n on n.oid = t.typnamespace where n.nspname ${NS}
    union all select 'f ' || p.oid || ' ' || p.proname || ' ' || p.prorettype || ' ' || p.proretset || ' ' || p.proargtypes::text || ' ' || md5(coalesce(p.prosrc, '')) from pg_proc p join pg_namespace n on n.oid = p.pronamespace where n.nspname ${NS}
    union all select 'w ' || c.oid || ' ' || md5(pg_get_viewdef(c.oid)) from pg_class c join pg_namespace n on n.oid = c.relnamespace where c.relkind in ('v', 'm') and n.nspname ${NS}
  ) t`,
  mysql: `select concat_ws(' ', @@version, @@sql_mode, count(*), sum(crc32(x)), bit_xor(crc32(x))) from (
    select concat_ws(' ', 'c', table_name, column_name, ordinal_position, column_type, is_nullable, column_key, extra, coalesce(generation_expression, '')) as x from information_schema.columns where table_schema = database()
    union all select concat_ws(' ', 's', table_name, index_name, non_unique, seq_in_index, column_name) from information_schema.statistics where table_schema = database()
    union all select concat_ws(' ', 'v', table_name, md5(view_definition)) from information_schema.views where table_schema = database()
    union all select concat_ws(' ', 'r', routine_name, routine_type, coalesce(dtd_identifier, ''), md5(coalesce(routine_definition, ''))) from information_schema.routines where routine_schema = database()
  ) t`,
  sqlite: `select type, name, tbl_name, coalesce(sql, '') from sqlite_schema order by type, name`,
};
