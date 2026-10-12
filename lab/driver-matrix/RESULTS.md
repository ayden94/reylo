# Driver matrix results

Generated 2026-10-12T20:17:59.902Z by `./run.sh` against the working tree at ab86ebc (uncommitted changes in src/: yes; src/execute.ts b7dfb84f5035, src/index.ts bf6bacc47423). The report and the diagnoses are in [README.md](README.md).

3510 rows: 3457 pass, 2 fail, 48 unsupported (reylo refuses them with its own TypeError, as its contract says), 0 skip, 3 not run. Failing controls: 27 of 93 control rows fail, as designed.

## Per job

| Package@version | Runtime | Target | Pass | Fail | Unsupported | Skip | Not run |
| --- | --- | --- | --- | --- | --- | --- | --- |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | 93 | 0 | 0 | 0 | 0 |
| pg@7.18.2 | node 26.11.1 | Postgres 14 (local) | 0 | 0 | 0 | 0 | 1 |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | 103 | 0 | 0 | 0 | 0 |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | 49 | 0 | 0 | 0 | 0 |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | 49 | 0 | 0 | 0 | 0 |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | 49 | 0 | 0 | 0 | 0 |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | 49 | 0 | 0 | 0 | 0 |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | 137 | 0 | 0 | 0 | 0 |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | 137 | 0 | 0 | 0 | 0 |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | 137 | 0 | 0 | 0 | 0 |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | 98 | 0 | 0 | 0 | 0 |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | 103 | 0 | 0 | 0 | 0 |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | 131 | 0 | 0 | 0 | 0 |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | 131 | 0 | 0 | 0 | 0 |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | 131 | 0 | 0 | 0 | 0 |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | 131 | 0 | 0 | 0 | 0 |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | 131 | 0 | 0 | 0 | 0 |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | 131 | 0 | 0 | 0 | 0 |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | 136 | 0 | 0 | 0 | 0 |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | 136 | 0 | 0 | 0 | 0 |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | 136 | 0 | 0 | 0 | 0 |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | 136 | 0 | 0 | 0 | 0 |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | 136 | 0 | 0 | 0 | 0 |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | 136 | 0 | 0 | 0 | 0 |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | 87 | 0 | 0 | 0 | 0 |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | 87 | 0 | 0 | 0 | 0 |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | 13 | 0 | 46 | 0 | 0 |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | 57 | 2 | 0 | 0 | 0 |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | 22 | 0 | 0 | 0 | 0 |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | 22 | 0 | 0 | 0 | 0 |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | 22 | 0 | 0 | 0 | 0 |
| better-sqlite3@9.6.0 | node 24.19.0 | SQLite file | 0 | 0 | 0 | 0 | 1 |
| better-sqlite3@8.7.0 | node 24.19.0 | SQLite file | 0 | 0 | 0 | 0 | 1 |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | 56 | 0 | 1 | 0 | 0 |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | 56 | 0 | 1 | 0 | 0 |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | 22 | 0 | 0 | 0 | 0 |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | 61 | 0 | 0 | 0 | 0 |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | 39 | 0 | 0 | 0 | 0 |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | 39 | 0 | 0 | 0 | 0 |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | 39 | 0 | 0 | 0 | 0 |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | 39 | 0 | 0 | 0 | 0 |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | 39 | 0 | 0 | 0 | 0 |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | 39 | 0 | 0 | 0 | 0 |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | 39 | 0 | 0 | 0 | 0 |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | 39 | 0 | 0 | 0 | 0 |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | 34 | 0 | 0 | 0 | 0 |

## Failures, grouped

One row per package, shape and set of failing scenarios, across the versions, runtimes and targets that fail the same way. The error is the one of the first failing scenario; every row is in the last table.

| Package | Versions | Runtimes | Targets | Shape | Failing scenarios | First error |
| --- | --- | --- | --- | --- | --- | --- |
| pg | 7.18.2 | node 26.11.1 | Postgres 14 (local) | (job) | preflight | not run: the driver cannot run a query without reylo: Error: Connection terminated due to connection timeout |
| @planetscale/database | 1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S8 | UnknownError: Expected JSON response from database API, got HTTP 403 Forbidden [body.code=UNKNOWN] |
| @planetscale/database | 1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S8 | UnknownError: Expected JSON response from database API, got HTTP 403 Forbidden [body.code=UNKNOWN] |
| better-sqlite3 | 9.6.0, 8.7.0 | node 24.19.0 | SQLite file | (job) | setup | not run: no prebuilt binary for Node 24.19.0 (ABI 137), and native-old/ has no source build of it against that Node's headers (see README) |

## Unsupported by design

