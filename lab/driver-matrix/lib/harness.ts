// The scenario runner every driver job shares. A job describes one driver version against one
// server: the client shapes it offers (pool, connection, checked-out connection, transaction,
// savepoint, reserved connection, ...) and a raw row count that reads through the driver
// itself, never through dotsql. Each scenario runs a statement through dotsql's
// defineQuery(...).run(client, params) and asserts the result; a record per scenario is appended
// to the JSON Lines file named by DM_OUT.
//
// Everything here is erasable TypeScript, so Node runs it with its built-in type stripping and
// Bun runs it as is. The runtime under test is imported by relative path.
import { appendFileSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { defineQuery } from "../../../src/index.ts";

export type Dialect = "postgres" | "mysql" | "sqlite";
/** How `run` must answer on this client: the value itself (better-sqlite3, node:sqlite, bun:sqlite) or a promise. */
export type Kind = "sync" | "async";
export type Client = any;
export type Body = (client: Client) => Promise<unknown>;

/** The four transaction scenarios, written against the shape's own transaction API. */
export type TxOps = {
  /** Runs body in a transaction that commits. */
  commit(body: Body): Promise<unknown>;
  /** Runs body in a transaction that is then rolled back (a thrown error or an explicit rollback). */
  rollback(body: Body): Promise<unknown>;
  /** Runs outer, then inner inside a savepoint that is rolled back, then commits. */
  savepoint?(outer: Body, inner: Body): Promise<unknown>;
  /** Runs body, which rejects with a driver error, the way a user writes it: the transaction must roll back and the error reach the caller. */
  failing(body: Body): Promise<unknown>;
};

export type Shape = {
  name: string;
  kind: Kind;
  /** Runs fn with a fresh client object of this shape, and cleans up after it. Omitted for a shape that only has transaction scenarios. */
  scope?: (fn: (client: Client) => Promise<void>) => Promise<void>;
  tx?: TxOps;
  /** better-sqlite3 and bun:sqlite: the database whose synchronous db.transaction(fn) is under test. */
  syncTx?: Client;
  /** Scenarios that do not apply to this shape, with the reason. */
  skip?: Partial<Record<string, string>>;
  /** "shape only": the client is real but its transport is stubbed. */
  live?: boolean;
  /** The scope runs inside an open transaction, so a write is not visible to the raw count until it ends. */
  inTransaction?: boolean;
  /** dotsql does not support this client: run() must reject with its own TypeError matching this. Only S9 runs. */
  refused?: RegExp;
  /** dotsql refuses any statement with parameters on this client, with a TypeError matching this whose cause is the driver's error. */
  paramsRefused?: RegExp;
};

export type Job = {
  pkg: string;
  version: string;
  target: string;
  dialect: Dialect;
  shapes: Shape[];
  /** Rows in the scratch table with this tag, read through the driver without dotsql. */
  count(tag: string): Promise<number>;
  close?(): Promise<unknown>;
  /** Scenarios that do not apply to the whole job, with the reason. */
  skip?: Partial<Record<string, string>>;
};

/** "unsupported": dotsql refused the client or the statement with its own TypeError, as its contract says for this shape. */
export type Status = "pass" | "fail" | "skip" | "not run" | "unsupported";
export type Result = {
  pkg: string;
  version: string;
  runtime: string;
  target: string;
  shape: string;
  scenario: string;
  status: Status;
  live: boolean;
  error?: string;
  detail?: string;
};

export const runtime = typeof (globalThis as any).Bun !== "undefined" ? `bun ${(globalThis as any).Bun.version}` : `node ${process.versions.node}`;
export const here = fileURLToPath(new URL("..", import.meta.url));
export const requireFrom = createRequire(new URL("../package.json", import.meta.url));

/** --pkg <alias> --version <version> [--target <label>] [--from <dir>] [key value ...] */
export function args(): Record<string, string> {
  const out: Record<string, string> = {};
  const argv = process.argv.slice(2);
  for (let i = 0; i < argv.length; i += 2) out[argv[i]!.replace(/^--/, "")] = argv[i + 1]!;
  return out;
}

export function seedSql(dialect: "postgres" | "mysql" | "sqlite"): string {
  return readFileSync(new URL(`../seed/${dialect}.sql`, import.meta.url), "utf8");
}

/** Splits a seed file into statements (no statement in them contains a semicolon). */
export function statements(sql: string): string[] {
  return sql
    .split("\n")
    .filter((line) => !line.startsWith("--"))
    .join("\n")
    .split(";")
    .map((s) => s.trim())
    .filter(Boolean);
}

const out = process.env.DM_OUT;
export function emit(result: Result): void {
  const line = JSON.stringify(result);
  if (out) appendFileSync(out, line + "\n");
  const mark = result.status === "pass" ? "ok  " : result.status === "fail" ? "FAIL" : result.status === "skip" ? "skip" : result.status === "unsupported" ? "unsp" : "----";
  console.log(`${mark} ${result.pkg}@${result.version} [${result.target}] ${result.shape} :: ${result.scenario}${result.error ? ` -- ${result.error}` : ""}`);
}

// ---------------------------------------------------------------------------------------------
// Assertions and error description

/** A breach of dotsql's contract seen by the harness (for example run() throwing synchronously on an asynchronous client). */
export class Violation extends Error {}

class Mismatch extends Error {}

export function describe(e: unknown): string {
  if (e instanceof Error) {
    const fields = codeFields(e);
    const tags = Object.entries(fields)
      .filter(([k]) => k !== "message")
      .map(([k, v]) => `${k}=${String(v)}`)
      .join(" ");
    return `${e.constructor?.name ?? "Error"}: ${oneLine(e.message)}${tags ? ` [${tags}]` : ""}`;
  }
  try {
    return `non-Error thrown: ${oneLine(JSON.stringify(e))}`;
  } catch {
    return `non-Error thrown: ${String(e)}`;
  }
}

export function oneLine(s: string, max = 300): string {
  const t = String(s).replace(/\s+/g, " ").trim();
  return t.length > max ? t.slice(0, max) + "..." : t;
}

function codeFields(e: any): Record<string, unknown> {
  const out: Record<string, unknown> = {};
  for (const key of ["code", "errno", "sqlState", "sqlstate", "errcode", "errstr", "rawCode", "extendedCode", "severity"]) {
    if (e?.[key] !== undefined && e?.[key] !== null && e?.[key] !== "") out[key] = e[key];
  }
  if (e?.cause && typeof e.cause === "object") {
    for (const key of ["code", "errno", "sqlState"]) if (e.cause[key] !== undefined) out[`cause.${key}`] = e.cause[key];
  }
  if (e?.body && typeof e.body === "object" && e.body.code !== undefined) out["body.code"] = e.body.code;
  return out;
}

function eq(actual: unknown, expected: unknown, what: string): void {
  const a = JSON.stringify(normalize(actual));
  const b = JSON.stringify(normalize(expected));
  if (a !== b) throw new Mismatch(`${what}: expected ${b}, got ${oneLine(safeJson(actual), 400)}`);
}

function safeJson(v: unknown): string {
  try {
    return JSON.stringify(v, (_k, x) => (typeof x === "bigint" ? `${x}n` : x)) ?? String(v);
  } catch {
    return String(v);
  }
}

// Decoding is not under test: a BIGINT count, a TINYINT(1) boolean or a numeric string compares
// equal to the plain value, so a row fails only when it carries the wrong data.
function normalize(v: unknown): unknown {
  if (typeof v === "bigint") return Number(v);
  if (typeof v === "boolean") return v ? 1 : 0;
  if (typeof v === "string" && /^-?\d+$/.test(v)) return Number(v);
  if (Array.isArray(v)) return v.map(normalize);
  if (v && typeof v === "object") return Object.fromEntries(Object.keys(v).sort().map((k) => [k, normalize((v as any)[k])]));
  return v;
}

type ErrorExpectation = { codes: (string | number | RegExp)[]; message: RegExp };
const expected: Record<Dialect, { syntax: ErrorExpectation; dup: ErrorExpectation }> = {
  postgres: {
    syntax: { codes: ["42601"], message: /syntax error/i },
    dup: { codes: ["23505"], message: /duplicate key/i },
  },
  mysql: {
    syntax: { codes: ["ER_PARSE_ERROR", 1064, "1064"], message: /errno 1064|SQL syntax/i },
    dup: { codes: ["ER_DUP_ENTRY", 1062, "1062"], message: /errno 1062|Duplicate entry/i },
  },
  sqlite: {
    syntax: { codes: [/^(ERR_)?SQLITE_ERROR$/, 1], message: /syntax error/i },
    dup: { codes: [/SQLITE_CONSTRAINT/, 19, 1555, 2067], message: /UNIQUE constraint failed|SQLITE_CONSTRAINT/i },
  },
};

/** The rejection must be the driver's own error with its code, not dotsql's TypeError or a harness violation. */
function expectDriverError(e: unknown, dialect: Dialect, which: "syntax" | "dup"): string {
  if (e instanceof Violation) throw e;
  if (e instanceof Mismatch) throw e;
  if (e instanceof TypeError && /dotsql/.test(e.message)) throw new Mismatch(`dotsql's TypeError instead of the driver's error: ${describe(e)}`);
  if (!(e instanceof Error)) throw new Mismatch(`rejected with a non-Error: ${describe(e)}`);
  const want = expected[dialect][which];
  const fields = codeFields(e);
  const values = Object.values(fields);
  const byCode = values.some((v) => want.codes.some((c) => (c instanceof RegExp ? c.test(String(v)) : c === v)));
  if (byCode) return describe(e);
  if (want.message.test(e.message)) return `${describe(e)} (matched by message: the error carries no code field)`;
  throw new Mismatch(`not the driver's ${which === "dup" ? "unique-violation" : "syntax"} error: ${describe(e)}`);
}

// ---------------------------------------------------------------------------------------------
// Calling dotsql, with its return discipline checked on every call

type Ctx = { job: Job; shape: Shape; kind: Kind; refusals: unknown[] };

function isThenable(v: unknown): v is PromiseLike<unknown> {
  return v != null && (typeof v === "object" || typeof v === "function") && typeof (v as any).then === "function";
}

/** q.run(client, params), or q.with(client)(params) when via is "with". */
export async function call(ctx: Ctx, q: any, client: Client, params?: Record<string, unknown>, via: "run" | "with" = "run"): Promise<any> {
  let r: unknown;
  try {
    if (via === "with") r = params === undefined ? q.with(client)() : q.with(client)(params);
    else r = params === undefined ? q.run(client) : q.run(client, params);
  } catch (e) {
    throw new Violation(`run threw synchronously: ${describe(e)}`);
  }
  if (!isThenable(r)) throw new Violation(`run returned ${r === null ? "null" : typeof r} instead of a promise`);
  try {
    return await r;
  } catch (e) {
    if (ctx.shape.paramsRefused && e instanceof TypeError && ctx.shape.paramsRefused.test(e.message)) ctx.refusals.push(e);
    throw e;
  }
}

// ---------------------------------------------------------------------------------------------
// The statements, per dialect

export function queries(d: Dialect) {
  const t = (n: string) => (d === "sqlite" ? n : `driver_matrix.${n}`);
  const cast = (p: string, ty: string) => (d === "postgres" ? `${p}::${ty}` : p);
  const q = (sql: string, params: string[], cardinality: "many" | "one" | "maybeOne" | "none" = "many", extra: Record<string, unknown> = {}) =>
    defineQuery<any, any, any, any>({ dialect: d, sql, params, cardinality, ...extra });
  const repeatedSql = `select id, name from ${t("people")} where id = :id and name = (select name from ${t("people")} where id = :id)`;
  const repeatedText =
    d === "postgres"
      ? `select id, name from ${t("people")} where id = $1 and name = (select name from ${t("people")} where id = $1)`
      : `select id, name from ${t("people")} where id = ? and name = (select name from ${t("people")} where id = ?)`;
  return {
    params: q(`select id, name, active, score from ${t("people")} where id = :i and name = :t and active = :b and ${cast(":nul", "int")} is null`, ["i", "t", "b", "nul"], "one"),
    omitted: q(`select id from ${t("people")} where id = :i and ${cast(":omit", "text")} is null`, ["i", "omit"], "many"),
    many: q(`select id, name from ${t("people")} order by id`, [], "many"),
    one: q(`select id, name from ${t("people")} where id = :id`, ["id"], "one"),
    maybeOne: q(`select id, name from ${t("people")} where id = :id`, ["id"], "maybeOne"),
    maybeOneOfTwo: q(`select id from ${t("people")} where id <= :id order by id`, ["id"], "maybeOne"),
    insert: q(`insert into ${t("scratch")} (tag, n) values (:tag, :n)`, ["tag", "n"], "none"),
    returning: q(`insert into ${t("scratch")} (tag, n) values (:tag, :n) returning tag, n`, ["tag", "n"], "one"),
    byTag: q(`select n from ${t("scratch")} where tag = :tag`, ["tag"], "one"),
    repeated: q(repeatedSql, ["id"], "one"),
    repeatedMapper: q(repeatedSql, ["id"], "one", { text: repeatedText, order: d === "postgres" ? ["id"] : ["id", "id"] }),
    syntax: q(`selec 1`, [], "many"),
    dup: q(`insert into ${t("uniq")} (k) values (:k)`, ["k"], "none"),
  };
}

const PEOPLE = [
  { id: 1, name: "ada" },
  { id: 2, name: "bob" },
  { id: 3, name: "cy" },
];

let tagCounter = 0;
const runId = `${Date.now().toString(36)}${Math.random().toString(36).slice(2, 6)}`;
function newTag(ctx: Ctx, scenario: string): string {
  return `${ctx.job.pkg}@${ctx.job.version}/${ctx.shape.name}/${scenario}/${runId}/${++tagCounter}`.slice(0, 190);
}

// ---------------------------------------------------------------------------------------------
// Scenarios

type Scenario = {
  id: string;
  /** "client" scenarios run inside shape.scope; "tx" ones use shape.tx; "syncTx" ones use shape.syncTx. */
  on: "client" | "tx" | "syncTx";
  dialects?: Dialect[];
  /** false for a scenario whose statements take no parameters. */
  params?: false;
  run(ctx: Ctx, client: Client, q: ReturnType<typeof queries>): Promise<string | void>;
};

const SENTINEL = "dm-sentinel-rollback";
export function sentinel(): Error {
  return new Error(SENTINEL);
}
export function isSentinel(e: unknown): boolean {
  return e instanceof Error && e.message.includes(SENTINEL);
}
export function ignoreSentinel(e: unknown): void {
  if (!isSentinel(e)) throw e;
}

/** An INSERT with its values written into the text, so the statement takes no parameters. */
function literalInsert(d: Dialect, tag: string, n: number) {
  const table = d === "sqlite" ? "scratch" : "driver_matrix.scratch";
  return defineQuery<any, any, any, any>({ dialect: d, sql: `insert into ${table} (tag, n) values ('${tag.replace(/'/g, "''")}', ${n})`, params: [], cardinality: "none" });
}

const S9 = "S9 unsupported client: run() rejects with dotsql's TypeError";

export const SCENARIOS: Scenario[] = [
  {
    id: "S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere",
    on: "client",
    params: false,
    async run(ctx, client, q) {
      let r: unknown;
      let threw: unknown = null;
      const via = spy(client);
      try {
        r = q.many.run(client);
      } catch (e) {
        threw = e;
      }
      if (threw) throw new Violation(`run threw synchronously: ${describe(threw)}`);
      if (!isThenable(r)) throw new Violation(`run returned ${r === null ? "null" : typeof r}; expected a promise`);
      await r;
      const dispatched = via.stop();
      let errThrew = false;
      let e2: unknown;
      try {
        e2 = q.syntax.run(client);
      } catch {
        errThrew = true;
      }
      if (errThrew) throw new Violation("run threw synchronously on a failing statement; expected a rejected promise");
      await (e2 as Promise<unknown>).then(() => {}, () => {});
      if (ctx.kind === "sync") {
        const rows = q.many.runSync(client);
        if (isThenable(rows)) throw new Violation("runSync returned a promise on a synchronous SQLite client");
        let syncThrew = false;
        try {
          q.syntax.runSync(client);
        } catch {
          syncThrew = true;
        }
        if (!syncThrew) throw new Violation("runSync did not throw on a failing statement");
        return `dotsql called ${dispatched}; run returned a promise and rejected on a failing statement; runSync returned the value and threw`;
      }
      const sent = spy(client);
      let refused: unknown = null;
      try {
        q.many.runSync(client);
      } catch (e) {
        refused = e;
      }
      const calledByRunSync = sent.stop();
      if (!(refused instanceof TypeError) || !/^runSync takes a synchronous SQLite client/.test(refused.message)) throw new Violation(`runSync on an asynchronous client did not throw dotsql's TypeError: ${describe(refused)}`);
      if (!calledByRunSync.startsWith("no method") && !calledByRunSync.startsWith("the client as a function")) throw new Violation(`runSync called ${calledByRunSync} before refusing the client`);
      return `dotsql called ${dispatched}; run returned a promise and rejected on a failing statement; runSync refused the client before calling it`;
    },
  },
  {
    id: "S1a params: int, text, bool, null",
    on: "client",
    async run(ctx, client, q) {
      const d = ctx.job.dialect;
      // SQLite has no boolean type (dotsql types the column as a number, and better-sqlite3 and node:sqlite refuse a JS boolean).
      const row = await call(ctx, q.params, client, { i: 1, t: "ada", b: d === "sqlite" ? 1 : true, nul: null });
      eq(row, { id: 1, name: "ada", active: true, score: 10 }, "row");
    },
  },
  {
    id: "S1b omitted param (undefined) binds NULL",
    on: "client",
    async run(ctx, client, q) {
      const rows = await call(ctx, q.omitted, client, { i: 1 });
      eq(rows, [{ id: 1 }], "rows");
    },
  },
  {
    id: "S2a many: 3 rows",
    on: "client",
    params: false,
    async run(ctx, client, q) {
      eq(await call(ctx, q.many, client), PEOPLE, "rows");
    },
  },
  {
    id: "S2b one: the row",
    on: "client",
    async run(ctx, client, q) {
      eq(await call(ctx, q.one, client, { id: 2 }), PEOPLE[1], "row");
    },
  },
  {
    id: "S2c maybeOne: zero rows gives null",
    on: "client",
    async run(ctx, client, q) {
      const r = await call(ctx, q.maybeOne, client, { id: 999 });
      if (r !== null) throw new Mismatch(`expected null, got ${oneLine(safeJson(r))}`);
    },
  },
  {
    id: "S2d maybeOne: two rows throw dotsql's error",
    on: "client",
    async run(ctx, client, q) {
      try {
        const r = await call(ctx, q.maybeOneOfTwo, client, { id: 2 });
        throw new Mismatch(`expected dotsql's "at most one row" error, got ${oneLine(safeJson(r))}`);
      } catch (e) {
        if (e instanceof Mismatch || e instanceof Violation) throw e;
        if (!(e instanceof Error) || !/Expected at most one row, got 2/.test(e.message)) throw new Mismatch(`wrong error: ${describe(e)}`);
      }
    },
  },
  {
    id: "S2e with(client)(params)",
    on: "client",
    async run(ctx, client, q) {
      eq(await call(ctx, q.one, client, { id: 3 }, "with"), PEOPLE[2], "row");
    },
  },
  {
    id: "S3 write without RETURNING (none), then a separate read",
    on: "client",
    async run(ctx, client, q) {
      const tag = newTag(ctx, "S3");
      const r = await call(ctx, q.insert, client, { tag, n: 3 });
      if (r !== undefined) throw new Mismatch(`expected undefined for cardinality none, got ${oneLine(safeJson(r))}`);
      // Inside a transaction scope the row is not committed yet, so read it through the same client.
      eq(await call(ctx, q.byTag, client, { tag }), { n: 3 }, "read back through dotsql");
      if (!ctx.shape.inTransaction) eq(await ctx.job.count(tag), 1, "raw count");
    },
  },
  {
    id: "S3n write without parameters (none), then a separate read",
    on: "client",
    params: false,
    async run(ctx, client) {
      const tag = newTag(ctx, "S3n");
      const r = await call(ctx, literalInsert(ctx.job.dialect, tag, 7), client);
      if (r !== undefined) throw new Mismatch(`expected undefined for cardinality none, got ${oneLine(safeJson(r))}`);
      if (!ctx.shape.inTransaction) eq(await ctx.job.count(tag), 1, "raw count");
    },
  },
  {
    id: "S4 INSERT ... RETURNING",
    on: "client",
    dialects: ["postgres", "sqlite"],
    async run(ctx, client, q) {
      const tag = newTag(ctx, "S4");
      eq(await call(ctx, q.returning, client, { tag, n: 4 }), { tag, n: 4 }, "returned row");
    },
  },
  {
    id: "S5a repeated :id (runtime lexer)",
    on: "client",
    async run(ctx, client, q) {
      eq(await call(ctx, q.repeated, client, { id: 3 }), PEOPLE[2], "row");
    },
  },
  {
    id: "S5b repeated :id (mapper text/order)",
    on: "client",
    async run(ctx, client, q) {
      eq(await call(ctx, q.repeatedMapper, client, { id: 3 }), PEOPLE[2], "row");
    },
  },
  {
    id: "S6a syntax error rejects with the driver's error",
    on: "client",
    params: false,
    async run(ctx, client, q) {
      try {
        const r = await call(ctx, q.syntax, client);
        throw new Mismatch(`expected a syntax error, got ${oneLine(safeJson(r))}`);
      } catch (e) {
        return expectDriverError(e, ctx.job.dialect, "syntax");
      }
    },
  },
  {
    id: "S6b unique violation rejects with the driver's error",
    on: "client",
    async run(ctx, client, q) {
      try {
        const r = await call(ctx, q.dup, client, { k: "taken" });
        throw new Mismatch(`expected a unique violation, got ${oneLine(safeJson(r))}`);
      } catch (e) {
        return expectDriverError(e, ctx.job.dialect, "dup");
      }
    },
  },
  {
    id: "S7a tx: write then commit is visible",
    on: "tx",
    async run(ctx, _client, q) {
      const tag = newTag(ctx, "S7a");
      await ctx.shape.tx!.commit((c) => call(ctx, q.insert, c, { tag, n: 1 }));
      eq(await ctx.job.count(tag), 1, "raw count after commit");
    },
  },
  {
    id: "S7b tx: write then rollback is not visible",
    on: "tx",
    async run(ctx, _client, q) {
      const tag = newTag(ctx, "S7b");
      await ctx.shape.tx!.rollback((c) => call(ctx, q.insert, c, { tag, n: 1 }));
      eq(await ctx.job.count(tag), 0, "raw count after rollback");
    },
  },
  {
    id: "S7c tx: savepoint rollback keeps the outer write",
    on: "tx",
    async run(ctx, _client, q) {
      if (!ctx.shape.tx!.savepoint) return "skip: no savepoint API on this shape";
      const outer = newTag(ctx, "S7c-outer");
      const inner = newTag(ctx, "S7c-inner");
      await ctx.shape.tx!.savepoint(
        (c) => call(ctx, q.insert, c, { tag: outer, n: 1 }),
        (c) => call(ctx, q.insert, c, { tag: inner, n: 2 }),
      );
      eq([await ctx.job.count(outer), await ctx.job.count(inner)], [1, 0], "raw counts [outer, inner]");
    },
  },
  {
    id: "S7d tx: a failing statement rolls the transaction back",
    on: "tx",
    async run(ctx, _client, q) {
      const tag = newTag(ctx, "S7d");
      let err: unknown = null;
      try {
        await ctx.shape.tx!.failing(async (c) => {
          await call(ctx, q.insert, c, { tag, n: 1 });
          await call(ctx, q.dup, c, { k: "taken" });
        });
      } catch (e) {
        err = e;
      }
      if (err === null) throw new Mismatch("the transaction did not reject");
      const seen = expectDriverError(err, ctx.job.dialect, "dup");
      eq(await ctx.job.count(tag), 0, "raw count after the failed transaction");
      return seen;
    },
  },
  {
    id: "S7n tx: a write without parameters, then commit, is visible",
    on: "tx",
    params: false,
    async run(ctx, _client) {
      const tag = newTag(ctx, "S7n");
      await ctx.shape.tx!.commit((c) => call(ctx, literalInsert(ctx.job.dialect, tag, 1), c));
      eq(await ctx.job.count(tag), 1, "raw count after commit");
    },
  },
  {
    id: "S7a tx: write then commit is visible",
    on: "syncTx",
    async run(ctx, db, q) {
      const tag = newTag(ctx, "S7a");
      const trap = promiseTrap();
      db.transaction(() => {
        trap.check(q.insert.runSync(db, { tag, n: 1 }));
      })();
      trap.assertNone();
      eq(await ctx.job.count(tag), 1, "raw count after commit");
    },
  },
  {
    id: "S7b tx: write then rollback is not visible",
    on: "syncTx",
    async run(ctx, db, q) {
      const tag = newTag(ctx, "S7b");
      const trap = promiseTrap();
      try {
        db.transaction(() => {
          trap.check(q.insert.runSync(db, { tag, n: 1 }));
          throw sentinel();
        })();
      } catch (e) {
        ignoreSentinel(e);
      }
      trap.assertNone();
      eq(await ctx.job.count(tag), 0, "raw count after rollback");
    },
  },
  {
    id: "S7c tx: savepoint rollback keeps the outer write",
    on: "syncTx",
    async run(ctx, db, q) {
      const outer = newTag(ctx, "S7c-outer");
      const inner = newTag(ctx, "S7c-inner");
      const trap = promiseTrap();
      db.transaction(() => {
        trap.check(q.insert.runSync(db, { tag: outer, n: 1 }));
        try {
          // A transaction function called inside another runs as a savepoint.
          db.transaction(() => {
            trap.check(q.insert.runSync(db, { tag: inner, n: 2 }));
            throw sentinel();
          })();
        } catch (e) {
          ignoreSentinel(e);
        }
      })();
      trap.assertNone();
      eq([await ctx.job.count(outer), await ctx.job.count(inner)], [1, 0], "raw counts [outer, inner]");
    },
  },
  {
    id: "S7d tx: a failing statement rolls the transaction back",
    on: "syncTx",
    async run(ctx, db, q) {
      const tag = newTag(ctx, "S7d");
      const trap = promiseTrap();
      let err: unknown = null;
      try {
        db.transaction(() => {
          trap.check(q.insert.runSync(db, { tag, n: 1 }));
          trap.check(q.dup.runSync(db, { k: "taken" }));
        })();
      } catch (e) {
        err = e;
      }
      const leaked = await trap.settle();
      const count = await ctx.job.count(tag);
      if (err === null) {
        throw new Mismatch(`transaction() did not throw, so it committed (raw count ${count})${leaked.length ? `; runSync returned promises and the error arrived as a rejection instead: ${leaked.join("; ")}` : ""}`);
      }
      const seen = expectDriverError(err, ctx.job.dialect, "dup");
      eq(count, 0, "raw count after the failed transaction");
      return seen;
    },
  },
  {
    id: "S7e tx: a read inside transaction(fn) is usable synchronously",
    on: "syncTx",
    async run(ctx, db, q) {
      const tag = newTag(ctx, "S7e");
      const got = db.transaction(() => {
        q.insert.runSync(db, { tag, n: 5 });
        const row = q.byTag.runSync(db, { tag });
        if (isThenable(row)) {
          (row as Promise<unknown>).then(
            () => {},
            () => {},
          );
          return "a promise";
        }
        return row;
      })();
      if (got === "a promise") throw new Violation("runSync returned a promise inside transaction(fn); the row is not usable there");
      eq(got, { n: 5 }, "row read inside the transaction");
    },
  },
  {
    id: "S8 20 concurrent queries resolve with the right rows",
    on: "client",
    async run(ctx, client, q) {
      const ids = Array.from({ length: 20 }, (_, i) => (i % 3) + 1);
      const rows = await Promise.all(ids.map((id) => call(ctx, q.one, client, { id })));
      eq(
        rows.map((r: any) => r?.name),
        ids.map((id) => PEOPLE[id - 1]!.name),
        "names in order",
      );
    },
  },
];

/** Records which of the client's methods dotsql calls, by shadowing them with own properties until stop(). */
function spy(client: Client): { stop(): string } {
  const names = ["query", "unsafe", "execute", "all", "prepare", "batch", "promise", "manyOrNone"];
  const calls: string[] = [];
  const restore: (() => void)[] = [];
  for (const name of names) {
    try {
      const orig = client?.[name];
      if (typeof orig !== "function") continue;
      const own = Object.getOwnPropertyDescriptor(client, name);
      if (own && !own.configurable) continue;
      Object.defineProperty(client, name, {
        configurable: true,
        writable: true,
        value: function (this: unknown, ...rest: unknown[]) {
          calls.push(`${name}()`);
          return orig.apply(this, rest);
        },
      });
      restore.push(() => (own ? Object.defineProperty(client, name, own) : delete client[name]));
    } catch {
      // A native or frozen object: leave it alone.
    }
  }
  return {
    stop() {
      for (const undo of restore) undo();
      const seen = [...new Set(calls)];
      if (seen.length) return seen.join(" then ");
      return typeof client === "function" ? "the client as a function (or a method the harness could not observe)" : "no method the harness could observe";
    },
  };
}

/** Collects promises that runSync() returned inside a synchronous transaction(fn), where it must return values. */
function promiseTrap() {
  const promises: Promise<unknown>[] = [];
  return {
    check(v: unknown) {
      if (isThenable(v)) promises.push(Promise.resolve(v));
    },
    assertNone() {
      for (const p of promises) p.then(() => {}, () => {});
      if (promises.length) throw new Violation(`runSync returned ${promises.length} promise(s) inside transaction(fn); expected values`);
    },
    async settle(): Promise<string[]> {
      const out: string[] = [];
      for (const p of promises) {
        try {
          await p;
          out.push("resolved");
        } catch (e) {
          out.push(`rejected: ${describe(e)}`);
        }
      }
      return out;
    },
  };
}

// ---------------------------------------------------------------------------------------------
// Running a job

const SCENARIO_TIMEOUT = Number(process.env.DM_SCENARIO_TIMEOUT ?? 20000);

function timeout<T>(p: Promise<T>, ms: number, what: string): Promise<T> {
  let timer: ReturnType<typeof setTimeout>;
  return Promise.race([
    p.finally(() => clearTimeout(timer)),
    new Promise<T>((_, reject) => {
      timer = setTimeout(() => reject(new TimedOut(`timed out after ${ms / 1000} s: ${what} never settled`)), ms);
    }),
  ]);
}
class TimedOut extends Error {}

/** Lazily creates a resource once, so a shape whose setup hangs does not take the others with it. */
export function lazy<T>(make: () => Promise<T> | T): () => Promise<T> {
  let p: Promise<T> | undefined;
  return () => (p ??= Promise.resolve().then(make));
}

export type JobMeta = { pkg: string; version: string; target: string; live?: boolean };

/** Reports a job that could not start (the package does not load, the server is unreachable). */
export function notRun(meta: JobMeta, why: string): void {
  emit({ pkg: meta.pkg, version: meta.version, runtime, target: meta.target, shape: "(job)", scenario: "setup", status: "not run", live: meta.live ?? true, error: why });
}

export async function runJob(meta: JobMeta, setup: () => Promise<Job>): Promise<void> {
  process.on("unhandledRejection", (e) => {
    console.error(`unhandled rejection (ignored): ${describe(e)}`);
  });
  // A driver's process warnings (a deprecation, say) are attached to the scenario that raised them.
  const warnings: string[] = [];
  process.on("warning", (w) => {
    warnings.push(`${w.name}: ${oneLine(w.message, 200)}`);
  });
  let job: Job;
  try {
    job = await timeout(setup(), 60000, "setup");
  } catch (e) {
    emit({ pkg: meta.pkg, version: meta.version, runtime, target: meta.target, shape: "(job)", scenario: "setup", status: "fail", live: meta.live ?? true, error: describe(e) });
    finish(1);
    return;
  }
  // The raw count, without dotsql, first: a driver that cannot query the server at all (pg 7 on
  // Node 26, say) is reported once, not as a failure of every dotsql scenario.
  try {
    await timeout(job.count("dm-preflight"), 20000, "the raw count");
  } catch (e) {
    emit({ pkg: job.pkg, version: job.version, runtime, target: job.target, shape: "(job)", scenario: "preflight", status: "not run", live: meta.live ?? true, error: `the driver cannot run a query without dotsql: ${e instanceof TimedOut ? e.message : describe(e)}` });
    try {
      await timeout(Promise.resolve(job.close?.()), 5000, "close");
    } catch {}
    finish(0);
    return;
  }
  const q = queries(job.dialect);
  for (const shape of job.shapes) {
    const ctx: Ctx = { job, shape, kind: shape.kind, refusals: [] };
    const base = { pkg: job.pkg, version: job.version, runtime, target: job.target, shape: shape.name, live: shape.live ?? meta.live ?? true };
    if (shape.refused) {
      emit({ ...base, scenario: S9, ...(await checkRefused(shape, q)) });
      continue;
    }
    let timeouts = 0;
    for (const s of SCENARIOS) {
      if (s.dialects && !s.dialects.includes(job.dialect)) continue;
      if (s.on === "client" && !shape.scope) continue;
      if (s.on === "tx" && !shape.tx) continue;
      if (s.on === "syncTx" && !shape.syncTx) continue;
      const reason = shape.skip?.[s.id.split(" ")[0]!] ?? job.skip?.[s.id.split(" ")[0]!];
      if (reason) {
        emit({ ...base, scenario: s.id, status: "skip", detail: reason });
        continue;
      }
      if (timeouts >= 2) {
        emit({ ...base, scenario: s.id, status: "not run", error: "earlier scenarios on this shape timed out" });
        continue;
      }
      warnings.length = 0;
      ctx.refusals.length = 0;
      const refusalExpected = !!shape.paramsRefused && s.params !== false;
      try {
        let detail: string | void = undefined;
        let done = false;
        const work =
          s.on === "client"
            ? shape.scope!(async (client) => {
                detail = await s.run(ctx, client, q);
                done = true;
              }).catch((e) => {
                if (!done) throw e;
                // A statement that failed inside an open transaction aborts it (Postgres), so the
                // shape's own commit then fails: the driver's semantics, not dotsql's.
                if (shape.inTransaction) {
                  detail = [detail, `then the enclosing transaction failed, as the database aborts it after an error: ${describe(e)}`].filter(Boolean).join("; ");
                  return;
                }
                throw new Error(`the scenario passed, then the shape's teardown failed: ${describe(e)}`);
              })
            : s.run(ctx, s.on === "syncTx" ? shape.syncTx : null, q).then((d) => {
                detail = d;
              });
        await timeout(work, SCENARIO_TIMEOUT, "the scenario");
        await new Promise((r) => setImmediate(r));
        const d = [detail as string | void, ...warnings.map((w) => `warning: ${w}`)].filter(Boolean).join("; ") || undefined;
        if (refusalExpected) emit({ ...base, scenario: s.id, ...refusalResult(ctx, null) });
        else if (typeof d === "string" && d.startsWith("skip: ")) emit({ ...base, scenario: s.id, status: "skip", detail: d.slice(6) });
        else emit({ ...base, scenario: s.id, status: "pass", ...(d ? { detail: d } : {}) });
      } catch (e) {
        if (refusalExpected) {
          emit({ ...base, scenario: s.id, ...refusalResult(ctx, e) });
          continue;
        }
        if (e instanceof TimedOut) timeouts++;
        const msg = e instanceof Mismatch || e instanceof TimedOut ? e.message : e instanceof Violation ? `dotsql contract: ${e.message}` : describe(e);
        emit({ ...base, scenario: s.id, status: "fail", error: msg, ...(warnings.length ? { detail: warnings.map((w) => `warning: ${w}`).join("; ") } : {}) });
      }
    }
  }
  try {
    await timeout(Promise.resolve(job.close?.()), 10000, "close");
  } catch (e) {
    console.error(`close failed: ${describe(e)}`);
  }
  finish(0);
}

/** A statement with parameters on a client that binds none: dotsql must refuse it with its TypeError, carrying the driver's error as cause. */
function refusalResult(ctx: Ctx, e: unknown): { status: Status; error?: string; detail?: string } {
  const refusal = ctx.refusals[0] as (TypeError & { cause?: unknown }) | undefined;
  if (refusal) {
    if (!(refusal.cause instanceof Error)) return { status: "fail", error: `dotsql refused the statement, but its TypeError carries no driver error as cause: ${describe(refusal)}` };
    return { status: "unsupported", detail: `dotsql refused it: ${oneLine(refusal.message, 160)} (cause: ${describe(refusal.cause)})` };
  }
  if (e === null) return { status: "fail", error: "expected dotsql to refuse a statement with parameters on this client, but the scenario ran" };
  return { status: "fail", error: `expected dotsql's refusal of a statement with parameters, got: ${e instanceof Mismatch || e instanceof TimedOut ? e.message : describe(e)}` };
}

async function checkRefused(shape: Shape, q: ReturnType<typeof queries>): Promise<{ status: Status; error?: string; detail?: string }> {
  let r: unknown;
  try {
    r = q.many.run(shape.scope ? await captureClient(shape) : null);
  } catch (e) {
    return { status: "fail", error: `run threw synchronously instead of rejecting: ${describe(e)}` };
  }
  if (!isThenable(r)) return { status: "fail", error: `run returned ${typeof r}, not a rejected promise` };
  try {
    const rows = await timeout(Promise.resolve(r), SCENARIO_TIMEOUT, "run");
    return { status: "fail", error: `dotsql ran the statement on an unsupported client and resolved ${oneLine(safeJson(rows))}` };
  } catch (e) {
    if (e instanceof TypeError && shape.refused!.test(e.message)) return { status: "unsupported", detail: `dotsql refused it: ${oneLine(e.message, 1000)}` };
    return { status: "fail", error: `not dotsql's refusal: ${describe(e)}` };
  }
}

/** The client object a shape's scope hands out, for a check that runs outside it. */
function captureClient(shape: Shape): Promise<Client> {
  return new Promise((resolve, reject) => {
    shape.scope!(async (c) => resolve(c)).catch(reject);
  });
}

function finish(code: number): void {
  if (out) appendFileSync(out, JSON.stringify({ done: true }) + "\n");
  // Some drivers keep sockets or workers open; the job is over.
  setTimeout(() => process.exit(code), 50);
}

// ---------------------------------------------------------------------------------------------
// Transaction helpers shared by the drivers

/** BEGIN / COMMIT / ROLLBACK issued by hand on one connection (pg PoolClient, mysql2 and mariadb beginTransaction, node:sqlite exec). */
export function manualTx(opts: {
  open: () => Promise<{ client: Client; release?: () => unknown }>;
  begin: (c: Client) => Promise<unknown> | unknown;
  commit: (c: Client) => Promise<unknown> | unknown;
  rollback: (c: Client) => Promise<unknown> | unknown;
  exec?: (c: Client, sql: string) => Promise<unknown> | unknown;
}): TxOps {
  const inTx = async (body: (c: Client) => Promise<unknown>) => {
    const { client, release } = await opts.open();
    try {
      await opts.begin(client);
      try {
        await body(client);
        await opts.commit(client);
      } catch (e) {
        await Promise.resolve(opts.rollback(client)).catch(() => {});
        throw e;
      }
    } finally {
      await release?.();
    }
  };
  const ops: TxOps = {
    commit: (body) => inTx(body),
    rollback: async (body) => {
      const { client, release } = await opts.open();
      try {
        await opts.begin(client);
        await body(client);
        await opts.rollback(client);
      } finally {
        await release?.();
      }
    },
    failing: (body) => inTx(body),
  };
  if (opts.exec) {
    const exec = opts.exec;
    ops.savepoint = (outer, inner) =>
      inTx(async (c) => {
        await outer(c);
        await exec(c, "SAVEPOINT dm_sp");
        await inner(c);
        await exec(c, "ROLLBACK TO SAVEPOINT dm_sp");
      });
  }
  return ops;
}

/** A driver whose transaction takes a callback and commits when it resolves (postgres.js begin, PGlite transaction, pg-promise tx, PlanetScale transaction). */
export function callbackTx(begin: (fn: (tx: Client) => Promise<unknown>) => Promise<unknown>, savepoint?: (tx: Client, fn: (sp: Client) => Promise<unknown>) => Promise<unknown>): TxOps {
  const ops: TxOps = {
    commit: (body) => begin((tx) => body(tx)),
    rollback: (body) =>
      begin(async (tx) => {
        await body(tx);
        throw sentinel();
      }).catch(ignoreSentinel),
    failing: (body) => begin((tx) => body(tx)),
  };
  if (savepoint) {
    ops.savepoint = (outer, inner) =>
      begin(async (tx) => {
        await outer(tx);
        await savepoint(tx, async (sp) => {
          await inner(sp);
          throw sentinel();
        }).catch(ignoreSentinel);
      });
  }
  return ops;
}

/** A savepoint by hand inside a callback transaction whose API has none (PGlite, PlanetScale, libSQL). */
export function rawSavepoint(exec: (tx: Client, sql: string) => Promise<unknown>) {
  return async (tx: Client, fn: (sp: Client) => Promise<unknown>) => {
    await exec(tx, "SAVEPOINT dm_sp");
    try {
      await fn(tx);
      await exec(tx, "RELEASE SAVEPOINT dm_sp");
    } catch (e) {
      await exec(tx, "ROLLBACK TO SAVEPOINT dm_sp");
      throw e;
    }
  };
}
