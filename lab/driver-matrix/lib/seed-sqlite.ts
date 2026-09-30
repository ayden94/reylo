// Creates a fresh SQLite database file with the fixture tables: node lib/seed-sqlite.ts <file>
import { rmSync } from "node:fs";
import { DatabaseSync } from "node:sqlite";
import { seedSql } from "./harness.ts";

export function seedSqliteFile(file: string): void {
  for (const suffix of ["", "-wal", "-shm", "-journal"]) rmSync(file + suffix, { force: true });
  const db = new DatabaseSync(file);
  db.exec(seedSql("sqlite"));
  db.close();
}

if (import.meta.url === `file://${process.argv[1]}`) seedSqliteFile(process.argv[2]!);
