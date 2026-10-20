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
//   statement's own integer mode to BigInt, whatever the database's: every integer then arrives
//   exact and is a number, or an error naming its column past 2^53, and no other statement of the
//   app's changes. bun:sqlite's statement comes from `prepare`, never `query`, whose cache by text
//   would carry Reylo's mode into the app's statement.
// - sqlite3 and the sqlite wrapper, libSQL, D1 and Bun.SQL's SQLite: the rows the driver gives,
//   normalized. sqlite3 and sqlite are not cached: a statement left open makes the app's close()
//   fail with SQLITE_BUSY. An INTEGER the driver already rounded (sqlite3, sqlite, D1, Bun.SQL
//   without safeIntegers) is refused in an INTEGER column, where a number past 2^53 can only be one.
import { coerce, exactInteger, type Columns, type Kind } from "./values.ts";

type AnyClient = Record<string, any>;
type Statement = AnyClient;

const statements = new WeakMap<object, Map<string, Statement>>();
/** How many statements Reylo keeps per database: more distinct texts than that drop the oldest. */
const KEPT = 500;

/** Reylo's own statement for `text` on the app's database, with integers read as BigInts. */
function statementFor(db: AnyClient, text: string): Statement {
  let kept = statements.get(db);
  if (!kept) statements.set(db, (kept = new Map()));
  let st = kept.get(text);
  if (st) return st;
  st = db.prepare(text) as Statement;
  // node:sqlite's StatementSync; better-sqlite3's and bun:sqlite's Statement.
  if (typeof st.setReadBigInts === "function") {
    st.setReadBigInts(true);
    if (typeof st.setReturnArrays === "function") st.setReturnArrays(false);
  } else if (typeof st.safeIntegers === "function") st.safeIntegers(true);
  if (kept.size >= KEPT) kept.delete(kept.keys().next().value!);
  kept.set(text, st);
  return st;
}

/** Runs `text` on a synchronous SQLite database (better-sqlite3, node:sqlite, bun:sqlite) and returns its normalized rows. */
export function runSyncSqlite(db: AnyClient, text: string, values: unknown[], columns: Columns | undefined): unknown[] {
  const st = statementFor(db, text);
  // better-sqlite3 refuses all() on a statement that returns no rows.
  if (st.reader === false) {
    st.run(...values);
    return [];
  }
  let rows = st.all(...values) as unknown[];
  // A database opened with returnArrays, on a Node whose statements cannot turn it off.
  if (rows.length && Array.isArray(rows[0]) && typeof st.columns === "function") {
    const names = (st.columns() as { name: string }[]).map((c) => c.name);
    rows = (rows as unknown[][]).map((r) => Object.fromEntries(names.map((n, i) => [n, r[i]])));
  }
  return normalize(rows, columns, { exact: true });
}

/** The kinds that need a value converted whatever the driver; an INTEGER's and a BLOB's are by the value's type. */
const CONVERTED = /^(json|bool)$|\|/;

/**
 * The rows with each value in Reylo's form: a BigInt a number (an error past 2^53), a number past
 * 2^53 in an INTEGER column refused as rounded, a BLOB a Uint8Array, a JSON column parsed and a
 * truth value a boolean. In place, since every driver builds new rows per call. `exact`: the driver
 * read every INTEGER as a BigInt, so a number is a REAL, never a rounded integer. `d1`: an array is
 * D1's BLOB.
 */
export function normalize(rows: unknown[], columns: Columns | undefined, { exact = false, d1 = false } = {}): unknown[] {
  if (!Array.isArray(rows) || !rows.length) return Array.isArray(rows) ? rows : [];
  const plan: (readonly [string, Kind])[] = columns ? [...columns] : Object.keys(rows[0] as object).map((name) => [name, "unknown"] as const);
  const n = plan.length;
  for (const row of rows as Record<string, unknown>[]) {
    for (let i = 0; i < n; i++) {
      const [name, kind] = plan[i]!;
      const v = row[name];
      if (v === null || v === undefined) continue;
      switch (typeof v) {
        case "string":
          // JSON, a truth value, and libSQL's integers under intMode "string".
          if (CONVERTED.test(kind) || kind === "int") row[name] = coerce(kind, v, name);
          continue;
        case "number":
          if ((kind === "int" && !exact) || CONVERTED.test(kind)) row[name] = coerce(kind, v, name);
          continue;
        case "bigint":
          row[name] = kind === "unknown" || kind === "float" ? exactInteger(v, name) : coerce(kind, v, name);
          continue;
        case "object":
          if (v instanceof Uint8Array) continue;
          // libSQL's ArrayBuffer, D1's array of numbers.
          if (v instanceof ArrayBuffer || (d1 && Array.isArray(v))) row[name] = coerce("bytes", v, name);
          continue;
      }
    }
  }
  return rows;
}
