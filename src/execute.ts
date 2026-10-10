// Runtime execution over a caller-supplied client, typed structurally so that
// reylo has no dependency (not even a type-only one) on any driver package.
// Each type is the smallest slice of the real driver that execution needs, and
// the query's dialect selects which clients are accepted at compile time. A
// transaction, savepoint, reserved or checked-out connection has the same shape
// as the client it came from, so it is accepted wherever that client is.

import { queryPostgres } from "./decode.ts";
import { lexPlaceholders } from "./lex.ts";

export type Dialect = "postgres" | "mysql" | "sqlite";

/**
 * pg's Pool, Client and PoolClient; PGlite and its transactions; Neon's Pool and Client;
 * pg-promise's database, tasks and transactions; @vercel/postgres: query(text, values) → { rows }
 * (pg-promise resolves to the rows themselves, and its types say any).
 */
export type PgQueryClient = {
  query(text: string, values?: any[]): PromiseLike<{ rows: unknown[] }>;
  /** mariadb's query() resolves to any as well; its escapeId() tells it apart. */
  escapeId?: never;
};
/**
 * Neon's neon() function from 1.0: query(text, values) → rows. Its transaction() tells it from
 * mysql2/promise, whose query(text, values) resolves to a [rows, fields] array as well.
 */
export type NeonHttpClient = {
  query(text: string, values?: any[]): PromiseLike<Record<string, unknown>[]>;
  transaction(...args: any[]): unknown;
};
/** postgres.js `sql`, its begin and savepoint transactions and reserved connections; Bun.sql, likewise: unsafe(text, params) → rows. */
export type UnsafeClient = {
  unsafe(text: string, params?: any[]): PromiseLike<unknown[]>;
};
/** Bun.sql on MySQL or SQLite; commitDistributed() tells it from postgres.js, which has unsafe() too. */
export type BunSqlClient = UnsafeClient & { commitDistributed(name: string): Promise<unknown> };
/** Neon's neon() function before 1.0, which is called with (text, params). */
export type CallableClient = (text: string, params?: any[]) => PromiseLike<unknown[] | { rows: unknown[] }>;
/**
 * mysql2/promise Pool, Connection and PoolConnection (→ [rows, fields]); PlanetScale's Connection,
 * Client and transaction from 1.x (→ { rows }). PlanetScale 2.0 binds no parameters, so its
 * execute(query, options) is not one of these.
 */
export type MysqlExecuteClient = {
  execute(text: string, values?: any[]): PromiseLike<[unknown, unknown] | { rows: unknown[]; statement: string }>;
};
/** mysql2's callback Pool, Connection and PoolConnection, run through their promise() wrapper. */
export type Mysql2CallbackClient = { promise(): MysqlExecuteClient };
/** mariadb's Pool, Connection and PoolConnection: query(text, values) → rows, or an OK packet for a write. */
export type MariadbClient = {
  query(text: string, values?: any): Promise<unknown>;
  batch(text: string, values?: any): Promise<unknown>;
};
/** The same from mariadb's callback API (`mariadb/callback`): query(text, values, callback). */
export type MariadbCallbackClient = {
  query(text: string, values: any, callback: (error: Error | null, rows?: any) => void): void;
  batch(text: string, values: any, callback: (error: Error | null, result?: any) => void): void;
};
/**
 * better-sqlite3, node:sqlite's DatabaseSync and bun:sqlite's Database, which answer at once:
 * `runSync` takes them and returns the result itself, so it works inside their
 * `db.transaction(fn)`, whose function must not await.
 */
export type SyncSqliteClient = {
  prepare(sql: string): { all(...params: any[]): unknown[] };
};
/** Cloudflare D1's database and sessions: prepare(sql).bind(...values).all() → { results }. */
export type D1Client = {
  prepare(sql: string): { bind(...values: any[]): { all(): Promise<{ results: unknown[] }> } };
};
/** @libsql/client's Client and Transaction: execute({ sql, args }) → { rows }. */
export type LibsqlClient = {
  execute(statement: { sql: string; args: any[] }): Promise<{ rows: unknown[] }>;
  /** mariadb's execute() takes an object with sql and resolves to any; executeMultiple() tells libSQL apart. */
  executeMultiple(sql: string): Promise<unknown>;
};
/** The sqlite3 package's callback Database: all(sql, params, callback). */
export type Sqlite3Client = {
  all(sql: string, params: any[], callback: (error: Error | null, rows: any[]) => void): unknown;
};
/** The sqlite package's promise Database: all(sql, params) → rows. */
export type SqlitePromiseClient = {
  all(sql: string, params?: any): Promise<unknown[]>;
  getDatabaseInstance(): unknown;
};

