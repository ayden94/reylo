// MySQL describe: a server-side prepare reports result columns with type codes, NOT NULL flags
// and charsets, and the server's type guess per `?` parameter, without executing anything. The
// engine reads those as they are (docs/mysql-sources.md); this side only carries them, and runs
// the engine's catalog queries. The project's own mysql2 comes first, resolved from the PROJECT
// directory; a project without it, such as one on PlanetScale's serverless driver, mariadb or
// Bun.sql, is described through porg's own connection (mysql-wire.ts).
import { createRequire } from "node:module";
import { Unreachable, type MysqlDescribed } from "./engine.ts";
import { connectMysql } from "./mysql-wire.ts";

export type { MysqlDescribed } from "./engine.ts";

export interface MysqlDescriber {
  describe(sql: string): Promise<MysqlDescribed>;
  /** Rows as arrays of values, in column order. */
  values(sql: string, params: unknown[]): Promise<unknown[][]>;
  close(): Promise<void>;
}

/** How long a connection attempt may take, as for Postgres; mysql2's default connectTimeout is the same. */
const CONNECT_TIMEOUT_MS = 10_000;

// The server's answer to a statement carries a SQLSTATE; both connections mark `fatal` the errors
// that end the connection (a refused socket, a handshake the server rejects, a lost connection).
// A statement's error is reported with the server's error number, as MySQL's own client prints it.
const rethrow = (e: unknown): never => {
  const err = e as { sqlState?: string; fatal?: boolean; errno?: number; message?: string };
  if (!err?.sqlState || err.fatal) throw new Unreachable(e);
  throw Object.assign(new Error(err.message), { code: String(err.errno) });
};

export async function openMysqlDescriber(projectDir: string, url: string): Promise<MysqlDescriber> {
  const require = createRequire(projectDir.replace(/\/?$/, "/package.json"));
  let path: string | undefined;
  try {
    path = require.resolve("mysql2/promise");
  } catch {}

  if (!path) {
    const conn = await connectMysql(url, CONNECT_TIMEOUT_MS).catch(rethrow);
    return {
      async describe(sql) {
        const { columns, params } = await conn.prepare(sql).catch(rethrow);
        return { kind: "mysql", columns, params: params.map(({ type, flags, charset }) => ({ type, flags, charset })) };
      },
      values: (sql, params) => conn.query(sql, params).catch(rethrow),
      close: () => conn.close(),
    };
  }

  const mysql = (await import(path)).default as { createConnection(url: string): Promise<any> };
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
          columns: (info.columns as any[]).map((c) => ({ name: c.name, type: c.columnType, flags: c.flags, charset: c.characterSet ?? 0 })),
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
