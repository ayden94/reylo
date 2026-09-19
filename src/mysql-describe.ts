// MySQL describe through the project's mysql2: a server-side prepare reports
// result columns with type codes, NOT NULL flags and table provenance, and one
// entry per `?` parameter (weakly typed), without executing anything.
import { createRequire } from "node:module";
import { defaultRepr, scalarType, type Conversion, type Repr } from "./render.ts";

export type MysqlColumn = { name: string; type: number; flags: number; table: string; orgName: string; charset: number };
export type MysqlDescribed = { columns: MysqlColumn[]; params: { type: number; flags: number }[] };

export interface MysqlDescriber {
  describe(sql: string): Promise<MysqlDescribed>;
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
  const conn = await mysql.createConnection(url);
  return {
    async describe(sql) {
      const prepared = await conn.prepare(sql);
      const info = prepared.statement ?? prepared;
      try {
        return {
          columns: (info.columns as any[]).map((c) => ({ name: c.name, type: c.columnType, flags: c.flags, table: c.table, orgName: c.orgName, charset: c.characterSet })),
          params: (info.parameters as any[]).map((p) => ({ type: p.columnType, flags: p.flags })),
        };
      } finally {
        await prepared.close();
      }
    },
    close: () => conn.end(),
  };
}

const NOT_NULL = 1;
const BINARY_CHARSET = 63;

/** mysql2's decoding of each protocol type code, under the chosen representation of temporal and BIGINT values. */
export function mysqlType(c: { type: number; charset?: number }, repr: Repr = defaultRepr): string {
  const cls = mysqlConversion(c);
  if (cls) return scalarType(cls, repr);
  switch (c.type) {
    case 1: case 2: case 3: case 9: case 13: return "number"; // TINY SHORT LONG INT24 YEAR
    case 4: case 5: return "number"; // FLOAT DOUBLE
    case 0: case 246: return "string"; // DECIMAL NEWDECIMAL
    case 11: return "string"; // TIME
    case 245: return "unknown"; // JSON
    case 16: return "Uint8Array"; // BIT
    case 249: case 250: case 251: case 252: case 253: case 254: case 15:
      return c.charset === BINARY_CHARSET ? "Uint8Array" : "string"; // BLOBs, VAR_STRING, STRING, VARCHAR
    case 6: return "null";
    default: return "unknown";
  }
}

/**
 * The runtime conversion a column needs. MySQL's TIMESTAMP is converted to the session zone on
 * read and DATETIME has no zone, so both arrive as a wall clock; mysql2 decodes them as a local
 * Date, a DATE as local midnight, and a BIGINT as a lossy number unless `supportBigNumbers` is on.
 */
export function mysqlConversion(c: { type: number }): Conversion | undefined {
  switch (c.type) {
    case 7: case 12: return "timestamp"; // TIMESTAMP DATETIME
    case 10: case 14: return "date"; // DATE NEWDATE
    case 8: return "int8"; // LONGLONG
    default: return undefined;
  }
}

export function mysqlNotNull(flags: number): boolean {
  return (flags & NOT_NULL) !== 0;
}
