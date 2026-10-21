// Does every real driver satisfy the structural interfaces without reylo importing them?
// Every client below is also a transaction, savepoint, reserved or checked-out connection
// where the driver has one, because those are what a caller passes inside a transaction.
import type { Pool, Client, PoolClient } from "pg";
import type postgres from "postgres";
import type { PGlite, Transaction as PGliteTransaction } from "@electric-sql/pglite";
import type { Pool as NeonPool, PoolClient as NeonPoolClient, NeonQueryFunction } from "@neondatabase/serverless";
import type { IDatabase, ITask } from "pg-promise";
import type * as mysql from "mysql2/promise";
import type * as mysqlCallback from "mysql2";
import type { Pool as MariaPool, PoolConnection as MariaPoolConnection, Connection as MariaConnection } from "mariadb";
import type * as mariaCallback from "mariadb/callback";
import type { Connection as PlanetScaleConnection, Transaction as PlanetScaleTransaction } from "@planetscale/database";
import type { Connection as PlanetScale2Connection } from "planetscale-2";
import type Database from "better-sqlite3";
import type { DatabaseSync } from "node:sqlite";
import type { Database as BunDatabase } from "bun:sqlite";
import type { SQL as BunSQL, TransactionSQL as BunTransaction, SavepointSQL as BunSavepoint, ReservedSQL as BunReserved } from "bun";
import type { Client as LibsqlClient, Transaction as LibsqlTransaction } from "@libsql/client";
import type { Database as TursoDatabase, Transaction as TursoTransaction } from "@tursodatabase/database";
import type { Database as TursoCompat } from "@tursodatabase/database/compat";
import type { Database as Sqlite3Database } from "sqlite3";
import type { Database as SqliteDatabase } from "sqlite";
import type { D1Database, D1DatabaseSession } from "@cloudflare/workers-types";
import { execute, type QueryModule } from "../../src/execute.ts";
import type { Query } from "../../src/index.ts";

declare const pgq: QueryModule<"postgres">;
declare const myq: QueryModule<"mysql">;
declare const sqq: QueryModule<"sqlite">;

// Postgres
declare const pool: Pool, client: Client, poolClient: PoolClient;
declare const sql: postgres.Sql, sqlTx: postgres.TransactionSql, sqlReserved: postgres.ReservedSql;
declare const pglite: PGlite, pgliteTx: PGliteTransaction;
declare const neonPool: NeonPool, neonClient: NeonPoolClient, neonHttp: NeonQueryFunction<false, false>, neonHttpFull: NeonQueryFunction<false, true>;
declare const pgp: IDatabase<{}>, pgpTask: ITask<{}>;
declare const bunSql: BunSQL, bunTx: BunTransaction, bunSavepoint: BunSavepoint, bunReserved: BunReserved;
for (const c of [pool, client, poolClient, sql, sqlTx, sqlReserved, pglite, pgliteTx, neonPool, neonClient, neonHttp, neonHttpFull, pgp, pgpTask, bunSql, bunTx, bunSavepoint, bunReserved]) execute(c, pgq, { id: 1 });

// MySQL
declare const my: mysql.Pool, myConn: mysql.Connection, myPoolConn: mysql.PoolConnection;
declare const myCallback: mysqlCallback.Pool, myCallbackConn: mysqlCallback.PoolConnection;
declare const maria: MariaPool, mariaConn: MariaConnection, mariaPoolConn: MariaPoolConnection;
declare const mariaCb: mariaCallback.Pool, mariaCbConn: mariaCallback.Connection, mariaCbPoolConn: mariaCallback.PoolConnection;
declare const planetscale: PlanetScaleConnection, planetscaleTx: PlanetScaleTransaction;
for (const c of [my, myConn, myPoolConn, myCallback, myCallbackConn, maria, mariaConn, mariaPoolConn, mariaCb, mariaCbConn, mariaCbPoolConn, planetscale, planetscaleTx, bunSql, bunTx]) execute(c, myq, { id: 1 });

// SQLite
declare const bs3: Database.Database, nodeDb: DatabaseSync, bunDb: BunDatabase;
declare const libsql: LibsqlClient, libsqlTx: LibsqlTransaction;
declare const sqlite3: Sqlite3Database, sqlite: SqliteDatabase;
declare const d1: D1Database, d1Session: D1DatabaseSession;
declare const turso: TursoDatabase, tursoTx: TursoTransaction, tursoCompat: TursoCompat;
for (const c of [bs3, nodeDb, bunDb, libsql, libsqlTx, turso, tursoTx, tursoCompat, sqlite3, sqlite, d1, d1Session, bunSql, bunTx]) execute(c, sqq, { id: 1 });

