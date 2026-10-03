import { test } from "node:test";
import assert from "node:assert/strict";
import { DatabaseSync } from "node:sqlite";
import { execute, type QueryModule } from "../src/execute.ts";
import { defineQuery } from "../src/index.ts";

// Each fake client records what execute handed it and answers the way its driver does; the
// driver behaviour each one copies was checked against the real package (lab/driver-matrix).
type Call = { method: string; args: unknown[] };
const pg: QueryModule<"postgres"> = { dialect: "postgres", sql: "select :id::int as id, :name as name, :id as again", params: ["id", "name"] };
const my: QueryModule<"mysql"> = { dialect: "mysql", sql: "select :id as id, :name as name, :id as again", params: ["id", "name"] };
const lite: QueryModule<"sqlite"> = { dialect: "sqlite", sql: "select :id as id, :name as name, :id as again", params: ["id", "name"] };
const row = { id: 1, name: "ada", again: 1 };

test("postgres: query(text, values) → { rows }, as on pg, PGlite and Neon's Pool, with $n numbered once per name", async () => {
  const calls: Call[] = [];
  const client = { query: async (...args: unknown[]) => (calls.push({ method: "query", args }), { rows: [row], rowCount: 1 }) };
  assert.deepEqual(await execute(client, pg, { id: 1, name: "ada" }), [row]);
  assert.deepEqual(calls, [{ method: "query", args: ["select $1::int as id, $2 as name, $1 as again", [1, "ada"]] }]);
});

test("postgres: a query() that resolves to the rows, as Neon's neon() from 1.0 and pg-promise do", async () => {
  const neon = Object.assign(() => Promise.reject(new Error("not a template")), {
    query: async () => [row],
    // Neon's unsafe() builds a fragment for a template; it must never be taken for a query.
    unsafe: () => ({ sql: "fragment" }),
  });
  assert.deepEqual(await execute(neon as never, pg, { id: 1, name: "ada" }), [row]);
});

test("postgres: pg-promise gets a { text, values } object, so it binds instead of formatting $1 into the text", async () => {
  const calls: Call[] = [];
  // pg-promise types query() as resolving to any.
  const db = { manyOrNone: async () => [], query: async (...args: unknown[]): Promise<any> => (calls.push({ method: "query", args }), [row]) };
  const q: QueryModule<"postgres"> = { dialect: "postgres", sql: "select '$2' as literal, :id::int as id", params: ["id"] };
  assert.deepEqual(await execute(db, q, { id: 1 }), [row]);
  assert.deepEqual(calls[0]!.args, [{ text: "select '$2' as literal, $1::int as id", values: [1] }]);
});

test("postgres: unsafe(text, values) → rows, as on postgres.js and Bun.sql and their transactions", async () => {
  const calls: Call[] = [];
  const tx = { unsafe: (...args: unknown[]) => (calls.push({ method: "unsafe", args }), Promise.resolve([row])) };
  assert.deepEqual(await execute(tx, pg, { id: 1, name: "ada" }), [row]);
  assert.deepEqual(calls, [{ method: "unsafe", args: ["select $1::int as id, $2 as name, $1 as again", [1, "ada"]] }]);
});

test("postgres: Neon's neon() before 1.0 is called with (text, values)", async () => {
  const calls: Call[] = [];
  const sql = (...args: unknown[]) => (calls.push({ method: "call", args }), Promise.resolve([row]));
  assert.deepEqual(await execute(sql, pg, { id: 1, name: "ada" }), [row]);
  assert.deepEqual(calls[0]!.args, ["select $1::int as id, $2 as name, $1 as again", [1, "ada"]]);
});

test("a parameter left out binds NULL, which postgres.js, mysql2, node:sqlite and libSQL would reject", async () => {
  const seen: unknown[][] = [];
  const client = { unsafe: async (_: string, values?: unknown[]) => (seen.push(values!), []) };
  await execute(client, pg, { id: 1 });
  await execute(client, { ...pg, text: "select $1, $2", order: ["name", "id"] }, { id: undefined });
  assert.deepEqual(seen, [[1, null], [null, null]]);
});

