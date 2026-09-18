// The runtime query object behind `import query from "./file.sql"`.
// Types come from the content mapper; values come from the runtime loader.
import { execute as run, type ClientFor, type Dialect } from "./execute.ts";
import type { StandardJSONSchemaV1, StandardTypedV1 } from "./standard.ts";

export type { ClientFor, Dialect } from "./execute.ts";

export type Cardinality = "many" | "one" | "maybeOne";

export type Materialize<Row, C extends Cardinality> = C extends "many" ? Row[] : C extends "one" ? Row : Row | null;

/** The JSON.stringify shape of a driver-decoded value. */
export type Json<T> = T extends Date | bigint | Uint8Array
  ? string
  : T extends readonly (infer U)[]
    ? Json<U>[]
    : T extends object
      ? { [K in keyof T]: Json<T[K]> }
      : T;

export type LandoTypes<Params, Row, C extends Cardinality> = {
  readonly element: Row;
  readonly cardinality: C;
  readonly json: Materialize<Json<Row>, C>;
  readonly input: Params;
};

export interface Query<Params, Row, C extends Cardinality, D extends Dialect>
  extends StandardTypedV1<Params, Materialize<Row, C>>, StandardJSONSchemaV1<Params, Materialize<Row, C>> {
  readonly dialect: D;
  /** The statement as written, with lando's placeholders left in place. */
  readonly sql: string;
  /** Parameter names in binding order. */
  readonly params: readonly string[];
  readonly cardinality: C;
  readonly "~standard": StandardJSONSchemaV1.Props<Params, Materialize<Row, C>>;
  readonly "~lando": LandoTypes<Params, Row, C>;
  execute(client: ClientFor<D>, params: Params): Promise<Materialize<Row, C>>;
}

export type QueryDefinition = {
  dialect: Dialect;
  sql: string;
  params: readonly string[];
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
  const query: Query<Params, Row, C, D> = {
    dialect: def.dialect,
    sql: def.sql,
    params: def.params,
    cardinality,
    "~standard": {
      version: 1,
      vendor: "lando",
      jsonSchema: {
        input: () => def.jsonSchema?.input ?? missingSchema(),
        output: () => def.jsonSchema?.output ?? missingSchema(),
      },
    },
    "~lando": undefined as unknown as LandoTypes<Params, Row, C>,
    async execute(client, params) {
      const rows = (await run(client, query, params as Record<string, unknown>)) as Row[];
      switch (cardinality) {
        case "many":
          return rows as Materialize<Row, C>;
        case "one":
          if (rows.length !== 1) throw new Error(`Expected exactly one row, got ${rows.length}`);
          return rows[0] as Materialize<Row, C>;
        default:
          if (rows.length > 1) throw new Error(`Expected at most one row, got ${rows.length}`);
          return (rows[0] ?? null) as Materialize<Row, C>;
      }
    },
  };
  return query;
}
