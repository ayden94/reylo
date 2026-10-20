// Structured parameters at run time (docs/structured-params.md): the value of a `:events::rows(…)`
// or `:user::row(…)` parameter is checked against its column list before the statement runs,
// written as the JSON the statement reads, and an insert's column lists are written again
// without the keys a call leaves out, so that each of those columns takes its default (the
// maintainer, 2026-10-15). Nothing here changes a value: a value the database would round,
// truncate or read in a zone nobody chose is refused (the maintainer, 2026-10-15: "Never, except
// defaults").
import type { EngineType } from "./engine-type.ts";
import { validate, withDefaults } from "./json-schema.ts";

type Dialect = "postgres" | "mysql" | "sqlite";
export type RowsType = Extract<EngineType, { kind: "rows" }>;
export type RowField = RowsType["fields"][number];

/** One comma-separated list of an insert's expansion, as UTF-16 offsets into the module's `text`: an entry per member, in the column list's order. */
export type StructuredList = { readonly start: number; readonly end: number; readonly entries: readonly string[] };
/** A structured parameter in insert position and the lists the run time writes again for the keys a call carries. */
export type Structured = { readonly name: string; readonly lists: readonly StructuredList[] };

/**
 * A string for a `timestamptz` names its zone: without one the server reads it in the session's
 * TimeZone, which the driver or the server sets.
 */
export const zoned = /(?:\d:\d\d(?::\d\d(?:\.\d+)?)?(?:\s*(?:z|[+-]\d\d?(?::?\d\d){0,2})|\s+[a-z][\w/+-]*)|^-?infinity|^epoch|^now)(?:\s+bc)?$/i;

const INTEGERS: Record<string, readonly [bigint, bigint]> = {
  int2: [-32768n, 32767n],
  int4: [-2147483648n, 2147483647n],
  int8: [-(2n ** 63n), 2n ** 63n - 1n],
  // SQLite's INTEGER is a 64-bit integer.
  integer: [-(2n ** 63n), 2n ** 63n - 1n],
};

/**
 * A 64-bit integer, which JSON carries as digits: a string on Postgres, which reads one exactly, and
 * a bare number elsewhere (MySQL's `CAST(… AS SIGNED)` wraps a string out of range;
 * docs/structured-params/verify.md). A Postgres or MySQL `int8` takes a number, a BigInt or a string
 * of digits, as its parameter type says; SQLite's `integer` takes a number or a BigInt (src/render.ts).
 */
const WIDE = new Set(["int8", "integer"]);

/** Raw JSON text for a value the writer puts in as it is. */
class Raw {
  readonly text: string;
  constructor(text: string) {
    this.text = text;
  }
}

/**
 * Checks a structured parameter's value against its column list, and gives the value to bind:
 * on Postgres the JSON value, which the run time binds as it binds every JSON parameter, and on
 * MySQL and SQLite its JSON text. `inserted` is a parameter whose rows an insert writes, whose
 * keys are the same in every row; `present` is the keys they carry.
 */
