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
  next_id: number;
  total: number;
};
const a: ExpectedRow = null! as Row;
const b: Row = null! as ExpectedRow;
const c: { id: number; pattern: string } = null! as Params;
const d: Params = null! as { id: number; pattern: string };
const e: Row | null = await query.execute(db, { id: 1, pattern: "a%" });
const f: readonly ["id", "pattern"] = q.params;
// @ts-expect-error a postgres pool cannot run a sqlite query
await query.execute({ query: async () => ({ rows: [] }) }, { id: 1, pattern: "a%" });

// Inference without directives: parameter types from context, cardinality from the keys,
// nullability from the schema and the joins.
import insert from "./insert.sql";
import byEmail from "./by_email.sql";
import search from "./search.sql";
const i1: { id: number; name: string; created_at: string } = null! as Awaited<ReturnType<typeof insert.execute>>;
const i2: Awaited<ReturnType<typeof insert.execute>> = null! as { id: number; name: string; created_at: string };
const i3: { name: string; email: string } = null! as Parameters<typeof insert.execute>[1];
const b1: { id: number; name: string } | null = null! as Awaited<ReturnType<typeof byEmail.execute>>;
const b2: Awaited<ReturnType<typeof byEmail.execute>> = null! as { id: number; name: string } | null;
const s1: { id: number; name: string; title: string | null; min_score: number | null }[] = null! as Awaited<ReturnType<typeof search.execute>>;
const s2: Awaited<ReturnType<typeof search.execute>> = null! as { id: number; name: string; title: string | null; min_score: number | null }[];
const s3: { min: number; q: string; n: number; o: number } = null! as Parameters<typeof search.execute>[1];
const s4: Parameters<typeof search.execute>[1] = null! as { min: number; q: string; n: number; o: number };
const cards: ["one", "maybeOne", "many"] = [insert.cardinality, byEmail.cardinality, search.cardinality];
// SQLite-only syntax parses, because the engine prepares the statement with SQLite itself:
// `limit o, n` (offset first, as SQLite reads it), `glob`, `insert or ignore`, `on conflict do nothing`.
// A statement without RETURNING is cardinality `none` and `execute` resolves to void.
import page from "./page.sql";
import upsert from "./upsert.sql";
import ignore from "./ignore.sql";
const p1: { id: number; name: string }[] = null! as Awaited<ReturnType<typeof page.execute>>;
const p2: { g: string; n: number; o: number } = null! as Parameters<typeof page.execute>[1];
const p3: Parameters<typeof page.execute>[1] = null! as { g: string; n: number; o: number };
const u1: { id: number } | null = null! as Awaited<ReturnType<typeof upsert.execute>>;
const u2: Awaited<ReturnType<typeof upsert.execute>> = null! as { id: number } | null;
const more: ["many", "maybeOne", "none"] = [page.cardinality, upsert.cardinality, ignore.cardinality];
const v: void = await ignore.execute(db, { email: "x", name: "y" });
