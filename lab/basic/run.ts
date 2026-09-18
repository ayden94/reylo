import postgres from "postgres";
import query from "./query.sql";
const sql = postgres("postgres://localhost:5432/postgres");
const ada = await query.execute(sql, { id: 1, name: "ada" });
const nobody = await query.execute(sql, { id: 999, name: "x" });
console.log(JSON.stringify({ params: query.params, cardinality: query.cardinality, ada, nobody, keys: Object.keys(query["~standard"]) }));
await sql.end();
