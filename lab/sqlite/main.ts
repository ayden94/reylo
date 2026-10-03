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
const e: Row | null = await query.run(db, { id: 1, pattern: "a%" });
const f: readonly ["id", "pattern"] = q.params;
// @ts-expect-error a postgres pool cannot run a sqlite query
await query.run({ query: async () => ({ rows: [] }) }, { id: 1, pattern: "a%" });

// Inference without directives: parameter types from context, cardinality from the keys,
// nullability from the schema and the joins.
import insert from "./insert.sql";
import byEmail from "./by_email.sql";
import search from "./search.sql";
const i1: { id: number; name: string; created_at: string } = null! as Awaited<ReturnType<typeof insert.run>>;
const i2: Awaited<ReturnType<typeof insert.run>> = null! as { id: number; name: string; created_at: string };
// :email is written whole to a nullable column, so the caller may pass null; :name fills a NOT NULL one.
const i3: { name: string; email: string | null } = null! as Exclude<Parameters<typeof insert.run>[1], readonly unknown[]>;
const i4: Exclude<Parameters<typeof insert.run>[1], readonly unknown[]> = null! as { name: string; email: string | null };
const b1: { id: number; name: string } | null = null! as Awaited<ReturnType<typeof byEmail.run>>;
const b2: Awaited<ReturnType<typeof byEmail.run>> = null! as { id: number; name: string } | null;
const s1: { id: number; name: string; title: string | null; min_score: number | null }[] = null! as Awaited<ReturnType<typeof search.run>>;
const s2: Awaited<ReturnType<typeof search.run>> = null! as { id: number; name: string; title: string | null; min_score: number | null }[];
const s3: { min: number; q: string; n: number; o: number } = null! as Exclude<Parameters<typeof search.run>[1], readonly unknown[]>;
const s4: Exclude<Parameters<typeof search.run>[1], readonly unknown[]> = null! as { min: number; q: string; n: number; o: number };
// The array form names each parameter once, in order of first use, though :min appears twice.
const s5: readonly [min: number, q: string, n: number, o: number] = null! as Extract<Parameters<typeof search.run>[1], readonly unknown[]>;
const s6: Extract<Parameters<typeof search.run>[1], readonly unknown[]> = null! as readonly [min: number, q: string, n: number, o: number];
const cards: ["one", "maybeOne", "many"] = [insert.cardinality, byEmail.cardinality, search.cardinality];
// SQLite-only syntax parses, because the engine prepares the statement with SQLite itself:
// `limit o, n` (offset first, as SQLite reads it), `glob`, `insert or ignore`, `on conflict do nothing`.
// A statement without RETURNING is cardinality `none` and `run` resolves to void.
import page from "./page.sql";
import upsert from "./upsert.sql";
import ignore from "./ignore.sql";
const p1: { id: number; name: string }[] = null! as Awaited<ReturnType<typeof page.run>>;
const p2: { g: string; n: number; o: number } = null! as Exclude<Parameters<typeof page.run>[1], readonly unknown[]>;
const p3: Exclude<Parameters<typeof page.run>[1], readonly unknown[]> = null! as { g: string; n: number; o: number };
const u1: { id: number } | null = null! as Awaited<ReturnType<typeof upsert.run>>;
const u2: Awaited<ReturnType<typeof upsert.run>> = null! as { id: number } | null;
const more: ["many", "maybeOne", "none"] = [page.cardinality, upsert.cardinality, ignore.cardinality];
const v: void = await ignore.run(db, { email: "x", name: "y" });
// A positional parameter's key is its number; a bare `?` is one past the largest number before it.
import positional from "./positional.sql";
const pos1: { name: string } | null = null! as Awaited<ReturnType<typeof positional.run>>;
const pos2: Awaited<ReturnType<typeof positional.run>> = null! as { name: string } | null;
const pos3: { 1: number; 2: string } = null! as Exclude<Parameters<typeof positional.run>[1], readonly unknown[]>;
const pos4: Exclude<Parameters<typeof positional.run>[1], readonly unknown[]> = null! as { 1: number; 2: string };