Rows where reylo refuses the client or the statement with its own TypeError, which is its contract for that shape: the row checks the refusal (its message, and for PlanetScale 2.0 that its cause is the driver's error).

| Package | Versions | Runtimes | Shape | Scenarios | reylo's refusal |
| --- | --- | --- | --- | --- | --- |
| @planetscale/database | 2.0.0 | node 26.11.1 | connect() Connection | S1a, S1b, S1c, S1g, S1h, S2b, S2c, S2d, S2e, S3, S5a, S5b, S6b, S8 | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database | 2.0.0 | node 26.11.1 | Client | S1a, S1b, S1c, S1g, S1h, S2b, S2c, S2d, S2e, S3, S5a, S5b, S6b, S8 | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database | 2.0.0 | node 26.11.1 | conn.transaction(tx) | S1a, S1b, S1c, S1g, S1h, S2b, S2c, S2d, S2e, S3, S5a, S5b, S6b, S7a, S7b, S7c, S7d, S8 | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| node:sqlite | 26.11.1, 24.21.0 | node 26.11.1, node 24.21.0 | db.createTagStore() | S9 | reylo refused it: reylo cannot run a sqlite statement on an object with get, all, iterate, run, clear. It takes a client with prepare(sql) as on better-sqlite3, node:sqlite, bun:sqlite and Cloudflare D1; execute({ sql, args }) as on @libsql/client; all(sql, params) as on sqlite3 and sqlite; or unsafe(text, values) as on Bun.sql. A tag store is not a client: pass its database, store.db. |

## Failing controls

Deliberately broken clients (drivers/control.ts). Every row marked FAIL here is a scenario that caught the breakage; a pass is a scenario the breakage does not touch.

| Control | Shape | Scenario | Result | Error |
| --- | --- | --- | --- | --- |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | PASS |  |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S1a params: int, text, bool, null | FAIL | Error: Expected exactly one row, got 0 |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S1b omitted param (undefined) binds NULL | FAIL | rows: expected [{"id":1}], got [{"id":2}] |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | FAIL | Error: Invalid input for bytea type |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S1d array params: = any(:ids) over an int and a text array | PASS |  |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | PASS |  |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | PASS |  |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | PASS |  |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S1f array params: bool[], timestamptz[] and bytea[] | PASS |  |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S2a many: 3 rows | PASS |  |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S2b one: the row | FAIL | row: expected {"id":2,"name":"bob"}, got {"id":3,"name":"cy"} |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S2c maybeOne: zero rows gives null | PASS |  |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S2d maybeOne: two rows throw reylo's error | FAIL | wrong error: Error: Expected at most one row, got 3 |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S2e with(client)(params) | FAIL | Error: Expected exactly one row, got 0 |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S3 write without RETURNING (none), then a separate read | FAIL | read back through reylo: expected {"n":3}, got {"n":4} |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S3n write without parameters (none), then a separate read | PASS |  |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S4 INSERT ... RETURNING | FAIL | returned row: expected {"n":4,"tag":"control@pg/parameters corrupted: an integer plus one, a Uint8Array as JSON/S4/mv5p20bimg6k/3"}, got {"tag":"control@pg/parameters corrupted: an integer plus one, a Uint8Array as JSON/S4/mv5p20bimg6k/3","n":5} |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S5a repeated :id (runtime lexer) | FAIL | Error: Expected exactly one row, got 0 |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S5b repeated :id (mapper text/order) | FAIL | Error: Expected exactly one row, got 0 |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S6a syntax error rejects with the driver's error | PASS |  |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S6b unique violation rejects with the driver's error | PASS |  |
| control/pg | parameters corrupted: an integer plus one, a Uint8Array as JSON | S8 20 concurrent queries resolve with the right rows | FAIL | Error: Expected exactly one row, got 0 |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S1a params: int, text, bool, null | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S1b omitted param (undefined) binds NULL | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S1d array params: = any(:ids) over an int and a text array | FAIL | me: malformed array literal: "1,3,4" [code=22P02 severity=ERROR] |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | FAIL | me: invalid input syntax for type json [code=22P02 severity=ERROR] |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | FAIL | me: invalid input syntax for type json [code=22P02 severity=ERROR] |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | FAIL | reylo contract: a Date for a date parameter was not refused with reylo's TypeError: non-Error thrown: null |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S1f array params: bool[], timestamptz[] and bytea[] | FAIL | me: malformed array literal: "true,false" [code=22P02 severity=ERROR] |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S2a many: 3 rows | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S2b one: the row | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S2c maybeOne: zero rows gives null | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S2d maybeOne: two rows throw reylo's error | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S2e with(client)(params) | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S3 write without RETURNING (none), then a separate read | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S3n write without parameters (none), then a separate read | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S4 INSERT ... RETURNING | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S5a repeated :id (runtime lexer) | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S5b repeated :id (mapper text/order) | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S6a syntax error rejects with the driver's error | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S6b unique violation rejects with the driver's error | PASS |  |
| control/pg | unsafe() binding an array as a,b, statements without inputs | S8 20 concurrent queries resolve with the right rows | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S1a params: int, text, bool, null | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S1b omitted param (undefined) binds NULL | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S1d array params: = any(:ids) over an int and a text array | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | FAIL | me: invalid input syntax for type json [code=22P02 severity=ERROR] |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | FAIL | me: invalid input syntax for type json [code=22P02 severity=ERROR] |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S1f array params: bool[], timestamptz[] and bytea[] | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S2a many: 3 rows | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S2b one: the row | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S2c maybeOne: zero rows gives null | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S2d maybeOne: two rows throw reylo's error | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S2e with(client)(params) | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S3 write without RETURNING (none), then a separate read | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S3n write without parameters (none), then a separate read | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S4 INSERT ... RETURNING | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S5a repeated :id (runtime lexer) | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S5b repeated :id (mapper text/order) | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S6a syntax error rejects with the driver's error | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S6b unique violation rejects with the driver's error | PASS |  |
| control/pg | unsafe() binding every array as its array literal, a JSON parameter's too | S8 20 concurrent queries resolve with the right rows | PASS |  |
| control/pg | rollback that commits | S7a tx: write then commit is visible | PASS |  |
| control/pg | rollback that commits | S7b tx: write then rollback is not visible | FAIL | raw count after rollback: expected 0, got 1 |
| control/pg | rollback that commits | S7c tx: savepoint rollback keeps the outer write | FAIL | raw counts [outer, inner]: expected [1,0], got [1,1] |
| control/pg | rollback that commits | S7d tx: a failing statement rolls the transaction back | FAIL | the transaction did not reject |
| control/pg | rollback that commits | S7n tx: a write without parameters, then commit, is visible | PASS |  |
| control/sqlite | sync client declared async | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | FAIL | reylo contract: runSync on an asynchronous client did not throw reylo's TypeError: non-Error thrown: null |
| control/sqlite | sync client declared async | S1a params: int, text, bool, null | PASS |  |
| control/sqlite | sync client declared async | S1b omitted param (undefined) binds NULL | PASS |  |
| control/sqlite | sync client declared async | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | PASS |  |
| control/sqlite | sync client declared async | S2a many: 3 rows | PASS |  |
| control/sqlite | sync client declared async | S2b one: the row | PASS |  |
| control/sqlite | sync client declared async | S2c maybeOne: zero rows gives null | PASS |  |
| control/sqlite | sync client declared async | S2d maybeOne: two rows throw reylo's error | PASS |  |
| control/sqlite | sync client declared async | S2e with(client)(params) | PASS |  |
| control/sqlite | sync client declared async | S3 write without RETURNING (none), then a separate read | PASS |  |
| control/sqlite | sync client declared async | S3n write without parameters (none), then a separate read | PASS |  |
| control/sqlite | sync client declared async | S4 INSERT ... RETURNING | PASS |  |
| control/sqlite | sync client declared async | S5a repeated :id (runtime lexer) | PASS |  |
| control/sqlite | sync client declared async | S5b repeated :id (mapper text/order) | PASS |  |
| control/sqlite | sync client declared async | S6a syntax error rejects with the driver's error | PASS |  |
| control/sqlite | sync client declared async | S6b unique violation rejects with the driver's error | PASS |  |
| control/sqlite | sync client declared async | S8 20 concurrent queries resolve with the right rows | PASS |  |
| control/sqlite | db.transaction(fn) whose statements answer through a promise | S7a tx: write then commit is visible | FAIL | TypeError: runSync takes a synchronous SQLite client (better-sqlite3, node:sqlite or bun:sqlite); this client's statement answered with a promise. run takes every client. |
| control/sqlite | db.transaction(fn) whose statements answer through a promise | S7b tx: write then rollback is not visible | FAIL | TypeError: runSync takes a synchronous SQLite client (better-sqlite3, node:sqlite or bun:sqlite); this client's statement answered with a promise. run takes every client. |
| control/sqlite | db.transaction(fn) whose statements answer through a promise | S7c tx: savepoint rollback keeps the outer write | FAIL | TypeError: runSync takes a synchronous SQLite client (better-sqlite3, node:sqlite or bun:sqlite); this client's statement answered with a promise. run takes every client. |
| control/sqlite | db.transaction(fn) whose statements answer through a promise | S7d tx: a failing statement rolls the transaction back | FAIL | not the driver's unique-violation error: TypeError: runSync takes a synchronous SQLite client (better-sqlite3, node:sqlite or bun:sqlite); this client's statement answered with a promise. run takes every client. |
| control/sqlite | db.transaction(fn) whose statements answer through a promise | S7e tx: a read inside transaction(fn) is usable synchronously | FAIL | TypeError: runSync takes a synchronous SQLite client (better-sqlite3, node:sqlite or bun:sqlite); this client's statement answered with a promise. run takes every client. |

## Type check

`tsc` Version 7.1.0-dev.20260918.1 over typecheck/cases.ts: every client and transaction object passed to a query's run (ClientFor<D>), and the type run returns. Details in [typecheck/RESULTS.md](typecheck/RESULTS.md).

552 cases over 94 client types: 552 as expected, 0 wrongly accepted, 0 wrongly rejected.

## Every row

| Package@version | Runtime | Target | Shape | Scenario | Result | Error or detail |
| --- | --- | --- | --- | --- | --- | --- |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S1a params: int, text, bool, null | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S1b omitted param (undefined) binds NULL | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S1d array params: = any(:ids) over an int and a text array | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S2a many: 3 rows | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S2b one: the row | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S2c maybeOne: zero rows gives null | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S2e with(client)(params) | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S3n write without parameters (none), then a separate read | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S4 INSERT ... RETURNING | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S5a repeated :id (runtime lexer) | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S5b repeated :id (mapper text/order) | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S1a params: int, text, bool, null | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S1b omitted param (undefined) binds NULL | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S2a many: 3 rows | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S2b one: the row | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S2c maybeOne: zero rows gives null | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S2d maybeOne: two rows throw reylo's error | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S2e with(client)(params) | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S3 write without RETURNING (none), then a separate read | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S3n write without parameters (none), then a separate read | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S4 INSERT ... RETURNING | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S5a repeated :id (runtime lexer) | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S5b repeated :id (mapper text/order) | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | Client (connected) | S8 20 concurrent queries resolve with the right rows | pass | warning: DeprecationWarning: Calling client.query() when the client is already executing a query is deprecated and will be removed in pg@9.0. Use async/await or an external async flow control mechanism instead. |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S1a params: int, text, bool, null | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S1b omitted param (undefined) binds NULL | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S2a many: 3 rows | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S2b one: the row | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S2c maybeOne: zero rows gives null | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S2e with(client)(params) | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S3 write without RETURNING (none), then a separate read | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S3n write without parameters (none), then a separate read | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S4 INSERT ... RETURNING | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S5a repeated :id (runtime lexer) | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S5b repeated :id (mapper text/order) | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient (pool.connect()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S1a params: int, text, bool, null | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S1b omitted param (undefined) binds NULL | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S1d array params: = any(:ids) over an int and a text array | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S2a many: 3 rows | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S2b one: the row | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S2c maybeOne: zero rows gives null | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S2d maybeOne: two rows throw reylo's error | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S2e with(client)(params) | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S3 write without RETURNING (none), then a separate read | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S3n write without parameters (none), then a separate read | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S4 INSERT ... RETURNING | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S5a repeated :id (runtime lexer) | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S5b repeated :id (mapper text/order) | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S7a tx: write then commit is visible | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S7b tx: write then rollback is not visible | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S7c tx: savepoint rollback keeps the outer write | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S7d tx: a failing statement rolls the transaction back | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S7n tx: a write without parameters, then commit, is visible | pass |  |
| pg@8.23.0 | node 26.11.1 | Postgres 14 (local) | PoolClient in BEGIN/COMMIT | S8 20 concurrent queries resolve with the right rows | pass |  |
| pg@7.18.2 | node 26.11.1 | Postgres 14 (local) | (job) | preflight | NOT RUN | the driver cannot run a query without reylo: Error: Connection terminated due to connection timeout |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S1a params: int, text, bool, null | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S1b omitted param (undefined) binds NULL | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S1d array params: = any(:ids) over an int and a text array | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S2a many: 3 rows | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S2b one: the row | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S2c maybeOne: zero rows gives null | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S2d maybeOne: two rows throw reylo's error | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S2e with(client)(params) | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S3 write without RETURNING (none), then a separate read | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S3n write without parameters (none), then a separate read | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S4 INSERT ... RETURNING | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S5a repeated :id (runtime lexer) | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S5b repeated :id (mapper text/order) | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S6a syntax error rejects with the driver's error | pass | PostgresError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S6b unique violation rejects with the driver's error | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql | S8 20 concurrent queries resolve with the right rows | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it; then the enclosing transaction failed, as the database aborts it after an error: PostgresError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S1a params: int, text, bool, null | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S1b omitted param (undefined) binds NULL | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S2a many: 3 rows | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S2b one: the row | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S2c maybeOne: zero rows gives null | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S2d maybeOne: two rows throw reylo's error | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S2e with(client)(params) | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S3 write without RETURNING (none), then a separate read | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S3n write without parameters (none), then a separate read | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S4 INSERT ... RETURNING | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S5a repeated :id (runtime lexer) | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S5b repeated :id (mapper text/order) | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S6a syntax error rejects with the driver's error | pass | PostgresError: syntax error at or near "selec" [code=42601 severity=ERROR]; then the enclosing transaction failed, as the database aborts it after an error: PostgresError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S6b unique violation rejects with the driver's error | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR]; then the enclosing transaction failed, as the database aborts it after an error: PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S7a tx: write then commit is visible | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S7b tx: write then rollback is not visible | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S7d tx: a failing statement rolls the transaction back | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.begin(tx) | S8 20 concurrent queries resolve with the right rows | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it; then the enclosing transaction failed, as the database aborts it after an error: PostgresError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S1a params: int, text, bool, null | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S1b omitted param (undefined) binds NULL | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S2a many: 3 rows | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S2b one: the row | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S2c maybeOne: zero rows gives null | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S2d maybeOne: two rows throw reylo's error | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S2e with(client)(params) | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S3 write without RETURNING (none), then a separate read | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S3n write without parameters (none), then a separate read | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S4 INSERT ... RETURNING | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S5a repeated :id (runtime lexer) | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S5b repeated :id (mapper text/order) | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S6a syntax error rejects with the driver's error | pass | PostgresError: syntax error at or near "selec" [code=42601 severity=ERROR]; then the enclosing transaction failed, as the database aborts it after an error: PostgresError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S6b unique violation rejects with the driver's error | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR]; then the enclosing transaction failed, as the database aborts it after an error: PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S7a tx: write then commit is visible | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S7b tx: write then rollback is not visible | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S7d tx: a failing statement rolls the transaction back | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | tx.savepoint(sp) | S8 20 concurrent queries resolve with the right rows | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S1a params: int, text, bool, null | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S1b omitted param (undefined) binds NULL | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S1d array params: = any(:ids) over an int and a text array | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S2a many: 3 rows | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S2b one: the row | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S2c maybeOne: zero rows gives null | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S2d maybeOne: two rows throw reylo's error | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S2e with(client)(params) | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S3 write without RETURNING (none), then a separate read | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S3n write without parameters (none), then a separate read | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S4 INSERT ... RETURNING | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S5a repeated :id (runtime lexer) | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S5b repeated :id (mapper text/order) | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S6a syntax error rejects with the driver's error | pass | PostgresError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S6b unique violation rejects with the driver's error | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S7a tx: write then commit is visible | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S7b tx: write then rollback is not visible | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S7d tx: a failing statement rolls the transaction back | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| postgres@3.4.9 | node 26.11.1 | Postgres 14 (local) | sql.reserve() | S8 20 concurrent queries resolve with the right rows | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S1a params: int, text, bool, null | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S1b omitted param (undefined) binds NULL | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S2a many: 3 rows | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S2b one: the row | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S2c maybeOne: zero rows gives null | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S2d maybeOne: two rows throw reylo's error | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S2e with(client)(params) | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S3 write without RETURNING (none), then a separate read | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S3n write without parameters (none), then a separate read | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S4 INSERT ... RETURNING | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S5a repeated :id (runtime lexer) | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S5b repeated :id (mapper text/order) | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S6a syntax error rejects with the driver's error | pass | me: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S6b unique violation rejects with the driver's error | pass | me: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | PGlite | S8 20 concurrent queries resolve with the right rows | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1a params: int, text, bool, null | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1b omitted param (undefined) binds NULL | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2a many: 3 rows | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2b one: the row | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2c maybeOne: zero rows gives null | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2e with(client)(params) | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S3 write without RETURNING (none), then a separate read | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S3n write without parameters (none), then a separate read | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S4 INSERT ... RETURNING | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S5a repeated :id (runtime lexer) | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S5b repeated :id (mapper text/order) | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S6a syntax error rejects with the driver's error | pass | me: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S6b unique violation rejects with the driver's error | pass | me: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7a tx: write then commit is visible | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7b tx: write then rollback is not visible | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7d tx: a failing statement rolls the transaction back | pass | me: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @electric-sql/pglite@0.5.8 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S1a params: int, text, bool, null | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S1b omitted param (undefined) binds NULL | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S2a many: 3 rows | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S2b one: the row | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S2c maybeOne: zero rows gives null | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S2d maybeOne: two rows throw reylo's error | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S2e with(client)(params) | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S3 write without RETURNING (none), then a separate read | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S3n write without parameters (none), then a separate read | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S4 INSERT ... RETURNING | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S5a repeated :id (runtime lexer) | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S5b repeated :id (mapper text/order) | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S6a syntax error rejects with the driver's error | pass | me: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S6b unique violation rejects with the driver's error | pass | me: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | PGlite | S8 20 concurrent queries resolve with the right rows | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1a params: int, text, bool, null | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1b omitted param (undefined) binds NULL | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2a many: 3 rows | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2b one: the row | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2c maybeOne: zero rows gives null | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2e with(client)(params) | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S3 write without RETURNING (none), then a separate read | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S3n write without parameters (none), then a separate read | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S4 INSERT ... RETURNING | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S5a repeated :id (runtime lexer) | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S5b repeated :id (mapper text/order) | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S6a syntax error rejects with the driver's error | pass | me: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S6b unique violation rejects with the driver's error | pass | me: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7a tx: write then commit is visible | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7b tx: write then rollback is not visible | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7d tx: a failing statement rolls the transaction back | pass | me: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @electric-sql/pglite@0.4.6 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S1a params: int, text, bool, null | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S1b omitted param (undefined) binds NULL | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S2a many: 3 rows | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S2b one: the row | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S2c maybeOne: zero rows gives null | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S2d maybeOne: two rows throw reylo's error | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S2e with(client)(params) | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S3 write without RETURNING (none), then a separate read | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S3n write without parameters (none), then a separate read | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S4 INSERT ... RETURNING | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S5a repeated :id (runtime lexer) | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S5b repeated :id (mapper text/order) | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S6a syntax error rejects with the driver's error | pass | ne: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S6b unique violation rejects with the driver's error | pass | ne: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | PGlite | S8 20 concurrent queries resolve with the right rows | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1a params: int, text, bool, null | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1b omitted param (undefined) binds NULL | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2a many: 3 rows | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2b one: the row | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2c maybeOne: zero rows gives null | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2e with(client)(params) | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S3 write without RETURNING (none), then a separate read | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S3n write without parameters (none), then a separate read | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S4 INSERT ... RETURNING | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S5a repeated :id (runtime lexer) | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S5b repeated :id (mapper text/order) | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S6a syntax error rejects with the driver's error | pass | ne: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S6b unique violation rejects with the driver's error | pass | ne: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7a tx: write then commit is visible | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7b tx: write then rollback is not visible | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7d tx: a failing statement rolls the transaction back | pass | ne: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @electric-sql/pglite@0.3.16 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S1a params: int, text, bool, null | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S1b omitted param (undefined) binds NULL | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S2a many: 3 rows | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S2b one: the row | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S2c maybeOne: zero rows gives null | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S2d maybeOne: two rows throw reylo's error | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S2e with(client)(params) | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S3 write without RETURNING (none), then a separate read | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S3n write without parameters (none), then a separate read | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S4 INSERT ... RETURNING | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S5a repeated :id (runtime lexer) | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S5b repeated :id (mapper text/order) | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S6a syntax error rejects with the driver's error | pass | xe: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S6b unique violation rejects with the driver's error | pass | xe: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | PGlite | S8 20 concurrent queries resolve with the right rows | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1a params: int, text, bool, null | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1b omitted param (undefined) binds NULL | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2a many: 3 rows | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2b one: the row | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2c maybeOne: zero rows gives null | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S2e with(client)(params) | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S3 write without RETURNING (none), then a separate read | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S3n write without parameters (none), then a separate read | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S4 INSERT ... RETURNING | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S5a repeated :id (runtime lexer) | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S5b repeated :id (mapper text/order) | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S6a syntax error rejects with the driver's error | pass | xe: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S6b unique violation rejects with the driver's error | pass | xe: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7a tx: write then commit is visible | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7b tx: write then rollback is not visible | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7d tx: a failing statement rolls the transaction back | pass | xe: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @electric-sql/pglite@0.2.17 | node 26.11.1 | PGlite (in-process) | db.transaction(tx) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2b one: the row | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2b one: the row | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2b one: the row | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2b one: the row | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7a tx: write then commit is visible | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7b tx: write then rollback is not visible | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7c tx: savepoint rollback keeps the outer write | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7d tx: a failing statement rolls the transaction back | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2b one: the row | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S6a syntax error rejects with the driver's error | pass | NeonDbError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S6b unique violation rejects with the driver's error | pass | NeonDbError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2b one: the row | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S6a syntax error rejects with the driver's error | pass | NeonDbError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S6b unique violation rejects with the driver's error | pass | NeonDbError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@1.1.0 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2b one: the row | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2b one: the row | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2b one: the row | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2b one: the row | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7a tx: write then commit is visible | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7b tx: write then rollback is not visible | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7c tx: savepoint rollback keeps the outer write | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7d tx: a failing statement rolls the transaction back | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called the client as a function (or a method the harness could not observe); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2b one: the row | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S6a syntax error rejects with the driver's error | pass | NeonDbError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S6b unique violation rejects with the driver's error | pass | NeonDbError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called the client as a function (or a method the harness could not observe); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2b one: the row | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S6a syntax error rejects with the driver's error | pass | NeonDbError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S6b unique violation rejects with the driver's error | pass | NeonDbError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.10.4 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2b one: the row | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Pool (WebSocket) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2b one: the row | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | Client (WebSocket, connected) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2b one: the row | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient (pool.connect()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2b one: the row | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7a tx: write then commit is visible | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7b tx: write then rollback is not visible | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7c tx: savepoint rollback keeps the outer write | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7d tx: a failing statement rolls the transaction back | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | PoolClient in BEGIN/COMMIT | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called the client as a function (or a method the harness could not observe); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2b one: the row | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S6a syntax error rejects with the driver's error | pass | NeonDbError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S6b unique violation rejects with the driver's error | pass | NeonDbError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon() HTTP function | S8 20 concurrent queries resolve with the right rows | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called the client as a function (or a method the harness could not observe); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1a params: int, text, bool, null | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1b omitted param (undefined) binds NULL | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2a many: 3 rows | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2b one: the row | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2c maybeOne: zero rows gives null | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S2e with(client)(params) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S3 write without RETURNING (none), then a separate read | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S3n write without parameters (none), then a separate read | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S4 INSERT ... RETURNING | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S5a repeated :id (runtime lexer) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S5b repeated :id (mapper text/order) | pass |  |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S6a syntax error rejects with the driver's error | pass | NeonDbError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S6b unique violation rejects with the driver's error | pass | NeonDbError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| @neondatabase/serverless@0.9.5 | node 26.11.1 | Postgres 14 via local Neon proxy | neon(url, { fullResults: true }) | S8 20 concurrent queries resolve with the right rows | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S1a params: int, text, bool, null | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S1b omitted param (undefined) binds NULL | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S1d array params: = any(:ids) over an int and a text array | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S2a many: 3 rows | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S2b one: the row | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S2c maybeOne: zero rows gives null | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S2d maybeOne: two rows throw reylo's error | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S2e with(client)(params) | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S3 write without RETURNING (none), then a separate read | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S3n write without parameters (none), then a separate read | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S4 INSERT ... RETURNING | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S5a repeated :id (runtime lexer) | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S5b repeated :id (mapper text/order) | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db | S8 20 concurrent queries resolve with the right rows | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S1a params: int, text, bool, null | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S1b omitted param (undefined) binds NULL | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S2a many: 3 rows | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S2b one: the row | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S2c maybeOne: zero rows gives null | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S2d maybeOne: two rows throw reylo's error | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S2e with(client)(params) | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S3 write without RETURNING (none), then a separate read | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S3n write without parameters (none), then a separate read | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S4 INSERT ... RETURNING | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S5a repeated :id (runtime lexer) | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S5b repeated :id (mapper text/order) | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.task(t) | S8 20 concurrent queries resolve with the right rows | pass | warning: DeprecationWarning: Calling client.query() when the client is already executing a query is deprecated and will be removed in pg@9.0. Use async/await or an external async flow control mechanism instead. |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S1a params: int, text, bool, null | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S1b omitted param (undefined) binds NULL | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S2a many: 3 rows | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S2b one: the row | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S2c maybeOne: zero rows gives null | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S2d maybeOne: two rows throw reylo's error | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S2e with(client)(params) | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S3 write without RETURNING (none), then a separate read | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S3n write without parameters (none), then a separate read | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S4 INSERT ... RETURNING | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S5a repeated :id (runtime lexer) | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S5b repeated :id (mapper text/order) | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR] |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S7a tx: write then commit is visible | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S7b tx: write then rollback is not visible | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S7d tx: a failing statement rolls the transaction back | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | db.tx(t) | S8 20 concurrent queries resolve with the right rows | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it; then the enclosing transaction failed, as the database aborts it after an error: DatabaseError: current transaction is aborted, commands ignored until end of transaction block [code=25P02 severity=ERROR] |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S1a params: int, text, bool, null | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S1b omitted param (undefined) binds NULL | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S2a many: 3 rows | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S2b one: the row | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S2c maybeOne: zero rows gives null | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S2d maybeOne: two rows throw reylo's error | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S2e with(client)(params) | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S3 write without RETURNING (none), then a separate read | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S3n write without parameters (none), then a separate read | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S4 INSERT ... RETURNING | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S5a repeated :id (runtime lexer) | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S5b repeated :id (mapper text/order) | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S6a syntax error rejects with the driver's error | pass | DatabaseError: syntax error at or near "selec" [code=42601 severity=ERROR]; then the enclosing transaction failed, as the database aborts it after an error: DatabaseError: current transaction is aborted, commands ignored until end of transaction block [code=25P02 severity=ERROR] |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S6b unique violation rejects with the driver's error | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR]; then the enclosing transaction failed, as the database aborts it after an error: DatabaseError: current transaction is aborted, commands ignored until end of transaction block [code=25P02 severity=ERROR] |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S7a tx: write then commit is visible | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S7b tx: write then rollback is not visible | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S7d tx: a failing statement rolls the transaction back | pass | DatabaseError: duplicate key value violates unique constraint "uniq_pkey" [code=23505 severity=ERROR] |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| pg-promise@12.7.1 | node 26.11.1 | Postgres 14 (local) | t.tx(t2) (savepoint) | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S1a params: int, text, bool, null | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S1d array params: = any(:ids) over an int and a text array | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S2a many: 3 rows | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S2b one: the row | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S2e with(client)(params) | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S4 INSERT ... RETURNING | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S6a syntax error rejects with the driver's error | pass | PostgresError: syntax error at or near "selec" [code=ERR_POSTGRES_SYNTAX_ERROR errno=42601 severity=ERROR] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S6b unique violation rejects with the driver's error | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=ERR_POSTGRES_SERVER_ERROR errno=23505 severity=ERROR] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it; then the enclosing transaction failed, as the database aborts it after an error: PostgresError: COMMIT was rolled back by the server because the transaction was already aborted [code=ERR_POSTGRES_COMMIT_ROLLED_BACK] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S1a params: int, text, bool, null | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S2a many: 3 rows | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S2b one: the row | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S2e with(client)(params) | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S4 INSERT ... RETURNING | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S6a syntax error rejects with the driver's error | pass | PostgresError: syntax error at or near "selec" [code=ERR_POSTGRES_SYNTAX_ERROR errno=42601 severity=ERROR]; then the enclosing transaction failed, as the database aborts it after an error: PostgresError: COMMIT was rolled back by the server because the transaction was already aborted [code=ERR_POSTGRES_COMMIT_ROLLED_BACK] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S6b unique violation rejects with the driver's error | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=ERR_POSTGRES_SERVER_ERROR errno=23505 severity=ERROR]; then the enclosing transaction failed, as the database aborts it after an error: PostgresError: COMMIT was rolled back by the server because the transaction was already aborted [code=ERR_POSTGRES_COMMIT_ROLLED_BACK] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S7a tx: write then commit is visible | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S7b tx: write then rollback is not visible | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S7d tx: a failing statement rolls the transaction back | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=ERR_POSTGRES_SERVER_ERROR errno=23505 severity=ERROR] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.begin(tx) | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it; then the enclosing transaction failed, as the database aborts it after an error: PostgresError: current transaction is aborted, commands ignored until end of transaction block [code=ERR_POSTGRES_SERVER_ERROR errno=25P02 severity=ERROR] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S1a params: int, text, bool, null | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S1d array params: = any(:ids) over an int and a text array | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S2a many: 3 rows | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S2b one: the row | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S2e with(client)(params) | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S4 INSERT ... RETURNING | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S6a syntax error rejects with the driver's error | pass | PostgresError: syntax error at or near "selec" [code=ERR_POSTGRES_SYNTAX_ERROR errno=42601 severity=ERROR]; then the enclosing transaction failed, as the database aborts it after an error: PostgresError: current transaction is aborted, commands ignored until end of transaction block [code=ERR_POSTGRES_SERVER_ERROR errno=25P02 severity=ERROR] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S6b unique violation rejects with the driver's error | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=ERR_POSTGRES_SERVER_ERROR errno=23505 severity=ERROR]; then the enclosing transaction failed, as the database aborts it after an error: PostgresError: current transaction is aborted, commands ignored until end of transaction block [code=ERR_POSTGRES_SERVER_ERROR errno=25P02 severity=ERROR] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S7a tx: write then commit is visible | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S7b tx: write then rollback is not visible | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S7d tx: a failing statement rolls the transaction back | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=ERR_POSTGRES_SERVER_ERROR errno=23505 severity=ERROR] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | tx.savepoint(sp) | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S1a params: int, text, bool, null | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S1d array params: = any(:ids) over an int and a text array | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S1e JSON params: an array bound to :a::jsonb is stored as that JSON array | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S1f array params: bool[], timestamptz[] and bytea[] | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S2a many: 3 rows | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S2b one: the row | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S2e with(client)(params) | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S4 INSERT ... RETURNING | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S6a syntax error rejects with the driver's error | pass | PostgresError: syntax error at or near "selec" [code=ERR_POSTGRES_SYNTAX_ERROR errno=42601 severity=ERROR] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S6b unique violation rejects with the driver's error | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=ERR_POSTGRES_SERVER_ERROR errno=23505 severity=ERROR] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S7a tx: write then commit is visible | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S7b tx: write then rollback is not visible | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S7d tx: a failing statement rolls the transaction back | pass | PostgresError: duplicate key value violates unique constraint "uniq_pkey" [code=ERR_POSTGRES_SERVER_ERROR errno=23505 severity=ERROR] |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| Bun.sql (postgres)@1.4.3 | bun 1.4.3 | Postgres 14 (local) | sql.reserve() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise Connection | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7a tx: write then commit is visible | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7b tx: write then rollback is not visible | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7d tx: a failing statement rolls the transaction back | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise Connection | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7a tx: write then commit is visible | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7b tx: write then rollback is not visible | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7d tx: a failing statement rolls the transaction back | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2b one: the row | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.24.5 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7a tx: write then commit is visible | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7b tx: write then rollback is not visible | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7d tx: a failing statement rolls the transaction back | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7a tx: write then commit is visible | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7b tx: write then rollback is not visible | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7d tx: a failing statement rolls the transaction back | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2b one: the row | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@3.15.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise Connection | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection (getConnection()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7a tx: write then commit is visible | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7b tx: write then rollback is not visible | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7d tx: a failing statement rolls the transaction back | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | promise PoolConnection in beginTransaction() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Pool (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback Connection (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.4 | callback PoolConnection (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise Connection | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection (getConnection()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7a tx: write then commit is visible | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7b tx: write then rollback is not visible | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7d tx: a failing statement rolls the transaction back | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | promise PoolConnection in beginTransaction() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Pool (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback Connection (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called promise() then execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1a params: int, text, bool, null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1b omitted param (undefined) binds NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2a many: 3 rows | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2b one: the row | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2c maybeOne: zero rows gives null | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S2e with(client)(params) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S3n write without parameters (none), then a separate read | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S5a repeated :id (runtime lexer) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S5b repeated :id (mapper text/order) | pass |  |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S6a syntax error rejects with the driver's error | pass | Error: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S6b unique violation rejects with the driver's error | pass | Error: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mysql2@2.3.3 | node 26.11.1 | MySQL 8.0 | callback PoolConnection (no .promise()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S6a syntax error rejects with the driver's error | pass | we: (conn:223, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S6b unique violation rejects with the driver's error | pass | we: (conn:224, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S6a syntax error rejects with the driver's error | pass | we: (conn:226, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S6b unique violation rejects with the driver's error | pass | we: (conn:226, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | Connection | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S6a syntax error rejects with the driver's error | pass | we: (conn:225, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S6b unique violation rejects with the driver's error | pass | we: (conn:222, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S6a syntax error rejects with the driver's error | pass | we: (conn:214, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S6b unique violation rejects with the driver's error | pass | we: (conn:211, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7a tx: write then commit is visible | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7b tx: write then rollback is not visible | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7d tx: a failing statement rolls the transaction back | pass | we: (conn:224, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | we: (conn:227, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | we: (conn:227, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | we: (conn:215, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | we: (conn:217, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | we: (conn:215, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | we: (conn:217, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7a tx: write then commit is visible | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7b tx: write then rollback is not visible | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7d tx: a failing statement rolls the transaction back | pass | we: (conn:215, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S6a syntax error rejects with the driver's error | pass | we: (conn:884, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S6b unique violation rejects with the driver's error | pass | we: (conn:874, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S6a syntax error rejects with the driver's error | pass | we: (conn:887, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S6b unique violation rejects with the driver's error | pass | we: (conn:887, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | Connection | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S6a syntax error rejects with the driver's error | pass | we: (conn:883, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S6b unique violation rejects with the driver's error | pass | we: (conn:877, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S6a syntax error rejects with the driver's error | pass | we: (conn:879, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S6b unique violation rejects with the driver's error | pass | we: (conn:882, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7a tx: write then commit is visible | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7b tx: write then rollback is not visible | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7d tx: a failing statement rolls the transaction back | pass | we: (conn:885, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | we: (conn:888, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | we: (conn:888, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | we: (conn:876, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | we: (conn:878, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | we: (conn:878, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | we: (conn:873, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7a tx: write then commit is visible | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7b tx: write then rollback is not visible | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7d tx: a failing statement rolls the transaction back | pass | we: (conn:878, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mariadb@3.5.4 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:240, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:233, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:243, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:243, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | Connection | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:242, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:238, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:241, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:231, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7a tx: write then commit is visible | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7b tx: write then rollback is not visible | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7d tx: a failing statement rolls the transaction back | pass | SqlError: (conn:235, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:244, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:244, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:232, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:234, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:232, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:236, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7a tx: write then commit is visible | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7b tx: write then rollback is not visible | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7d tx: a failing statement rolls the transaction back | pass | SqlError: (conn:232, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:889, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:900, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:904, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:904, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | Connection | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:889, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:891, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:903, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:897, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7a tx: write then commit is visible | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7b tx: write then rollback is not visible | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7d tx: a failing statement rolls the transaction back | pass | SqlError: (conn:900, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:905, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:905, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:894, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:896, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn:896, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn:898, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7a tx: write then commit is visible | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7b tx: write then rollback is not visible | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7d tx: a failing statement rolls the transaction back | pass | SqlError: (conn:896, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mariadb@3.4.5 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=258, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=255, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=260, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=260, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | Connection | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=259, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=256, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection (getConnection()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=250, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=257, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7a tx: write then commit is visible | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7b tx: write then rollback is not visible | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7d tx: a failing statement rolls the transaction back | pass | SqlError: (conn=246, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | PoolConnection in beginTransaction() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=261, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=261, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Connection (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=249, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=251, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback Pool (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=245, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MariaDB server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=249, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7a tx: write then commit is visible | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7b tx: write then rollback is not visible | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7d tx: a failing statement rolls the transaction back | pass | SqlError: (conn=245, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MariaDB 11 | callback PoolConnection in beginTransaction() (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=917, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=921, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Pool | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=926, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=926, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | Connection | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=930, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=919, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection (getConnection()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=917, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=929, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7a tx: write then commit is visible | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7b tx: write then rollback is not visible | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7d tx: a failing statement rolls the transaction back | pass | SqlError: (conn=928, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | PoolConnection in beginTransaction() | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=932, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=932, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Connection (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=920, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=922, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback Pool (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called query(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1a params: int, text, bool, null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1b omitted param (undefined) binds NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2a many: 3 rows | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2b one: the row | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2c maybeOne: zero rows gives null | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2d maybeOne: two rows throw reylo's error | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S2e with(client)(params) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S3 write without RETURNING (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S3n write without parameters (none), then a separate read | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S5a repeated :id (runtime lexer) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S5b repeated :id (mapper text/order) | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S6a syntax error rejects with the driver's error | pass | SqlError: (conn=922, no: 1064, SQLState: 42000) You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 sql: selec 1 - parameters:[] [code=ER_PARSE_ERROR errno=1064 sqlState=42000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S6b unique violation rejects with the driver's error | pass | SqlError: (conn=924, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7a tx: write then commit is visible | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7b tx: write then rollback is not visible | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7d tx: a failing statement rolls the transaction back | pass | SqlError: (conn=922, no: 1062, SQLState: 23000) Duplicate entry 'taken' for key 'uniq.PRIMARY' sql: insert into driver_matrix.uniq (k) values (?) - parameters:['taken'] [code=ER_DUP_ENTRY errno=1062 sqlState=23000] |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| mariadb@2.5.6 | node 26.11.1 | MySQL 8.4 | callback PoolConnection in beginTransaction() (mariadb/callback) | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S1a params: int, text, bool, null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S2a many: 3 rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S2b one: the row | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S2e with(client)(params) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S6a syntax error rejects with the driver's error | pass | MySQLError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ERR_MYSQL_SYNTAX_ERROR errno=1064 sqlState=42000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S6b unique violation rejects with the driver's error | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S1a params: int, text, bool, null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S2a many: 3 rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S2b one: the row | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S2e with(client)(params) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S6a syntax error rejects with the driver's error | pass | MySQLError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ERR_MYSQL_SYNTAX_ERROR errno=1064 sqlState=42000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S6b unique violation rejects with the driver's error | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S7a tx: write then commit is visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S7b tx: write then rollback is not visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S7d tx: a failing statement rolls the transaction back | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.begin(tx) | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S1a params: int, text, bool, null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S2a many: 3 rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S2b one: the row | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S2e with(client)(params) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S6a syntax error rejects with the driver's error | pass | MySQLError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ERR_MYSQL_SYNTAX_ERROR errno=1064 sqlState=42000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S6b unique violation rejects with the driver's error | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S7a tx: write then commit is visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S7b tx: write then rollback is not visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S7d tx: a failing statement rolls the transaction back | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | tx.savepoint(sp) | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S1a params: int, text, bool, null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S2a many: 3 rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S2b one: the row | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S2e with(client)(params) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S6a syntax error rejects with the driver's error | pass | MySQLError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ERR_MYSQL_SYNTAX_ERROR errno=1064 sqlState=42000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S6b unique violation rejects with the driver's error | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S7a tx: write then commit is visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S7b tx: write then rollback is not visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S7d tx: a failing statement rolls the transaction back | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.4 | sql.reserve() | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S1a params: int, text, bool, null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S2a many: 3 rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S2b one: the row | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S2e with(client)(params) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S6a syntax error rejects with the driver's error | pass | MySQLError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ERR_MYSQL_SYNTAX_ERROR errno=1064 sqlState=42000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S6b unique violation rejects with the driver's error | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S1a params: int, text, bool, null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S2a many: 3 rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S2b one: the row | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S2e with(client)(params) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S6a syntax error rejects with the driver's error | pass | MySQLError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ERR_MYSQL_SYNTAX_ERROR errno=1064 sqlState=42000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S6b unique violation rejects with the driver's error | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S7a tx: write then commit is visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S7b tx: write then rollback is not visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S7d tx: a failing statement rolls the transaction back | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.begin(tx) | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S1a params: int, text, bool, null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S2a many: 3 rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S2b one: the row | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S2e with(client)(params) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S6a syntax error rejects with the driver's error | pass | MySQLError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ERR_MYSQL_SYNTAX_ERROR errno=1064 sqlState=42000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S6b unique violation rejects with the driver's error | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S7a tx: write then commit is visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S7b tx: write then rollback is not visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S7d tx: a failing statement rolls the transaction back | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | tx.savepoint(sp) | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S1a params: int, text, bool, null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S2a many: 3 rows | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S2b one: the row | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S2e with(client)(params) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S6a syntax error rejects with the driver's error | pass | MySQLError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 [code=ERR_MYSQL_SYNTAX_ERROR errno=1064 sqlState=42000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S6b unique violation rejects with the driver's error | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S7a tx: write then commit is visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S7b tx: write then rollback is not visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S7c tx: savepoint rollback keeps the outer write | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S7d tx: a failing statement rolls the transaction back | pass | MySQLError: Duplicate entry 'taken' for key 'uniq.PRIMARY' [code=ERR_MYSQL_SERVER_ERROR errno=1062 sqlState=23000] |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S7n tx: a write without parameters, then commit, is visible | pass |  |
| Bun.sql (mysql)@1.4.3 | bun 1.4.3 | MySQL 8.0 | sql.reserve() | S8 20 concurrent queries resolve with the right rows | pass |  |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S1a params: int, text, bool, null | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S1b omitted param (undefined) binds NULL | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S2a many: 3 rows | pass |  |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S2b one: the row | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S2c maybeOne: zero rows gives null | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S2d maybeOne: two rows throw reylo's error | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S2e with(client)(params) | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S3 write without RETURNING (none), then a separate read | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S3n write without parameters (none), then a separate read | pass |  |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S5a repeated :id (runtime lexer) | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S5b repeated :id (mapper text/order) | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S6a syntax error rejects with the driver's error | pass | DatabaseError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 (errno 1064) (sqlstate 42000) during query: selec 1 [body.code=UNKNOWN] (matched by message: the error carries no code field) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S6b unique violation rejects with the driver's error | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S8 20 concurrent queries resolve with the right rows | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S1a params: int, text, bool, null | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S1b omitted param (undefined) binds NULL | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S2a many: 3 rows | pass |  |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S2b one: the row | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S2c maybeOne: zero rows gives null | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S2d maybeOne: two rows throw reylo's error | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S2e with(client)(params) | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S3 write without RETURNING (none), then a separate read | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S3n write without parameters (none), then a separate read | pass |  |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S5a repeated :id (runtime lexer) | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S5b repeated :id (mapper text/order) | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S6a syntax error rejects with the driver's error | pass | DatabaseError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 (errno 1064) (sqlstate 42000) during query: selec 1 [body.code=UNKNOWN] (matched by message: the error carries no code field) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S6b unique violation rejects with the driver's error | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S8 20 concurrent queries resolve with the right rows | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S1a params: int, text, bool, null | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S1b omitted param (undefined) binds NULL | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S2a many: 3 rows | pass |  |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S2b one: the row | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S2c maybeOne: zero rows gives null | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S2d maybeOne: two rows throw reylo's error | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S2e with(client)(params) | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S3 write without RETURNING (none), then a separate read | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S3n write without parameters (none), then a separate read | pass |  |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S5a repeated :id (runtime lexer) | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S5b repeated :id (mapper text/order) | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S6a syntax error rejects with the driver's error | pass | DatabaseError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 (errno 1064) (sqlstate 42000) during query: selec 1 [body.code=UNKNOWN] (matched by message: the error carries no code field) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S6b unique violation rejects with the driver's error | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S7a tx: write then commit is visible | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S7b tx: write then rollback is not visible | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S7c tx: savepoint rollback keeps the outer write | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S7d tx: a failing statement rolls the transaction back | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @planetscale/database@2.0.0 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S8 20 concurrent queries resolve with the right rows | UNSUPPORTED | reylo refused it: @planetscale/database 2.0 binds no parameters, so reylo cannot pass this statement's values to it; use its 1.x line, or mysql2 on PlanetScale's MySQL endpoint. (cause: Error: Query parameters are not supported: pass a complete SQL string and options to execute(query, options)) |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S1a params: int, text, bool, null | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S1b omitted param (undefined) binds NULL | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S2a many: 3 rows | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S2b one: the row | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S2c maybeOne: zero rows gives null | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S2d maybeOne: two rows throw reylo's error | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S2e with(client)(params) | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S3 write without RETURNING (none), then a separate read | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S3n write without parameters (none), then a separate read | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S5a repeated :id (runtime lexer) | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S5b repeated :id (mapper text/order) | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S6a syntax error rejects with the driver's error | pass | DatabaseError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 (errno 1064) (sqlstate 42000) during query: selec 1 [body.code=UNKNOWN] (matched by message: the error carries no code field) |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S6b unique violation rejects with the driver's error | pass | DatabaseError: Duplicate entry 'taken' for key 'uniq.PRIMARY' (errno 1062) (sqlstate 23000) during query: insert into driver_matrix.uniq (k) values ('taken') [body.code=UNKNOWN] (matched by message: the error carries no code field) |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | connect() Connection | S8 20 concurrent queries resolve with the right rows | FAIL | UnknownError: Expected JSON response from database API, got HTTP 403 Forbidden [body.code=UNKNOWN] |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S1a params: int, text, bool, null | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S1b omitted param (undefined) binds NULL | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S2a many: 3 rows | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S2b one: the row | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S2c maybeOne: zero rows gives null | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S2d maybeOne: two rows throw reylo's error | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S2e with(client)(params) | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S3 write without RETURNING (none), then a separate read | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S3n write without parameters (none), then a separate read | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S5a repeated :id (runtime lexer) | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S5b repeated :id (mapper text/order) | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S6a syntax error rejects with the driver's error | pass | DatabaseError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 (errno 1064) (sqlstate 42000) during query: selec 1 [body.code=UNKNOWN] (matched by message: the error carries no code field) |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S6b unique violation rejects with the driver's error | pass | DatabaseError: Duplicate entry 'taken' for key 'uniq.PRIMARY' (errno 1062) (sqlstate 23000) during query: insert into driver_matrix.uniq (k) values ('taken') [body.code=UNKNOWN] (matched by message: the error carries no code field) |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | Client | S8 20 concurrent queries resolve with the right rows | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S1a params: int, text, bool, null | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S1b omitted param (undefined) binds NULL | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S1g JSON params: a string, a number, a boolean, JSON text, an object and arrays are stored as their JSON, null as NULL | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S1h date and time params: a Date into timestamptz is its instant; timestamp and date take text, and a Date for them is refused before the driver is called | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S2a many: 3 rows | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S2b one: the row | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S2c maybeOne: zero rows gives null | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S2d maybeOne: two rows throw reylo's error | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S2e with(client)(params) | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S3 write without RETURNING (none), then a separate read | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S3n write without parameters (none), then a separate read | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S5a repeated :id (runtime lexer) | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S5b repeated :id (mapper text/order) | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S6a syntax error rejects with the driver's error | pass | DatabaseError: You have an error in your SQL syntax; check the manual that corresponds to your MySQL server version for the right syntax to use near 'selec 1' at line 1 (errno 1064) (sqlstate 42000) during query: selec 1 [body.code=UNKNOWN] (matched by message: the error carries no code field) |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S6b unique violation rejects with the driver's error | pass | DatabaseError: Duplicate entry 'taken' for key 'uniq.PRIMARY' (errno 1062) (sqlstate 23000) during query: insert into driver_matrix.uniq (k) values ('taken') [body.code=UNKNOWN] (matched by message: the error carries no code field) |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S7a tx: write then commit is visible | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S7b tx: write then rollback is not visible | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S7d tx: a failing statement rolls the transaction back | pass | DatabaseError: Duplicate entry 'taken' for key 'uniq.PRIMARY' (errno 1062) (sqlstate 23000) during query: insert into driver_matrix.uniq (k) values ('taken') [body.code=UNKNOWN] (matched by message: the error carries no code field) |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @planetscale/database@1.20.2 | node 26.11.1 | MySQL 8.4 via ps-http-sim | conn.transaction(tx) | S8 20 concurrent queries resolve with the right rows | FAIL | UnknownError: Expected JSON response from database API, got HTTP 403 Forbidden [body.code=UNKNOWN] |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called prepare(); run returned a promise and rejected on a failing statement; runSync returned the value and threw |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S1a params: int, text, bool, null | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S1b omitted param (undefined) binds NULL | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S2a many: 3 rows | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S2b one: the row | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S2c maybeOne: zero rows gives null | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S2d maybeOne: two rows throw reylo's error | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S2e with(client)(params) | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S3 write without RETURNING (none), then a separate read | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S3n write without parameters (none), then a separate read | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S4 INSERT ... RETURNING | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S5a repeated :id (runtime lexer) | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S5b repeated :id (mapper text/order) | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S6a syntax error rejects with the driver's error | pass | SqliteError: near "selec": syntax error [code=SQLITE_ERROR] |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S6b unique violation rejects with the driver's error | pass | SqliteError: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | Database | S8 20 concurrent queries resolve with the right rows | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | db.transaction(fn) | S7a tx: write then commit is visible | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | db.transaction(fn) | S7b tx: write then rollback is not visible | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | db.transaction(fn) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | db.transaction(fn) | S7d tx: a failing statement rolls the transaction back | pass | SqliteError: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| better-sqlite3@13.0.3 | node 26.11.1 | SQLite file | db.transaction(fn) | S7e tx: a read inside transaction(fn) is usable synchronously | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called prepare(); run returned a promise and rejected on a failing statement; runSync returned the value and threw |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S1a params: int, text, bool, null | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S1b omitted param (undefined) binds NULL | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S2a many: 3 rows | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S2b one: the row | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S2c maybeOne: zero rows gives null | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S2d maybeOne: two rows throw reylo's error | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S2e with(client)(params) | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S3 write without RETURNING (none), then a separate read | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S3n write without parameters (none), then a separate read | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S4 INSERT ... RETURNING | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S5a repeated :id (runtime lexer) | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S5b repeated :id (mapper text/order) | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S6a syntax error rejects with the driver's error | pass | SqliteError: near "selec": syntax error [code=SQLITE_ERROR] |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S6b unique violation rejects with the driver's error | pass | SqliteError: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | Database | S8 20 concurrent queries resolve with the right rows | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | db.transaction(fn) | S7a tx: write then commit is visible | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | db.transaction(fn) | S7b tx: write then rollback is not visible | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | db.transaction(fn) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | db.transaction(fn) | S7d tx: a failing statement rolls the transaction back | pass | SqliteError: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| better-sqlite3@12.11.1 | node 26.11.1 | SQLite file | db.transaction(fn) | S7e tx: a read inside transaction(fn) is usable synchronously | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called prepare(); run returned a promise and rejected on a failing statement; runSync returned the value and threw |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S1a params: int, text, bool, null | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S1b omitted param (undefined) binds NULL | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S2a many: 3 rows | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S2b one: the row | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S2c maybeOne: zero rows gives null | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S2d maybeOne: two rows throw reylo's error | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S2e with(client)(params) | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S3 write without RETURNING (none), then a separate read | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S3n write without parameters (none), then a separate read | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S4 INSERT ... RETURNING | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S5a repeated :id (runtime lexer) | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S5b repeated :id (mapper text/order) | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S6a syntax error rejects with the driver's error | pass | SqliteError: near "selec": syntax error [code=SQLITE_ERROR] |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S6b unique violation rejects with the driver's error | pass | SqliteError: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | Database | S8 20 concurrent queries resolve with the right rows | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | db.transaction(fn) | S7a tx: write then commit is visible | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | db.transaction(fn) | S7b tx: write then rollback is not visible | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | db.transaction(fn) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | db.transaction(fn) | S7d tx: a failing statement rolls the transaction back | pass | SqliteError: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| better-sqlite3@11.10.0 | node 24.21.0 | SQLite file | db.transaction(fn) | S7e tx: a read inside transaction(fn) is usable synchronously | pass |  |
| better-sqlite3@9.6.0 | node 24.19.0 | SQLite file | (job) | setup | NOT RUN | no prebuilt binary for Node 24.19.0 (ABI 137), and native-old/ has no source build of it against that Node's headers (see README) |
| better-sqlite3@8.7.0 | node 24.19.0 | SQLite file | (job) | setup | NOT RUN | no prebuilt binary for Node 24.19.0 (ABI 137), and native-old/ has no source build of it against that Node's headers (see README) |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called prepare(); run returned a promise and rejected on a failing statement; runSync returned the value and threw |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S1a params: int, text, bool, null | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S1b omitted param (undefined) binds NULL | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S2a many: 3 rows | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S2b one: the row | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S2c maybeOne: zero rows gives null | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S2d maybeOne: two rows throw reylo's error | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S2e with(client)(params) | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S3 write without RETURNING (none), then a separate read | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S3n write without parameters (none), then a separate read | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S4 INSERT ... RETURNING | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S5a repeated :id (runtime lexer) | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S5b repeated :id (mapper text/order) | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S6a syntax error rejects with the driver's error | pass | Error: near "selec": syntax error [code=ERR_SQLITE_ERROR errcode=1 errstr=SQL logic error] |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S6b unique violation rejects with the driver's error | pass | Error: UNIQUE constraint failed: uniq.k [code=ERR_SQLITE_ERROR errcode=1555 errstr=constraint failed] |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync | S8 20 concurrent queries resolve with the right rows | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called prepare(); run returned a promise and rejected on a failing statement; runSync returned the value and threw |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S1a params: int, text, bool, null | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S1b omitted param (undefined) binds NULL | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S2a many: 3 rows | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S2b one: the row | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S2c maybeOne: zero rows gives null | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S2d maybeOne: two rows throw reylo's error | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S2e with(client)(params) | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S3 write without RETURNING (none), then a separate read | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S3n write without parameters (none), then a separate read | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S4 INSERT ... RETURNING | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S5a repeated :id (runtime lexer) | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S5b repeated :id (mapper text/order) | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S6a syntax error rejects with the driver's error | pass | Error: near "selec": syntax error [code=ERR_SQLITE_ERROR errcode=1 errstr=SQL logic error] |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S6b unique violation rejects with the driver's error | pass | Error: UNIQUE constraint failed: uniq.k [code=ERR_SQLITE_ERROR errcode=1555 errstr=constraint failed] |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S7a tx: write then commit is visible | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S7b tx: write then rollback is not visible | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S7d tx: a failing statement rolls the transaction back | pass | Error: UNIQUE constraint failed: uniq.k [code=ERR_SQLITE_ERROR errcode=1555 errstr=constraint failed] |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S8 20 concurrent queries resolve with the right rows | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore() | S9 unsupported client: run() rejects with reylo's TypeError | UNSUPPORTED | reylo refused it: reylo cannot run a sqlite statement on an object with get, all, iterate, run, clear. It takes a client with prepare(sql) as on better-sqlite3, node:sqlite, bun:sqlite and Cloudflare D1; execute({ sql, args }) as on @libsql/client; all(sql, params) as on sqlite3 and sqlite; or unsafe(text, values) as on Bun.sql. A tag store is not a client: pass its database, store.db. |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called prepare(); run returned a promise and rejected on a failing statement; runSync returned the value and threw |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S1a params: int, text, bool, null | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S1b omitted param (undefined) binds NULL | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S2a many: 3 rows | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S2b one: the row | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S2c maybeOne: zero rows gives null | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S2d maybeOne: two rows throw reylo's error | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S2e with(client)(params) | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S3 write without RETURNING (none), then a separate read | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S3n write without parameters (none), then a separate read | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S4 INSERT ... RETURNING | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S5a repeated :id (runtime lexer) | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S5b repeated :id (mapper text/order) | pass |  |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S6a syntax error rejects with the driver's error | pass | Error: near "selec": syntax error [code=ERR_SQLITE_ERROR errcode=1 errstr=SQL logic error] |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S6b unique violation rejects with the driver's error | pass | Error: UNIQUE constraint failed: uniq.k [code=ERR_SQLITE_ERROR errcode=1555 errstr=constraint failed] |
| node:sqlite@26.11.1 | node 26.11.1 | SQLite file | db.createTagStore().db | S8 20 concurrent queries resolve with the right rows | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called prepare(); run returned a promise and rejected on a failing statement; runSync returned the value and threw |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S1a params: int, text, bool, null | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S1b omitted param (undefined) binds NULL | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S2a many: 3 rows | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S2b one: the row | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S2c maybeOne: zero rows gives null | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S2d maybeOne: two rows throw reylo's error | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S2e with(client)(params) | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S3 write without RETURNING (none), then a separate read | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S3n write without parameters (none), then a separate read | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S4 INSERT ... RETURNING | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S5a repeated :id (runtime lexer) | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S5b repeated :id (mapper text/order) | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S6a syntax error rejects with the driver's error | pass | Error: near "selec": syntax error [code=ERR_SQLITE_ERROR errcode=1 errstr=SQL logic error] |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S6b unique violation rejects with the driver's error | pass | Error: UNIQUE constraint failed: uniq.k [code=ERR_SQLITE_ERROR errcode=1555 errstr=constraint failed] |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync | S8 20 concurrent queries resolve with the right rows | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called prepare(); run returned a promise and rejected on a failing statement; runSync returned the value and threw |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S1a params: int, text, bool, null | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S1b omitted param (undefined) binds NULL | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S2a many: 3 rows | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S2b one: the row | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S2c maybeOne: zero rows gives null | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S2d maybeOne: two rows throw reylo's error | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S2e with(client)(params) | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S3 write without RETURNING (none), then a separate read | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S3n write without parameters (none), then a separate read | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S4 INSERT ... RETURNING | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S5a repeated :id (runtime lexer) | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S5b repeated :id (mapper text/order) | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S6a syntax error rejects with the driver's error | pass | Error: near "selec": syntax error [code=ERR_SQLITE_ERROR errcode=1 errstr=SQL logic error] |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S6b unique violation rejects with the driver's error | pass | Error: UNIQUE constraint failed: uniq.k [code=ERR_SQLITE_ERROR errcode=1555 errstr=constraint failed] |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S7a tx: write then commit is visible | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S7b tx: write then rollback is not visible | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S7d tx: a failing statement rolls the transaction back | pass | Error: UNIQUE constraint failed: uniq.k [code=ERR_SQLITE_ERROR errcode=1555 errstr=constraint failed] |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | DatabaseSync in BEGIN/COMMIT (exec) | S8 20 concurrent queries resolve with the right rows | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore() | S9 unsupported client: run() rejects with reylo's TypeError | UNSUPPORTED | reylo refused it: reylo cannot run a sqlite statement on an object with get, all, iterate, run, clear. It takes a client with prepare(sql) as on better-sqlite3, node:sqlite, bun:sqlite and Cloudflare D1; execute({ sql, args }) as on @libsql/client; all(sql, params) as on sqlite3 and sqlite; or unsafe(text, values) as on Bun.sql. A tag store is not a client: pass its database, store.db. |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called prepare(); run returned a promise and rejected on a failing statement; runSync returned the value and threw |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S1a params: int, text, bool, null | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S1b omitted param (undefined) binds NULL | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S2a many: 3 rows | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S2b one: the row | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S2c maybeOne: zero rows gives null | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S2d maybeOne: two rows throw reylo's error | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S2e with(client)(params) | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S3 write without RETURNING (none), then a separate read | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S3n write without parameters (none), then a separate read | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S4 INSERT ... RETURNING | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S5a repeated :id (runtime lexer) | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S5b repeated :id (mapper text/order) | pass |  |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S6a syntax error rejects with the driver's error | pass | Error: near "selec": syntax error [code=ERR_SQLITE_ERROR errcode=1 errstr=SQL logic error] |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S6b unique violation rejects with the driver's error | pass | Error: UNIQUE constraint failed: uniq.k [code=ERR_SQLITE_ERROR errcode=1555 errstr=constraint failed] |
| node:sqlite@24.21.0 | node 24.21.0 | SQLite file | db.createTagStore().db | S8 20 concurrent queries resolve with the right rows | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called prepare(); run returned a promise and rejected on a failing statement; runSync returned the value and threw |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S1a params: int, text, bool, null | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S1b omitted param (undefined) binds NULL | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S2a many: 3 rows | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S2b one: the row | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S2c maybeOne: zero rows gives null | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S2d maybeOne: two rows throw reylo's error | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S2e with(client)(params) | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S3 write without RETURNING (none), then a separate read | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S3n write without parameters (none), then a separate read | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S4 INSERT ... RETURNING | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S5a repeated :id (runtime lexer) | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S5b repeated :id (mapper text/order) | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S6a syntax error rejects with the driver's error | pass | Error: near "selec": syntax error [code=SQLITE_ERROR errno=1] |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S6b unique violation rejects with the driver's error | pass | Error: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY errno=1555] |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | Database | S8 20 concurrent queries resolve with the right rows | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | db.transaction(fn) | S7a tx: write then commit is visible | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | db.transaction(fn) | S7b tx: write then rollback is not visible | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | db.transaction(fn) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | db.transaction(fn) | S7d tx: a failing statement rolls the transaction back | pass | Error: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY errno=1555] |
| bun:sqlite@1.4.3 | bun 1.4.3 | SQLite file | db.transaction(fn) | S7e tx: a read inside transaction(fn) is usable synchronously | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S1a params: int, text, bool, null | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S2a many: 3 rows | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S2b one: the row | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S2e with(client)(params) | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S4 INSERT ... RETURNING | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S6a syntax error rejects with the driver's error | pass | SQLiteError: near "selec": syntax error [code=SQLITE_ERROR errno=1] |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S6b unique violation rejects with the driver's error | pass | SQLiteError: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY errno=1555] |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S1a params: int, text, bool, null | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S2a many: 3 rows | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S2b one: the row | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S2e with(client)(params) | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S4 INSERT ... RETURNING | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S6a syntax error rejects with the driver's error | pass | SQLiteError: near "selec": syntax error [code=SQLITE_ERROR errno=1] |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S6b unique violation rejects with the driver's error | pass | SQLiteError: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY errno=1555] |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S7a tx: write then commit is visible | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S7b tx: write then rollback is not visible | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S7d tx: a failing statement rolls the transaction back | pass | SQLiteError: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY errno=1555] |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | sql.begin(tx) | S8 20 concurrent queries resolve with the right rows | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called unsafe(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S1a params: int, text, bool, null | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S1b omitted param (undefined) binds NULL | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S2a many: 3 rows | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S2b one: the row | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S2c maybeOne: zero rows gives null | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S2d maybeOne: two rows throw reylo's error | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S2e with(client)(params) | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S3 write without RETURNING (none), then a separate read | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S3n write without parameters (none), then a separate read | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S4 INSERT ... RETURNING | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S5a repeated :id (runtime lexer) | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S5b repeated :id (mapper text/order) | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S6a syntax error rejects with the driver's error | pass | SQLiteError: near "selec": syntax error [code=SQLITE_ERROR errno=1] |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S6b unique violation rejects with the driver's error | pass | SQLiteError: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY errno=1555] |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S7a tx: write then commit is visible | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S7b tx: write then rollback is not visible | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S7d tx: a failing statement rolls the transaction back | pass | SQLiteError: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY errno=1555] |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| Bun.sql (sqlite)@1.4.3 | bun 1.4.3 | SQLite file | tx.savepoint(sp) | S8 20 concurrent queries resolve with the right rows | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S1a params: int, text, bool, null | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S1b omitted param (undefined) binds NULL | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S2a many: 3 rows | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S2b one: the row | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S2c maybeOne: zero rows gives null | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S2d maybeOne: two rows throw reylo's error | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S2e with(client)(params) | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S3 write without RETURNING (none), then a separate read | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S3n write without parameters (none), then a separate read | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S4 INSERT ... RETURNING | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S5a repeated :id (runtime lexer) | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S5b repeated :id (mapper text/order) | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S6a syntax error rejects with the driver's error | pass | LibsqlError: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR rawCode=1 extendedCode=SQLITE_ERROR cause.code=SQLITE_ERROR] |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S6b unique violation rejects with the driver's error | pass | LibsqlError: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT rawCode=1555 extendedCode=SQLITE_CONSTRAINT_PRIMARYKEY cause.code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | Client | S8 20 concurrent queries resolve with the right rows | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S1a params: int, text, bool, null | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S1b omitted param (undefined) binds NULL | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2a many: 3 rows | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2b one: the row | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2c maybeOne: zero rows gives null | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2d maybeOne: two rows throw reylo's error | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2e with(client)(params) | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S3 write without RETURNING (none), then a separate read | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S3n write without parameters (none), then a separate read | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S4 INSERT ... RETURNING | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S5a repeated :id (runtime lexer) | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S5b repeated :id (mapper text/order) | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S6a syntax error rejects with the driver's error | pass | LibsqlError: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR rawCode=1 extendedCode=SQLITE_ERROR cause.code=SQLITE_ERROR] |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S6b unique violation rejects with the driver's error | pass | LibsqlError: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT rawCode=1555 extendedCode=SQLITE_CONSTRAINT_PRIMARYKEY cause.code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7a tx: write then commit is visible | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7b tx: write then rollback is not visible | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7c tx: savepoint rollback keeps the outer write | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7d tx: a failing statement rolls the transaction back | pass | LibsqlError: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT rawCode=1555 extendedCode=SQLITE_CONSTRAINT_PRIMARYKEY cause.code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @libsql/client@0.17.4 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S8 20 concurrent queries resolve with the right rows | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S1a params: int, text, bool, null | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S1b omitted param (undefined) binds NULL | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S2a many: 3 rows | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S2b one: the row | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S2c maybeOne: zero rows gives null | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S2d maybeOne: two rows throw reylo's error | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S2e with(client)(params) | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S3 write without RETURNING (none), then a separate read | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S3n write without parameters (none), then a separate read | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S4 INSERT ... RETURNING | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S5a repeated :id (runtime lexer) | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S5b repeated :id (mapper text/order) | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S6a syntax error rejects with the driver's error | pass | LibsqlError: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR rawCode=1 extendedCode=SQLITE_ERROR cause.code=SQLITE_ERROR] |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S6b unique violation rejects with the driver's error | pass | LibsqlError: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT rawCode=1555 extendedCode=SQLITE_CONSTRAINT_PRIMARYKEY cause.code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | Client | S8 20 concurrent queries resolve with the right rows | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S1a params: int, text, bool, null | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S1b omitted param (undefined) binds NULL | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2a many: 3 rows | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2b one: the row | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2c maybeOne: zero rows gives null | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2d maybeOne: two rows throw reylo's error | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2e with(client)(params) | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S3 write without RETURNING (none), then a separate read | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S3n write without parameters (none), then a separate read | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S4 INSERT ... RETURNING | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S5a repeated :id (runtime lexer) | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S5b repeated :id (mapper text/order) | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S6a syntax error rejects with the driver's error | pass | LibsqlError: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR rawCode=1 extendedCode=SQLITE_ERROR cause.code=SQLITE_ERROR] |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S6b unique violation rejects with the driver's error | pass | LibsqlError: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT rawCode=1555 extendedCode=SQLITE_CONSTRAINT_PRIMARYKEY cause.code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7a tx: write then commit is visible | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7b tx: write then rollback is not visible | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7c tx: savepoint rollback keeps the outer write | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7d tx: a failing statement rolls the transaction back | pass | LibsqlError: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT rawCode=1555 extendedCode=SQLITE_CONSTRAINT_PRIMARYKEY cause.code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @libsql/client@0.18.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S8 20 concurrent queries resolve with the right rows | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S1a params: int, text, bool, null | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S1b omitted param (undefined) binds NULL | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S2a many: 3 rows | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S2b one: the row | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S2c maybeOne: zero rows gives null | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S2d maybeOne: two rows throw reylo's error | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S2e with(client)(params) | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S3 write without RETURNING (none), then a separate read | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S3n write without parameters (none), then a separate read | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S4 INSERT ... RETURNING | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S5a repeated :id (runtime lexer) | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S5b repeated :id (mapper text/order) | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S6a syntax error rejects with the driver's error | pass | LibsqlError: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR rawCode=1 cause.code=SQLITE_ERROR] |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S6b unique violation rejects with the driver's error | pass | LibsqlError: SQLITE_CONSTRAINT_PRIMARYKEY: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY rawCode=1555 cause.code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | Client | S8 20 concurrent queries resolve with the right rows | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S1a params: int, text, bool, null | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S1b omitted param (undefined) binds NULL | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2a many: 3 rows | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2b one: the row | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2c maybeOne: zero rows gives null | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2d maybeOne: two rows throw reylo's error | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2e with(client)(params) | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S3 write without RETURNING (none), then a separate read | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S3n write without parameters (none), then a separate read | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S4 INSERT ... RETURNING | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S5a repeated :id (runtime lexer) | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S5b repeated :id (mapper text/order) | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S6a syntax error rejects with the driver's error | pass | LibsqlError: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR rawCode=1 cause.code=SQLITE_ERROR] |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S6b unique violation rejects with the driver's error | pass | LibsqlError: SQLITE_CONSTRAINT_PRIMARYKEY: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY rawCode=1555 cause.code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7a tx: write then commit is visible | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7b tx: write then rollback is not visible | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7c tx: savepoint rollback keeps the outer write | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7d tx: a failing statement rolls the transaction back | pass | LibsqlError: SQLITE_CONSTRAINT_PRIMARYKEY: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY rawCode=1555 cause.code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @libsql/client@0.15.15 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S8 20 concurrent queries resolve with the right rows | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S1a params: int, text, bool, null | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S1b omitted param (undefined) binds NULL | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S2a many: 3 rows | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S2b one: the row | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S2c maybeOne: zero rows gives null | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S2d maybeOne: two rows throw reylo's error | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S2e with(client)(params) | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S3 write without RETURNING (none), then a separate read | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S3n write without parameters (none), then a separate read | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S4 INSERT ... RETURNING | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S5a repeated :id (runtime lexer) | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S5b repeated :id (mapper text/order) | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S6a syntax error rejects with the driver's error | pass | LibsqlError: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR rawCode=1 cause.code=SQLITE_ERROR] |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S6b unique violation rejects with the driver's error | pass | LibsqlError: SQLITE_CONSTRAINT_PRIMARYKEY: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY rawCode=1555 cause.code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | Client | S8 20 concurrent queries resolve with the right rows | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called execute(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S1a params: int, text, bool, null | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S1b omitted param (undefined) binds NULL | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2a many: 3 rows | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2b one: the row | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2c maybeOne: zero rows gives null | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2d maybeOne: two rows throw reylo's error | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S2e with(client)(params) | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S3 write without RETURNING (none), then a separate read | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S3n write without parameters (none), then a separate read | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S4 INSERT ... RETURNING | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S5a repeated :id (runtime lexer) | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S5b repeated :id (mapper text/order) | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S6a syntax error rejects with the driver's error | pass | LibsqlError: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR rawCode=1 cause.code=SQLITE_ERROR] |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S6b unique violation rejects with the driver's error | pass | LibsqlError: SQLITE_CONSTRAINT_PRIMARYKEY: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY rawCode=1555 cause.code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7a tx: write then commit is visible | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7b tx: write then rollback is not visible | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7c tx: savepoint rollback keeps the outer write | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7d tx: a failing statement rolls the transaction back | pass | LibsqlError: SQLITE_CONSTRAINT_PRIMARYKEY: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT_PRIMARYKEY rawCode=1555 cause.code=SQLITE_CONSTRAINT_PRIMARYKEY] |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S7n tx: a write without parameters, then commit, is visible | pass |  |
| @libsql/client@0.14.0 | node 26.11.1 | SQLite file (file: URL) | client.transaction("write") | S8 20 concurrent queries resolve with the right rows | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called all(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S1a params: int, text, bool, null | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S1b omitted param (undefined) binds NULL | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S2a many: 3 rows | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S2b one: the row | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S2c maybeOne: zero rows gives null | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S2d maybeOne: two rows throw reylo's error | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S2e with(client)(params) | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S3 write without RETURNING (none), then a separate read | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S3n write without parameters (none), then a separate read | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S4 INSERT ... RETURNING | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S5a repeated :id (runtime lexer) | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S5b repeated :id (mapper text/order) | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S6a syntax error rejects with the driver's error | pass | Error: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR errno=1] |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S6b unique violation rejects with the driver's error | pass | Error: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT errno=19] |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database | S8 20 concurrent queries resolve with the right rows | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called all(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S1a params: int, text, bool, null | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S1b omitted param (undefined) binds NULL | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S2a many: 3 rows | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S2b one: the row | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S2c maybeOne: zero rows gives null | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S2d maybeOne: two rows throw reylo's error | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S2e with(client)(params) | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S3 write without RETURNING (none), then a separate read | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S3n write without parameters (none), then a separate read | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S4 INSERT ... RETURNING | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S5a repeated :id (runtime lexer) | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S5b repeated :id (mapper text/order) | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S6a syntax error rejects with the driver's error | pass | Error: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR errno=1] |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S6b unique violation rejects with the driver's error | pass | Error: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT errno=19] |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S7a tx: write then commit is visible | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S7b tx: write then rollback is not visible | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S7d tx: a failing statement rolls the transaction back | pass | Error: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT errno=19] |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| sqlite3@5.1.7 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S8 20 concurrent queries resolve with the right rows | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called all(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S1a params: int, text, bool, null | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S1b omitted param (undefined) binds NULL | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S2a many: 3 rows | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S2b one: the row | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S2c maybeOne: zero rows gives null | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S2e with(client)(params) | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S3 write without RETURNING (none), then a separate read | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S3n write without parameters (none), then a separate read | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S4 INSERT ... RETURNING | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S5a repeated :id (runtime lexer) | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S5b repeated :id (mapper text/order) | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S6a syntax error rejects with the driver's error | pass | Error: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR errno=1] |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S6b unique violation rejects with the driver's error | pass | Error: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT errno=19] |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database (open()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called all(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S1a params: int, text, bool, null | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S1b omitted param (undefined) binds NULL | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S2a many: 3 rows | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S2b one: the row | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S2c maybeOne: zero rows gives null | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S2d maybeOne: two rows throw reylo's error | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S2e with(client)(params) | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S3 write without RETURNING (none), then a separate read | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S3n write without parameters (none), then a separate read | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S4 INSERT ... RETURNING | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S5a repeated :id (runtime lexer) | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S5b repeated :id (mapper text/order) | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S6a syntax error rejects with the driver's error | pass | Error: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR errno=1] |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S6b unique violation rejects with the driver's error | pass | Error: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT errno=19] |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S7a tx: write then commit is visible | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S7b tx: write then rollback is not visible | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S7d tx: a failing statement rolls the transaction back | pass | Error: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT errno=19] |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 5.1.7) | Database in BEGIN/COMMIT (exec) | S8 20 concurrent queries resolve with the right rows | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called all(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S1a params: int, text, bool, null | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S1b omitted param (undefined) binds NULL | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S2a many: 3 rows | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S2b one: the row | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S2c maybeOne: zero rows gives null | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S2d maybeOne: two rows throw reylo's error | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S2e with(client)(params) | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S3 write without RETURNING (none), then a separate read | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S3n write without parameters (none), then a separate read | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S4 INSERT ... RETURNING | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S5a repeated :id (runtime lexer) | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S5b repeated :id (mapper text/order) | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S6a syntax error rejects with the driver's error | pass | Error: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR errno=1] |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S6b unique violation rejects with the driver's error | pass | Error: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT errno=19] |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database | S8 20 concurrent queries resolve with the right rows | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called all(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S1a params: int, text, bool, null | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S1b omitted param (undefined) binds NULL | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S2a many: 3 rows | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S2b one: the row | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S2c maybeOne: zero rows gives null | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S2d maybeOne: two rows throw reylo's error | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S2e with(client)(params) | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S3 write without RETURNING (none), then a separate read | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S3n write without parameters (none), then a separate read | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S4 INSERT ... RETURNING | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S5a repeated :id (runtime lexer) | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S5b repeated :id (mapper text/order) | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S6a syntax error rejects with the driver's error | pass | Error: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR errno=1] |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S6b unique violation rejects with the driver's error | pass | Error: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT errno=19] |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S7a tx: write then commit is visible | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S7b tx: write then rollback is not visible | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S7d tx: a failing statement rolls the transaction back | pass | Error: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT errno=19] |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| sqlite3@6.0.1 | node 26.11.1 | SQLite file | Database in BEGIN/COMMIT (serialize) | S8 20 concurrent queries resolve with the right rows | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called all(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S1a params: int, text, bool, null | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S1b omitted param (undefined) binds NULL | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S2a many: 3 rows | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S2b one: the row | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S2c maybeOne: zero rows gives null | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S2d maybeOne: two rows throw reylo's error | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S2e with(client)(params) | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S3 write without RETURNING (none), then a separate read | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S3n write without parameters (none), then a separate read | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S4 INSERT ... RETURNING | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S5a repeated :id (runtime lexer) | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S5b repeated :id (mapper text/order) | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S6a syntax error rejects with the driver's error | pass | Error: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR errno=1] |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S6b unique violation rejects with the driver's error | pass | Error: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT errno=19] |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database (open()) | S8 20 concurrent queries resolve with the right rows | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called all(); run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S1a params: int, text, bool, null | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S1b omitted param (undefined) binds NULL | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S2a many: 3 rows | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S2b one: the row | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S2c maybeOne: zero rows gives null | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S2d maybeOne: two rows throw reylo's error | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S2e with(client)(params) | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S3 write without RETURNING (none), then a separate read | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S3n write without parameters (none), then a separate read | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S4 INSERT ... RETURNING | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S5a repeated :id (runtime lexer) | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S5b repeated :id (mapper text/order) | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S6a syntax error rejects with the driver's error | pass | Error: SQLITE_ERROR: near "selec": syntax error [code=SQLITE_ERROR errno=1] |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S6b unique violation rejects with the driver's error | pass | Error: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT errno=19] |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S7a tx: write then commit is visible | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S7b tx: write then rollback is not visible | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S7c tx: savepoint rollback keeps the outer write | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S7d tx: a failing statement rolls the transaction back | pass | Error: SQLITE_CONSTRAINT: UNIQUE constraint failed: uniq.k [code=SQLITE_CONSTRAINT errno=19] |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S7n tx: a write without parameters, then commit, is visible | pass |  |
| sqlite@5.1.1 | node 26.11.1 | SQLite file (over sqlite3 6.0.1) | Database in BEGIN/COMMIT (exec) | S8 20 concurrent queries resolve with the right rows | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called no method the harness could observe; run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S1a params: int, text, bool, null | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S1b omitted param (undefined) binds NULL | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S2a many: 3 rows | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S2b one: the row | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S2c maybeOne: zero rows gives null | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S2d maybeOne: two rows throw reylo's error | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S2e with(client)(params) | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S3 write without RETURNING (none), then a separate read | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S3n write without parameters (none), then a separate read | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S4 INSERT ... RETURNING | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S5a repeated :id (runtime lexer) | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S5b repeated :id (mapper text/order) | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S6a syntax error rejects with the driver's error | pass | Error: D1_ERROR: near "selec": syntax error at offset 0: SQLITE_ERROR (matched by message: the error carries no code field) |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S6b unique violation rejects with the driver's error | pass | Error: D1_ERROR: UNIQUE constraint failed: uniq.k: SQLITE_CONSTRAINT (extended: SQLITE_CONSTRAINT_PRIMARYKEY) (matched by message: the error carries no code field) |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | D1Database | S8 20 concurrent queries resolve with the right rows | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S0 run() returns a promise on every client; runSync() returns the value on a sync client and is refused elsewhere | pass | reylo called no method the harness could observe; run returned a promise and rejected on a failing statement; runSync refused the client before calling it |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S1a params: int, text, bool, null | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S1b omitted param (undefined) binds NULL | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S1c binary params: a plain Uint8Array and a view into a larger buffer bind as their bytes | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S2a many: 3 rows | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S2b one: the row | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S2c maybeOne: zero rows gives null | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S2d maybeOne: two rows throw reylo's error | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S2e with(client)(params) | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S3 write without RETURNING (none), then a separate read | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S3n write without parameters (none), then a separate read | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S4 INSERT ... RETURNING | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S5a repeated :id (runtime lexer) | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S5b repeated :id (mapper text/order) | pass |  |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S6a syntax error rejects with the driver's error | pass | Error: D1_ERROR: near "selec": syntax error at offset 0: SQLITE_ERROR (matched by message: the error carries no code field) |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S6b unique violation rejects with the driver's error | pass | Error: D1_ERROR: UNIQUE constraint failed: uniq.k: SQLITE_CONSTRAINT (extended: SQLITE_CONSTRAINT_PRIMARYKEY) (matched by message: the error carries no code field) |
| miniflare D1@4.20260730.0 | node 26.11.1 | D1 (Miniflare, local workerd) | db.withSession() | S8 20 concurrent queries resolve with the right rows | pass |  |
