// The JSON Schema of a query's input and output: `query.inputSchema`, `query.outputSchema` (the
// maintainer, 2026-10-20: "just createAccounts.inputSchema and createAccounts.outputSchema"), and
// Standard Schema's `jsonSchema` converter. A build step writes both into the module. The
// spellings are the engine's (engine/src/schema.rs, the framework's encoding table); what differs
// is where a value stands. The output is the result as JSON writes it (`Json<T>` in src/index.ts):
// a value the project's driver returns as a Date, a BigInt or a decimal string is the text, or the
// number, JSON.stringify makes of it. The input is the parameters the caller passes.
import type { EngineType, JsonSchema } from "./engine-type.ts";
import type { Repr } from "./render.ts";

type Dialect = "postgres" | "mysql" | "sqlite";
type Schema = { [key: string]: unknown };
type Analysis = {
  params: readonly { name: string; type: EngineType; nullable: boolean; schema?: JsonSchema }[];
  columns: readonly { name: string; type: EngineType; nullable: boolean }[];
  cardinality: "one" | "maybeOne" | "many" | "none";
};

/** The schema admitting null as well: a bare `type` gets `"null"` beside it, anything else an `anyOf` with null. */
function nullable(schema: Schema | JsonSchema): Schema | JsonSchema {
  if (schema === true || (typeof schema === "object" && Object.keys(schema).length === 0)) return schema;
  if (typeof schema === "object" && Object.keys(schema).length === 1 && typeof schema.type === "string") return { type: [schema.type, "null"] };
  return { anyOf: [schema, { type: "null" }] };
}

/** A base type by its catalog name, standing where `inJson` says, read as the project's driver reads it. */
function scalar(name: string, dialect: Dialect, repr: Repr, inJson: boolean, input: boolean): Schema {
  if (name.includes("|")) return { anyOf: name.split("|").map((part) => scalar(part, dialect, repr, inJson, input)) };
  const int64 = (asNumber: boolean): Schema => (input ? { type: ["integer", "string"], format: "int64" } : asNumber ? { type: "integer", format: "int64" } : { type: "string", format: "int64" });
  if (dialect === "sqlite") {
    switch (name) {
      // A SQLite INTEGER parameter takes a number or a BigInt, never a string (src/render.ts).
      case "integer": return input ? { type: "integer", format: "int64" } : int64(inJson || repr.bigint === "number");
      case "real": return { type: "number", format: "double" };
      case "text": return { type: "string" };
      case "blob": return { type: "string", format: "byte", contentEncoding: "base64" };
      case "numeric": return { type: ["number", "string"] };
      default: return {};
    }
  }
  switch (name) {
    case "bool": return { type: "boolean" };
    case "int2": case "int4": return { type: "integer", format: "int32" };
    case "int8": return int64(inJson || repr.bigint === "number");
    case "float4": return { type: "number", format: "float" };
    case "float8": return { type: "number", format: "double" };
    case "numeric": return inJson || (!input && repr.decimal === "number") ? { type: "number", format: "decimal" } : input && repr.decimal === "number" ? { type: ["number", "string"], format: "decimal" } : { type: "string", format: "decimal" };
    case "text": case "varchar": case "bpchar": case "name": case "char": case "citext": return { type: "string" };
    case "uuid": return { type: "string", format: "uuid" };
    case "timestamptz": case "timestamp": return { type: "string", format: "date-time" };
    case "date": return { type: "string", format: "date" };
    case "time": return { type: "string", format: "time" };
    case "bytea": return inJson ? { type: "string", format: "hex" } : { type: "string", format: "byte", contentEncoding: "base64" };
    case "json": case "jsonb": case "unknown": return {};
    default: return { type: "string", description: `${dialect === "mysql" ? "MySQL" : "pg"} type ${name}` };
  }
}

/** An object of `fields`, each spelled by `field`, every key required unless `optional` says otherwise. */
function object(fields: readonly { name: string }[], field: (f: number) => Schema | JsonSchema, optional: (f: number) => boolean = () => false, closed = false): Schema {
  const properties: Record<string, unknown> = {};
  const required: string[] = [];
  fields.forEach((f, i) => {
    properties[f.name] = field(i);
    if (!optional(i)) required.push(f.name);
  });
  return { type: "object", properties, required, ...(closed ? { additionalProperties: false } : {}) };
}

/** The JSON Schema of a value of `t`, as a result column (`input` false) or a parameter the caller passes (`input` true). */
export function typeSchema(t: EngineType, dialect: Dialect, repr: Repr, input = false, inJson = false): Schema | JsonSchema {
  switch (t.kind) {
    case "scalar": return scalar(t.name, dialect, repr, inJson, input);
    case "enum": return { type: "string", enum: t.labels };
    case "array": {
      const items = typeSchema(t.of, dialect, repr, input, inJson);
      return { type: "array", items: t.nullable_items ? nullable(items) : items };
    }
    case "composite": return inJson ? object(t.fields, (i) => (t.fields[i]!.nullable ? nullable : (s: Schema | JsonSchema) => s)(typeSchema(t.fields[i]!.type, dialect, repr, input, true))) : { type: "string", description: `the row value of ${t.name} as Postgres prints it` };
    case "object": return object(t.fields, (i) => (t.fields[i]!.nullable ? nullable : (s: Schema | JsonSchema) => s)(typeSchema(t.fields[i]!.type, dialect, repr, input, true)));
    case "list": {
      const items = typeSchema(t.of, dialect, repr, input, true);
      return { type: "array", items: t.nullable_items ? nullable(items) : items };
    }
    case "json": return t.schema ?? {};
    // What the caller passes: each key as a parameter of its type takes it.
    case "rows": {
      // A JSON member's schema as written, with its defaults and formats.
      const row = object(t.fields, (i) => (t.fields[i]!.nullable ? nullable : (s: Schema | JsonSchema) => s)(t.fields[i]!.schema ?? typeSchema(t.fields[i]!.type, dialect, repr, true)), (i) => t.fields[i]!.optional, true);
      return t.one ? row : { type: "array", items: row };
    }
    case "unknown": return {};
  }
}

/** The query's input and output schemas: the parameters as an object, and the result as `run` returns it once JSON. */
export function querySchemas(analysis: Analysis, dialect: Dialect, repr: Repr): { input: Schema; output: Schema } {
  // A typed JSON parameter's schema as written, with its defaults; one with a `default` at its root may be left out.
  const input = object(analysis.params, (i) => {
    const p = analysis.params[i]!;
    const s = p.schema ?? typeSchema(p.type, dialect, repr, true);
    return p.nullable ? nullable(s) : s;
  }, (i) => {
    const s = analysis.params[i]!.schema;
    return s !== null && typeof s === "object" && s !== undefined && "default" in s;
  }, true);
  const row = object(analysis.columns, (i) => {
    const c = analysis.columns[i]!;
    const s = typeSchema(c.type, dialect, repr);
    return c.nullable ? nullable(s) : s;
  });
  const output: Schema = analysis.cardinality === "many" ? { type: "array", items: row } : analysis.cardinality === "one" ? row : analysis.cardinality === "maybeOne" ? { anyOf: [row, { type: "null" }] } : { type: "null" };
  return { input: { $schema: "https://json-schema.org/draft/2020-12/schema", ...input }, output: { $schema: "https://json-schema.org/draft/2020-12/schema", ...output } };
}
