// Runs every read-only statement of the lab against the local database and prints the rows,
// so the inferred type can be compared with what a driver actually hands back.
// node --env-file=.env --import ../../src/register.ts run.ts
import postgres from "postgres";
import jsonAgg from "./json_agg.sql";
import arrays from "./arrays.sql";
import rows from "./rows.sql";
import aggPlain from "./agg_plain.sql";
import aggGrouped from "./agg_grouped.sql";
import ctePlain from "./cte_plain.sql";
import cteJoin from "./cte_join.sql";
import cteRecursive from "./cte_recursive.sql";
import fnScalar from "./fn_scalar.sql";
import fnSetof from "./fn_setof.sql";
import fnTable from "./fn_table.sql";
import fnSrf from "./fn_srf.sql";
import columns from "./columns.sql";
import joinLeft from "./join_left.sql";
import joinRight from "./join_right.sql";
import joinFull from "./join_full.sql";
import joinNotNull from "./join_notnull.sql";
import paramWhere from "./param_where.sql";
import paramAny from "./param_any.sql";
import paramLimit from "./param_limit.sql";
import paramExpr from "./param_expr.sql";
import paramPositional from "./param_positional.sql";
import views from "./views.sql";
import casts from "./casts.sql";
import types from "./types.sql";

// cte_dml.sql and param_insert.sql write rows, so this script leaves both to the type check.
const sql = postgres(process.env["DATABASE_URL"] ?? "postgres://localhost:5432/postgres");
const show = (name: string, cardinality: string, value: unknown) =>
  // A 64-bit integer is a BigInt, which JSON.stringify refuses: printed with its `n`.
  console.log(`${name} [${cardinality}] ${JSON.stringify(value, (_, v) => (typeof v === "bigint" ? `${v}n` : v))}`);

show("json_agg", jsonAgg.cardinality, await jsonAgg.run(sql, { id: 1 }));
show("arrays", arrays.cardinality, await arrays.run(sql, { id: 1 }));
show("arrays(empty)", arrays.cardinality, await arrays.run(sql, { id: 3 }));
show("rows", rows.cardinality, await rows.run(sql, { id: 1 }));
show("agg_plain", aggPlain.cardinality, await aggPlain.run(sql));
show("agg_grouped", aggGrouped.cardinality, await aggGrouped.run(sql));
show("cte_plain", ctePlain.cardinality, await ctePlain.run(sql));
show("cte_join", cteJoin.cardinality, await cteJoin.run(sql, { id: 1 }));
show("cte_recursive", cteRecursive.cardinality, await cteRecursive.run(sql));
show("fn_scalar", fnScalar.cardinality, await fnScalar.run(sql, { id: 1 }));
show("fn_scalar(null email)", fnScalar.cardinality, await fnScalar.run(sql, { id: 2 }));
show("fn_setof", fnSetof.cardinality, await fnSetof.run(sql, { uid: 1 }));
show("fn_table", fnTable.cardinality, await fnTable.run(sql));
show("fn_srf", fnSrf.cardinality, await fnSrf.run(sql, { id: 1 }));
show("columns", columns.cardinality, await columns.run(sql));
show("join_left", joinLeft.cardinality, await joinLeft.run(sql));
show("join_right", joinRight.cardinality, await joinRight.run(sql));
show("join_full", joinFull.cardinality, await joinFull.run(sql));
show("join_notnull", joinNotNull.cardinality, await joinNotNull.run(sql));
show("param_where", paramWhere.cardinality, await paramWhere.run(sql, { id: 1 }));
show("param_where(miss)", paramWhere.cardinality, await paramWhere.run(sql, { id: 999 }));
show("param_any", paramAny.cardinality, await paramAny.run(sql, { names: ["ada", "cyd"] }));
show("param_limit", paramLimit.cardinality, await paramLimit.run(sql, { n: 2 }));
show("param_expr", paramExpr.cardinality, await paramExpr.run(sql, { x: 7, y: "z" }));
show("param_positional", paramPositional.cardinality, await paramPositional.run(sql, { 1: 7, 2: "z" }));
show("views", views.cardinality, await views.run(sql));
show("casts", casts.cardinality, await casts.run(sql, { id: 1, since: "1970-01-01T00:00:00Z" }));
show("types", types.cardinality, await types.run(sql, { id: 1 }));
// The runtime shape of a record column: the text Postgres prints.
console.log("typeof rows.pair_anon:", typeof (await rows.run(sql, { id: 1 }))!.pair_anon);
console.log("typeof arrays.agg_rows:", typeof (await arrays.run(sql, { id: 1 }))!.agg_rows);
await sql.end();
