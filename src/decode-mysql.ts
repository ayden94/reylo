// Reylo decodes every value of its own MySQL queries itself, on the app's own pool, connection or
// transaction (the maintainer, 2026-10-20; the representation is src/values.ts). Each driver but
// Bun.sql takes its decoding choices per query, laid over the app's without changing them
// (docs/driver-decoding/spike/mysql.md):
//
// - mysql2 stays on `execute`, the binary protocol (its `query` writes the values into the text
//   with backslash escapes, which matched every row of a table under NO_BACKSLASH_ESCAPES). Its
//   per-query options make a BIGINT and a date the server's text, and Reylo's per-query `typeCast`
//   reads a DECIMAL and a JSON value as their text, which the app's `decimalNumbers` and
//   `jsonStrings` (read from the app's config only) would otherwise decide. `decimalNumbers` is
//   never passed per query: mysql2 keys its row-parser cache, shared by every pool in the process,
//   by the per-query value and builds the parser from the app's, so one query would poison
//   another pool's parser.
// - mariadb takes every option per query, both ways, and its `metaAsArray: false` keeps the rows
//   an array of rows. A SET comes back an array, which no option prevents: Reylo joins it.
// - PlanetScale takes a `cast` per query, which replaces the app's: Reylo's is the driver's own
//   default, with a DATETIME's `T`.
// - Bun.sql takes nothing per query: its values are converted (src/decode.ts, `bunRows`).
import { coerce, utf8, type Columns } from "./values.ts";

type AnyClient = Record<string, any>;

// ---------------------------------------------------------------------------------------------
// mysql2

/** Options mysql2 ORs with the app's (a BIGINT and a date as text) and the row's shape, which the query's own replace. */
const MYSQL2_OPTIONS = { supportBigNumbers: true, bigNumberStrings: true, dateStrings: true, rowsAsArray: false, nestTables: false };

type Mysql2Field = { type: string; string(encoding?: string): string | null; buffer(): Uint8Array | null };

/**
 * Reylo's typeCast: a DECIMAL as its text and a JSON value parsed from its bytes (both are
 * length-coded in the binary protocol too, so they read whole), JSON here because mysql2's own
 * parser turns an integer past 2^53 inside a document into a string under `supportBigNumbers`; a
 * GEOMETRY as its bytes; every other type mysql2's own reading with the options above.
 */
function mysql2Cast(f: Mysql2Field, next: () => unknown): unknown {
  switch (f.type) {
    case "NEWDECIMAL":
    case "DECIMAL":
      return f.string("ascii");
    case "JSON": {
      const bytes = f.buffer();
      return bytes === null ? null : JSON.parse(utf8(bytes));
    }
    case "GEOMETRY":
      return f.buffer();
    default:
      return next();
  }
}

/** A column's protocol type and fraction digits, as mysql2's fields give them. */
type Mysql2Column = { name: string; columnType: number; decimals: number };
const DATETIME = 12, TIMESTAMP = 7, TIME = 11;

/**
 * A DATETIME or TIMESTAMP with `T`, and the fraction digits the binary protocol drops when they are
 * all zero (mysql2 writes `09:31:00` for a DATETIME(3) of .000, and TIME's trailing zeros never),
 * from the column's own count.
 */
function wallClock(s: string, decimals: number, type: number): string {
  let text = s;
  if (decimals > 0) {
    const dot = text.indexOf(".");
    text = dot < 0 ? `${text}.${"0".repeat(decimals)}` : text.padEnd(dot + 1 + decimals, "0");
  }
  return type === TIME ? text : text.replace(" ", "T");
}

/** mysql2's execute(), resolving to [rows, fields]. */
export async function mysql2(m: AnyClient, text: string, values: unknown[]): Promise<unknown[]> {
  const [rows, fields] = await m.execute({ sql: text, ...MYSQL2_OPTIONS, typeCast: mysql2Cast }, values);
  if (!Array.isArray(rows)) return [];
  const dates = ((fields ?? []) as Mysql2Column[]).filter((f) => f.columnType === DATETIME || f.columnType === TIMESTAMP || (f.columnType === TIME && f.decimals > 0));
  if (dates.length) {
    for (const row of rows as Record<string, unknown>[]) {
      for (const f of dates) {
        const v = row[f.name];
        if (typeof v === "string") row[f.name] = wallClock(v, f.decimals > 6 ? 0 : f.decimals, f.columnType);
      }
    }
  }
  return rows;
}

// ---------------------------------------------------------------------------------------------
// mariadb

