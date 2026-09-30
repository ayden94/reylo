import { DatabaseSync } from "node:sqlite";
import query from "./query.sql";
import search from "./search.sql";
import byEmail from "./by_email.sql";
import page from "./page.sql";
import upsert from "./upsert.sql";
import positional from "./positional.sql";
const db = new DatabaseSync(new URL("./lab.db", import.meta.url).pathname);
console.log(JSON.stringify({ dialect: query.dialect, cardinality: query.cardinality, ada: await query.run(db, { id: 1, pattern: "a%" }), nobody: await query.run(db, { id: 99, pattern: "%" }), found: await search.run(db, { q: "a%", min: 1, n: 5, o: 0 }), byEmail: await byEmail.run(db, { email: "ada@example.com" }), page: await page.run(db, { g: "a*", o: 0, n: 1 }), upsert: await upsert.run(db, { email: "ada@example.com", name: "again" }), positional: await positional.run(db, { 1: 1, 2: "a%" }) }));
