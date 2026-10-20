// The runtime query object behind `import query from "./file.sql"`.
// Types come from the content mapper; values come from the runtime loader.
import { execute, executeSync, type ClientFor, type Dialect, type Inputs, type SyncClientFor } from "./execute.ts";
import type { StandardJSONSchemaV1, StandardTypedV1 } from "./standard.ts";
import type { Structured } from "./rows.ts";

export type { ClientFor, Dialect, Inputs, SyncClientFor } from "./execute.ts";

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

export type ReyloTypes<Params, Row, C extends Cardinality, Values extends readonly unknown[] = readonly unknown[]> = {
  readonly element: Row;
  readonly cardinality: C;
  readonly json: Materialize<Json<Row>, C>;
  readonly input: Params;
  /** The parameters as an array, in the order of `params`. */
  readonly values: Values;
};

export interface Query<Params, Row, C extends Cardinality, D extends Dialect, Values extends readonly unknown[] = readonly unknown[]>
  extends StandardTypedV1<Params, Materialize<Row, C>>, StandardJSONSchemaV1<Params, Materialize<Row, C>> {
  readonly dialect: D;
  /** The statement as written, with reylo's placeholders left in place. */
  readonly sql: string;
  /** Parameter names in binding order. */
  readonly params: readonly string[];
  readonly cardinality: C;
  /** The JSON Schema (2020-12) of the parameters, as the object `run` takes. */
  readonly inputSchema: Record<string, unknown>;
  /** The JSON Schema (2020-12) of the result as JSON: what `run` returns, once JSON.stringify has written it. */
  readonly outputSchema: Record<string, unknown>;
  readonly "~standard": StandardJSONSchemaV1.Props<Params, Materialize<Row, C>>;
  readonly "~reylo": ReyloTypes<Params, Row, C, Values>;
  /**
   * Runs the statement on the client, or on a transaction or connection taken from it. The
   * parameters are an object keyed by name, or an array in the order of `params`, and may be left
   * out when the statement takes none. Always a promise, which rejects with the driver's own error.
   */
  run(client: ClientFor<D>, ...params: ParamsArg<Params, Values>): Promise<Materialize<Row, C>>;
  /**
   * Runs the statement on a synchronous SQLite client (better-sqlite3, node:sqlite, bun:sqlite)
   * and returns the result itself, throwing the driver's error, so it works inside their
   * `db.transaction(fn)`, whose function cannot await.
   */
  runSync(client: SyncClientFor<D>, ...params: ParamsArg<Params, Values>): Materialize<Row, C>;
  /** The statement bound to a client, such as a transaction: `const get = q.with(tx); await get({ id: 1 })`. */
  with(client: ClientFor<D>): (...params: ParamsArg<Params, Values>) => Promise<Materialize<Row, C>>;
}

/** The parameters argument, an object or an array: required when the statement takes any, optional when it takes none. */
export type ParamsArg<Params, Values extends readonly unknown[] = readonly unknown[]> = {} extends Params ? [params?: Params | Readonly<Values>] : [params: Params | Readonly<Values>];

export type QueryDefinition = {
  dialect: Dialect;
  sql: string;
  params: readonly string[];
  /** The statement as the engine rewrote it for the database, and its binding order: every build step puts them in the module. */
  text?: string;
  order?: readonly string[];
  /** Each parameter's type, by name, as the engine found it: every build step puts it in the module, and a driver that binds some values wrong gets them converted by it. */
  inputs?: Inputs;
  /** Each structured parameter in insert position, with the lists of `text` the run time writes again for the keys a call carries. */
  structured?: readonly Structured[];
  cardinality?: Cardinality;
  jsonSchema?: { input: Record<string, unknown>; output: Record<string, unknown> };
};

export function defineQuery<Params, Row, C extends Cardinality, D extends Dialect, Values extends readonly unknown[] = readonly unknown[]>(
  def: QueryDefinition & { dialect: D; cardinality?: C },
): Query<Params, Row, C, D, Values> {
  const cardinality = (def.cardinality ?? "many") as C;
  const missingSchema = () => {
    throw new Error(`No JSON Schema is available for this query: a module a build step writes carries it, and a query defined by hand has none.`);
  };
  const query: Query<Params, Row, C, D, Values> & { text?: string | undefined; order?: readonly string[] | undefined; inputs?: Inputs | undefined; structured?: readonly Structured[] | undefined } = {
    dialect: def.dialect,
    sql: def.sql,
    params: def.params,
    text: def.text,
    order: def.order,
    inputs: def.inputs,
    structured: def.structured,
    cardinality,
    get inputSchema() {
      return def.jsonSchema?.input ?? missingSchema();
    },
    get outputSchema() {
      return def.jsonSchema?.output ?? missingSchema();
    },
    "~standard": {
      version: 1,
      vendor: "reylo",
      jsonSchema: {
        input: () => def.jsonSchema?.input ?? missingSchema(),
        output: () => def.jsonSchema?.output ?? missingSchema(),
      },
    },
    "~reylo": undefined as unknown as ReyloTypes<Params, Row, C, Values>,
    run: ((client: ClientFor<D>, params?: Params) => run(client, params ?? ({} as Params))) as Query<Params, Row, C, D, Values>["run"],
    runSync: ((client: SyncClientFor<D>, params?: Params) => materialize(executeSync(client, query, (params ?? {}) as Record<string, unknown>) as Row[])) as Query<Params, Row, C, D, Values>["runSync"],
    with: ((client: ClientFor<D>) => (params?: Params) => run(client, params ?? ({} as Params))) as Query<Params, Row, C, D, Values>["with"],
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
