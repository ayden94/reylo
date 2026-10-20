// How Reylo represents a SQL value in JavaScript: one table, which the type renderer
// (src/render.ts), the JSON Schema writer (src/schema.ts) and the run time's decoders
// (src/decode.ts, src/decode-mysql.ts, src/decode-sqlite.ts) all read, so that the type a module
// declares and the value `run` returns cannot drift apart (the maintainer, 2026-10-20). Reylo sets
// every value's type itself, on every driver, whatever the app configured its driver to return:
// one representation per SQL type.
//
// A value's representation is its kind, which the build writes into the module for each output
// column (`columns`, as `[name, kind]`): `bool` is a boolean; `int` and `float` a number; `int8`
// and `decimal` the exact decimal text; `date` the text `YYYY-MM-DD`; `timestamp` the server's
// wall-clock text with `T` between the date and the time; `timestamptz` a Date, the instant;
// `json` the parsed value; `bytes` a Uint8Array (a Buffer is one); `text` the server's text, which
// every other type is (time, interval, uuid, an enum's label, a range, a row value); `unknown`
// whatever the driver gives (a SQLite column with no declared type). `k[]` is an array of `k`, a
// NULL element `null`, and `a|b` either (a MySQL expression whose type the bound value decides).
//
// This module runs in every generated module's process, so it names no Node type: a Buffer is
// reached through `globalThis` where one exists.
import type { EngineType } from "./engine-type.ts";

export type Dialect = "postgres" | "mysql" | "sqlite";

/** A scalar value's representation. */
export type ScalarKind = "bool" | "int" | "float" | "int8" | "decimal" | "text" | "date" | "timestamp" | "timestamptz" | "json" | "bytes" | "unknown";
/** A column's kind as a module carries it: a scalar kind, `k[]` for an array of `k`, `a|b` for either. */
export type Kind = string;
/** The output columns a module carries, in the statement's order. */
export type Columns = readonly (readonly [name: string, kind: Kind])[];

/** The TypeScript type of each scalar kind: what a value of that kind is at run time. */
export const TS_TYPE: Readonly<Record<ScalarKind, string>> = {
  bool: "boolean",
  int: "number",
  float: "number",
  int8: "string",
  decimal: "string",
  text: "string",
  date: "string",
  timestamp: "string",
  timestamptz: "Date",
  json: "unknown",
  bytes: "Uint8Array",
  unknown: "unknown",
};

/**
 * The kind of a Postgres or MySQL base type, by the engine's name for it (MySQL's are the Postgres
 * names: a DATETIME is `timestamp`, a BIT `bytea`, a YEAR `int2`). Every type not named here is
 * its text: Reylo owns every oid, so the driver never parses one (an enum's label, a range, `time`,
 * `interval`, `uuid`, `money`, `point`, `xml`, a row value).
 */
const SQL_KINDS: Readonly<Record<string, ScalarKind>> = {
  bool: "bool",
  int2: "int",
  int4: "int",
  oid: "int",
  float4: "float",
  float8: "float",
  int8: "int8",
  numeric: "decimal",
  date: "date",
  timestamp: "timestamp",
  timestamptz: "timestamptz",
  json: "json",
  jsonb: "json",
  bytea: "bytes",
};

/**
 * SQLite's: the engine names a column by its affinity. An INTEGER is a number, exact or an error;
 * a NUMERIC holds whichever storage class the row stored; a column with no declared type, or ANY
 * in a STRICT table, is whatever the driver gives.
 */
const SQLITE_KINDS: Readonly<Record<string, Kind>> = {
  integer: "int",
  real: "float",
  text: "text",
  blob: "bytes",
  numeric: "int|float|text",
  bool: "bool",
};

/** The kind of a base type by its engine name, `a|b` naming either. */
export function scalarKind(name: string, dialect: Dialect): Kind {
  const kinds = name.split("|").map((part) => (dialect === "sqlite" ? (SQLITE_KINDS[part] ?? "unknown") : (SQL_KINDS[part] ?? "text")));
  return [...new Set(kinds.flatMap((k) => k.split("|")))].join("|");
}

/** The kind of a value of the engine's type `t`, standing in a result row (not inside a JSON document). */
export function columnKind(t: EngineType, dialect: Dialect): Kind {
  switch (t.kind) {
    case "scalar":
      return scalarKind(t.name, dialect);
    case "enum":
      return "text";
    case "array":
      // A row value's array is the text Postgres prints, as a row value is.
      if ((t.of.kind === "scalar" && t.of.name === "record") || t.of.kind === "composite") return "text";
      return `${columnKind(t.of, dialect)}[]`;
    case "object":
    case "list":
    case "json":
    case "rows":
      return "json";
    case "composite":
      return "text";
    case "unknown":
      return "unknown";
  }
}

