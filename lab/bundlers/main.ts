import postgres from "postgres";
import query from "./query.sql";
const sql = postgres("postgres://localhost:5432/postgres");
const ada = await query.run(sql, { id: 1, name: "ada" });
console.log(JSON.stringify({ bundler: process.env["PORG_BUNDLER"] ?? (typeof Bun === "undefined" ? "node" : "bun"), cardinality: query.cardinality, params: query.params, ada }));
await sql.end();
declare const Bun: unknown;
