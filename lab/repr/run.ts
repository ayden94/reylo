import postgres from "postgres";
import query from "./query.sql";
const sql = postgres("postgres://localhost:5432/postgres");
const ada = await query.run(sql, { id: 1 });
const kind = (v: unknown) => (typeof v === "object" && v !== null ? v.constructor.name : typeof v);
// JSON.stringify has no rule for a BigInt; Temporal values serialize through their toJSON.
console.log(JSON.stringify({
  kinds: ada && Object.fromEntries(Object.entries(ada).map(([k, v]) => [k, Array.isArray(v) ? `${kind(v[0])}[]` : kind(v)])),
  values: ada,
}, (_, v) => (typeof v === "bigint" ? `${v}n` : v)));
await sql.end();
