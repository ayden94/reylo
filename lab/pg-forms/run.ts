// Runs every read-only statement of the lab against the local database and prints the rows,
// so the inferred type can be compared with what a driver actually hands back.
// node --import ../../src/register.ts run.ts
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
import views from "./views.sql";
import casts from "./casts.sql";
import types from "./types.sql";

// cte_dml.sql and param_insert.sql write rows, so this script leaves both to the type check.
const sql = postgres("postgres://localhost:5432/postgres");
const show = (name: string, cardinality: string, value: unknown) =>
  console.log(`${name} [${cardinality}] ${JSON.stringify(value)}`);

show("json_agg", jsonAgg.cardinality, await jsonAgg.execute(sql, { id: 1 }));
show("arrays", arrays.cardinality, await arrays.execute(sql, { id: 1 }));
show("arrays(empty)", arrays.cardinality, await arrays.execute(sql, { id: 3 }));
show("rows", rows.cardinality, await rows.execute(sql, { id: 1 }));
show("agg_plain", aggPlain.cardinality, await aggPlain.execute(sql, {}));
show("agg_grouped", aggGrouped.cardinality, await aggGrouped.execute(sql, {}));
show("cte_plain", ctePlain.cardinality, await ctePlain.execute(sql, {}));
show("cte_join", cteJoin.cardinality, await cteJoin.execute(sql, { id: 1 }));
show("cte_recursive", cteRecursive.cardinality, await cteRecursive.execute(sql, {}));
show("fn_scalar", fnScalar.cardinality, await fnScalar.execute(sql, { id: 1 }));
show("fn_scalar(null email)", fnScalar.cardinality, await fnScalar.execute(sql, { id: 2 }));
show("fn_setof", fnSetof.cardinality, await fnSetof.execute(sql, { uid: 1 }));
show("fn_table", fnTable.cardinality, await fnTable.execute(sql, {}));
show("fn_srf", fnSrf.cardinality, await fnSrf.execute(sql, { id: 1 }));
show("columns", columns.cardinality, await columns.execute(sql, {}));
show("join_left", joinLeft.cardinality, await joinLeft.execute(sql, {}));
show("join_right", joinRight.cardinality, await joinRight.execute(sql, {}));
show("join_full", joinFull.cardinality, await joinFull.execute(sql, {}));
show("join_notnull", joinNotNull.cardinality, await joinNotNull.execute(sql, {}));
show("param_where", paramWhere.cardinality, await paramWhere.execute(sql, { id: 1 }));
show("param_where(miss)", paramWhere.cardinality, await paramWhere.execute(sql, { id: 999 }));
show("param_any", paramAny.cardinality, await paramAny.execute(sql, { names: ["ada", "cyd"] }));
show("param_limit", paramLimit.cardinality, await paramLimit.execute(sql, { n: 2 }));
show("param_expr", paramExpr.cardinality, await paramExpr.execute(sql, { x: 7, y: "z" }));
show("views", views.cardinality, await views.execute(sql, {}));
show("casts", casts.cardinality, await casts.execute(sql, { id: 1, since: new Date(0) }));
show("types", types.cardinality, await types.execute(sql, { id: 1 }));
// The runtime shape of a record column: the text Postgres prints.
console.log("typeof rows.pair_anon:", typeof (await rows.execute(sql, { id: 1 }))!.pair_anon);
console.log("typeof arrays.agg_rows:", typeof (await arrays.execute(sql, { id: 1 }))!.agg_rows);
await sql.end();
