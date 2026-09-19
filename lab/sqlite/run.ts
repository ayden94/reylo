import { DatabaseSync } from "node:sqlite";
import query from "./query.sql";
import search from "./search.sql";
import byEmail from "./by_email.sql";
import page from "./page.sql";
import upsert from "./upsert.sql";
const db = new DatabaseSync(new URL("./lab.db", import.meta.url).pathname);
console.log(JSON.stringify({ dialect: query.dialect, cardinality: query.cardinality, ada: await query.execute(db, { id: 1, pattern: "a%" }), nobody: await query.execute(db, { id: 99, pattern: "%" }), found: await search.execute(db, { q: "a%", min: 1, n: 5, o: 0 }), byEmail: await byEmail.execute(db, { email: "ada@example.com" }), page: await page.execute(db, { g: "a*", o: 0, n: 1 }), upsert: await upsert.execute(db, { email: "ada@example.com", name: "again" }) }));
