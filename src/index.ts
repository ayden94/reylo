// The runtime query object behind `import query from "./file.sql"`.
// Types come from the content mapper; values come from the runtime loader.
import { execute, executeSync, type ClientFor, type Dialect, type SyncClientFor } from "./execute.ts";
import type { StandardJSONSchemaV1, StandardTypedV1 } from "./standard.ts";

export type { ClientFor, Dialect, SyncClientFor } from "./execute.ts";

/** How many rows a statement returns: `none` is a data-modifying statement without RETURNING. */
export type Cardinality = "many" | "one" | "maybeOne" | "none";

export type Materialize<Row, C extends Cardinality> = C extends "many" ? Row[] : C extends "one" ? Row : C extends "none" ? void : Row | null;

/** The JSON.stringify shape of a row value: a Date, a Temporal value, a BigInt or a byte array becomes text. */
export type Json<T> = T extends Date | bigint | Uint8Array | { toJSON(): string }
  ? string
  : T extends readonly (infer U)[]
    ? Json<U>[]
    : T extends object
      ? { [K in keyof T]: Json<T[K]> }
      : T;

export type PorgTypes<Params, Row, C extends Cardinality> = {
  readonly element: Row;
  readonly cardinality: C;
  readonly json: Materialize<Json<Row>, C>;
  readonly input: Params;
};

export interface Query<Params, Row, C extends Cardinality, D extends Dialect>
  extends StandardTypedV1<Params, Materialize<Row, C>>, StandardJSONSchemaV1<Params, Materialize<Row, C>> {
  readonly dialect: D;
  /** The statement as written, with porg's placeholders left in place. */
  readonly sql: string;
  /** Parameter names in binding order. */
  readonly params: readonly string[];
  readonly cardinality: C;
  readonly "~standard": StandardJSONSchemaV1.Props<Params, Materialize<Row, C>>;
  readonly "~porg": PorgTypes<Params, Row, C>;
  /**
   * Runs the statement on the client, or on a transaction or connection taken from it; the
   * parameters may be left out when the statement takes none. Always a promise, which rejects
   * with the driver's own error.
   */
  run(client: ClientFor<D>, ...params: ParamsArg<Params>): Promise<Materialize<Row, C>>;
  /**
   * Runs the statement on a synchronous SQLite client (better-sqlite3, node:sqlite, bun:sqlite)
   * and returns the result itself, throwing the driver's error, so it works inside their
   * `db.transaction(fn)`, whose function cannot await.
   */
  runSync(client: SyncClientFor<D>, ...params: ParamsArg<Params>): Materialize<Row, C>;
  /** The statement bound to a client, such as a transaction: `const get = q.with(tx); await get({ id: 1 })`. */
  with(client: ClientFor<D>): (...params: ParamsArg<Params>) => Promise<Materialize<Row, C>>;
}

/** The parameters argument: required when the statement takes any, optional when it takes none. */
export type ParamsArg<Params> = {} extends Params ? [params?: Params] : [params: Params];

export type QueryDefinition = {
  dialect: Dialect;
  sql: string;
  params: readonly string[];
  /** The statement as the engine rewrote it for the database, and its binding order: every build step puts them in the module. */
  text?: string;
  order?: readonly string[];
  cardinality?: Cardinality;
  jsonSchema?: { input: Record<string, unknown>; output: Record<string, unknown> };
};

export function defineQuery<Params, Row, C extends Cardinality, D extends Dialect>(
  def: QueryDefinition & { dialect: D; cardinality?: C },
): Query<Params, Row, C, D> {
  const cardinality = (def.cardinality ?? "many") as C;
  const missingSchema = () => {
    throw new Error(`No JSON Schema is available for this query yet; run tsc --runExternalCode so the mapper can compute it.`);
  };
  const query: Query<Params, Row, C, D> & { text?: string | undefined; order?: readonly string[] | undefined } = {
    dialect: def.dialect,
    sql: def.sql,
    params: def.params,
    text: def.text,
    order: def.order,
    cardinality,
    "~standard": {
      version: 1,
      vendor: "porg",
      jsonSchema: {
        input: () => def.jsonSchema?.input ?? missingSchema(),
        output: () => def.jsonSchema?.output ?? missingSchema(),
      },
    },
    "~porg": undefined as unknown as PorgTypes<Params, Row, C>,
    run: ((client: ClientFor<D>, params?: Params) => run(client, params ?? ({} as Params))) as Query<Params, Row, C, D>["run"],
    runSync: ((client: SyncClientFor<D>, params?: Params) => materialize(executeSync(client, query, (params ?? {}) as Record<string, unknown>) as Row[])) as Query<Params, Row, C, D>["runSync"],
    with: ((client: ClientFor<D>) => (params?: Params) => run(client, params ?? ({} as Params))) as Query<Params, Row, C, D>["with"],
  };
  // A synchronous client answers before run returns, but its rows and its error still come
  // back through the promise, so run means the same on every client.
  function run(client: ClientFor<D>, params: Params): Promise<Materialize<Row, C>> {
    try {
      const rows = execute(client, query, params as Record<string, unknown>);
      return Array.isArray(rows) ? Promise.resolve(materialize(rows as Row[])) : rows.then((rows) => materialize(rows as Row[]));
    } catch (error) {
      return Promise.reject(error);
    }
  }
  function materialize(rows: Row[]): Materialize<Row, C> {
    switch (cardinality) {
      case "none":
        return undefined as Materialize<Row, C>;
      case "many":
        return rows as Materialize<Row, C>;
      case "one":
        if (rows.length !== 1) throw new Error(`Expected exactly one row, got ${rows.length}`);
        return rows[0] as Materialize<Row, C>;
      default:
        if (rows.length > 1) throw new Error(`Expected at most one row, got ${rows.length}`);
        return (rows[0] ?? null) as Materialize<Row, C>;
    }
  }
  return query;
}
