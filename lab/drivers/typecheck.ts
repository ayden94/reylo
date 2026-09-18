// Does every real driver satisfy the structural interfaces without lando importing them?
import { Pool, Client } from "pg";
import postgres from "postgres";
import { PGlite } from "@electric-sql/pglite";
import mysql from "mysql2/promise";
import type { Pool as MariaPool } from "mariadb";
import Database from "better-sqlite3";
import { DatabaseSync } from "node:sqlite";
import { Database as BunDatabase } from "bun:sqlite";
import { SQL as BunSQL } from "bun";
import { execute, type QueryModule } from "../../src/execute.ts";

declare const pgq: QueryModule<"postgres">;
declare const myq: QueryModule<"mysql">;
declare const sqq: QueryModule<"sqlite">;
declare const pool: Pool;
declare const client: Client;
declare const sql: ReturnType<typeof postgres>;
declare const pglite: PGlite;
declare const my: mysql.Pool;
declare const myConn: mysql.Connection;
declare const maria: MariaPool;
declare const bs3: Database.Database;
declare const nodeDb: DatabaseSync;
declare const bunDb: BunDatabase;
declare const bunSql: BunSQL;

execute(pool, pgq, { id: 1 });
execute(client, pgq, { id: 1 });
execute(sql, pgq, { id: 1 });
execute(pglite, pgq, { id: 1 });
execute(bunSql, pgq, { id: 1 });
execute(my, myq, { id: 1 });
execute(myConn, myq, { id: 1 });
execute(maria, myq, { id: 1 });
execute(bunSql, myq, { id: 1 });
execute(bs3, sqq, { id: 1 });
execute(nodeDb, sqq, { id: 1 });
execute(bunDb, sqq, { id: 1 });

// Expected errors: wrong dialect for the client.
// @ts-expect-error a sqlite database cannot run a postgres query
execute(bs3, pgq, { id: 1 });
// @ts-expect-error a postgres pool cannot run a sqlite query
execute(pool, sqq, { id: 1 });
// @ts-expect-error mysql2 cannot run a postgres query
execute(my, pgq, { id: 1 });