// `run` is a promise on every client; `runSync` takes only a synchronous SQLite client and hands back the row.
declare const pgQuery: Query<{ id: number }, { id: number }, "one", "postgres">;
declare const liteQuery: Query<{ id: number }, { id: number }, "one", "sqlite">;
const asyncBs3: Promise<{ id: number }> = liteQuery.run(bs3, { id: 1 });
const asyncBun: Promise<{ id: number }> = liteQuery.with(bunDb)({ id: 1 });
const asyncD1: Promise<{ id: number }> = liteQuery.run(d1, { id: 1 });
const asyncLibsql: Promise<{ id: number }> = liteQuery.run(libsqlTx, { id: 1 });
const asyncPg: Promise<{ id: number }> = pgQuery.run(poolClient, { id: 1 });
const asyncTx: Promise<{ id: number }> = pgQuery.with(sqlTx)({ id: 1 });
const syncRow: { id: number } = liteQuery.runSync(bs3, { id: 1 });
const syncNode: { id: number } = liteQuery.runSync(nodeDb, { id: 1 });
const syncBun: { id: number } = liteQuery.runSync(bunDb, { id: 1 });
const asyncTurso: Promise<{ id: number }> = liteQuery.run(turso, { id: 1 });
const asyncTursoTx: Promise<{ id: number }> = liteQuery.run(tursoTx, { id: 1 });
const syncTursoCompat: { id: number } = liteQuery.runSync(tursoCompat, { id: 1 });
// @ts-expect-error runSync takes no client that answers through a promise
liteQuery.runSync(turso, { id: 1 });
// @ts-expect-error Turso's engine cannot run a postgres query
execute(turso, pgq, { id: 1 });
// @ts-expect-error Turso's engine cannot run a mysql query
execute(tursoTx, myq, { id: 1 });
// @ts-expect-error run's result is a promise even on a synchronous client
const notRow: { id: number } = liteQuery.run(bs3, { id: 1 });
// @ts-expect-error runSync takes no client that answers through a promise
liteQuery.runSync(libsql, { id: 1 });
// @ts-expect-error runSync takes no client that answers through a promise
liteQuery.runSync(d1, { id: 1 });
// @ts-expect-error a postgres statement has no synchronous client
pgQuery.runSync(pool, { id: 1 });
// @ts-expect-error a postgres statement has no synchronous client, not even a SQLite one
pgQuery.runSync(bs3, { id: 1 });

// Expected errors: wrong dialect for the client.
// @ts-expect-error a sqlite database cannot run a postgres query
execute(bs3, pgq, { id: 1 });
// @ts-expect-error a postgres pool cannot run a sqlite query
execute(pool, sqq, { id: 1 });
// @ts-expect-error mysql2 cannot run a postgres query (its query() resolves to a [rows, fields] array)
execute(my, pgq, { id: 1 });
// @ts-expect-error a mysql2 connection cannot run a postgres query
execute(myPoolConn, pgq, { id: 1 });
// @ts-expect-error mysql2 cannot run a sqlite query
execute(my, sqq, { id: 1 });
// @ts-expect-error mariadb cannot run a postgres query (its query() resolves to any; escapeId() tells it apart)
execute(maria, pgq, { id: 1 });
// @ts-expect-error mariadb cannot run a sqlite query (its execute() takes { sql } and resolves to any)
execute(maria, sqq, { id: 1 });
// @ts-expect-error postgres.js cannot run a mysql query (it has unsafe() as Bun.sql does, but not commitDistributed())
execute(sql, myq, { id: 1 });
// @ts-expect-error postgres.js cannot run a sqlite query
execute(sqlTx, sqq, { id: 1 });
// @ts-expect-error PGlite cannot run a mysql query
execute(pglite, myq, { id: 1 });
// @ts-expect-error libSQL cannot run a mysql query
execute(libsql, myq, { id: 1 });
// @ts-expect-error PlanetScale cannot run a sqlite query
execute(planetscale, sqq, { id: 1 });
// @ts-expect-error D1 cannot run a postgres query
execute(d1, pgq, { id: 1 });
// @ts-expect-error Neon's neon() cannot run a sqlite query
execute(neonHttp, sqq, { id: 1 });
// @ts-expect-error sqlite3 cannot run a mysql query
execute(sqlite3, myq, { id: 1 });
declare const planetscale2: PlanetScale2Connection;
// @ts-expect-error PlanetScale 2.0 binds no parameters (its execute takes a complete SQL string and options)
execute(planetscale2, myq, { id: 1 });
