import q, { probe } from "./query.sql";
console.log(JSON.stringify({ sql: q.sql.trim(), probe }));