/** The TypeScript type of a scalar kind, `a|b` as the union of both; `unknown` where any part is. */
export function kindType(kind: Kind): string {
  const parts = kind.split("|").map((k) => TS_TYPE[k as ScalarKind] ?? "unknown");
  return parts.includes("unknown") ? "unknown" : [...new Set(parts)].join(" | ");
}

// ---------------------------------------------------------------------------------------------
// The value of each kind from the server's text, as every Postgres driver but Bun.sql hands it
// over (pg, pg-promise, Neon, PGlite, and postgres.js through `.raw()`), at the server's default
// DateStyle (ISO), IntervalStyle and bytea_output, which the drivers' own parsers read too.

export type Decoder = (text: string) => unknown;

const identity: Decoder = (x) => x;
const number: Decoder = (x) => +x;

/** `2026-10-20 10:00:00.123456` → `2026-10-20T10:00:00.123456`; `infinity` and a BC year's suffix as written. */
const timestamp: Decoder = (x) => x.replace(" ", "T");

// timestamptz in DateStyle ISO: `2026-10-20 03:03:00.123456-07`, offsets with seconds before
// standard time (`-07:52:58`), five-digit years, ` BC`. `new Date(text)`, which postgres.js and
// PGlite use, returns Invalid Date for the last three and reads year 0001 as 2001
// (docs/driver-decoding/spike/postgres.md).
const TSTZ = /^(\d{4,})-(\d\d)-(\d\d) (\d\d):(\d\d):(\d\d)(\.\d{1,6})?([+-])(\d\d)(?::(\d\d))?(?::(\d\d))?( BC)?$/;

/** A timestamptz's text as the instant it names; ±infinity is an Invalid Date. */
export function timestamptz(x: string): Date {
  const m = TSTZ.exec(x);
  if (m === null) return new Date(NaN);
  const year = m[12] === undefined ? +m[1]! : 1 - +m[1]!;
  const ms = m[7] === undefined ? 0 : Math.trunc(+m[7] * 1000);
  const offset = (m[8] === "-" ? -1 : 1) * (+m[9]! * 3600 + (m[10] === undefined ? 0 : +m[10] * 60) + (m[11] === undefined ? 0 : +m[11])) * 1000;
  if (year >= 100) return new Date(Date.UTC(year, +m[2]! - 1, +m[3]!, +m[4]!, +m[5]!, +m[6]!, ms) - offset);
  const d = new Date(0);
  d.setUTCFullYear(year, +m[2]! - 1, +m[3]!);
  d.setUTCHours(+m[4]!, +m[5]!, +m[6]!, ms);
  return new Date(d.getTime() - offset);
}

type BufferClass = { from(data: string, encoding: string): Uint8Array; isBuffer(v: unknown): boolean };
const NodeBuffer = (globalThis as { Buffer?: BufferClass }).Buffer;
// TextDecoder is a global in every runtime Reylo supports; it is typed here rather than through
// the DOM lib or Node's, which a project that type-checks Reylo's sources need not have.
const Utf8 = (globalThis as unknown as { TextDecoder: new (label?: string) => { decode(bytes: Uint8Array): string } }).TextDecoder;
const utf8Decoder = new Utf8("utf-8");

/** UTF-8 bytes as text: a Buffer's own toString where the bytes are one. */
export function utf8(bytes: Uint8Array): string {
  return NodeBuffer?.isBuffer(bytes) ? (bytes as Uint8Array & { toString(encoding: string): string }).toString("utf8") : utf8Decoder.decode(bytes);
}
const fromHex = (Uint8Array as unknown as { fromHex?: (hex: string) => Uint8Array }).fromHex;

/** bytea's text: hex (`\x0102ff`, the server's default) or the escape format (`bytea_output = escape`). */
export function bytea(x: string): Uint8Array {
  if (x.charCodeAt(0) === 92 && x.charCodeAt(1) === 120) {
    const hex = x.slice(2);
    if (NodeBuffer) return NodeBuffer.from(hex, "hex");
    if (fromHex) return fromHex(hex);
    const out = new Uint8Array(hex.length / 2);
    for (let i = 0; i < out.length; i++) out[i] = parseInt(hex.slice(2 * i, 2 * i + 2), 16);
    return out;
  }
  const out: number[] = [];
  for (let i = 0; i < x.length; i++) {
    if (x[i] !== "\\") out.push(x.charCodeAt(i));
    else if (x[i + 1] === "\\") out.push(92), i++;
    else out.push(parseInt(x.slice(i + 1, i + 4), 8)), (i += 3);
  }
  return Uint8Array.from(out);
}