test("mysql: execute() → [rows, fields] on mysql2, a repeated name repeated in the values; a write's OK packet gives no rows", async () => {
  const calls: Call[] = [];
  const answers: unknown[] = [[[row], [{ name: "id" }]], [{ affectedRows: 1, insertId: 7 }, undefined]];
  const pool = { execute: async (...args: unknown[]) => (calls.push({ method: "execute", args }), answers.shift() as [unknown, unknown]) };
  assert.deepEqual(await execute(pool, my, { id: 1, name: "ada" }), [row]);
  assert.deepEqual(calls[0]!.args, ["select ? as id, ? as name, ? as again", [1, "ada", 1]]);
  assert.deepEqual(await execute(pool, my, { id: 1, name: "ada" }), []);
});

test("mysql: PlanetScale's execute() → { rows }", async () => {
  const conn = { execute: async () => ({ rows: [row], statement: "select", headers: ["id"], insertId: "0" }) };
  assert.deepEqual(await execute(conn, my, { id: 1, name: "ada" }), [row]);
});

test("mysql: mariadb runs through query(), not its own execute(); a write's OK packet gives no rows", async () => {
  const calls: string[] = [];
  const answers: unknown[] = [Object.assign([row], { meta: [] }), { affectedRows: 1, insertId: 7n, warningStatus: 0 }];
  const conn = {
    query: async () => (calls.push("query"), answers.shift()),
    batch: async () => (calls.push("batch"), {}),
    execute: async () => (calls.push("execute"), [row]),
  };
  assert.deepEqual([...((await execute(conn, my, { id: 1, name: "ada" })) as unknown[])], [row]);
  assert.deepEqual(await execute(conn, my, { id: 1, name: "ada" }), []);
  assert.deepEqual(calls, ["query", "query"]);
});

test("mysql: mariadb's callback API answers through the callback, and its errors reject", async () => {
  const failure = Object.assign(new Error("Duplicate entry 'a' for key 'email'"), { code: "ER_DUP_ENTRY" });
  const conn = {
    batch: () => ({}),
    // The callback query() returns a command object, not a promise.
    query: (sql: string, values: unknown[], done: (e: Error | null, rows?: unknown) => void) => (setImmediate(() => (sql.startsWith("insert") ? done(failure) : done(null, [{ values }]))), { on() {} }),
  };
  assert.deepEqual(await execute(conn as never, my, { id: 1, name: "ada" }), [{ values: [1, "ada", 1] }]);
  await assert.rejects(() => execute(conn as never, { dialect: "mysql", sql: "insert into t values (:a)", params: ["a"] }, { a: 1 }) as Promise<unknown>, (e) => e === failure);
});

test("mysql: PlanetScale 2.0, which binds no parameters, runs a statement without any and refuses one with some", async () => {
  const calls: unknown[][] = [];
  const conn = {
    execute: async (...args: unknown[]) => {
      calls.push(args);
      if (args.length > 1) throw new Error("Query parameters are not supported: pass a complete SQL string and options to execute(query, options)");
      return { rows: [row], statement: String(args[0]), headers: [] };
    },
  };
  assert.deepEqual(await execute(conn, { dialect: "mysql", sql: "select 1", params: [] }), [row]);
  await assert.rejects(() => execute(conn, my, { id: 1, name: "ada" }) as Promise<unknown>, (e: TypeError) => e instanceof TypeError && /2\.0 binds no parameters/.test(e.message) && /not supported/.test((e.cause as Error).message));
  assert.deepEqual(calls[0], ["select 1"]);
});

test("mysql: a mysql2 callback pool or connection runs through its promise() wrapper", async () => {
  const callback = {
    // The callback execute() returns a Query object, not a promise.
    execute: () => ({ on() {} }),
    promise: () => ({ execute: async () => [[row], []] as [unknown, unknown] }),
  };
  assert.deepEqual(await execute(callback, my, { id: 1, name: "ada" }), [row]);
});

test("mysql: Bun.sql's unsafe()", async () => {
  assert.deepEqual(await execute({ unsafe: async () => [row], commitDistributed: async () => {} }, my, { id: 1, name: "ada" }), [row]);
});

test("sqlite: node:sqlite answers at once, with one ? per placeholder; a write gives no rows", () => {
  const db = new DatabaseSync(":memory:");
  db.exec("create table t (a int)");
  const rows = execute(db, lite, { id: 1, name: "ada" });
  assert.ok(Array.isArray(rows), "a synchronous client's rows are not a promise");
  assert.deepEqual(rows.map((r) => ({ ...(r as object) })), [row]);
  assert.deepEqual(execute(db, { dialect: "sqlite", sql: "insert into t values (:a)", params: ["a"] }, { a: 2 }), []);
  assert.deepEqual(db.prepare("select a from t").all().map((r) => ({ ...r })), [{ a: 2 }]);
});

