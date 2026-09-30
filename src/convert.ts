// The representation of temporal and 64-bit integer values, applied to the rows a driver
// returns. Drivers agree on little here: every Postgres and MySQL driver decodes a timestamp
// as a Date, but an int8 arrives as a string (pg, postgres, Bun.sql), a number (mysql2, the
// SQLite drivers, lossy above 2^53) or a BigInt (mariadb, PGlite above 2^53). The mapper's
// `dates` and `bigint` options name one representation, and this pass produces it.

/** How `run` represents temporal and 64-bit integer values, from the mapper options. */
export type Repr = { dates: "string" | "Date" | "temporal"; bigint: "string" | "number" | "bigint" };
export const defaultRepr: Repr = { dates: "string", bigint: "string" };

/** What a temporal or int8 column needs at runtime: its class, `[]` for an array of them. */
export type Conversion = "timestamptz" | "timestamp" | "date" | "int8" | "timestamptz[]" | "timestamp[]" | "date[]" | "int8[]";

const pad = (n: number, width = 2) => String(n).padStart(width, "0");

/**
 * A zone-less `timestamp` or `date` arrives as a Date built from wall-clock fields; every
 * driver measured builds it in local time, except that `postgres`, PGlite and Bun.sql build a
 * `date` at UTC midnight. The fields are read back in the frame that holds midnight, which
 * is the only frame that can (a local offset of zero makes both agree).
 */
function fields(d: Date, cls: "timestamp" | "date") {
  const utc = cls === "date" && d.getUTCHours() === 0 && d.getUTCMinutes() === 0 && !(d.getHours() === 0 && d.getMinutes() === 0);
  return utc
    ? { year: d.getUTCFullYear(), month: d.getUTCMonth() + 1, day: d.getUTCDate(), hour: d.getUTCHours(), minute: d.getUTCMinutes(), second: d.getUTCSeconds(), millisecond: d.getUTCMilliseconds() }
    : { year: d.getFullYear(), month: d.getMonth() + 1, day: d.getDate(), hour: d.getHours(), minute: d.getMinutes(), second: d.getSeconds(), millisecond: d.getMilliseconds() };
}

// The slice of the global Temporal this pass calls, so the runtime needs no Temporal types.
type TemporalLike = {
  Instant: { from(iso: string): unknown; fromEpochMilliseconds(ms: number): unknown };
  PlainDateTime: { from(value: string | object): unknown };
  PlainDate: { from(value: string | object): unknown };
};

function temporalOf(): TemporalLike {
  const t = (globalThis as { Temporal?: TemporalLike }).Temporal;
  if (!t) throw new Error(`The mapper option "dates": "temporal" needs a global Temporal (Node 26 has one; Bun needs a polyfill assigned to globalThis.Temporal).`);
  return t;
}

/** One value of the class, in the chosen representation. A null stays null; a string the driver was configured to return stays a string. */
export function convertValue(value: unknown, cls: Conversion, repr: Repr): unknown {
  if (value === null || value === undefined) return value;
  if (cls.endsWith("[]")) return Array.isArray(value) ? value.map((v) => convertValue(v, cls.slice(0, -2) as Conversion, repr)) : value;
  if (cls === "int8") {
    switch (repr.bigint) {
      case "string": return typeof value === "string" ? value : String(value);
      case "number": return typeof value === "number" ? value : Number(value);
      case "bigint": return typeof value === "bigint" ? value : BigInt(value as string | number);
    }
  }
  if (!(value instanceof Date)) {
    // A driver configured to return the text form: only `temporal` has something to build.
    if (repr.dates === "temporal" && typeof value === "string") {
      const T = temporalOf();
      const iso = value.replace(" ", "T");
      return cls === "timestamptz" ? T.Instant.from(iso) : cls === "timestamp" ? T.PlainDateTime.from(iso) : T.PlainDate.from(iso);
    }
    return value;
  }
  if (repr.dates === "Date") return value;
  if (cls === "timestamptz") return repr.dates === "temporal" ? temporalOf().Instant.fromEpochMilliseconds(value.getTime()) : value.toISOString();
  const f = fields(value, cls as "timestamp" | "date");
  if (repr.dates === "temporal") return cls === "timestamp" ? temporalOf().PlainDateTime.from(f) : temporalOf().PlainDate.from({ year: f.year, month: f.month, day: f.day });
  const day = `${pad(f.year, 4)}-${pad(f.month)}-${pad(f.day)}`;
  return cls === "date" ? day : `${day}T${pad(f.hour)}:${pad(f.minute)}:${pad(f.second)}.${pad(f.millisecond, 3)}`;
}

/** The rows with every converted column in the chosen representation; rows are updated in place. */
export function convertRows(rows: unknown[], convert: Record<string, Conversion>, repr: Repr): unknown[] {
  const names = Object.keys(convert);
  if (!names.length) return rows;
  for (const row of rows as Record<string, unknown>[]) {
    if (!row || typeof row !== "object") continue;
    for (const name of names) if (name in row) row[name] = convertValue(row[name], convert[name]!, repr);
  }
  return rows;
}