const TEXT_DECODERS: Readonly<Record<ScalarKind, Decoder>> = {
  bool: (x) => x === "t",
  int: number,
  float: number,
  int8: identity,
  decimal: identity,
  text: identity,
  date: identity,
  timestamp,
  timestamptz,
  json: (x) => JSON.parse(x),
  bytes: bytea,
  unknown: identity,
};

/**
 * Postgres's array output: `{a,"b,c",NULL,{d}}`, optionally after its bounds (`[0:1]={...}`).
 * An element is quoted when it holds the delimiter, a brace, a quote, a backslash or white space,
 * or is empty or the word NULL; inside quotes a backslash escapes the next character.
 */
export function parseArray(text: string, element: (text: string) => unknown, delim = ","): unknown[] {
  const n = text.length;
  let i = text[0] === "[" ? text.indexOf("=") + 1 : 0;
  const level = (): unknown[] => {
    const out: unknown[] = [];
    i++;
    if (text[i] === "}") return i++, out;
    while (i < n) {
      if (text[i] === "{") out.push(level());
      else if (text[i] === '"') {
        let s = "";
        let from = ++i;
        while (i < n && text[i] !== '"') {
          if (text[i] === "\\") (s += text.slice(from, i)), (from = ++i);
          i++;
        }
        out.push(element(s + text.slice(from, i++)));
      } else {
        const from = i;
        while (i < n && text[i] !== delim && text[i] !== "}") i++;
        const s = text.slice(from, i);
        out.push(s === "NULL" ? null : element(s));
      }
      if (text[i++] === "}") return out;
    }
    return out;
  };
  return level();
}

/** The decoder of a Postgres value of `kind` from its text; `delim` is an array's (`;` for `box[]`). */
export function textDecoder(kind: Kind, delim = ","): Decoder {
  if (kind.endsWith("[]")) {
    const element = textDecoder(kind.slice(0, -2), delim);
    return (x) => parseArray(x, element, delim);
  }
  return TEXT_DECODERS[kind as ScalarKind] ?? identity;
}

// ---------------------------------------------------------------------------------------------
// The value of each kind from whatever a driver decoded it to: for the drivers that take no
// decoder (Bun.sql; the SQLite drivers, whose JSON is text and whose truth values are integers),
// and for the rare value a driver decoded before Reylo could name its type. Each converts the
// forms a driver is measured to give (docs/driver-decoding/spike/) and leaves a value already in
// Reylo's form as it is.

/** Why a value cannot be given its kind: Reylo's error, naming the column. */
export class DecodeError extends TypeError {}

const MAX_SAFE = Number.MAX_SAFE_INTEGER;

/** An integer as a number, refused where a number cannot hold it exactly. */
export function exactInteger(v: number | bigint, column: string): number {
  if (typeof v === "bigint") {
    if (v > BigInt(MAX_SAFE) || v < -BigInt(MAX_SAFE)) throw outOfRange(String(v), column);
    return Number(v);
  }
  if (v > MAX_SAFE || v < -MAX_SAFE) throw outOfRange(String(v), column, true);
  return v;
}

function outOfRange(value: string, column: string, rounded = false): DecodeError {
  return new DecodeError(
    rounded
      ? `Column "${column}" holds an integer past ±(2^53 - 1), which the driver already rounded to ${value}: a number cannot hold it exactly, so Reylo refuses it rather than return a rounded value. Cast it to text in the statement (cast(${column} as text)) to read it exactly.`
      : `Column "${column}" holds ${value}, an integer past ±(2^53 - 1), which a number cannot hold exactly, so Reylo refuses it rather than return a rounded value. Cast it to text in the statement (cast(${column} as text)) to read it exactly.`,
  );
}

const pad = (n: number, width = 2) => String(n).padStart(width, "0");

/**
 * A Date a driver read as UTC (Bun.sql's date and timestamp) back to the wall-clock text the server
 * wrote: `YYYY-MM-DD`, or with `T` and the fraction the driver kept, and ` BC` for a year before 1.
 */
