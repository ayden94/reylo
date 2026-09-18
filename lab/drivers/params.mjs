import postgres from "postgres";
import { DatabaseSync } from "node:sqlite";
import Database from "better-sqlite3";
import { createRequire } from "node:module";
const sql = postgres("postgres://localhost:5432/postgres");
for (const q of ["select :id::int as a", "select $1::int as a, :name::text as b", "select $1::int as a, $1::int as b", "select @x as a"]) {
  const r = await sql.unsafe(q).describe().then(d => `ok params=${JSON.stringify(d.types)}`, e => `ERR ${e.message}`);
  console.log("PG  ", JSON.stringify(q), "→", r);
}
await sql.end();
const nd = new DatabaseSync(":memory:");
for (const [q, args] of [
  ["select :a as a, ? as b, $c as c, @d as d, ?5 as e", [{ a: 1, c: 3, d: 4 }, 2, 5]],
  ["select :a as a, :a as a2, ? as b", [{ a: 1 }, 2]],
  ["select ? as a, ? as b", [1, 2]],
]) {
  try { const st = nd.prepare(q); console.log("SQLITE(node)", JSON.stringify(q), "→", JSON.stringify(st.all(...args)), "expanded:", st.expandedSQL); }
  catch (e) { console.log("SQLITE(node)", JSON.stringify(q), "→ ERR", e.message); }
}
const bs = new Database(":memory:");
try { console.log("SQLITE(bs3) mixed →", JSON.stringify(bs.prepare("select :a as a, ? as b, $c as c").all({ a: 1, c: 3 }, 2))); } catch (e) { console.log("SQLITE(bs3) mixed → ERR", e.message); }
try { console.log("SQLITE(bs3) named+named →", JSON.stringify(bs.prepare("select :a as a, @b as b").all({ a: 1, b: 2 }))); } catch (e) { console.log("SQLITE(bs3) → ERR", e.message); }
// mysql2 client-side named placeholders (no server needed)
const require = createRequire(import.meta.url);
const compile = require("named-placeholders")();
for (const q of ["select :id as a, :name as b", "select :id as a, ? as b", "select ? as a, ? as b", "select :id as a, :id as b"]) {
  try { const [text, vals] = compile(q, { id: 1, name: "x" }); console.log("MYSQL(named-placeholders)", JSON.stringify(q), "→", JSON.stringify(text), JSON.stringify(vals)); }
  catch (e) { console.log("MYSQL(named-placeholders)", JSON.stringify(q), "→ ERR", e.message); }
}
