// Creates the tables of setup.sql, with their rows, in the database DATABASE_URL names.
import { readFileSync } from "node:fs";
import pg from "pg";

const url = process.env["DATABASE_URL"];
if (!url) throw new Error("Set DATABASE_URL, in .env or in the environment");

const client = new pg.Client({ connectionString: url });
await client.connect();
// A query without parameters goes over the simple protocol, which takes several statements.
await client.query(readFileSync("setup.sql", "utf8"));
await client.end();
console.log("created users and posts");