export type ClientFor<D extends Dialect> = D extends "postgres"
  ? PgQueryClient | NeonHttpClient | UnsafeClient | CallableClient
  : D extends "mysql"
    ? MysqlExecuteClient | Mysql2CallbackClient | MariadbClient | MariadbCallbackClient | BunSqlClient
    : SyncSqliteClient | D1Client | LibsqlClient | Sqlite3Client | SqlitePromiseClient | BunSqlClient;

/** The clients `runSync` takes: the synchronous SQLite drivers, and none for Postgres or MySQL. */
export type SyncClientFor<D extends Dialect> = D extends "sqlite" ? SyncSqliteClient : never;

export type QueryModule<D extends Dialect = Dialect> = {
  readonly dialect: D;
  /** SQL with placeholders in the dialect's native positional form ($1 / ?). */
  readonly sql: string;
  /** Parameter names in placeholder order, so an object of params binds positionally. */
  readonly params: readonly string[];
  /** The statement as the mapper's engine rewrote it for the database, when the mapper ran. */
  readonly text?: string | undefined;
  /** The binding order for `text`: one name per placeholder of the rewritten statement. */
  readonly order?: readonly string[] | undefined;
};

type AnyClient = Record<string, any> & ((...args: any[]) => any);

/**
 * The parameters: an object keyed by name, or an array whose element i binds the query's
 * `params[i]` (for `$1` and `$2`, `params` is `["1", "2"]`), for callers that hold the values
 * in order already.
 */
export type ParamsInput = Record<string, unknown> | readonly unknown[];

/**
 * Runs the statement and returns its rows (an empty array for a statement that returns none).
 * On a synchronous SQLite client the rows come back at once and an error is thrown; on every
 * other client they come back as a promise, which rejects with the driver's own error.
 */
export function execute<D extends Dialect>(
  client: ClientFor<D>,
  query: QueryModule<D>,
  params: ParamsInput = {},
): unknown[] | Promise<unknown[]> {
  const c = client as AnyClient;
  if (query.dialect === "sqlite" && isSyncSqlite(c)) return executeSyncSqlite(c, query, params);
  return executeAsync(c, query, params);
}

/** The synchronous path alone, for `runSync`: any other client is refused before anything runs. */
export function executeSync<D extends Dialect>(client: SyncClientFor<D>, query: QueryModule<D>, params: ParamsInput = {}): unknown[] {
  const c = client as unknown as AnyClient;
  const refuse = (why: string) => new TypeError(`runSync takes a synchronous SQLite client (better-sqlite3, node:sqlite or bun:sqlite); ${why}. run takes every client.`);
  if (query.dialect !== "sqlite") throw refuse(`a ${query.dialect} statement has none`);
  if (!isSyncSqlite(c)) throw refuse("this client answers through a promise");
  const rows = executeSyncSqlite(c, query, params);
  if (Array.isArray(rows)) return rows;
  rows.catch(() => {});
  throw refuse("this client's statement answered with a promise");
}

function executeSyncSqlite(c: AnyClient, query: QueryModule, params: ParamsInput): unknown[] | Promise<unknown[]> {
  const { text, values } = statement(query, params);
  const prepared = c.prepare(text);
  // better-sqlite3 refuses all() on a statement that returns no rows.
  if (prepared.reader === false) {
    prepared.run(...values);
    return [];
  }
  return prepared.all(...values);
}

// A SQLite client with prepare() whose statements answer at once. D1 (batch()), sqlite3 and
// sqlite (all() on the database), libSQL (execute()) and Bun.sql (unsafe()) are asynchronous.
function isSyncSqlite(c: AnyClient | null | undefined): boolean {
  return (
    c != null &&
    typeof c.prepare === "function" &&
    typeof c.batch !== "function" &&
    typeof c.all !== "function" &&
    typeof c.execute !== "function" &&
    typeof c.unsafe !== "function"
  );
}

