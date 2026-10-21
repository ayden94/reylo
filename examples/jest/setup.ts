// Creates the SQLite file DATABASE_URL names, from setup.sql: `nub run setup`.
import { readFileSync } from "node:fs";
import { openDatabase } from "./src/db.ts";

const db = openDatabase();
db.exec(readFileSync("setup.sql", "utf8"));
db.close();
