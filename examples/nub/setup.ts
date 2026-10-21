// Creates app.db, with the tables of setup.sql and their rows.
import { readFileSync, rmSync } from "node:fs";
import { DatabaseSync } from "node:sqlite";

rmSync("app.db", { force: true });
const db = new DatabaseSync("app.db");
db.exec(readFileSync("setup.sql", "utf8"));
db.close();
console.log("created app.db");
