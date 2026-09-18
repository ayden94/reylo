import { Database } from "bun:sqlite";
import query from "./query.sql";
const db = new Database(new URL("./lab.db", import.meta.url).pathname, { readonly: true });
console.log(JSON.stringify({ runtime: "bun:sqlite", ada: await query.execute(db, { id: 1, pattern: "a%" }), nobody: await query.execute(db, { id: 99, pattern: "%" }) }));
