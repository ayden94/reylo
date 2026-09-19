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
  created_at: string;
  big: string | null;
  next_id: number;
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

// The engine's inference, without directives: cardinality from the primary key, an aggregate, a
// left join; nested JSON shapes from the constructors; expression nullability from the rules.
import nested from "./nested.sql";
import count from "./count.sql";
import joined from "./joined.sql";
type ExpectedNested = {
  id: number;
  name: string;
  posts: { id: number; title: string; body: string | null; at: string | null }[];
  row: { id: number; name: string; email: string | null; role: "admin" | "member"; tags: string[]; created_at: string; big: number | null };
  post_count: string;
};
const n1: ExpectedNested | null = null! as Awaited<ReturnType<typeof nested.execute>>;
const n2: Awaited<ReturnType<typeof nested.execute>> = null! as ExpectedNested | null;
const c1: { n: string; latest: string | null } = null! as Awaited<ReturnType<typeof count.execute>>;
const c2: Awaited<ReturnType<typeof count.execute>> = null! as { n: string; latest: string | null };
const j1: { id: number; title: string; author: string | null; email: string | null }[] = null! as Awaited<ReturnType<typeof joined.execute>>;
const j2: Awaited<ReturnType<typeof joined.execute>> = null! as { id: number; title: string; author: string | null; email: string | null }[];
const cardinalities: ["maybeOne", "maybeOne", "one", "many"] = [query.cardinality, nested.cardinality, count.cardinality, joined.cardinality];
// A GROUP BY on the pinned relation's columns is one group at most, whatever is joined; an
// ARRAY constructor is never null.
import grouped from "./grouped.sql";
const g1: { id: number; name: string; posts: string; ids: number[] } | null = null! as Awaited<ReturnType<typeof grouped.execute>>;
const g2: Awaited<ReturnType<typeof grouped.execute>> = null! as { id: number; name: string; posts: string; ids: number[] } | null;
const g3: "maybeOne" = grouped.cardinality;
