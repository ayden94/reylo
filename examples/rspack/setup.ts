// Creates the tables of setup.sql, with their rows, in the database DATABASE_URL names.
import { readFile } from "node:fs/promises";
import pg from "pg";

const url = process.env["DATABASE_URL"];
if (!url) throw new Error("Set DATABASE_URL, in .env or in the environment");

const client = new pg.Client({ connectionString: url });
await client.connect();
await client.query(await readFile("setup.sql", "utf8"));
await client.end();
console.log("created users and posts");
