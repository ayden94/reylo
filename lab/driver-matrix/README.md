# Driver matrix

A live test of the client dispatch in [src/execute.ts](../../src/execute.ts): every driver version and every client object a user can hold, run through `defineQuery(...).run(client, params)` against real servers. The matrix imports the runtime by relative path, so it tests whatever `src/` holds when it runs.

```sh
./run.sh                    # servers, type check, every job; writes results.json and RESULTS.md
./run.sh --only mariadb     # only the jobs whose id contains "mariadb" (the result files are not rewritten)
node typecheck/check.ts     # the type check alone; writes typecheck/results.json and typecheck/RESULTS.md
```

Each job runs in its own process: Node 26.7.0 by default, Node 24.19.0 for the second node:sqlite job, Bun 1.3.14 for Bun.sql and bun:sqlite. The jobs call those binaries directly rather than through nub, because nub adds polyfills (a global `WebSocket`, for one) that a user's plain Node does not have. The binaries can be changed with `DM_NODE26`, `DM_NODE24` and `DM_BUN`.

## Coverage

| Package | Versions | Target | Client shapes |
| --- | --- | --- | --- |
| pg | 8.23.0, 7.18.2 | Postgres 14 (local) | Pool, connected Client, PoolClient from `pool.connect()`, PoolClient inside BEGIN/COMMIT |
| postgres | 3.4.9 | Postgres 14 (local) | `sql`, `sql.begin(tx)`, `tx.savepoint(sp)`, `sql.reserve()` |
| @electric-sql/pglite | 0.5.8, 0.4.6, 0.3.16, 0.2.17 | in-process | PGlite, `db.transaction(tx)` |
| @neondatabase/serverless | 1.1.0, 0.10.4, 0.9.5 | Postgres 14 behind a local Neon proxy | Pool, Client, PoolClient, PoolClient inside BEGIN/COMMIT (all over WebSocket), `neon()` over HTTP, `neon(url, { fullResults: true })` |
| pg-promise | 12.7.1 | Postgres 14 (local) | `db`, `db.task(t)`, `db.tx(t)`, nested `t.tx(t2)` (a savepoint) |
| Bun.sql | Bun 1.3.14 | Postgres 14, MySQL 8.4, MySQL 8.0, SQLite file | `sql`, `sql.begin(tx)`, `tx.savepoint(sp)`, `sql.reserve()` (not on SQLite) |
| mysql2 | 3.24.5, 3.15.3, 2.3.3 | MySQL 8.4, MySQL 8.0 | promise Pool, Connection, PoolConnection, PoolConnection inside `beginTransaction()`; callback Pool, Connection and PoolConnection passed without `.promise()` |
| mariadb | 3.5.4, 3.4.5, 2.5.6 | MariaDB 11, MySQL 8.4 | Pool, Connection, PoolConnection, PoolConnection inside `beginTransaction()`; `mariadb/callback` Connection, Pool, and PoolConnection inside `beginTransaction()` |
| @planetscale/database | 2.0.0, 1.20.2 | MySQL 8.4 behind ps-http-sim | `connect()` Connection, Client, `conn.transaction(tx)` |
| better-sqlite3 | 13.0.3, 12.11.1; 11.10.0 on Node 24 from a source build (9.6.0, 8.7.0: see below) | SQLite file | Database, `db.transaction(fn)` |
| node:sqlite | Node 26.7.0, Node 24.19.0 | SQLite file | DatabaseSync, DatabaseSync inside BEGIN/COMMIT, `db.createTagStore()`, `db.createTagStore().db` |
| bun:sqlite | Bun 1.3.14 | SQLite file | Database, `db.transaction(fn)` |
| @libsql/client | 0.17.4, 0.18.0, 0.15.15, 0.14.0 | SQLite file (`file:` URL) | Client, `client.transaction("write")` |
| sqlite3 | 5.1.7, 6.0.1 | SQLite file | Database, BEGIN/COMMIT inside `db.serialize()` |
| sqlite | 5.1.1 over sqlite3 5.1.7 and 6.0.1 | SQLite file | Database from `open()`, Database inside BEGIN/COMMIT |
| Cloudflare D1 | miniflare 4.20260730.0 | local workerd | D1Database, `db.withSession()` |

