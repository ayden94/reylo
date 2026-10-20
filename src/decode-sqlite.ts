// Reylo gives every value of its own SQLite queries its type itself (the maintainer, 2026-10-20;
// the representation is src/values.ts). Every SQLite driver already turns each storage class into
// a JavaScript value in native code, and they agree but on two: an INTEGER (a number that may be
// rounded past 2^53, an error, a BigInt, or libSQL's string) and a BLOB (a Buffer, a Uint8Array, an
// ArrayBuffer or D1's array of numbers). And only the analysis knows that a TEXT holds JSON or that
// an INTEGER is a truth value. So Reylo normalizes the driver's rows, by each column's kind where the
// module carries it (docs/driver-decoding/spike/sqlite.md):
//
// - better-sqlite3, node:sqlite and bun:sqlite: Reylo prepares its own statement on the app's
//   database, once per database and text (a WeakMap keyed by the database), and sets that
//   statement's own integer mode, whatever the database's, and no other statement of the app's
//   changes. node:sqlite's statement, and better-sqlite3's and bun:sqlite's where a column has no
//   type the module knows, read every INTEGER as a BigInt: each is then a number, or an error naming
//   its column past 2^53. Where every column is typed, better-sqlite3's and bun:sqlite's read numbers
//   (a BigInt per integer cost +70% on 10,000 rows), and a number past 2^53 in a column that may hold
//   an INTEGER can only be a rounded one, which is refused. bun:sqlite's statement comes from
//   `prepare`, never `query`, whose cache by text would carry Reylo's mode into the app's statement.
// - sqlite3 and the sqlite wrapper, libSQL, D1 and Bun.SQL's SQLite: the rows the driver gives,
//   normalized. sqlite3 and sqlite are not cached: a statement left open makes the app's close()
//   fail with SQLITE_BUSY. An INTEGER the driver already rounded (sqlite3, sqlite, D1, Bun.SQL
//   without safeIntegers) is refused in an INTEGER column, where a number past 2^53 can only be one.
import { coerce, exactInteger, type Columns, type Kind } from "./values.ts";

type AnyClient = Record<string, any>;
type Statement = AnyClient;

/** Reylo's statement, and whether it reads every INTEGER as a BigInt (a driver of the three). */
type Kept = { st: Statement; exact: boolean };
const statements = new WeakMap<object, Map<string, Kept>>();
/** How many statements Reylo keeps per database: more distinct texts than that drop the oldest. */
const KEPT = 500;

/** Reylo's own statement for `text` on the app's database, with integers read as BigInts. */
function statementFor(db: AnyClient, text: string, typed: boolean): Kept {
  let kept = statements.get(db);
  if (!kept) statements.set(db, (kept = new Map()));
  const key = `${typed ? "n" : "b"}${text}`;
  const found = kept.get(key);
  if (found) return found;
  const st = db.prepare(text) as Statement;
  let exact = false;
  // node:sqlite's StatementSync; better-sqlite3's and bun:sqlite's Statement.
  if (typeof st.setReadBigInts === "function") {
    st.setReadBigInts(true);
    if (typeof st.setReturnArrays === "function") st.setReturnArrays(false);
    exact = true;
  } else if (typeof st.safeIntegers === "function") {
    st.safeIntegers(!typed);
    exact = !typed;
  }
  if (kept.size >= KEPT) kept.delete(kept.keys().next().value!);
  const entry = { st, exact };
  kept.set(key, entry);
  return entry;
}

/**
 * Runs `text` on a synchronous SQLite database (better-sqlite3, node:sqlite, bun:sqlite) and returns
 * its normalized rows; a statement that answers through a promise (no such database) gives the
 * promise, which `runSync` refuses.
 */
