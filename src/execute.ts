// Runtime execution over a caller-supplied client, typed structurally so that
// lando has no dependency (not even a type-only one) on any driver package.
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
};

export async function execute<D extends Dialect>(
  client: ClientFor<D>,
  query: QueryModule<D>,
  params: Record<string, unknown> = {},
): Promise<unknown[]> {
  const c = client as Partial<PgLikeClient & UnsafeClient & Mysql2LikeClient & MariadbLikeClient & SqliteLikeClient>;
  switch (query.dialect) {
    case "sqlite": {
      // SQLite binds names natively; positional ones are passed in order.
      const { text, values } = rewrite(query, params, (i) => `?${i + 1}`);
      return c.prepare!(text).all(...values);
    }
    case "mysql": {
      // MySQL has only `?`, so a repeated name is repeated in the value list.
      const { text, values } = rewriteRepeating(query, params);
      if (typeof c.unsafe === "function") return await c.unsafe(text, values);
      if (typeof c.batch === "function") return (await c.query!(text, values)) as unknown as unknown[];
      return (await c.execute!(text, values))[0] as unknown[];
    }
    case "postgres": {
      const { text, values } = rewrite(query, params, (i) => `$${i + 1}`);
      if (typeof c.unsafe === "function") return await c.unsafe(text, values);
      return (await c.query!(text, values) as { rows: unknown[] }).rows;
    }
  }
}

function rewrite(query: QueryModule, params: Record<string, unknown>, placeholder: (index: number) => string) {
  const { placeholders } = lexPlaceholders(query.sql);
  let text = "";
  let last = 0;
  for (const p of placeholders) {
    text += query.sql.slice(last, p.start) + placeholder(query.params.indexOf(p.name));
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