The packages are installed side by side under npm aliases (`"pg-7": "npm:pg@7.18.2"`) in [package.json](package.json); nub wrote the lockfile as pnpm-lock.yaml. The three old better-sqlite3 lines live in [native-old/](native-old/package.json), a project of their own, because their source build must not block the rest.

## Scenarios

Every scenario asserts its result. Each call also checks dotsql's return contract: `run` returns a promise on every client and never throws synchronously. S0 checks `runSync` as well: it returns the value itself and throws on a synchronous SQLite client (better-sqlite3, node:sqlite, bun:sqlite), and on any other client it throws dotsql's TypeError before it calls the client.

| Id | Checks |
| --- | --- |
| S0 | The return contract of `run` and `runSync` on a select and on a syntax error, and which client method dotsql called |
| S1a | Integer, text, boolean and null parameters; the returned row (SQLite gets 1 for the boolean, since it has no boolean type) |
| S1b | An omitted (undefined) parameter binds NULL |
| S2a–S2e | Three rows for `many`; the row for `one`; null for `maybeOne` on zero rows; dotsql's "Expected at most one row, got 2" on two; `with(client)(params)` |
| S3 | An INSERT without RETURNING (cardinality `none`) resolves undefined; the row is read back through dotsql and counted through the driver |
| S3n | The same with the values written into the SQL, so dotsql passes no parameters |
| S4 | INSERT ... RETURNING (Postgres and SQLite) |
| S5a, S5b | One named parameter used twice, through the runtime lexer and through the mapper's `text` and `order` |
| S6a, S6b | A syntax error and a unique violation reject with the driver's own error and code (SQLSTATE 42601 and 23505, ER_PARSE_ERROR and ER_DUP_ENTRY, SQLITE_ERROR and SQLITE_CONSTRAINT), never dotsql's TypeError |
| S7a–S7d, S7n | On a transaction: a committed write is visible; a rolled-back write is not; a savepoint rollback keeps the outer write; a failing statement rolls the whole transaction back and its error reaches the caller; a write without parameters commits |
| S7e | On better-sqlite3 and bun:sqlite `db.transaction(fn)`, where S7a–S7d call `runSync`: a row read inside the function is usable synchronously |
| S8 | 20 queries in flight on the shape resolve with the right rows |
| S9 | An unsupported client: `run` rejects with dotsql's own TypeError |

Visibility is always checked with a raw count through the driver, never through dotsql. A raw count also runs before each job as a preflight, so a driver that cannot reach its server at all is reported once as "not run" instead of failing every scenario. [drivers/control.ts](drivers/control.ts) holds the failing controls: a client that binds every integer plus one, a transaction whose rollback commits, a synchronous database declared asynchronous, and a `db.transaction(fn)` whose statements answer through a promise. 19 of their 42 rows fail, and each of the 23 that pass is a scenario that the breakage does not touch. Since `run` returns a promise on every client, a synchronous database declared asynchronous changes only what S0 expects of `runSync`, so 15 of the 23 are its rows.

## Servers

[setup.sh](setup.sh) starts the containers idempotently and seeds every fixture; [run.sh](run.sh) calls it first. The containers are left running.

| Server | Where | Used by |
| --- | --- | --- |
| Postgres 14 (shared, local) | `postgres://localhost:5432/postgres`, schema `driver_matrix` only | pg, postgres, pg-promise, Bun.sql |
| porg-dm-mysql84 (mysql:8.4) | 127.0.0.1:33406, root/porg | mysql2, mariadb, Bun.sql, ps-http-sim |
| porg-dm-mysql80 (mysql:8.0) | 127.0.0.1:33380, root/porg | mysql2, Bun.sql |
| porg-dm-mariadb11 (mariadb:11) | 127.0.0.1:33411, root/porg | mariadb |
| porg-dm-neon-pg (postgres:14) | 127.0.0.1:55414, postgres/postgres | behind the Neon proxy |
| porg-dm-neon-proxy (ghcr.io/timowilhelm/local-neon-http-proxy:main) | 127.0.0.1:54444, HTTP `/sql` and WebSocket `/v1` | @neondatabase/serverless |
| porg-dm-ps-sim (ghcr.io/mattrobenolt/ps-http-sim:latest) | 127.0.0.1:38080 | @planetscale/database |

