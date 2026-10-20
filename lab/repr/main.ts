import query from "./query.sql";
import type { Row, Params } from "./query.sql";

// Reylo's one representation per SQL type (src/values.ts), whatever the app's driver is set up to
// return: a timestamptz is the instant's ISO 8601 text in UTC, which Temporal.Instant.from reads, a
// timestamp and a date their text, an int8 (count(*) too) a BigInt, and an array of each follows.
// The `lib` is `es2024`: nothing in the module needs Temporal or any type a lib past it names.
type ExpectedRow = {
  id: number;
  created_at: string;
  big: bigint | null;
  day: string;
  local: string;
  stamps: string[];
  n: bigint;
};
const a: ExpectedRow = null! as Row;
const b: Row = null! as ExpectedRow;
const c: { id: number } = null! as Params;
const d: Params = null! as { id: number };
declare const pool: { query(text: string, values?: unknown[]): Promise<{ rows: unknown[] }> };
const e: ExpectedRow | null = await query.run(pool, { id: 1 });
// As JSON a BigInt is the integer it is (src/schema.ts).
const j: { id: number; created_at: string; big: number | null; day: string; local: string; stamps: string[]; n: number } | null = query["~reylo"].json;