async function executeAsync(c: AnyClient | null | undefined, query: QueryModule, params: ParamsInput): Promise<unknown[]> {
  if (c == null || (typeof c !== "object" && typeof c !== "function")) throw unsupported(c, query.dialect);
  const { text, values } = statement(query, params);
  switch (query.dialect) {
    case "postgres":
      // query() before unsafe(): Neon's neon() has both, and its unsafe() builds a fragment
      // for a template instead of running anything. Where the driver takes a decoder per
      // query, reylo passes its own (decode.ts).
      if (typeof c.query === "function") return await queryPostgres(c, text, values);
      if (typeof c.unsafe === "function") return await c.unsafe(text, values);
      if (typeof c === "function") {
        const result = await c(text, values);
        return Array.isArray(result) ? result : result.rows;
      }
      break;
    case "mysql": {
      if (typeof c.unsafe === "function") return await c.unsafe(text, values);
      // mariadb, checked before execute() because it has one too (returning rows, not a pair).
      // Its callback API (mariadb/callback) answers through the callback and returns a command
      // object; its promise API returns a promise and ignores the callback.
      if (typeof c.batch === "function" && typeof c.query === "function") {
        const result = await new Promise((resolve, reject) => {
          const returned = c.query(text, values, (error: Error | null, rows: unknown) => (error ? reject(error) : resolve(rows)));
          if (typeof returned?.then === "function") returned.then(resolve, reject);
        });
        return Array.isArray(result) ? result : [];
      }
      // mysql2's callback API: the promise() wrapper runs on the same connection.
      const m = typeof c.promise === "function" ? c.promise() : c;
      if (typeof m.execute === "function") {
        // mysql2 resolves to [rows, fields], PlanetScale to { rows }; a write's rows are an OK packet.
        // A statement without parameters gets no values, which PlanetScale 2.0 would refuse.
        let result;
        try {
          result = await (values.length ? m.execute(text, values) : m.execute(text));
        } catch (error) {
          if (!/^Query parameters are not supported/.test((error as Error)?.message)) throw error;
          throw new TypeError(`@planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint.`, { cause: error });
        }
        const rows = Array.isArray(result) ? result[0] : result.rows;
        return Array.isArray(rows) ? rows : [];
      }
      break;
    }
    case "sqlite": {
      if (typeof c.unsafe === "function") return await c.unsafe(text, values);
      if (typeof c.execute === "function") return (await c.execute({ sql: text, args: values })).rows;
      if (typeof c.getDatabaseInstance === "function") return await c.all(text, values);
      if (typeof c.all === "function" && typeof c.exec === "function") {
        return await new Promise<unknown[]>((resolve, reject) =>
          c.all(text, values, (error: Error | null, rows: unknown[]) => (error ? reject(error) : resolve(rows))),
        );
      }
      if (typeof c.prepare === "function") {
        const prepared = c.prepare(text);
        if (typeof prepared.first === "function") return (await prepared.bind(...values).all()).results;
        return await prepared.all(...values);
      }
      break;
    }
  }
  throw unsupported(c, query.dialect);
}

// The rows are bound positionally in every dialect. A value left out binds NULL, as pg,
// PGlite, better-sqlite3 and Bun.sql already do; postgres.js, mysql2, node:sqlite and libSQL
// would reject it instead. An array shorter than `params` leaves the rest out.
function statement(query: QueryModule, input: ParamsInput) {
  let params: Record<string, unknown>;
  if (Array.isArray(input)) {
    if (input.length > query.params.length) {
      throw new TypeError(`The statement takes ${query.params.length} parameter${query.params.length === 1 ? "" : "s"} (${query.params.map((name) => JSON.stringify(name)).join(", ") || "none"}); got an array of ${input.length} value${input.length === 1 ? "" : "s"}.`);
    }
    params = Object.fromEntries(query.params.map((name, i) => [name, input[i]]));
  } else if (input === null || typeof input !== "object") {
    throw new TypeError(`The parameters must be an object keyed by parameter name (or by number, { 1: ... }, for positional placeholders), or an array in the order of the query's params; got ${String(input)}.`);
  } else params = input as Record<string, unknown>;
  // The mapper already rewrote the statement; only the values need ordering.
  if (query.text && query.order) return { text: query.text, values: query.order.map((name) => value(params, name)) };
  return query.dialect === "postgres" ? rewrite(query, params) : rewriteRepeating(query, params);
}

