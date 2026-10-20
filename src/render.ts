// The engine's type model as TypeScript text. On the wire the project's driver decodes a value
// its own way, and `run` returns what it decoded: reylo declares that type. Drivers disagree on
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

/** Each dialect's defaults when the project names no driver reylo knows: what postgres.js, mysql2 and node:sqlite return. */
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
    case "scalar": {
      if (dialect === "sqlite") return sqliteScalar(t.name, repr);
      // `a|b` is either: a MySQL column whose type the execution decides (`:n + 1` is a BIGINT
      // or a DOUBLE by what is bound), or a truth value a document holds as `true` or as 1.
      const parts = t.name.split("|").map((name) => (inJson ? json[name] ?? "unknown" : scalarType(name, repr)));
      return parts.includes("unknown") ? "unknown" : [...new Set(parts.flatMap((part) => part.split(" | ")))].join(" | ");
    }
    case "enum": return t.labels.length ? t.labels.map((l) => JSON.stringify(l)).join(" | ") : "string";
    case "array": {
      if (((t.of.kind === "scalar" && t.of.name === "record") || t.of.kind === "composite") && !inJson) return "string";
      // An element is nullable only where the statement builds the array from a value that may
      // be NULL (`array_agg` over a LEFT JOIN); any other array is `T[]`, as decided on 2026-10-03.
      const inner = renderType(t.of, inJson, dialect, repr) + (t.nullable_items ? " | null" : "");
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
    case "json": return !inJson && repr.json === "string" ? "string" : t.schema === undefined ? "unknown" : schemaType(t.schema);
    // Only a parameter is one; no column is.
    case "rows": return renderParamType(t, dialect, repr);
    case "unknown": return "unknown";
  }
}

/**
 * The TypeScript type of a JSON value the engine's canonical JSON Schema describes: the values a
 * CHECK's schema admits (engine/src/schema_type.rs). An object that may hold keys besides the ones
 * it names says so with an index signature, `unknown` beside named keys, since TypeScript requires
 * every named key's type, `undefined` included for an optional one, to fit the signature.
 */
export function schemaType(schema: unknown, input = false): string {
  if (schema === false) return "never";
  if (schema === null || typeof schema !== "object" || Array.isArray(schema)) return "unknown";
  const m = schema as Record<string, unknown>;
  const sub = (s: unknown) => schemaType(s, input);
  if (Array.isArray(m.anyOf)) return union(m.anyOf.map(sub));
  if (Array.isArray(m.enum)) return union(m.enum.map(literalType));
  if (m.not !== undefined) return "never";
  const kinds = typeof m.type === "string" ? [m.type] : Array.isArray(m.type) ? m.type.filter((k): k is string => typeof k === "string") : [];
  return kinds.length ? union(kinds.map((kind) => kindType(kind, m, input))) : "unknown";
}

/**
 * `input`: the value is one the caller passes, a typed JSON parameter's or a `json('…')` member's,
 * whose keys the schema does not name are ignored unless it closes the object; its type names the
 * keys alone, so that TypeScript still flags a misspelled one in an object literal.
 */
function kindType(kind: string, m: Record<string, unknown>, input = false): string {
  const schemaType_ = (s: unknown) => schemaType(s, input);
  switch (kind) {
    case "string": return "string";
    case "integer": case "number": return "number";
    case "boolean": return "boolean";
    case "null": return "null";
    case "array": {
      const prefix = Array.isArray(m.prefixItems) ? m.prefixItems.map(schemaType_) : [];
      const rest = m.items === undefined ? "unknown" : schemaType_(m.items);
      if (!prefix.length) return rest === "never" ? "[]" : `${grouped(rest)}[]`;
      const min = typeof m.minItems === "number" ? m.minItems : 0;
      const items = prefix.map((item, i) => (i < min ? item : `${grouped(item)}?`));
      if (rest !== "never") items.push(`...${grouped(rest)}[]`);
      return `[${items.join(", ")}]`;
    }
    case "object": {
      const properties = m.properties !== null && typeof m.properties === "object" && !Array.isArray(m.properties) ? (m.properties as Record<string, unknown>) : {};
      const required = Array.isArray(m.required) ? m.required : [];
      const fields = Object.entries(properties).map(([key, value]) => `${ident(key)}${required.includes(key) ? "" : "?"}: ${schemaType_(value)}`);
      const open = m.additionalProperties === undefined || m.additionalProperties === true;
      const rest = m.additionalProperties === undefined || m.additionalProperties === false || (input && open && Object.keys(properties).length > 0) ? "never" : schemaType_(m.additionalProperties);
      if (rest !== "never") fields.push(`[key: string]: ${fields.length ? "unknown" : rest}`);
      if (fields.length) return `{ ${fields.join("; ")} }`;
      return m.additionalProperties === undefined ? "Record<string, unknown>" : "Record<never, never>";
    }
    default: return "unknown";
  }
}