The Neon proxy creates a `neon_control_plane` schema in the database it fronts, so it has a Postgres container of its own instead of the shared server. The connection string names the host `db.localtest.me`, which the proxy uses as the endpoint name. That host does not resolve here, and it does not need to, because `fetchEndpoint` and `wsProxy` send every request to 127.0.0.1:54444.

## First run

The first run tested src/execute.ts 1b03ae48c322: 46 jobs and 2495 rows, with 2335 passing, 156 failing and 4 not run.

| Failure | Rows | Diagnosis |
| --- | --- | --- |
| mariadb 3.5.4, 3.4.5 and 2.5.6 through `mariadb/callback` (Connection), on MariaDB 11 and on MySQL 8.4 | 13 per job, 78 in all | dotsql saw `batch` and `query` and called `query(text, values)` without a callback. The callback API then returns a command object, not a promise, and `Array.isArray(result) ? result : []` turned it into `[]`. Reads came back empty, writes still ran, and errors were lost: S6a resolved `[]` and S6b resolved undefined |
| @planetscale/database 2.0.0, all three shapes | 46 | Version 2.0 removed parameter binding from `execute`: it throws "Query parameters are not supported: pass a complete SQL string and options to execute(query, options)" for any array second argument, including the empty array dotsql passed for a statement without parameters |
| node:sqlite `db.createTagStore()`, Node 26.7 and 24.19 | 15 per Node | dotsql saw `all` and took the store for a sqlite3 Database, calling `all(text, values, callback)`. Node threw its own TypeError, "First argument must be an array of strings (template literal)", instead of dotsql's unsupported-client error |
| @planetscale/database 1.20.2, S8 on the Connection and inside `conn.transaction` | 2 | Not dotsql. ps-http-sim answers HTTP 403 ("session already in use") to concurrent requests on one session. The driver alone does the same: [probes/planetscale-session.mjs](probes/planetscale-session.mjs) |
| pg 7.18.2 (not run) | 1 | Not dotsql. The preflight, a raw `pool.query`, ends in "Connection terminated due to connection timeout" on Node 26: pg 7 predates the stream changes of Node 14 ([node-postgres#2317](https://github.com/brianc/node-postgres/issues/2317)). No Node version runs both dotsql (type stripping needs Node 22.6 or later; dotsql's `engines` says 24) and pg 7 |
| better-sqlite3 11.10.0, 9.6.0 and 8.7.0 (not run) | 3 | Not dotsql. None of them has a prebuilt binary for Node 26 (ABI 147), so they build from source (see below) |

Everything else passed on the first run, including every transaction shape listed above.

## Re-run

The re-run tested src/execute.ts c46b8b73fe74 and src/index.ts 0dac6252558a, after the orchestrator's fixes for the three dotsql failures: 46 jobs, 179 client shapes and 2913 rows. The results are 2868 pass, 2 fail, 39 unsupported and 4 not run. A run over src/execute.ts 3292aea4ccac, which changes only the client types (see the type check below), gives the same rows. The run after it adds better-sqlite3 11.10.0 on Node 24 from a source build (below): 46 jobs and 2933 rows, with 2889 pass, 2 fail, 39 unsupported and 3 not run. The last run, over src/execute.ts 332f629f3f63, where `run` returns a promise on every client and `runSync` is new, gives the same counts. [RESULTS.md](RESULTS.md) has every row.

| Rows | Result | Diagnosis |
| --- | --- | --- |
| mariadb 3.5.4, 3.4.5 and 2.5.6, every shape on both servers | pass (690 rows) | dotsql now calls `query(text, values, callback)` and takes whichever answers: the callback on `mariadb/callback`, the returned promise on the promise API. S0 shows `query()`; S6a and S6b get SqlError 1064 and 1062 |
| @planetscale/database 2.0.0: S0, S2a, S3n, S6a, S7n on all three shapes | pass (13 rows) | A statement without parameters now reaches `execute(text)` with no second argument |
| @planetscale/database 2.0.0: every scenario with parameters | unsupported (37 rows) | dotsql refuses the statement with its TypeError "@planetscale/database 2.0 binds no parameters, ..." and the row checks that its `cause` is PlanetScale's error. By design: 2.0 takes no parameters, and dotsql does not escape values into SQL. Version 2.0 has 1,217 downloads a week against 457,670 for 1.x |
| node:sqlite `db.createTagStore()`, Node 26.7 and 24.19 | unsupported (2 rows, S9) | dotsql refuses the store with its own TypeError, which ends "A tag store is not a client: pass its database, store.db." |
| node:sqlite `db.createTagStore().db`, Node 26.7 and 24.19 | pass (32 rows) | The store's database is a DatabaseSync |
| @planetscale/database 1.20.2, S8 on the Connection and inside `conn.transaction` | fail (2 rows) | ps-http-sim's session lock, as in the first run. The raw probe fails the same way |
| pg 7.18.2; better-sqlite3 9.6.0, 8.7.0 | not run (3 rows) | As in the first run |
| better-sqlite3 11.10.0 on Node 24.19 | pass (21 rows) | From a source build against Node 24's headers; see below |

Two driver behaviours show up as details on passing rows:

- On a pg connection (a Client, a PoolClient, a pg-promise task or tx), S8's 20 concurrent queries work, but pg 8.23 prints "Calling client.query() when the client is already executing a query is deprecated and will be removed in pg@9.0". dotsql issues one query per `run`, so the concurrency is the caller's.
- A statement that fails inside an open Postgres transaction aborts it. On postgres.js `sql.begin` and `tx.savepoint`, pg-promise's nested `t.tx`, and Bun.sql's `tx.savepoint`, the enclosing commit then fails with the driver's error. The 12 rows where this happens record it and pass, because dotsql already returned the driver's error.

## Type check

[typecheck/check.ts](typecheck/check.ts) generates [typecheck/cases.ts](typecheck/cases.ts), which declares one value of each client type and passes it to `run` on a Postgres, a MySQL and a SQLite query. It also checks the type `run`, `with(...)(...)` and `runSync` return (a promise from `run` and `with` on every client, the value from `runSync` on a synchronous SQLite client) and that `runSync` rejects every other client, then runs the repository's tsc (7.1.0-dev.20260918.1) and maps each diagnostic back to its line. Against the final src (src/execute.ts 332f629f3f63), all 552 cases over 94 client types are as expected: none is wrongly accepted and none is wrongly rejected. [typecheck/RESULTS.md](typecheck/RESULTS.md) lists every case.

Against the re-run's src (c46b8b73fe74), 21 were wrongly accepted. Each was closed with a property that only the right client has, used in the type alone: `PgQueryClient` has `escapeId?: never` (mariadb has `escapeId()`), `LibsqlClient` requires `executeMultiple()` (libSQL 0.14 to 0.18 have it, mariadb does not), and the MySQL and SQLite unions take `BunSqlClient`, which requires Bun's `commitDistributed()` beside `unsafe()` (postgres.js has no such method).

| Wrongly accepted before the fix | Why the type admitted it | At run time |
| --- | --- | --- |
| postgres 3.4.9 `Sql`, `TransactionSql` and `ReservedSql` on a MySQL or a SQLite query (6 cases) | `UnsafeClient` is in all three unions for Bun.sql, whose one class serves every adapter. postgres.js has `unsafe` too, and the two cannot be told apart by shape | The MySQL or SQLite text reaches Postgres: "PostgresError: syntax error at end of input" ([probes/cross-dialect.ts](probes/cross-dialect.ts), with postgres 3.4.9) |
| mariadb 3.5.4, 3.4.5 and 2.5.6 Pool, Connection and PoolConnection on a Postgres query (9 cases) | mariadb types `query<T = any>(...): Promise<T>`, so it satisfies `PgQueryClient`'s `query(...): PromiseLike<{ rows: unknown[] }>` | "Unknown column '$1' in 'WHERE'" (SqlError 1054, the same probe with mariadb 3.5.4) |
| mariadb 3.5.4 and 3.4.5 Pool, Connection and PoolConnection on a SQLite query (6 cases) | mariadb 3 has `execute<T = any>(sql: string \| QueryOptions, ...)`, which satisfies `LibsqlClient`'s `execute({ sql, args })` (2.5.6 has no `execute`) | "Malformed communication packet" (SqlError 1835) |

These were checked as expected:

- mysql2/promise Pool, Connection and PoolConnection are rejected on a Postgres query, though pg's query type now admits a rows array.
- @planetscale/database 2.0.0's Connection, Client and Transaction are rejected on every dialect.
- The `mariadb/callback` Pool, Connection and PoolConnection are accepted on MySQL only.
- node:sqlite's `SQLTagStore` is rejected everywhere.
- On better-sqlite3's Database, node:sqlite's DatabaseSync and bun:sqlite's Database, `run` and `with(...)(...)` are typed as the row. On every other accepted client they are typed as a promise.

## Not tested

- **pg 7.18.2.** It cannot connect on any Node that runs dotsql (see the first run). A shape-only test would add nothing, because its API is the same `query(text, values)` → `{ rows }` as pg 8.
- **better-sqlite3 9.6.0 and 8.7.0** (1.8 and 1.3 percent of the package's downloads). No prebuilt binary exists for Node 24 (ABI 137) or 26 (ABI 147), so they run only from a source build in native-old/ against Node 24's headers, which was not made for these two. 11.10.0 (18 percent) is in the same position and runs that way: its source does not compile against Node 26's V8 (it calls `Object::GetPrototype`, `Context::GetIsolate` and `PropertyCallbackInfo::This`, which V8 removed), and it compiles against Node 24's (`node <npm's node-gyp.js> rebuild --release` with Node 24.19 in `native-old/node_modules/bs3-11`, about 85 minutes at a load average near 100), so the runner runs these lines on Node 24. The first install had failed because a global node-gyp 3.8.0 on PATH still expects Python 2, so native-old/ builds with npm's node-gyp 13.
- **miniflare 5.** The version npm tags `latest` (5.20260926.1-alpha and later, which wrangler 4.144 depends on) rejects the v4 constructor options (`modules`, `script`, `d1Databases`) and wants a new `workers: [{ config }]` format, so the D1 job uses miniflare 4.20260730.0. D1 is reached through Miniflare's Node proxy, not from inside a Worker.
- **The real Neon and PlanetScale services.** Both are live against local stand-ins: Neon's proxy image (built 2026-03-27) and ps-http-sim. The real services could differ in details such as PlanetScale's session concurrency.
- **Neon's HTTP `transaction()`** takes an array of queries built by the `sql` function, not a client, so a dotsql query cannot join it. D1 has no interactive transaction either (`batch()` takes prepared statements).
- **Other server and driver pairs:** mysql2 and Bun.sql against MariaDB, @libsql/client against a sqld server (`http:` or `ws:` URLs), PGlite through `PGliteWorker`, `pg-native`, Neon's `arrayMode`, and better-sqlite3 or the npm drivers under Bun.
- **Other runtimes:** Deno, and a Cloudflare Worker's own D1 binding.

## Layout

| Path | Contents |
| --- | --- |
| [run.sh](run.sh) | The whole matrix |
| [setup.sh](setup.sh), [seed/](seed/) | Servers and fixtures (tables `people`, `scratch`, `uniq`) |
| [runner.ts](runner.ts) | The job list, the process pool (4 at a time, 300 s per job), results.json and RESULTS.md |
| [lib/harness.ts](lib/harness.ts) | Scenarios, assertions, the return-contract checks, transaction helpers |
| [lib/tagged.ts](lib/tagged.ts) | The shapes shared by postgres.js and Bun.sql |
| [drivers/](drivers/) | One file per driver family |
| [typecheck/](typecheck/) | The type check |
| [probes/](probes/) | The controls behind two diagnoses: PlanetScale's session lock and the cross-dialect run-time errors |
| .out/, .data/ | Per-job logs and JSON Lines, the SQLite files (gitignored) |
