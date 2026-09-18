// SQLite describe through node:sqlite (built into Node ≥ 22.5, so no peer
// dependency). SQLite reports, per result column, the declared type of the table column it
// comes from and nothing for an expression or a parameter; the engine reads the rest off the
// statement and the `pragma` tables it asks for through `values`.
import { fileURLToPath } from "node:url";
import { isAbsolute, resolve } from "node:path";

export type SqliteColumn = { name: string; declared: string | null; table: string | null; column: string | null };
export type SqliteDescribed = { columns: SqliteColumn[] };

export interface SqliteDescriber {
  describe(sql: string): Promise<SqliteDescribed>;
  /** Rows as arrays of values, in column order: the engine's `pragma` queries. */
  values(sql: string, params?: unknown[]): Promise<unknown[][]>;
  close(): Promise<void>;
}

/** `file:./dev.db`, `file:///abs/dev.db`, `sqlite:./dev.db`, `:memory:` → a path, relative ones resolved against baseDir. */
export function sqlitePath(url: string, baseDir: string): string {
  if (url === ":memory:" || url === "file::memory:") return ":memory:";
  const raw = url.startsWith("file://") ? fileURLToPath(url) : url.replace(/^(file|sqlite):(\/\/)?/, "");
  return isAbsolute(raw) ? raw : resolve(baseDir, raw);
}

export async function openSqliteDescriber(url: string, baseDir: string): Promise<SqliteDescriber> {
  const { DatabaseSync } = (await import("node:sqlite")) as typeof import("node:sqlite");
  const path = sqlitePath(url, baseDir);
  const db = new DatabaseSync(path, { readOnly: path !== ":memory:" });
  return {
    async describe(sql) {
      const stmt = db.prepare(sql);
      return { columns: stmt.columns().map((c) => ({ name: c.name, declared: c.type, table: c.table, column: c.column })) };
    },
    async values(sql, params = []) {
      const stmt = db.prepare(sql);
      const names = stmt.columns().map((c) => c.name);
      return (stmt.all(...(params as never[])) as Record<string, unknown>[]).map((row) => names.map((n) => row[n]));
    },
    async close() {
      db.close();
    },
  };
}

