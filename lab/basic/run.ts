import query from "./query.sql";
console.log(JSON.stringify({ sql: query.sql.trim(), params: query.params, cardinality: query.cardinality, vendor: query["~standard"].vendor }));