export function prepareRows(name: string, type: RowsType, value: unknown, dialect: Dialect, inserted: boolean): { value: unknown; present: ReadonlySet<string> } {
  const fail = (message: string): never => {
    throw new TypeError(`Parameter "${name}": ${message}`);
  };
  if (value === undefined || value === null) fail(type.one ? "pass one object, whose keys the column list names." : "pass an array of objects, whose keys the column list names; an empty array inserts nothing.");
  const rows = type.one ? [value] : Array.isArray(value) ? value : fail(`pass an array of objects, one per row; got ${describe(value)}.`);
  const known = new Map(type.fields.map((f) => [f.name, f]));
  let present: Set<string> | undefined;
  let first = -1;
  const out: Record<string, unknown>[] = [];
  rows.forEach((row: unknown, i: number) => {
    const where = type.one ? "" : `row ${i + 1}: `;
    if (row === null || typeof row !== "object" || Array.isArray(row) || row instanceof Date) fail(`${where}each row is an object whose keys the column list names; got ${describe(row)}.`);
    const r = row as Record<string, unknown>;
    for (const key of Object.keys(r)) {
      if (!known.has(key) && r[key] !== undefined) fail(`${where}the column list names no key "${key}" (it names ${type.fields.map((f) => `"${f.name}"`).join(", ")}).`);
    }
    const keys = new Set<string>();
    const written: Record<string, unknown> = {};
    for (const f of type.fields) {
      const v = r[f.name];
      if (v === undefined) {
        if (!f.optional) fail(`${where}"${f.name}" is ${f.written} not null, and the row leaves it out.`);
        continue;
      }
      keys.add(f.name);
      if (v === null) {
        if (!f.nullable) fail(`${where}"${f.name}" is ${f.written} not null, and the row gives null${f.optional ? `; leave the key out for the column's default` : ""}.`);
        written[f.name] = null;
        continue;
      }
      const checked = check(f.type, f.written, v, dialect, (message) => fail(`${where}"${f.name}" ${message}`));
      // A JSON member's schema fills its defaults and checks the value (docs/typed-json-params.md).
      if (f.schema !== undefined) {
        const filled = withDefaults(f.schema, checked);
        const failure = validate(f.schema, filled);
        if (failure) fail(`${where}"${f.name}"${failure.path ? ` at ${failure.path}` : ""} ${failure.message}.`);
        written[f.name] = filled;
      } else written[f.name] = checked;
    }
    // One statement inserts every row of a call, so its column list is the same for each: a key
    // one row leaves out and another gives cannot have its default in one and its value in the
    // other.
    if (inserted) {
      if (present === undefined) {
        present = keys;
        first = i;
      } else {
        const differ = type.fields.find((f) => present!.has(f.name) !== keys.has(f.name));
        if (differ) fail(`row ${i + 1} ${keys.has(differ.name) ? "gives" : "leaves out"} "${differ.name}", which row ${first + 1} ${keys.has(differ.name) ? "leaves out" : "gives"}. Every row of one call gives the same keys, since one insert writes them all and a key left out takes its column's default: give "${differ.name}" in every row, or in none.`);
      }
    }
    out.push(written);
  });
  const keys = present ?? new Set(type.fields.map((f) => f.name));
  if (inserted && !type.fields.some((f) => keys.has(f.name))) fail("every key is left out, so the insert would name no column; give one at least.");
  const shaped = type.one ? out[0] : out;
  return { value: dialect === "postgres" ? shaped : write(shaped), present: keys };
}

/** One value checked against its type, as the JSON the database reads; `fail` says why not. */
function check(type: EngineType, written: string, v: unknown, dialect: Dialect, fail: (message: string) => never): unknown {
  switch (type.kind) {
    case "json":
      // JSON.stringify throws on a BigInt and skips a function; a JSON member holds JSON.
      try {
        if (JSON.stringify(v) === undefined) fail(`is ${written}, which holds a JSON value; got ${describe(v)}.`);
      } catch (e) {
        fail(`is ${written}, which holds a JSON value: ${(e as Error).message}.`);
      }
      return v;
    case "enum":
      if (typeof v === "string" && (type.labels.length === 0 || type.labels.includes(v))) return v;
      return fail(`is the enum ${type.name}, one of ${type.labels.map((l) => JSON.stringify(l)).join(", ")}; got ${describe(v)}.`);
    case "array": {
      if (!Array.isArray(v)) return fail(`is ${written}, an array; got ${describe(v)}.`);
      const element = written.replace(/\s*\[\s*\]\s*$/, "");
      return v.map((e, i) => (e === null ? null : check(type.of, element, e, dialect, (message) => fail(`element ${i + 1} ${message}`))));
    }
    case "scalar":
      return scalar(type.name, written, v, dialect, fail);
    default:
      return v;
  }
}

function scalar(name: string, written: string, v: unknown, dialect: Dialect, fail: (message: string) => never): unknown {
  // `a|b` is either, where the database decides by the value.
  if (name.includes("|")) {
    for (const part of name.split("|")) {
      try {
        return scalar(part, written, v, dialect, (message) => {
          throw new TypeError(message);
        });
      } catch {}
    }
    return fail(`is ${written}; got ${describe(v)}.`);
  }
  const range = INTEGERS[name];
  if (range) {
    let n: bigint;
    if (typeof v === "bigint") n = v;
    else if (typeof v === "number" && Number.isInteger(v)) {
      if (!Number.isSafeInteger(v)) fail(`is ${written}; ${v} is past 2^53 - 1, where a number stops being exact, so pass it as a BigInt or a string of digits.`);
      n = BigInt(v);
    } else if (typeof v === "string" && /^[+-]?\d+$/.test(v.trim()) && name === "int8") n = BigInt(v.trim());
    else return fail(`is ${written}, an integer; got ${describe(v)}.`);
    if (n < range[0] || n > range[1]) fail(`is ${written}, from ${range[0]} to ${range[1]}; got ${n}.`);
    if (!WIDE.has(name)) return Number(n);
    return dialect === "postgres" ? n.toString() : new Raw(n.toString());
  }
  switch (name) {
    case "float4":
    case "float8":
    case "real":
      if (typeof v === "number" && Number.isFinite(v)) return v;
      return fail(`is ${written}, a finite number; got ${describe(v)}.`);
    case "numeric":
      return decimal(written, v, fail);
    case "bool":
      if (typeof v === "boolean") return v;
      return fail(`is ${written}, true or false; got ${describe(v)}.`);
    case "text":
    case "varchar":
    case "bpchar":
    case "char":
    case "name":
    case "citext": {
      if (typeof v !== "string") return fail(`is ${written}, a string; got ${describe(v)}.`);
      const n = /\(\s*(\d+)\s*\)/.exec(written)?.[1];
      // A length counts characters (code points), as Postgres and MySQL count them.
      if (n !== undefined && [...v].length > Number(n)) fail(`is ${written}, at most ${n} characters; got ${[...v].length}, which the database would refuse or cut.`);
      return v;
    }
    case "uuid":
      if (typeof v === "string" && /^\{?[0-9a-f]{8}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{4}-?[0-9a-f]{12}\}?$/i.test(v)) return v;
      return fail(`is ${written}, a UUID in hexadecimal; got ${describe(v)}.`);
    case "timestamptz":
      if (v instanceof Date) {
        if (Number.isNaN(v.getTime())) fail(`is ${written}, and the Date is invalid.`);
        return v.toISOString();
      }
      if (typeof v === "string" && zoned.test(v.trim())) return v;
      return fail(`is ${written}, an instant: a Date, or a string that names its zone ("2026-10-12T19:06:48.282Z", "2026-10-12 19:06:48+09"); got ${describe(v)}, which the server would read in the session's time zone.`);
    case "timestamp":
    case "date":
      if (typeof v === "string") return v;
      return fail(`is ${written}, a wall-clock reading with no zone, which takes its text ("2026-10-12 19:06:48.282", "2026-10-12"), not a Date: drivers write a Date's time in different zones.${v instanceof Date ? " A timestamptz takes a Date." : ` Got ${describe(v)}.`}`);
    case "bytea":
    case "blob": {
      // Binary data travels as hexadecimal text: Postgres's own `\x` form, and bare digits that
      // MySQL's and SQLite's readers decode with unhex().
      if (!(v instanceof Uint8Array)) return fail(`is ${written}, binary data, a Uint8Array; got ${describe(v)}.`);
      let hex = dialect === "postgres" ? "\\x" : "";
      for (const byte of v) hex += (byte < 16 ? "0" : "") + byte.toString(16);
      return hex;
    }
    default:
      // A type the run time has no rule for (time, interval, inet, …) takes the text the database
      // reads, and a number or a truth value as JSON writes them.
      if (typeof v === "string" || typeof v === "number" || typeof v === "boolean") return v;
      return fail(`is ${written}; got ${describe(v)}.`);
  }
}

/**
 * A decimal, as a number or as its digits. `numeric(p,s)` keeps `s` digits after the point and
 * `p` in all: a value with more decimals would be rounded, so it is refused, and so is one with
 * more digits before the point than the column holds.
 */
function decimal(written: string, v: unknown, fail: (message: string) => never): unknown {
  let text: string;
  if (typeof v === "number") {
    if (!Number.isFinite(v)) return fail(`is ${written}, a finite decimal; got ${v}.`);
    text = String(v);
  } else if (typeof v === "string") text = v.trim();
  else if (typeof v === "bigint") text = v.toString();
  else return fail(`is ${written}, a decimal: a number or a string of its digits; got ${describe(v)}.`);
  const m = /^([+-]?)(\d*)(?:\.(\d*))?(?:e([+-]?\d+))?$/i.exec(text);
  if (!m || (m[2] === "" && (m[3] ?? "") === "")) return fail(`is ${written}, a decimal: a number or a string of its digits; got ${describe(v)}.`);
  const mod = /\(\s*(\d+)\s*(?:,\s*(-?\d+)\s*)?\)/.exec(written);
  if (mod) {
    const precision = Number(mod[1]);
    const scale = Number(mod[2] ?? 0);
    const exponent = Number(m[4] ?? 0);
    let whole = (m[2] ?? "").replace(/^0+/, "");
    let fraction = (m[3] ?? "").replace(/0+$/, "");
    // Shift the point by the exponent.
    if (exponent > 0) {
      const moved = fraction.slice(0, exponent).padEnd(exponent, "0");
      whole = (whole + moved).replace(/^0+/, "");
      fraction = fraction.slice(exponent);
    } else if (exponent < 0) {
      const digits = whole.padStart(-exponent, "0");
      fraction = (digits.slice(digits.length + exponent) + fraction).replace(/0+$/, "");
      whole = digits.slice(0, digits.length + exponent).replace(/^0+/, "");
    }
    if (fraction.length > scale) fail(`is ${written}, which keeps ${scale} digit${scale === 1 ? "" : "s"} after the point, so ${text} would be rounded.`);
    if (whole.length > precision - scale) fail(`is ${written}, which holds at most ${precision - scale} digit${precision - scale === 1 ? "" : "s"} before the point; got ${text}.`);
  }
  return typeof v === "bigint" ? v.toString() : v;
}

/** The checked rows as JSON text, a 64-bit integer as its digits. */
function write(v: unknown): string {
  if (v instanceof Raw) return v.text;
  if (Array.isArray(v)) return `[${v.map(write).join(",")}]`;
  if (v !== null && typeof v === "object") {
    return `{${Object.entries(v as Record<string, unknown>)
      .map(([k, x]) => `${JSON.stringify(k)}:${write(x)}`)
      .join(",")}}`;
  }
  return JSON.stringify(v);
}

/** The statement with each list of a structured parameter written again for the keys present: an entry stays when its key is required or present. */
export function variant(text: string, structured: Structured, fields: readonly RowField[], present: ReadonlySet<string>): string {
  const lists = [...structured.lists].sort((a, b) => b.start - a.start);
  let out = text;
  for (const list of lists) {
    const kept = list.entries.filter((_, k) => {
      const f = fields[k];
      return f === undefined || !f.optional || present.has(f.name);
    });
    out = out.slice(0, list.start) + kept.join(", ") + out.slice(list.end);
  }
  return out;
}

function describe(v: unknown): string {
  if (v === null) return "null";
  if (v instanceof Date) return "a Date";
  if (Array.isArray(v)) return "an array";
  if (v instanceof Uint8Array) return "binary data";
  if (typeof v === "bigint") return `${v}n`;
  if (typeof v === "string") return JSON.stringify(v.length > 60 ? `${v.slice(0, 57)}...` : v);
  if (typeof v === "object") return "an object";
  return String(v);
}
