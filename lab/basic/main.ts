import query, * as q from "./query.sql";
import type { Row, Params } from "./query.sql";

declare const pool: { query(text: string, values?: unknown[]): Promise<{ rows: unknown[] }> };
declare const sqlite: { prepare(sql: string): { all(...p: unknown[]): unknown[] } };

const rows = await query.execute(pool, { id: 1 });
const printed: never = q.probe;
const row: never = null! as Row;
const params: never = null! as Params;
const result: never = rows;
const json: never = query["~lando"].json;
const std: never = query["~standard"].types!.output;
// @ts-expect-error a sqlite client cannot run a postgres query
await query.execute(sqlite, { id: 1 });
// @ts-expect-error wrong parameter shape
await query.execute(pool, { id: "1" });