function value(params: Record<string, unknown>, name: string) {
  const v = params[name];
  return v === undefined ? null : v;
}

// Without the engine's rewritten statement (a query defined by hand) the runtime lexes the
// placeholders itself (`lex.ts`); the engine's lexer is the authoritative one, and the two agree
// on every statement the runtime fallback is meant for. Postgres numbers each distinct parameter
// once.
function rewrite(query: QueryModule, params: Record<string, unknown>) {
  const { placeholders } = lexPlaceholders(query.sql, query.dialect);
  let text = "";
  let last = 0;
  for (const p of placeholders) {
    text += query.sql.slice(last, p.start) + `$${indexOf(query, p) + 1}`;
    last = p.end;
  }
  return { text: text + query.sql.slice(last), values: query.params.map((name) => value(params, name)) };
}

// MySQL has only `?`, and better-sqlite3 reads SQLite's `?NNN` as a named parameter, so both
// get one `?` per placeholder and a repeated name is repeated in the value list.
function rewriteRepeating(query: QueryModule, params: Record<string, unknown>) {
  const { placeholders } = lexPlaceholders(query.sql, query.dialect);
  let text = "";
  let last = 0;
  const values: unknown[] = [];
  for (const p of placeholders) {
    indexOf(query, p);
    text += query.sql.slice(last, p.start) + "?";
    values.push(value(params, p.name));
    last = p.end;
  }
  return { text: text + query.sql.slice(last), values };
}

function indexOf(query: QueryModule, p: { name: string; start: number; end: number }) {
  const index = query.params.indexOf(p.name);
  // A hand-built query whose `params` misses a placeholder would bind `$0`.
  if (index === -1) throw new Error(`The statement's placeholder ${query.sql.slice(p.start, p.end)} is parameter "${p.name}", which the query's params (${JSON.stringify(query.params)}) do not list.`);
  return index;
}

const accepted: Record<Dialect, string> = {
  postgres: "query(text, values) as on pg, PGlite, Neon and pg-promise; unsafe(text, values) as on postgres.js and Bun.sql; or Neon's neon() function",
  mysql: "execute(text, values) as on mysql2 and PlanetScale; query() and batch() as on mariadb; unsafe(text, values) as on Bun.sql; or a mysql2 callback pool or connection",
  sqlite: "prepare(sql) as on better-sqlite3, node:sqlite, bun:sqlite and Cloudflare D1; execute({ sql, args }) as on @libsql/client; all(sql, params) as on sqlite3 and sqlite; or unsafe(text, values) as on Bun.sql",
};

function unsupported(c: unknown, dialect: Dialect): TypeError {
  let seen: string;
  if (c === null || (typeof c !== "object" && typeof c !== "function")) seen = String(c);
  else {
    const names = new Set<string>();
    for (let o: object | null = c; o && o !== Object.prototype && o !== Function.prototype; o = Object.getPrototypeOf(o)) {
      for (const key of Object.getOwnPropertyNames(o)) {
        const d = Object.getOwnPropertyDescriptor(o, key);
        if (key !== "constructor" && typeof d?.value === "function") names.add(key);
      }
    }
    seen = names.size ? `an object with ${[...names].slice(0, 12).join(", ")}${names.size > 12 ? ", ..." : ""}` : "an object with no methods";
  }
  // Drizzle's database keeps its driver client on $client (its transactions keep none), and
  // node:sqlite's tag store keeps its database on db.
  const o = c as { $client?: unknown; db?: { prepare?: unknown } } | null;
  const hint = o && typeof o === "object" && "$client" in o ? " A Drizzle database is not a client: pass db.$client." : typeof o?.db?.prepare === "function" ? " A tag store is not a client: pass its database, store.db." : "";
  return new TypeError(`reylo cannot run a ${dialect} statement on ${seen}. It takes a client with ${accepted[dialect]}.${hint}`);
}
