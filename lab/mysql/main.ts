import query, * as q from "./query.sql";
import type { Row, Params } from "./query.sql";

declare const conn: { execute(text: string, values?: unknown): Promise<[unknown, unknown]> };
type ExpectedRow = {
  id: number;
  name: string;
  email: string | null;
  big: number;
  n: string;
  at: Date;
  j: unknown;
  flag: number;
  raw: Uint8Array | null;
  next_id: number;
};
const a: ExpectedRow = null! as Row;
const b: Row = null! as ExpectedRow;
const c: { id: number; skip: string } = null! as Params;
const d: Params = null! as { id: number; skip: string };
const e: Row | null = await query.execute(conn, { id: 1, skip: "" });
const f: readonly ["id", "skip"] = q.params;
// @ts-expect-error a postgres pool cannot run a mysql query
await query.execute({ query: async () => ({ rows: [] }) }, { id: 1, skip: "" });
