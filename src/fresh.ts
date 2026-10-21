// The schema freshness check at run time (docs/structured-params.md §6; the maintainer,
// 2026-10-20: any change fails). A module a build step writes carries the catalog facts its
// statement relied on, the columns it names with their types, whether each is not null and whether
// it has a default, and for an insert's table the columns that need a value, beside the catalog
// query that read them (engine/core/src/fresh.rs). Before a query runs, the run time runs that query
// through the caller's own client and compares: any difference fails the call, naming the table
// and the column, rather than letting the query run against a schema its types were not made for.
//
// A query is checked on its first run in a process, and again when its last check is a minute old,
// so a long-running server notices a migration that lands while it runs. A catalog query that
// fails (a role that may not read the catalog) checks nothing, and the query runs; so does one
// whose answer is not shaped as its rows are, which no database gives: a client that answers every
// query with rows of its own, as a stub or a mock does, is no catalog to check against.

/** What a module carries: the catalog query, which takes no parameter, and the facts as text. */
export type Fresh = {
  readonly query: string;
  readonly rows: readonly (readonly string[])[];
  readonly inserts: readonly string[];
};

/** How long a check holds, in milliseconds. */
export const FRESH_FOR = 60_000;

const checked = new WeakMap<object, number>();

// The clock is `performance.now()`, not `Date.now()`: an interval needs only a monotonic clock, and
// Next.js's Cache Components refuse `Date.now()` while a page prerenders, which failed `next build`
// on every page that ran a query (examples/nextjs, Next.js 16.4). It is reached through
// `globalThis`, typed here, since a project that type-checks Reylo's sources need not have the
// DOM's types or Node's, which declare it (the labs' type checks failed with "Cannot find name").
const clock = (globalThis as unknown as { performance: { now(): number } }).performance;

/** Whether `query` is due for a check now. */
export function due(query: object, now = clock.now()): boolean {
  const at = checked.get(query);
  return at === undefined || now - at >= FRESH_FOR;
}

/** Records a check of `query` that passed, or that could not run. */
export function passed(query: object, now = clock.now()): void {
  checked.set(query, now);
}

const yes = (v: string) => v === "true" || v === "1";
const describe = (r: readonly string[]) => `${r[2]}${yes(r[3]!) ? " not null" : ""}${yes(r[4]!) ? " with a default" : ""}`;
const table = (name: string) => name.replace(/^"|"$/g, "").replace(/"\."/g, ".");

/** The catalog query's columns, as engine/core/src/fresh.rs names them. */
const COLUMNS = ["fresh_table", "fresh_column", "fresh_type", "fresh_not_null", "fresh_default"] as const;

/**
 * The differences between the facts the build saw and the rows the catalog query returns now, one
 * line each; none when the schema still holds what the query relied on, and `undefined` when the
 * rows are not the catalog query's: each is an object with its five columns, or an array of five
 * values where a driver hands rows back as arrays.
 */
export function differences(fresh: Fresh, rows: readonly unknown[]): string[] | undefined {
  const now = new Map<string, string[]>();
  for (const row of rows) {
    if (row === null || typeof row !== "object") return undefined;
    const raw: unknown[] | undefined = Array.isArray(row) ? (row.length === 5 ? row : undefined) : Object.keys(row).length === 5 && COLUMNS.every((k) => k in row) ? COLUMNS.map((k) => (row as Record<string, unknown>)[k]) : undefined;
    if (raw === undefined) return undefined;
    const values = raw.map((v) => (v === null || v === undefined ? "" : String(v)));
    now.set(`${values[0]}\u0000${values[1]}`, values);
  }
  const out: string[] = [];
  const seen = new Set<string>();
  for (const was of fresh.rows) {
    const key = `${was[0]}\u0000${was[1]}`;
    seen.add(key);
    const is = now.get(key);
    if (!is) out.push(`${table(was[0]!)}.${was[1]} was ${describe(was)} and no longer exists`);
    else if (is[2] !== was[2] || yes(is[3]!) !== yes(was[3]!) || yes(is[4]!) !== yes(was[4]!)) out.push(`${table(was[0]!)}.${was[1]} was ${describe(was)} and is now ${describe(is)}`);
  }
  // A column an insert must now fill, which the build did not see.
  for (const [key, is] of now) {
    if (!seen.has(key) && fresh.inserts.includes(is[0]!) && yes(is[3]!) && !yes(is[4]!)) out.push(`${table(is[0]!)}.${is[1]} is now ${describe(is)}, which the insert does not write`);
  }
  return out;
}

/** The error a call fails with when the schema changed under its query. */
export function staleError(source: string | undefined, lines: readonly string[]): Error {
  return new Error(`The database changed after ${source ? `${source} was built` : "this query was built"}: ${lines.join("; ")}. Build it again (the type check, or reylo generate) and deploy the new build.`);
}
