import postgres from "postgres";
import query from "./query.sql";
import nested from "./nested.sql";
import count from "./count.sql";
import grouped from "./grouped.sql";
const sql = postgres(process.env["DATABASE_URL"] ?? "postgres://localhost:5432/postgres");
const ada = await query.run(sql, { id: 1, name: "ada" });
const nobody = await query.run(sql, { id: 999, name: "x" });
const curried = await query.with(sql)({ id: 1, name: "ada" });
const adaNested = await nested.run(sql, { id: 1 });
const counted = await count.run(sql, { role: "member" });
const group = await grouped.run(sql, { id: 1 });
// A 64-bit integer is a BigInt, which JSON.stringify refuses: printed with its `n`.
const bigints = (_: string, v: unknown) => (typeof v === "bigint" ? `${v}n` : v);
console.log(JSON.stringify({ params: query.params, cardinality: query.cardinality, ada, nobody, curried, keys: Object.keys(query["~standard"]), adaNested, counted, group }, bigints));
await sql.end();
