import { DatabaseSync } from "node:sqlite";
import query from "./query.sql";
const db = new DatabaseSync(new URL("./lab.db", import.meta.url).pathname);
console.log(JSON.stringify({ dialect: query.dialect, cardinality: query.cardinality, ada: await query.execute(db, { id: 1 }), nobody: await query.execute(db, { id: 99 }) }));