export function runSyncSqlite(db: AnyClient, text: string, values: unknown[], columns: Columns | undefined): unknown[] | Promise<unknown[]> {
  // Every column typed: a number past 2^53 in one can be told as rounded (src/values.ts).
  const typed = columns !== undefined && columns.length > 0 && !columns.some(([, kind]) => kind === "unknown");
  const { st, exact } = statementFor(db, text, typed);
  // better-sqlite3 refuses all() on a statement that returns no rows.
  if (st.reader === false) {
    st.run(...values);
    return [];
  }
  const all = st.all(...values) as unknown[] | Promise<unknown[]>;
  if (!Array.isArray(all)) return Promise.resolve(all).then((rows) => normalize(rows, columns));
  let rows = all;
  // A database opened with returnArrays, on a Node whose statements cannot turn it off.
  if (rows.length && Array.isArray(rows[0]) && typeof st.columns === "function") {
    const names = (st.columns() as { name: string }[]).map((c) => c.name);
    rows = (rows as unknown[][]).map((r) => Object.fromEntries(names.map((n, i) => [n, r[i]])));
  }
  return normalize(rows, columns, { exact });
}

type Plan = {
  name: string;
  /** An INTEGER the driver read as a BigInt. */
  big: (v: bigint) => unknown;
  /** A number, where it may need converting: a truth value, or an INTEGER a driver may have rounded. */
  num: ((v: number) => unknown) | undefined;
  /** A string, where it may need converting: JSON, a truth value, libSQL's integer under intMode "string". */
  str: ((v: string) => unknown) | undefined;
};

/** What each column's values need, worked out once per call rather than per value. */
function plan(name: string, kind: Kind, exact: boolean): Plan {
  if (kind === "bool") return { name, big: (v) => v !== BigInt(0), num: (v) => v !== 0, str: (v) => coerce(kind, v, name) };
  if (kind === "json") return { name, big: (v) => exactInteger(v, name), num: undefined, str: (v) => JSON.parse(v) };
  if (kind === "text") return { name, big: String, num: undefined, str: undefined };
  if (kind === "int") return { name, big: (v) => exactInteger(v, name), num: exact ? undefined : (v) => exactInteger(v, name), str: (v) => coerce(kind, v, name) };
  // A NUMERIC's either: read as numbers, one past 2^53 may be a rounded INTEGER, which is refused.
  if (!exact && kind.split("|").includes("int")) return { name, big: (v) => exactInteger(v, name), num: (v) => (Number.isInteger(v) ? exactInteger(v, name) : v), str: undefined };
  // A REAL, a BLOB, a column with no type: an integer is exact or refused, a number is as it is.
  return { name, big: (v) => exactInteger(v, name), num: undefined, str: undefined };
}

/**
 * The rows with each value in Reylo's form: a BigInt a number (an error past 2^53), a number past
 * 2^53 in an INTEGER column refused as rounded, a BLOB a Uint8Array, a JSON column parsed and a
 * truth value a boolean. In place, since every driver builds new rows per call. `exact`: the driver
 * read every INTEGER as a BigInt, so a number is a REAL, never a rounded integer. `d1`: an array is
 * D1's BLOB.
 */
export function normalize(rows: unknown[], columns: Columns | undefined, { exact = false, d1 = false } = {}): unknown[] {
  if (!Array.isArray(rows) || !rows.length) return Array.isArray(rows) ? rows : [];
  const plans = (columns ? [...columns] : Object.keys(rows[0] as object).map((name) => [name, "unknown"] as const)).map(([name, kind]) => plan(name, kind, exact));
  const n = plans.length;
  for (const row of rows as Record<string, unknown>[]) {
    for (let i = 0; i < n; i++) {
      const p = plans[i]!;
      const v = row[p.name];
      if (v === null || v === undefined) continue;
      switch (typeof v) {
        case "bigint":
          row[p.name] = p.big(v);
          break;
        case "number":
          if (p.num) row[p.name] = p.num(v);
          break;
        case "string":
          if (p.str) row[p.name] = p.str(v);
          break;
        case "object":
          // libSQL's ArrayBuffer, D1's array of numbers.
          if (!(v instanceof Uint8Array) && (v instanceof ArrayBuffer || (d1 && Array.isArray(v)))) row[p.name] = coerce("bytes", v, p.name);
          break;
      }
    }
  }
  return rows;
}
