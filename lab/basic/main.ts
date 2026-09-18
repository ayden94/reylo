import query, * as q from "./query.sql";
import type { Row, Params } from "./query.sql";

declare const pool: { query(text: string, values?: unknown[]): Promise<{ rows: unknown[] }> };
type Expand<T> = { [K in keyof T]: T[K] };
const row: never = null! as Expand<Row>;
const params: never = null! as Expand<Params>;
const result: never = await query.execute(pool, { id: 1, name: "ada" });
const names: never = q.params;
// @ts-expect-error wrong parameter type
await query.execute(pool, { id: "1", name: "ada" });
