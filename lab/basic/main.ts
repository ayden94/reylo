import query, * as q from "./query.sql";
import type { Row, Params } from "./query.sql";

declare const pool: { query(text: string, values?: unknown[]): Promise<{ rows: unknown[] }> };

// Exact-type checks: assignable both ways means the inferred type equals the expected one.
type ExpectedRow = {
  id: number;
  name: string;
  email: string | null;
  role: "admin" | "member";
  tags: string[];
  created_at: Date;
  big: string | null;
  next_id: number | null;
};
type ExpectedParams = { id: number; name: string };
const a: ExpectedRow = null! as Row;
const b: Row = null! as ExpectedRow;
const c: ExpectedParams = null! as Params;
const d: Params = null! as ExpectedParams;
const e: Row | null = await query.execute(pool, { id: 1, name: "ada" });
const f: readonly ["id", "name"] = q.params;
const g: ExpectedRow | null = query["~standard"].types!.output;
const h: { created_at: string; big: string | null } | null = query["~lando"].json;
// @ts-expect-error wrong parameter type
await query.execute(pool, { id: "1", name: "ada" });
// @ts-expect-error a sqlite client cannot run a postgres query
await query.execute({ prepare: (s: string) => ({ all: () => [] }) }, { id: 1, name: "ada" });