test("sqlite: better-sqlite3's statement that returns no rows is run(), since its all() throws", () => {
  const calls: Call[] = [];
  const db = {
    prepare: (sql: string) => ({
      reader: !sql.startsWith("insert"),
      all: (...args: unknown[]) => {
        if (sql.startsWith("insert")) throw new TypeError("This statement does not return data. Use run() instead");
        return calls.push({ method: "all", args }), [row];
      },
      run: (...args: unknown[]) => (calls.push({ method: "run", args }), { changes: 1 }),
    }),
  };
  assert.deepEqual(execute(db, { dialect: "sqlite", sql: "insert into t values (:a)", params: ["a"] }, { a: 2 }), []);
  assert.deepEqual(execute(db, lite, { id: 1, name: "ada" }), [row]);
  assert.deepEqual(calls, [{ method: "run", args: [2] }, { method: "all", args: [1, "ada", 1] }]);
});

test("sqlite: Cloudflare D1's prepare(sql).bind(...values).all() → { results }", async () => {
  const bound: unknown[][] = [];
  const d1 = {
    batch: async () => [],
    prepare: () => ({ first: async () => null, bind: (...values: unknown[]) => (bound.push(values), { all: async () => ({ results: [row], success: true, meta: {} }) }) }),
  };
  const rows = execute(d1, lite, { id: 1, name: "ada" });
  assert.ok(rows instanceof Promise);
  assert.deepEqual(await rows, [row]);
  assert.deepEqual(bound, [[1, "ada", 1]]);
});

test("sqlite: @libsql/client and its transactions take execute({ sql, args }) → { rows }", async () => {
  const calls: unknown[] = [];
  const tx = { execute: async (stmt: { sql: string; args: unknown[] }) => (calls.push(stmt), { rows: [row], columns: ["id"], rowsAffected: 0 }), executeMultiple: async () => {}, commit: async () => {} };
  assert.deepEqual(await execute(tx, lite, { id: 1, name: "ada" }), [row]);
  assert.deepEqual(calls, [{ sql: "select ? as id, ? as name, ? as again", args: [1, "ada", 1] }]);
});

test("sqlite: sqlite3's callback all(), its errors, and the sqlite package's promise all()", async () => {
  const failure = Object.assign(new Error("SQLITE_ERROR: near \"selec\": syntax error"), { code: "SQLITE_ERROR" });
  const sqlite3 = {
    prepare: () => ({}),
    exec: () => {},
    all: (sql: string, values: unknown[], done: (e: Error | null, rows: unknown[]) => void) => setImmediate(() => (sql.includes("selec ") ? done(failure, []) : done(null, [{ values }]))),
  };
  assert.deepEqual(await execute(sqlite3, lite, { id: 1, name: "ada" }), [{ values: [1, "ada", 1] }]);
  await assert.rejects(() => execute(sqlite3, { dialect: "sqlite", sql: "selec 1", params: [] }) as Promise<unknown>, (e) => e === failure);
  const wrapper = { getDatabaseInstance: () => sqlite3, prepare: async () => ({}), all: async (_: string, values: unknown[]) => [{ values }] };
  assert.deepEqual(await execute(wrapper, lite, { id: 2, name: "bo" }), [{ values: [2, "bo", 2] }]);
});

test("sqlite: Bun.sql's SQLite adapter, which has unsafe() and no prepare()", async () => {
  assert.deepEqual(await execute({ unsafe: async () => [row], commitDistributed: async () => {} }, lite, { id: 1, name: "ada" }), [row]);
});

test("the driver's own error comes through unchanged", async () => {
  const duplicate = Object.assign(new Error("duplicate key value violates unique constraint"), { code: "23505" });
  await assert.rejects(() => execute({ query: async () => Promise.reject(duplicate) }, pg, { id: 1 }) as Promise<unknown>, (e) => e === duplicate);
  const db = new DatabaseSync(":memory:");
  assert.throws(() => execute(db, { dialect: "sqlite", sql: "selec 1", params: [] }), /syntax error/);
});

