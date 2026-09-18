import * as q from "./query.sql";

// Force the checker to print what the mapper saw.
const printed: never = q.probe;
const sql: never = q.sql;
