// Runtime execution over a caller-supplied client, typed structurally so that
// porg has no dependency (not even a type-only one) on any driver package.
// Each interface is the smallest slice of the real driver that execution needs,
// and the query's dialect selects which clients are accepted at compile time.

import { lexPlaceholders } from "./lex.ts";

export type Dialect = "postgres" | "mysql" | "sqlite";

/** node-postgres Pool/Client, PGlite: query(text, values) → { rows }. */
export type PgLikeClient = {
  query(text: string, values?: unknown[]): Promise<{ rows: unknown[] }>;
};
/** porsager/postgres `sql` and Bun.sql: unsafe(text, params) → rows. */
export type UnsafeClient = {
  unsafe(text: string, params?: unknown[]): PromiseLike<unknown[]>;
};
/** mysql2/promise Pool/Connection: execute(text, values) → [rows, fields]. */
export type Mysql2LikeClient = {
  execute(text: string, values?: unknown): Promise<[unknown, unknown]>;
};
/** mariadb connector Pool/Connection: query(text, values) → rows (with meta). */
export type MariadbLikeClient = {
  query(text: string, values?: unknown): Promise<unknown[]>;
  batch(text: string, values?: unknown): Promise<unknown>;
};
/** better-sqlite3, node:sqlite DatabaseSync, bun:sqlite Database: prepare(sql).all(params). */
export type SqliteLikeClient = {
  prepare(sql: string): { all(...params: any[]): unknown[] };
};

export type ClientFor<D extends Dialect> = D extends "postgres"
  ? PgLikeClient | UnsafeClient
  : D extends "mysql"
    ? Mysql2LikeClient | MariadbLikeClient | UnsafeClient
    : SqliteLikeClient;

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

export async function execute<D extends Dialect>(
  client: ClientFor<D>,
  query: QueryModule<D>,
  params: Record<string, unknown> = {},
): Promise<unknown[]> {
  const c = client as Partial<PgLikeClient & UnsafeClient & Mysql2LikeClient & MariadbLikeClient & SqliteLikeClient>;
  switch (query.dialect) {
    case "sqlite": {
      // The mapper's text has one `?` per placeholder, bound in order; without it, SQLite's
      // own numbered form.
      const { text, values } = query.text && query.order ? bind(query.text, query.order, params) : rewrite(query, params, (i) => `?${i + 1}`);
      return c.prepare!(text).all(...values);
    }
    case "mysql": {
      // MySQL has only `?`, so a repeated name is repeated in the value list.
      const { text, values } = query.text && query.order ? bind(query.text, query.order, params) : rewriteRepeating(query, params);
      if (typeof c.unsafe === "function") return await c.unsafe(text, values);
      if (typeof c.batch === "function") return (await c.query!(text, values)) as unknown as unknown[];
      return (await c.execute!(text, values))[0] as unknown[];
    }
    case "postgres": {
      const { text, values } = query.text && query.order ? bind(query.text, query.order, params) : rewrite(query, params, (i) => `$${i + 1}`);
      if (typeof c.unsafe === "function") return await c.unsafe(text, values);
      return (await c.query!(text, values) as { rows: unknown[] }).rows;
    }
  }
}

/** The mapper already rewrote the statement; only the values need ordering. */
function bind(text: string, order: readonly string[], params: Record<string, unknown>) {
  return { text, values: order.map((name) => params[name]) };
}

// Without the mapper's cache the runtime lexes the placeholders itself (`lex.ts`); the
// engine's lexer is the authoritative one, and the two agree on every statement the
// runtime fallback is meant for.
function rewrite(query: QueryModule, params: Record<string, unknown>, placeholder: (index: number) => string) {
  const { placeholders } = lexPlaceholders(query.sql);
  let text = "";
  let last = 0;
  for (const p of placeholders) {
    const index = query.params.indexOf(p.name);
    // A hand-built query whose `params` misses a placeholder would bind `$0`.
    if (index === -1) throw new Error(`The statement's placeholder ${query.sql.slice(p.start, p.end)} is parameter "${p.name}", which the query's params (${JSON.stringify(query.params)}) do not list.`);
    text += query.sql.slice(last, p.start) + placeholder(index);
    last = p.end;
  }
  return { text: text + query.sql.slice(last), values: query.params.map((name) => params[name]) };
}

function rewriteRepeating(query: QueryModule, params: Record<string, unknown>) {
  const { placeholders } = lexPlaceholders(query.sql);
  let text = "";
  let last = 0;
  const values: unknown[] = [];
  for (const p of placeholders) {
    text += query.sql.slice(last, p.start) + "?";
    values.push(params[p.name]);
    last = p.end;
  }
  return { text: text + query.sql.slice(last), values };
}