/** Every decoding option, each laid over the app's for this query (mariadb's parser.js: Object.assign(Object.create(connOpts), opts)). */
const MARIADB_OPTIONS = {
  bigIntAsNumber: false,
  decimalAsNumber: false,
  supportBigNumbers: true,
  bigNumberStrings: true,
  checkNumberRange: false,
  dateStrings: true,
  jsonStrings: false,
  autoJsonMap: true,
  bitOneIsBoolean: false,
  rowsAsArray: false,
  nestTables: false,
  metaAsArray: false,
  typeCast: undefined,
};

/** A GEOMETRY's bytes, where mariadb would give GeoJSON; only for a statement whose columns hold one, since a typeCast sends every value through mariadb's slower path. */
const mariadbGeometry = (column: { type: string; buffer(): unknown }, next: () => unknown) => (column.type === "GEOMETRY" ? column.buffer() : next());

type MariadbColumn = { type?: string; columnType?: number; name?: string | (() => string); isSet?: () => boolean };

/** mariadb's query(), on its promise or its callback API, with Reylo's options. */
export async function mariadb(c: AnyClient, text: string, values: unknown[], columns: Columns | undefined): Promise<unknown[]> {
  const options = { sql: text, ...MARIADB_OPTIONS, ...(columns?.some(([, kind]) => kind === "unknown") ? { typeCast: mariadbGeometry } : {}) };
  const result = await new Promise((resolve, reject) => {
    const returned = c.query(options, values, (error: Error | null, rows: unknown) => (error ? reject(error) : resolve(rows)));
    if (typeof returned?.then === "function") returned.then(resolve, reject);
  });
  if (!Array.isArray(result)) return [];
  // A DATETIME and a TIMESTAMP with `T`, and a SET's members joined as MySQL writes them.
  const meta = ((result as { meta?: MariadbColumn[] }).meta ?? []).map((m) => ({ name: typeof m.name === "function" ? m.name() : m.name, type: m.type, set: typeof m.isSet === "function" && m.isSet() }));
  const work = meta.filter((m) => m.name !== undefined && (m.set || m.type === "DATETIME" || m.type === "TIMESTAMP"));
  if (!work.length) return result;
  for (const row of result as Record<string, unknown>[]) {
    for (const m of work) {
      const v = row[m.name!];
      if (v === null || v === undefined) continue;
      row[m.name!] = m.set ? coerce("text", v, m.name!) : typeof v === "string" ? v.replace(" ", "T") : v;
    }
  }
  return result;
}

// ---------------------------------------------------------------------------------------------
// PlanetScale 1.x

type PlanetscaleField = { type: string; charset?: number };
/** PlanetScale hands each value over as a string of bytes, one character each. */
const bytesOf = (s: string) => {
  const out = new Uint8Array(s.length);
  for (let i = 0; i < s.length; i++) out[i] = s.charCodeAt(i);
  return out;
};

/** PlanetScale's own default cast (cast.js), with a DATETIME's and a TIMESTAMP's `T`: it replaces any cast of the app's for this query. */
function planetscaleCast(field: PlanetscaleField, value: string | null): unknown {
  if (value === null || value === undefined) return value;
  switch (field.type) {
    case "INT64":
    case "UINT64":
    case "DATE":
    case "TIME":
    case "DECIMAL":
      return value;
    case "DATETIME":
    case "TIMESTAMP":
      return value.replace(" ", "T");
    case "JSON":
      return JSON.parse(utf8(bytesOf(value)));
    case "INT8":
    case "INT16":
    case "INT24":
    case "INT32":
    case "UINT8":
    case "UINT16":
    case "UINT24":
    case "UINT32":
    case "YEAR":
      return parseInt(value, 10);
    case "FLOAT32":
    case "FLOAT64":
      return parseFloat(value);
  }
  if (field.charset === 63) return bytesOf(value);
  return value ? utf8(bytesOf(value)) : "";
}

/** PlanetScale's execute(): 1.x binds parameters; 2.0 takes none, and its options come second. */
export async function planetscale(m: AnyClient, text: string, values: unknown[]): Promise<unknown[]> {
  const options = { as: "object", cast: planetscaleCast };
  let result;
  try {
    result = await m.execute(text, values.length ? values : null, options);
  } catch (error) {
    if (!/^Query parameters are not supported/.test((error as Error)?.message)) throw error;
    if (values.length) throw new TypeError(`@planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint.`, { cause: error });
    result = await m.execute(text, options);
  }
  return Array.isArray(result?.rows) ? result.rows : [];
}
