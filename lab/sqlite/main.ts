import query, * as q from "./query.sql";
import type { Row, Params } from "./query.sql";

declare const db: { prepare(sql: string): { all(...p: unknown[]): unknown[] } };
type ExpectedRow = {
  id: number;
  name: string;
  email: string | null;
  score: number | null;
  avatar: Uint8Array | null;
  created_at: string;
  next_id: unknown;
};
const a: ExpectedRow = null! as Row;
const b: Row = null! as ExpectedRow;
const c: { id: number } = null! as Params;
const d: Params = null! as { id: number };
const e: Row | null = await query.execute(db, { id: 1 });
const f: readonly ["id"] = q.params;
// @ts-expect-error a postgres pool cannot run a sqlite query
await query.execute({ query: async () => ({ rows: [] }) }, { id: 1 });