/** The TypeScript literal type of one JSON value. */
function literalType(v: unknown): string {
  if (v === null) return "null";
  if (typeof v === "string") return JSON.stringify(v);
  if (typeof v === "number" || typeof v === "boolean") return String(v);
  if (Array.isArray(v)) return `[${v.map(literalType).join(", ")}]`;
  if (typeof v === "object") {
    const entries = Object.entries(v as Record<string, unknown>);
    return entries.length ? `{ ${entries.map(([key, x]) => `${ident(key)}: ${literalType(x)}`).join("; ")} }` : "Record<never, never>";
  }
  return "unknown";
}

/** A union of types: `unknown` absorbs the rest, and `never` adds nothing. */
function union(parts: string[]): string {
  const kept = [...new Set(parts)];
  if (kept.includes("unknown")) return "unknown";
  const real = kept.filter((p) => p !== "never");
  return real.length ? real.join(" | ") : "never";
}

/** A type in parentheses where it is a union at its own top level, as an array's element or an optional tuple element must be. */
function grouped(ts: string): string {
  let depth = 0;
  let quoted = false;
  for (let i = 0; i < ts.length; i++) {
    const c = ts[i]!;
    if (quoted) {
      if (c === "\\") i++;
      else if (c === '"') quoted = false;
    } else if (c === '"') quoted = true;
    else if ("{([<".includes(c)) depth++;
    else if ("})]>".includes(c)) depth--;
    else if (c === "|" && depth === 0) return `(${ts})`;
  }
  return ts;
}

/** An object type of JSON-encoded fields, each nullable as the engine says. */
function renderFields(fields: { name: string; type: EngineType; nullable: boolean }[], dialect: "postgres" | "sqlite" | "mysql", repr: Repr): string {
  return fields.length ? `{ ${fields.map((f) => `${ident(f.name)}: ${renderType(f.type, true, dialect, repr)}${f.nullable ? " | null" : ""}`).join("; ")} }` : "Record<never, never>";
}

/**
 * The type of a value whose JSON Schema the statement writes, a typed JSON parameter's
 * (`:p::jsonb('…')`) or a `json('…')` member's: the caller's, so a key its schema does not name is
 * left out of the type (src/render.ts, `schemaType`'s `input`); `undefined` for any other.
 */
export function declaredType(t: EngineType, declared: unknown): string | undefined {
  return declared !== undefined && t.kind === "json" && t.schema !== undefined ? schemaType(t.schema, true) : undefined;
}

/**
 * The type a parameter accepts. The `dates` and `bigint` settings describe what the driver
 * returns, not what it takes: every Postgres and MySQL driver binds a number, a BigInt or a digit
 * string for a 64-bit integer, so `limit :n` takes a number. A `timestamptz` is an instant, which
 * every Postgres driver binds a `Date` as; a string for it must name its zone (`src/execute.ts`).
 * A `timestamp` or a `date` (MySQL's DATETIME and TIMESTAMP are the engine's `timestamp`) is a
 * wall-clock reading with no zone, and the drivers write a `Date`'s in different zones, the app's
 * (pg, mysql2), the server's (postgres.js) or UTC (PGlite, Bun.sql), so it takes the text alone
 * (the maintainer, 2026-10-12).
 */
export function renderParamType(t: EngineType, dialect: "postgres" | "sqlite" | "mysql" = "postgres", repr: Repr = defaultRepr): string {
  if (t.kind === "scalar" && dialect !== "sqlite") {
    if (t.name === "int8") return "number | bigint | string";
    if (t.name === "timestamptz") return "Date | string";
    if (t.name.split("|").every((name) => name in temporal)) return "string";
  }
  // A SQLite driver binds a number or a BigInt for an INTEGER, whatever it returns.
  if (t.kind === "scalar" && dialect === "sqlite" && t.name === "integer") return repr.bigint === "number" ? "number" : `number | ${repr.bigint}`;
  // Every driver binds a Uint8Array (a Buffer is one) for a binary value, mariadb and Neon's
  // neon() before 0.10 once `execute` has converted it, a digit string for a decimal, and any
  // JSON value for a JSON parameter, whatever it returns for them.
  if (t.kind === "scalar" && (t.name === "bytea" || t.name === "blob")) return "Uint8Array";
  if (t.kind === "scalar" && t.name === "numeric" && dialect !== "sqlite") return repr.decimal === "number" ? "number | string" : "string";
  if (t.kind === "json") return t.schema === undefined ? "unknown" : schemaType(t.schema);
  // A structured parameter: each key as a parameter of its type takes it, `?` where the caller
  // may leave it out, `| null` where it may be null.
  if (t.kind === "rows") {
    const fields = t.fields.map((f) => `${ident(f.name)}${f.optional ? "?" : ""}: ${declaredType(f.type, f.schema) ?? renderParamType(f.type, dialect, repr)}${f.nullable ? " | null" : ""}`);
    const row = fields.length ? `{ ${fields.join("; ")} }` : "Record<never, never>";
    return t.one ? row : `readonly ${row}[]`;
  }
  if (t.kind === "array") {
    const inner = renderParamType(t.of, dialect, repr);
    return /[|&]/.test(inner) ? `(${inner})[]` : `${inner}[]`;
  }
  return renderType(t, false, dialect, repr);
}