function utcText(d: Date, kind: "date" | "timestamp"): string {
  const t = d.getTime();
  if (Number.isNaN(t)) return "infinity";
  const year = d.getUTCFullYear();
  const date = `${pad(year > 0 ? year : 1 - year, 4)}-${pad(d.getUTCMonth() + 1)}-${pad(d.getUTCDate())}`;
  const bc = year > 0 ? "" : " BC";
  if (kind === "date") return date + bc;
  const ms = d.getUTCMilliseconds();
  return `${date}T${pad(d.getUTCHours())}:${pad(d.getUTCMinutes())}:${pad(d.getUTCSeconds())}${ms ? `.${pad(ms, 3).replace(/0+$/, "")}` : ""}${bc}`;
}

const isBytes = (v: unknown): v is Uint8Array => v instanceof Uint8Array;

/**
 * `v` as a value of `kind`, from the form a driver decoded it to; `column` names it in an error. A
 * number past ±(2^53 - 1) in an integer column is one the driver already rounded, and is refused.
 */
export function coerce(kind: Kind, v: unknown, column: string): unknown {
  if (v === null || v === undefined) return null;
  if (kind.endsWith("[]")) {
    const element = kind.slice(0, -2);
    if (typeof v === "string") return parseArray(v, (s) => coerce(element, s, column));
    if (Array.isArray(v)) return v.map((e) => (Array.isArray(e) ? coerce(kind, e, column) : coerce(element, e, column)));
    // Bun.sql's int4[] and float4[] are typed arrays.
    if (ArrayBuffer.isView(v) && !(v instanceof DataView)) return Array.from(v as unknown as ArrayLike<unknown>, (e) => coerce(element, e, column));
    return v;
  }
  if (kind.includes("|")) return coerceEither(kind, v, column);
  switch (kind as ScalarKind) {
    case "bool":
      if (typeof v === "boolean") return v;
      if (typeof v === "number" || typeof v === "bigint") return v != 0;
      if (typeof v === "string") return v === "t" || v === "true" || (v !== "f" && v !== "false" && v !== "0" && v !== "");
      return v;
    case "int":
    case "float":
      if (typeof v === "number") return kind === "int" ? exactInteger(v, column) : v;
      if (typeof v === "bigint") return exactInteger(v, column);
      if (typeof v === "string" && v !== "" && !Number.isNaN(+v)) return kind === "int" && /^[+-]?\d+$/.test(v) ? exactInteger(BigInt(v), column) : +v;
      return v;
    case "int8":
    case "decimal":
      if (typeof v === "bigint" || typeof v === "number") return String(v);
      return v;
    case "date":
    case "timestamp":
      if (v instanceof Date) return utcText(v, kind as "date" | "timestamp");
      // Bun.sql's `infinity`.
      if (v === Infinity || v === -Infinity) return v > 0 ? "infinity" : "-infinity";
      return v;
    case "timestamptz":
      if (typeof v === "string") return timestamptz(v);
      return v;
    case "json":
      return typeof v === "string" ? JSON.parse(v) : v;
    case "bytes":
      if (isBytes(v)) return v;
      if (v instanceof ArrayBuffer) return new Uint8Array(v);
      // D1's BLOB is an array of numbers; mariadb's and Bun.sql's BIT(1) a truth value.
      if (Array.isArray(v)) return Uint8Array.from(v as number[]);
      if (typeof v === "boolean") return Uint8Array.of(v ? 1 : 0);
      if (typeof v === "string") return bytea(v);
      return v;
    case "text":
      if (typeof v === "string") return v;
      // mariadb's SET is an array of its members.
      if (Array.isArray(v)) return v.join(",");
      if (typeof v === "number" || typeof v === "bigint") return String(v);
      return v;
    default:
      return v;
  }
}

/** A value of `a|b`: by what the driver gave, since only the value says which it is. */
function coerceEither(kind: Kind, v: unknown, column: string): unknown {
  const kinds = kind.split("|");
  // A MySQL BIGINT is the int8 text; a SQLite INTEGER a number, exact or refused.
  if (typeof v === "bigint") return kinds.includes("int8") ? String(v) : exactInteger(v, column);
  if (typeof v === "number") {
    // Where the column may hold a float, a number past 2^53 may be one.
    if (kinds.includes("float") || !Number.isInteger(v)) return v;
    if (kinds.includes("int8")) return String(v);
    return kinds.includes("int") ? exactInteger(v, column) : v;
  }
  if (v instanceof Date) return kinds.includes("timestamp") ? utcText(v, "timestamp") : kinds.includes("date") ? utcText(v, "date") : v;
  if (v instanceof ArrayBuffer || (Array.isArray(v) && kinds.includes("bytes"))) return coerce("bytes", v, column);
  return v;
}
