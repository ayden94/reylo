// MySQL describe through the project's mysql2: a server-side prepare reports
// result columns with type codes, NOT NULL flags, charsets, display lengths and
// the table each is a column of, and the server's type guess per `?` parameter,
// without executing anything. The engine reads
// those as they are (docs/mysql-sources.md); this side only carries them, and
// runs the engine's catalog queries.
import { createRequire } from "node:module";
import { Unreachable, type MysqlDescribed } from "./engine.ts";

export type { MysqlDescribed } from "./engine.ts";

export interface MysqlDescriber {
  describe(sql: string): Promise<MysqlDescribed>;
  /** Rows as arrays of values, in column order. */
  values(sql: string, params: unknown[]): Promise<unknown[][]>;
  close(): Promise<void>;
}

export async function openMysqlDescriber(projectDir: string, url: string): Promise<MysqlDescriber> {
  const require = createRequire(projectDir.replace(/\/?$/, "/package.json"));
  let path: string;
  try {
    path = require.resolve("mysql2/promise");
  } catch {
    throw new Error(`No MySQL client found in ${projectDir}; install "mysql2".`);
  }
  const mysql = (await import(path)).default as { createConnection(url: string): Promise<any> };
  // The server's answer to a statement carries a SQLSTATE; mysql2 marks `fatal` the errors that
  // end the connection (a refused socket, a handshake the server rejects, a lost connection).
  const rethrow = (e: unknown): never => {
    const err = e as { sqlState?: string; fatal?: boolean };
    throw err?.sqlState && !err.fatal ? e : new Unreachable(e);
  };
  // mysql2's default connectTimeout is already 10 s, the Postgres describer's limit.
  const conn = await mysql.createConnection(url).catch(rethrow);
  // A connection that drops while idle is reported as an event; unheard, it ends the process.
  conn.on("error", () => {});
  return {
    async describe(sql) {
      const prepared = await conn.prepare(sql).catch(rethrow);
      const info = prepared.statement ?? prepared;
      try {
        return {
          kind: "mysql",
          columns: (info.columns as any[]).map((c) => ({ name: c.name, type: c.columnType, flags: c.flags, charset: c.characterSet ?? 0, length: c.columnLength ?? 0, org_table: c.orgTable ?? "", org_name: c.orgName ?? "" })),
          params: (info.parameters as any[]).map((p) => ({ type: p.columnType, flags: p.flags ?? 0, charset: p.characterSet ?? 0 })),
        };
      } finally {
        await prepared.close();
      }
    },
    async values(sql, params) {
      const [rows] = await conn.query({ sql, values: params, rowsAsArray: true }).catch(rethrow);
      return rows as unknown[][];
    },
    close: () => conn.end(),
  };
}
