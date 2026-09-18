// SQLite describe through node:sqlite (built into Node ≥ 22.5, so no peer
// dependency). SQLite types columns only when they come straight from a table
// (declared type + NOT NULL from table_info); expressions and parameters are
// untyped by the engine and need annotations.
import { fileURLToPath } from "node:url";
import { isAbsolute, resolve } from "node:path";

export type SqliteColumn = { name: string; declared: string | null; table: string | null; column: string | null };
export type SqliteDescribed = { columns: SqliteColumn[]; notNull: Set<string> };

export interface SqliteDescriber {
  describe(sql: string): Promise<SqliteDescribed>;
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
  const tableInfo = new Map<string, Set<string>>();
  const notNullColumns = (table: string) => {
    let set = tableInfo.get(table);
    if (!set) {
      set = new Set(
        (db.prepare(`select name from pragma_table_info(?) where "notnull" = 1 or pk = 1`).all(table) as { name: string }[]).map((r) => r.name),
      );
      tableInfo.set(table, set);
    }
    return set;
  };
  return {
    async describe(sql) {
      const stmt = db.prepare(sql);
      const columns = stmt.columns().map((c) => ({ name: c.name, declared: c.type, table: c.table, column: c.column }));
      const notNull = new Set<string>();
      for (const c of columns) if (c.table && c.column && notNullColumns(c.table).has(c.column)) notNull.add(c.name);
      return { columns, notNull };
    },
    async close() {
      db.close();
    },
  };
}

/** Declared-type affinity → TypeScript, following SQLite's affinity rules. */
export function sqliteType(declared: string | null): string {
  if (!declared) return "unknown";
  const d = declared.toUpperCase();
  if (d.includes("INT")) return "number";
  if (d.includes("CHAR") || d.includes("CLOB") || d.includes("TEXT")) return "string";
  if (d.includes("BLOB")) return "Uint8Array";
  if (d.includes("REAL") || d.includes("FLOA") || d.includes("DOUB")) return "number";
  if (d.includes("BOOL")) return "number";
  return "number | string";
}
