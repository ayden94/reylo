import pg from "pg";

// Without it, pg would connect to its own defaults (localhost:5432) instead of failing.
const url = process.env["DATABASE_URL"];
if (!url) throw new Error("Set DATABASE_URL, in .env or in the environment");

// One pool for the server. `.server/` keeps it, and every query beside it, out of the browser bundle.
export const db = new pg.Pool({ connectionString: url });
