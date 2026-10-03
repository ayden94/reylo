// The engine's type model as TypeScript text. On the wire the project's driver decodes a value
// its own way, and `run` returns what it decoded: Porg declares that type. Drivers disagree on
// five kinds of value: timestamps (a Date or a string), 64-bit integers (a string, a number or a
// BigInt), decimals (a string or a number), JSON (parsed or its text) and binary values (a
// Uint8Array, a Buffer, an ArrayBuffer or an array of numbers). The `Repr` names what the
// project's driver returns for each (src/project.ts takes it from the driver in package.json and
// the project file's `types`), and the rendered type follows it. Inside a JSON value the database
// has already serialized it, so the same type reads differently there and the setting does not
// apply.
import type { EngineType } from "./engine.ts";

/** What the project's driver returns for the values drivers disagree on. */
export type Repr = {
  dates: "string" | "Date" | "temporal";
  /** `number | bigint` is PGlite's: a number, or a BigInt above 2^53. */
  bigint: "string" | "number" | "bigint" | "number | bigint";
  decimal: "string" | "number";
  json: "parsed" | "string";
  binary: "Uint8Array" | "Buffer" | "ArrayBuffer" | "number[]";
};

/** Each dialect's defaults when the project names no driver Porg knows: what postgres.js, mysql2 and node:sqlite return. */
export const defaultReprs: Record<"postgres" | "mysql" | "sqlite", Repr> = {
  postgres: { dates: "Date", bigint: "string", decimal: "string", json: "parsed", binary: "Uint8Array" },
  mysql: { dates: "Date", bigint: "number", decimal: "string", json: "parsed", binary: "Uint8Array" },
  // SQLite has no date, decimal or JSON type; an INTEGER is a number.
  sqlite: { dates: "string", bigint: "number", decimal: "string", json: "parsed", binary: "Uint8Array" },
};
export const defaultRepr: Repr = defaultReprs.postgres;

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

// SQLite: the engine names a column by its affinity; the drivers decode REAL as a number, TEXT as
// a string and BLOB as a Uint8Array. An INTEGER is a 64-bit integer, which the `bigint` setting
// describes as for the other dialects: a number by default, a BigInt under better-sqlite3's and
// bun:sqlite's safeIntegers or node:sqlite's readBigInts, a string or a BigInt under libsql's
// intMode. A NUMERIC column holds whichever the row stored. SQLite has no date type, so `dates`
// changes no SQLite type.
const sqlite: Record<string, string> = { real: "number", text: "string", blob: "Uint8Array" };

function sqliteScalar(name: string, repr: Repr): string {
  if (name === "integer") return repr.bigint;
  if (name === "numeric") return [...new Set([...repr.bigint.split(" | "), "number", "string"])].join(" | ");
  if (name === "blob") return repr.binary;
  return sqlite[name] ?? "unknown";
}

export const ident = (name: string) => (/^[A-Za-z_$][\w$]*$/.test(name) ? name : JSON.stringify(name));

/** The wire type of a scalar under the chosen representation. */
export function scalarType(name: string, repr: Repr): string {
  if (name in temporal) return repr.dates === "temporal" ? temporal[name]! : repr.dates === "Date" ? "Date" : "string";
  if (name === "int8") return repr.bigint;
  if (name === "numeric") return repr.decimal;
  if (name === "bytea") return repr.binary;
  return wire[name] ?? "unknown";
}

export function renderType(t: EngineType, inJson = false, dialect: "postgres" | "sqlite" | "mysql" = "postgres", repr: Repr = defaultRepr): string {
  switch (t.kind) {
    case "scalar": return dialect === "sqlite" ? sqliteScalar(t.name, repr) : inJson ? json[t.name] ?? "unknown" : scalarType(t.name, repr);
    case "enum": return t.labels.length ? t.labels.map((l) => JSON.stringify(l)).join(" | ") : "string";
    case "array": {
      if (((t.of.kind === "scalar" && t.of.name === "record") || t.of.kind === "composite") && !inJson) return "string";
      const inner = renderType(t.of, inJson, dialect, repr);
      return /[|&]/.test(inner) ? `(${inner})[]` : `${inner}[]`;
    }
    // A JSON value whose shape the statement builds; a driver set up to return JSON as text
    // (mysql2's jsonStrings, a pg parser for json) hands back the text.
    case "object": return !inJson && repr.json === "string" ? "string" : renderFields(t.fields, dialect, repr);
    // A composite whose columns the catalog gave: on the wire it is a row value like `record`,
    // which no driver parses; inside a JSON document the database writes it as an object.
    case "composite": return inJson ? renderFields(t.fields, dialect, repr) : "string";
    case "list": {
      if (!inJson && repr.json === "string") return "string";
      const inner = renderType(t.of, true, dialect, repr) + (t.nullable_items ? " | null" : "");
      return /[|&]/.test(inner) ? `(${inner})[]` : `${inner}[]`;
    }
    case "json": return !inJson && repr.json === "string" ? "string" : "unknown";
    case "unknown": return "unknown";
  }
}

/** An object type of JSON-encoded fields, each nullable as the engine says. */
function renderFields(fields: { name: string; type: EngineType; nullable: boolean }[], dialect: "postgres" | "sqlite" | "mysql", repr: Repr): string {
  return fields.length ? `{ ${fields.map((f) => `${ident(f.name)}: ${renderType(f.type, true, dialect, repr)}${f.nullable ? " | null" : ""}`).join("; ")} }` : "Record<never, never>";
}

/**
 * The type a parameter accepts. The `dates` and `bigint` settings describe what the driver
 * returns, not what it takes: every Postgres and MySQL driver serializes a `Date` or an ISO
 * string for a timestamp, and a number, a BigInt or a digit string for a 64-bit integer, so a
 * value read back under any representation can be passed in again, and `limit :n` takes a number.
 */
export function renderParamType(t: EngineType, dialect: "postgres" | "sqlite" | "mysql" = "postgres", repr: Repr = defaultRepr): string {
  if (t.kind === "scalar" && dialect !== "sqlite") {
    if (t.name === "int8") return "number | bigint | string";
    if (t.name in temporal) return "Date | string";
  }
  // A SQLite driver binds a number or a BigInt for an INTEGER, whatever it returns.
  if (t.kind === "scalar" && dialect === "sqlite" && t.name === "integer") return repr.bigint === "number" ? "number" : `number | ${repr.bigint}`;
  // Every driver binds a Uint8Array (a Buffer is one) for a binary value, a digit string for a
  // decimal, and any JSON value for a JSON parameter, whatever it returns for them.
  if (t.kind === "scalar" && (t.name === "bytea" || t.name === "blob")) return "Uint8Array";
  if (t.kind === "scalar" && t.name === "numeric" && dialect !== "sqlite") return repr.decimal === "number" ? "number | string" : "string";
  if (t.kind === "json") return "unknown";
  if (t.kind === "array") {
    const inner = renderParamType(t.of, dialect, repr);
    return /[|&]/.test(inner) ? `(${inner})[]` : `${inner}[]`;
  }
  return renderType(t, false, dialect, repr);
}
