// Creates the tables of setup.sql, with their rows, in the database DATABASE_URL names.
import postgres from "postgres";

const url = process.env["DATABASE_URL"];
if (!url) throw new Error("Set DATABASE_URL, in .env or in the environment");

const sql = postgres(url, { onnotice: () => {} });
await sql.file("setup.sql");
await sql.end();
console.log("created users and posts");
