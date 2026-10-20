import Database from "better-sqlite3";

/** Opens the SQLite file DATABASE_URL names (`file:./app.db`), the database Reylo analyzes the queries against. */
export function openDatabase(url = process.env["DATABASE_URL"]): Database.Database {
  if (!url) throw new Error("DATABASE_URL is not set: copy .env.example to .env");
  return new Database(url.replace(/^(file|sqlite):/, ""));
}