test("an unsupported client rejects, naming what it has and what motherload takes", async () => {
  await assert.rejects(() => execute({ connect() {}, end() {} } as never, pg) as Promise<unknown>, (e: TypeError) => e instanceof TypeError && /postgres statement on an object with connect, end\. It takes a client with query\(text, values\)/.test(e.message));
  await assert.rejects(() => execute({ $client: {}, select() {} } as never, pg) as Promise<unknown>, /pass db\.\$client/);
  // node:sqlite's tag store has all() for templates; it is not sqlite3's database.
  const store = new DatabaseSync(":memory:").createTagStore();
  await assert.rejects(() => execute(store as never, lite, { id: 1, name: "ada" }) as Promise<unknown>, /sqlite statement on an object with .*pass its database, store\.db/);
  await assert.rejects(() => execute(null as never, my) as Promise<unknown>, /mysql statement on null/);
  // A pg pool handed to a SQLite statement matches no SQLite shape.
  await assert.rejects(() => execute({ query: async () => ({ rows: [] }) } as never, lite) as Promise<unknown>, /sqlite statement on an object with query/);
});

test("parameters may be an array: element i binds params[i], for a named and a positional statement on every dialect", async () => {
  const seen: [string, unknown[]][] = [];
  const pgClient = { query: async (text: string, values: unknown[]) => (seen.push([text, values]), { rows: [] }) };
  const mysqlPool = { execute: async (text: string, values: unknown[]) => (seen.push([text, values]), [[], []] as [unknown, unknown]) };
  const d1 = { batch: async () => [], prepare: (text: string) => ({ first: async () => null, bind: (...values: unknown[]) => (seen.push([text, values]), { all: async () => ({ results: [] }) }) }) };
  const positional: Record<string, QueryModule> = {
    postgres: { dialect: "postgres", sql: "select $2::text as name, $1::int as id", params: ["1", "2"] },
    mysql: { dialect: "mysql", sql: "select ? as id, ? as name", params: ["1", "2"] },
    sqlite: { dialect: "sqlite", sql: "select ?1 as id, ?2 as name, ?1 as again", params: ["1", "2"] },
  };
  for (const [client, named, dialect] of [[pgClient, pg, "postgres"], [mysqlPool, my, "mysql"], [d1, lite, "sqlite"]] as const) {
    seen.length = 0;
    await execute(client as never, named, [1, "ada"]);
    await execute(client as never, named, { id: 1, name: "ada" });
    assert.deepEqual(seen[0], seen[1], `${dialect}: [1, "ada"] binds as { id: 1, name: "ada" }`);
    seen.length = 0;
    await execute(client as never, positional[dialect]!, [1, "ada"]);
    await execute(client as never, positional[dialect]!, { 1: 1, 2: "ada" });
    assert.deepEqual(seen[0], seen[1], `${dialect}: [1, "ada"] binds as { 1: 1, 2: "ada" }`);
  }
  assert.deepEqual(seen.at(-1), ["select ? as id, ? as name, ? as again", [1, "ada", 1]]);
  // The mapper's rewritten statement binds by `order`, which names params.
  const unsafe: unknown[][] = [];
  await execute({ unsafe: async (_: string, values: unknown[]) => (unsafe.push(values), []) }, { ...pg, text: "select $1, $2, $1", order: ["name", "id", "name"] }, [1, "ada"]);
  assert.deepEqual(unsafe, [["ada", 1, "ada"]]);
  // runSync takes an array too.
  assert.deepEqual((execute(new DatabaseSync(":memory:"), lite, [1, "ada"]) as unknown[]).map((r) => ({ ...(r as object) })), [row]);
});

test("an array shorter than params binds NULL for the rest; a longer one is a TypeError naming both lengths", async () => {
  const seen: unknown[][] = [];
  await execute({ unsafe: async (_: string, values: unknown[]) => (seen.push(values), []) }, pg, [1]);
  assert.deepEqual(seen, [[1, null]]);
  await assert.rejects(() => execute({ unsafe: async () => [] }, pg, [1, "ada", true]) as Promise<unknown>, (e: TypeError) => e instanceof TypeError && /takes 2 parameters \("id", "name"\); got an array of 3 values/.test(e.message));
  assert.throws(() => execute(new DatabaseSync(":memory:"), { dialect: "sqlite", sql: "select 1", params: [] }, [1]), /takes 0 parameters \(none\); got an array of 1 value\./);
  await assert.rejects(() => execute({ unsafe: async () => [] }, pg, "ada" as never) as Promise<unknown>, /must be an object .* or an array .*; got ada/);
});

