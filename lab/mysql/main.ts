import query, * as q from "./query.sql";
import type { Row, Params } from "./query.sql";
import count, { type Row as CountRow, type Params as CountParams } from "./count.sql";
import insert, { type Params as InsertParams } from "./insert.sql";
import nested, { type Row as NestedRow } from "./nested.sql";

declare const conn: { execute(text: string, values?: unknown): Promise<[unknown, unknown]> };
type ExpectedRow = {
  id: number;
  name: string;
  email: string | null;
  big: string;
  n: string;
  at: string;
  j: unknown;
  flag: number;
  raw: Uint8Array | null;
  next_id: string;
};
const a: ExpectedRow = null! as Row;
const b: Row = null! as ExpectedRow;
const c: { id: number; skip: string } = null! as Params;
const d: Params = null! as { id: number; skip: string };
// The primary key is equated, so the engine proves at most one row: no `-- @returns` needed.
const e: Row | null = await query.execute(conn, { id: 1, skip: "" });
const f: readonly ["id", "skip"] = q.params;
// @ts-expect-error a postgres pool cannot run a mysql query
await query.execute({ query: async () => ({ rows: [] }) }, { id: 1, skip: "" });
// An ungrouped aggregate is exactly one row; `count(*)` is a bigint (a string by default), `max`
// of a NOT NULL column is nullable because the server says so (no rows, no maximum).
const g: { n: string; latest: string | null } = null! as CountRow;
const h: CountRow = null! as { n: string; latest: string | null };
const i: { pattern: string } = null! as CountParams;
const j: { n: string; latest: string | null } = await count.execute(conn, { pattern: "a%" });
// MySQL has no RETURNING, so a data-modifying statement returns nothing; the `?` in a VALUES
// list is typed by the column it fills.
const k: { name: string; email: string } = null! as InsertParams;
const l: void = await insert.execute(conn, { name: "eve", email: "eve@example.com" });
// JSON constructors are typed from their arguments: the engine prepares `select p.id, p.title, p.tag
// from posts p` and `select u.email, u.flag from users u` and reads the server's types and NOT NULL
// flags back into the shape. The `coalesce(json_arrayagg(...), json_array())` under an ungrouped
// aggregate subquery is never null; `json_object` never is; `tag` is nullable in the table.
// The server flags `u.id` nullable next to a correlated aggregate subquery; the engine prepares
// the statement without the subqueries and takes NOT NULL from there. `count(*)` in an ungrouped
// subquery is one row and never null; `max` is null over no rows.
type ExpectedNested = { id: number; posts: { id: number; title: string; tag: string | null }[]; meta: { email: string | null; flag: number }; n_posts: string; last_post: number | null };
const m: ExpectedNested = null! as NestedRow;
const n: NestedRow = null! as ExpectedNested;
const o: ExpectedNested | null = await nested.execute(conn, { id: 1 });
