import query from "./query.sql";
import type { Row, Params } from "./query.sql";

// The mapper options `dates: "temporal"` and `bigint: "bigint"`: temporal columns are the
// Temporal type that matches the Postgres type, int8 columns are bigint, and an array of
// either follows. The types are TypeScript's own `esnext.temporal` lib, which the module references,
// so this lab's `lib` is `es2024` to prove the reference brings them in.
type ExpectedRow = {
  id: number;
  created_at: Temporal.Instant;
  big: bigint | null;
  day: Temporal.PlainDate;
  local: Temporal.PlainDateTime;
  stamps: Temporal.Instant[];
  n: bigint;
};
const a: ExpectedRow = null! as Row;
const b: Row = null! as ExpectedRow;
const c: { id: number } = null! as Params;
const d: Params = null! as { id: number };
declare const pool: { query(text: string, values?: unknown[]): Promise<{ rows: unknown[] }> };
const e: ExpectedRow | null = await query.run(pool, { id: 1 });
// JSON.stringify turns every Temporal value and bigint into text.
const j: { id: number; created_at: string; big: string | null; day: string; local: string; stamps: string[]; n: string } | null = query["~motherload"].json;