test("runSync: on a synchronous client the value comes back at once and an error is thrown, so db.transaction(fn) rolls back", () => {
  const db = new DatabaseSync(":memory:");
  db.exec("create table users (id integer primary key, email text unique)");
  const insert = defineQuery<{ email: string }, { id: number }, "one", "sqlite">({ dialect: "sqlite", sql: "insert into users (email) values (:email) returning id", params: ["email"], cardinality: "one" });
  const count = defineQuery<{}, { n: number }, "one", "sqlite">({ dialect: "sqlite", sql: "select count(*) as n from users", params: [], cardinality: "one" });
  // better-sqlite3's and bun:sqlite's transaction(fn): BEGIN, the function, then COMMIT, or ROLLBACK if it throws.
  const transaction = (fn: () => void) => {
    db.exec("begin");
    try {
      fn();
      db.exec("commit");
    } catch (e) {
      db.exec("rollback");
      throw e;
    }
  };
  const first = insert.runSync(db, { email: "ada@example.com" });
  assert.equal(first.id, 1);
  assert.throws(() => transaction(() => {
    insert.runSync(db, { email: "bo@example.com" });
    insert.runSync(db, { email: "ada@example.com" }); // the unique constraint fails here
  }), /UNIQUE constraint failed/);
  assert.equal(count.runSync(db).n, 1, "the transaction's first insert was rolled back");
  assert.throws(() => defineQuery<{}, unknown, "one", "sqlite">({ dialect: "sqlite", sql: "select 1 where 0", params: [], cardinality: "one" }).runSync(db), /Expected exactly one row, got 0/);
});

test("run: on a synchronous client the statement runs at once, and its rows and its error still come through the promise", async () => {
  const db = new DatabaseSync(":memory:");
  db.exec("create table users (id integer primary key, email text unique)");
  const insert = defineQuery<{ email: string }, { id: number }, "one", "sqlite">({ dialect: "sqlite", sql: "insert into users (email) values (:email) returning id", params: ["email"], cardinality: "one" });
  const pending = insert.run(db, { email: "ada@example.com" });
  assert.ok(pending instanceof Promise);
  assert.equal((db.prepare("select count(*) as n from users").get() as { n: number }).n, 1, "the insert ran before run returned");
  assert.equal((await pending).id, 1);
  let duplicate: unknown;
  assert.doesNotThrow(() => void (duplicate = insert.run(db, { email: "ada@example.com" })));
  await assert.rejects(duplicate as Promise<unknown>, /UNIQUE constraint failed/);
});

test("runSync: a client that answers through a promise, or a Postgres or MySQL statement, is refused before anything runs", () => {
  let called = false;
  const libsql = { execute: async () => ((called = true), { rows: [] }), executeMultiple: async () => {} };
  const lite = defineQuery<{}, unknown, "many", "sqlite">({ dialect: "sqlite", sql: "select 1", params: [] });
  assert.throws(() => lite.runSync(libsql as never), /runSync takes a synchronous SQLite client .*answers through a promise\. run takes every client/);
  const pgq = defineQuery<{}, unknown, "many", "postgres">({ dialect: "postgres", sql: "select 1", params: [] });
  assert.throws(() => pgq.runSync({ query: async () => ((called = true), { rows: [] }) } as never), /a postgres statement has none/);
  assert.equal(called, false);
});

test("run: on an asynchronous client every failure is a rejection, never a throw", async () => {
  const q = defineQuery<{ id: number }, { id: number }, "maybeOne", "postgres">({ dialect: "postgres", sql: "select :id::int as id", params: ["id"], cardinality: "maybeOne" });
  const two = { query: async () => ({ rows: [{ id: 1 }, { id: 2 }] }) };
  let returned: unknown;
  assert.doesNotThrow(() => void (returned = q.run(two, { id: 1 })));
  await assert.rejects(returned as Promise<unknown>, /Expected at most one row, got 2/);
  await assert.rejects(q.run({} as never, { id: 1 }), TypeError);
  assert.deepEqual(await q.with({ query: async () => ({ rows: [] }) })({ id: 1 }), null);
});
