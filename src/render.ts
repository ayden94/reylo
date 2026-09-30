// The engine's type model as TypeScript text. On the wire a driver decodes a Postgres value
// its own way (int8 as a string, timestamps as Date); the mapper's `dates` and `bigint`
// options pick the representation `run` hands back, and the rendered type follows them.
// Inside a JSON value the database has already serialized it, so the same type reads
// differently there and the options do not apply.
import type { EngineType } from "./engine.ts";
import { defaultRepr, type Conversion, type Repr } from "./convert.ts";

export { defaultRepr, type Conversion, type Repr } from "./convert.ts";

const wire: Record<string, string> = {
  bool: "boolean",
  int2: "number", int4: "number", oid: "number", float4: "number", float8: "number",
  numeric: "string",
  text: "string", varchar: "string", bpchar: "string", name: "string", char: "string", citext: "string",
  uuid: "string", time: "string", timetz: "string", interval: "string",
  bytea: "Uint8Array",
  void: "void",
  // Types no supported driver parses: the value arrives as the text Postgres prints.
  money: "string", inet: "string", cidr: "string", macaddr: "string", macaddr8: "string", xml: "string",
  bit: "string", varbit: "string", tsvector: "string", tsquery: "string", regclass: "string", regtype: "string",
  int4range: "string", int8range: "string", numrange: "string", tsrange: "string", tstzrange: "string", daterange: "string",
  int4multirange: "string", int8multirange: "string", nummultirange: "string", tsmultirange: "string", tstzmultirange: "string", datemultirange: "string",
  hstore: "string", ltree: "string", lquery: "string", ltxtquery: "string",
  // A row value (`row(...)`, a composite-typed column, a whole-row reference): no driver parses
  // one, so it arrives as its text form, and so does an array of them.
  record: "string",
};
const json: Record<string, string> = {
  ...wire,
  int8: "number", numeric: "number",
  date: "string", timestamp: "string", timestamptz: "string",
  bytea: "string",
};
const temporal: Record<string, string> = { timestamptz: "Temporal.Instant", timestamp: "Temporal.PlainDateTime", date: "Temporal.PlainDate" };

// SQLite: the engine names a column by its affinity; node:sqlite, better-sqlite3 and bun:sqlite
// decode INTEGER and REAL as numbers, TEXT as strings and BLOB as Uint8Array. A NUMERIC column
// holds whichever the row stored. SQLite has no date type, so `dates` has nothing to convert.
const sqlite: Record<string, string> = { integer: "number", real: "number", text: "string", blob: "Uint8Array", numeric: "number | string" };

export const ident = (name: string) => (/^[A-Za-z_$][\w$]*$/.test(name) ? name : JSON.stringify(name));

/** The wire type of a scalar under the chosen representation. */
export function scalarType(name: string, repr: Repr): string {
  if (name in temporal) return repr.dates === "temporal" ? temporal[name]! : repr.dates === "Date" ? "Date" : "string";
  if (name === "int8") return repr.bigint;
  return wire[name] ?? "unknown";
}

export function renderType(t: EngineType, inJson = false, dialect: "postgres" | "sqlite" | "mysql" = "postgres", repr: Repr = defaultRepr): string {
  switch (t.kind) {
    case "scalar": return dialect === "sqlite" ? sqlite[t.name] ?? "unknown" : inJson ? json[t.name] ?? "unknown" : scalarType(t.name, repr);
    case "enum": return t.labels.length ? t.labels.map((l) => JSON.stringify(l)).join(" | ") : "string";
    case "array": {
      if (((t.of.kind === "scalar" && t.of.name === "record") || t.of.kind === "composite") && !inJson) return "string";
      const inner = renderType(t.of, inJson, dialect, repr);
      return /[|&]/.test(inner) ? `(${inner})[]` : `${inner}[]`;
    }
    case "object": return renderFields(t.fields, dialect, repr);
    // A composite whose columns the catalog gave: on the wire it is a row value like `record`,
    // which no driver parses; inside a JSON document the database writes it as an object.
    case "composite": return inJson ? renderFields(t.fields, dialect, repr) : "string";
    case "list": { const inner = renderType(t.of, true, dialect, repr) + (t.nullable_items ? " | null" : ""); return /[|&]/.test(inner) ? `(${inner})[]` : `${inner}[]`; }
    case "json": return "unknown";
    case "unknown": return "unknown";
  }
}

/** An object type of JSON-encoded fields, each nullable as the engine says. */
function renderFields(fields: { name: string; type: EngineType; nullable: boolean }[], dialect: "postgres" | "sqlite" | "mysql", repr: Repr): string {
  return fields.length ? `{ ${fields.map((f) => `${ident(f.name)}: ${renderType(f.type, true, dialect, repr)}${f.nullable ? " | null" : ""}`).join("; ")} }` : "Record<never, never>";
}

/**
 * The type a parameter accepts. The `dates` and `bigint` options describe what `run` hands
 * back, not what a driver takes: every Postgres and MySQL driver serializes a `Date` or an ISO
 * string for a timestamp, and a number, a BigInt or a digit string for a 64-bit integer, so a
 * value read back under any representation can be passed in again, and `limit :n` takes a number.
 */
export function renderParamType(t: EngineType, dialect: "postgres" | "sqlite" | "mysql" = "postgres", repr: Repr = defaultRepr): string {
  if (t.kind === "scalar" && dialect !== "sqlite") {
    if (t.name === "int8") return "number | bigint | string";
    if (t.name in temporal) return "Date | string";
  }
  if (t.kind === "array") {
    const inner = renderParamType(t.of, dialect, repr);
    return /[|&]/.test(inner) ? `(${inner})[]` : `${inner}[]`;
  }
  return renderType(t, false, dialect, repr);
}

/** The runtime conversion a column's wire value needs, if any: a temporal scalar or int8, or an array of one. */
export function conversion(t: EngineType): Conversion | undefined {
  if (t.kind === "scalar") return t.name in temporal || t.name === "int8" ? (t.name as Conversion) : undefined;
  if (t.kind === "array" && t.of.kind === "scalar") { const inner = conversion(t.of); return inner ? (`${inner}[]` as Conversion) : undefined; }
  return undefined;
}
