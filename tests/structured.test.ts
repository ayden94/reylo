import { test } from "node:test";
import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { DatabaseSync } from "node:sqlite";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { Session } from "../src/analyze.ts";
import { sqlModule } from "../src/loader.ts";

// Structured parameters (docs/structured-params.md): a parameter cast to `rows(…)` or `row(…)` is
// typed by its column list and written for its position; the run time checks each value and
// writes the insert's column lists for the keys a call carries, so a key left out takes its
// column's default. Postgres and MySQL run on throwaway servers, never one anyone uses: each test
// makes a database of its own there and drops it after.
const PG = process.env.REYLO_TEST_PG;
const MYSQL = process.env.REYLO_TEST_MYSQL;
const DRIVERS = fileURLToPath(new URL("../lab/drivers/node_modules", import.meta.url));
const INDEX = pathToFileURL(fileURLToPath(new URL("../src/index.ts", import.meta.url))).href;
const pgLive = !PG ? "REYLO_TEST_PG names no server" : !existsSync(join(DRIVERS, "postgres")) ? "lab/drivers is not installed" : false;
const mysqlLive = !MYSQL ? "REYLO_TEST_MYSQL names no server" : !existsSync(join(DRIVERS, "mysql2")) ? "lab/drivers is not installed" : false;

function project(driver?: string): string {
  const root = mkdtempSync(join(tmpdir(), "reylo-structured-"));
  writeFileSync(join(root, "package.json"), "{}");
  if (driver) {
    mkdirSync(join(root, "node_modules"));
    symlinkSync(join(DRIVERS, driver), join(root, "node_modules", driver));
  }
  return root;
}

/** The module a build step writes for `sql`, imported. */
async function built(root: string, url: string, name: string, sql: string) {
  const file = join(root, `${name}.sql`);
  writeFileSync(file, sql);
  writeFileSync(join(root, ".env"), `DATABASE_URL=${url}\n`);
  const { code } = await sqlModule(file, sql);
  const out = join(root, `${name}.mjs`);
  writeFileSync(out, code.replace(`from "reylo"`, `from ${JSON.stringify(INDEX)}`));
  return (await import(pathToFileURL(out).href)).default;
}

async function analysis(root: string, url: string, sql: string) {
  const session = await Session.open(root, { DATABASE_URL: url });
  try {
    const outcome = await session.analyze(sql, { force: true });
    assert.ok(outcome.ok, outcome.ok ? "" : outcome.message);
    return outcome.analysis;
  } finally {
    await session.close();
  }
}

// The rows as plain objects: drivers return them with their own prototypes and extra properties.
const plain = (rows: readonly object[]) => [...rows].map((r) => ({ ...r }));

const closeAll = async () => {
  for (const session of Session.opened()) await session.close();
};

test("SQLite: an insert's rows are checked and written in their order, a key left out takes its default, and the column list types the parameter", async () => {
  const root = project();
  const db = new DatabaseSync(join(root, "app.db"));
  db.exec("create table events (id integer primary key, ts text not null, status text not null default 'new', amount real, meta text)");
  const url = "file:./app.db";
  const insert = `insert into events :events::rows(id integer not null, ts text not null, status? text not null, amount real, meta json('{"type": "object", "properties": {"source": {"type": "string"}}}')) returning id, status`;
  try {
    const a = await analysis(root, url, insert);
    assert.deepEqual(a.params.map((p) => [p.name, p.type_name]), [["events", "rows"]]);
    const events = a.params[0]!.type;
    assert.ok(events.kind === "rows");
    assert.deepEqual(events.fields.map((f) => [f.name, f.type.kind === "scalar" ? f.type.name : f.type.kind, f.nullable, f.optional]), [["id", "integer", false, false], ["ts", "text", false, false], ["status", "text", false, true], ["amount", "real", true, true], ["meta", "json", true, true]]);
    const q = await built(root, url, "insert_events", insert);
    assert.deepEqual(plain(await q.run(db, { events: [{ id: 2, ts: "2026-10-20", status: "done", amount: 1.5, meta: { source: "x" } }, { id: 1, ts: "2026-10-20", status: "late", amount: null, meta: null }] })), [{ id: 2, status: "done" }, { id: 1, status: "late" }]);
    assert.deepEqual(plain(await q.run(db, { events: [{ id: 3, ts: "2026-10-21" }] })), [{ id: 3, status: "new" }]);
    await assert.rejects(q.run(db, { events: [{ id: 4, ts: "x" }, { id: 5, ts: "x", status: "s" }] }), /row 2 gives "status", which row 1 leaves out/);
    await assert.rejects(q.run(db, { events: [{ id: 4, ts: "x", status: null }] }), /"status" is text not null, and the row gives null/);
    await assert.rejects(q.run(db, { events: [{ id: "4", ts: "x" }] }), /"id" is integer, an integer; got "4"/);
    // An explicit null stores NULL, never the text null, and a document stays JSON text.
    assert.deepEqual(db.prepare("select id, status, amount, meta, typeof(meta) as t from events order by id").all().map((r) => ({ ...r })), [
      { id: 1, status: "late", amount: null, meta: null, t: "null" },
      { id: 2, status: "done", amount: 1.5, meta: `{"source":"x"}`, t: "text" },
      { id: 3, status: "new", amount: null, meta: null, t: "null" },
    ]);
    // An upsert reads ON CONFLICT as its own, since the expansion ends in WHERE.
    const upsert = await built(root, url, "upsert", "insert into events :events::rows(id integer not null, ts text not null) on conflict (id) do update set ts = excluded.ts returning id, ts");
    assert.deepEqual(plain(await upsert.run(db, { events: [{ id: 3, ts: "2026-10-22" }] })), [{ id: 3, ts: "2026-10-22" }]);
  } finally {
    db.close();
    await closeAll();
  }
});

test("SQLite: elsewhere a rows parameter is a table named by its alias or by the parameter, a not null key never null and the rest nullable", async () => {
  const root = project();
  new DatabaseSync(join(root, "app.db")).close();
  const url = "file:./app.db";
  const sql = "select e.id, e.note from :events::rows(id integer not null, note text) as e order by e.id";
  try {
    const a = await analysis(root, url, sql);
    assert.deepEqual(a.columns.map((c) => [c.name, c.nullable]), [["id", false], ["note", true]]);
    const q = await built(root, url, "select_events", sql);
    const db = new DatabaseSync(join(root, "app.db"));
    try {
      assert.deepEqual(plain(await q.run(db, { events: [{ id: 2 }, { id: 1, note: "a" }] })), [{ id: 1, note: "a" }, { id: 2, note: null }]);
    } finally {
      db.close();
    }
    const unaliased = await analysis(root, url, "select events.id from :events::rows(id integer not null)");
    assert.deepEqual(unaliased.columns.map((c) => [c.name, c.nullable]), [["id", false]]);
  } finally {
    await closeAll();
  }
});

test("a column list that does not read is the type check's error at its place, and fails the build", async () => {
  const root = project();
  new DatabaseSync(join(root, "app.db")).close();
  const url = "file:./app.db";
  const cases: [string, string, RegExp][] = [
    ["insert into t :p::rows(a text not null default 'x')", "default", /never restates a default/],
    ["insert into t :p::rows(a? text)", "a?", /may be left out already/],
    ["select * from :p::rows(a? text not null)", "a?", /only where the rows are inserted/],
    ["insert into t :p::row(a text, posts rows(title text))", "rows(title text)", /nested row or rows member is not built yet/],
  ];
  try {
    for (const [sql, at, message] of cases) {
      const a = await analysis(root, url, sql);
      const d = a.diagnostics.find((d) => d.code === 20);
      assert.ok(d, sql);
      assert.match(d.message, message);
      assert.equal(sql.slice(d.start, d.start + d.length), at, sql);
      const file = join(root, "bad.sql");
      writeFileSync(file, sql);
      writeFileSync(join(root, ".env"), `DATABASE_URL=${url}\n`);
      await assert.rejects(sqlModule(file, sql), message);
    }
  } finally {
    await closeAll();
  }
});

test("SQLite: the column list is checked against the table its insert writes, and a disagreement fails the build", async () => {
  const root = project();
  const db = new DatabaseSync(join(root, "app.db"));
  db.exec("create table events (id integer primary key, ts text not null, status text not null default 'new', amount real, total real generated always as (amount * 2))");
  db.close();
  const url = "file:./app.db";
  const cases: [string, string, RegExp][] = [
    ["insert into events :e::rows(id integer not null, ts text, status? text not null)", "ts text", /events\.ts is not null, and `ts` lets a row give null or leave the key out: write `ts text not null`/],
    ["insert into events :e::rows(id integer not null, ts text not null, status text)", "status text", /write `status\? text not null`, which lets a row leave the key out so that the default applies/],
    ["insert into events :e::rows(id integer not null, ts text not null, amount? real not null)", "amount", /`amount\?` lets a row leave the key out, but events\.amount has no default/],
    ["insert into events :e::rows(id integer not null, status? text not null)", ":e::rows", /events\.ts is not null and has no default, and the column list leaves it out/],
    ["insert into events :e::rows(id text not null, ts text not null)", "text", /events\.id is INTEGER, and the column list says text: their affinities differ \(TEXT and INTEGER\)/],
    ["insert into events :e::rows(ts text not null, total real)", "total", /events\.total is a generated column/],
  ];
  try {
    for (const [sql, at, message] of cases) {
      const a = await analysis(root, url, sql);
      const d = a.diagnostics.filter((d) => d.code === 21);
      assert.equal(d.length, 1, `${sql}: ${JSON.stringify(a.diagnostics)}`);
      assert.match(d[0]!.message, message);
      assert.equal(sql.slice(d[0]!.start, d[0]!.start + d[0]!.length), at, sql);
      const file = join(root, "bad.sql");
      writeFileSync(file, sql);
      writeFileSync(join(root, ".env"), `DATABASE_URL=${url}\n`);
      await assert.rejects(sqlModule(file, sql), message);
    }
    // The rowid is filled when it is left out, so an INTEGER PRIMARY KEY needs no `?`.
    assert.deepEqual((await analysis(root, url, "insert into events :e::rows(ts text not null)")).diagnostics, []);
  } finally {
    await closeAll();
  }
});

test("Postgres: a rows insert keeps the array's order and exact 64-bit ids, a key left out takes its default, and every driver binds the rows", { skip: pgLive }, async () => {
  const require = createRequire(join(DRIVERS, "postgres", "package.json"));
  const postgres = require("postgres");
  const admin = postgres(PG, { max: 1, onnotice: () => {} });
  const name = `reylo_structured_${process.pid}`;
  await admin.unsafe(`drop database if exists ${name}`);
  await admin.unsafe(`create database ${name}`);
  const url = new URL(PG!);
  url.pathname = `/${name}`;
  const sql = postgres(url.href, { max: 1, onnotice: () => {} });
  const root = project("postgres");
  try {
    await sql.unsafe(`create table events (id int8 primary key, ts timestamptz not null, status text not null default 'new', amount numeric(12,2), meta jsonb);
      create table users (id int8 generated always as identity primary key, email text not null, name text not null)`);
    const insert = `insert into events :events::rows(id int8 not null, ts timestamptz not null, status? text not null, amount numeric(12,2), meta jsonb('{"type": "object"}')) returning id, status, amount`;
    const a = await analysis(root, url.href, insert);
    const events = a.params[0]!.type;
    assert.ok(events.kind === "rows");
    assert.deepEqual(events.fields.map((f) => [f.name, f.type.kind === "scalar" ? f.type.name : f.type.kind, f.written]), [["id", "int8", "int8"], ["ts", "timestamptz", "timestamptz"], ["status", "text", "text"], ["amount", "numeric", "numeric(12,2)"], ["meta", "json", "jsonb"]]);
    assert.deepEqual(a.columns.map((c) => [c.name, c.nullable]), [["id", false], ["status", false], ["amount", true]]);
    const q = await built(root, url.href, "insert_events", insert);
    // postgres.js serializes a JSON parameter itself; pg takes its text.
    assert.deepEqual(plain(await q.run(sql, { events: [{ id: 9007199254740993n, ts: new Date("2026-10-20T10:00:00Z"), status: "done", amount: "1.50", meta: { a: 1 } }] })), [{ id: "9007199254740993", status: "done", amount: "1.50" }]);
    const pg = require("pg");
    const client = new pg.Client(url.href);
    await client.connect();
    try {
      assert.deepEqual(plain(await q.run(client, { events: [{ id: 2, ts: "2026-10-20 12:00:00+02" }] })), [{ id: "2", status: "new", amount: null }]);
      await assert.rejects(q.run(client, { events: [{ id: 3, ts: new Date(), amount: "1.234" }] }), /would be rounded/);
    } finally {
      await client.end();
    }
    // The array's order is the insert's, and RETURNING follows it.
    const users = await built(root, url.href, "insert_users", "insert into users :users::rows(email text not null, name text not null) returning id, email");
    const many = Array.from({ length: 1000 }, (_, i) => ({ email: `u${i}@x`, name: `n${1000 - i}` }));
    const returned = await users.run(sql, { users: many });
    assert.deepEqual(returned.map((r: { email: string }) => r.email), many.map((u) => u.email));
    assert.deepEqual(returned.map((r: { id: string }) => Number(r.id)), many.map((_, i) => i + 1));
    // One row.
    const user = await built(root, url.href, "insert_user", "insert into users :user::row(email text not null, name text not null) returning id");
    assert.deepEqual(plain(await user.run(sql, { user: { email: "a@x", name: "Ada" } })), [{ id: "1001" }]);
    // Elsewhere, a table whose not null keys are never null.
    const read = "select e.id, e.amount from :events::rows(id int8 not null, amount numeric) as e order by e.id";
    assert.deepEqual((await analysis(root, url.href, read)).columns.map((c) => [c.name, c.nullable]), [["id", false], ["amount", true]]);
    const r = await built(root, url.href, "read_events", read);
    assert.deepEqual(plain(await r.run(sql, { events: [{ id: 2 }, { id: 1, amount: 3 }] })), [{ id: "1", amount: "3" }, { id: "2", amount: null }]);
    // A typed JSON parameter: Postgres receives `$1::jsonb`, and the run time fills its defaults and checks it.
    const typed = await built(root, url.href, "typed_users", `insert into users (email, name)
select x.email, x.name from jsonb_to_recordset(:users::jsonb('{"type": "array", "items": {"type": "object", "properties": {"email": {"type": "string", "format": "email"}, "name": {"type": "string", "default": "anon"}}, "required": ["email", "name"]}}')) as x(email text, name text)
returning email, name`);
    assert.deepEqual(plain(await typed.run(sql, { users: [{ email: "b@x" }, { email: "c@x", name: "Cy" }] })), [{ email: "b@x", name: "anon" }, { email: "c@x", name: "Cy" }]);
    await assert.rejects(typed.run(sql, { users: [{ email: "nope" }] }), /Parameter "users" at \/0\/email must be a email/);
    // A type the database does not know is an error at the member's type.
    const bad = "insert into events :events::rows(id int8 not null, ts timestamptz not null, amount numerc)";
    const d = (await analysis(root, url.href, bad)).diagnostics;
    assert.deepEqual(d.map((x) => [x.message, bad.slice(x.start, x.start + x.length)]), [[`type "numerc" does not exist (42704)`, "numerc"]]);
    // A member's type is its column's, modifiers included; an identity GENERATED ALWAYS is the database's to number.
    const checks: [string, string, RegExp][] = [
      ["insert into events :e::rows(id numeric not null, ts timestamptz not null)", "numeric", /events\.id is bigint, and the column list says numeric/],
      ["insert into events :e::rows(id int8 not null, ts timestamptz not null, amount numeric)", "numeric", /events\.amount is numeric\(12,2\), and the column list says numeric/],
      ["insert into events :e::rows(id int8 not null, ts timestamp not null)", "timestamp", /events\.ts is timestamp with time zone, and the column list says timestamp/],
      ["insert into users :u::rows(id int8 not null, email text not null, name text not null)", "id", /users\.id is GENERATED ALWAYS AS IDENTITY/],
    ];
    for (const [sql, at, message] of checks) {
      const found = (await analysis(root, url.href, sql)).diagnostics.filter((x) => x.code === 21);
      assert.equal(found.length, 1, `${sql}: ${JSON.stringify(found)}`);
      assert.match(found[0]!.message, message);
      assert.equal(sql.slice(found[0]!.start, found[0]!.start + found[0]!.length), at, sql);
    }
    // Aliases of one type are the type: bigint is int8, timestamp with time zone is timestamptz.
    assert.deepEqual((await analysis(root, url.href, "insert into events :e::rows(id bigint not null, ts timestamp with time zone not null, amount numeric(12, 2))")).diagnostics, []);
  } finally {
    await closeAll();
    await sql.end();
    await admin.unsafe(`drop database if exists ${name}`);
    await admin.end();
  }
});

test("MySQL: a rows insert reads each column with ERROR ON ERROR, keeps a 64-bit id exact and gives a key left out its default", { skip: mysqlLive }, async () => {
  const require = createRequire(join(DRIVERS, "mysql2", "package.json"));
  const mysql = require("mysql2/promise");
  const server = new URL(MYSQL!);
  const admin = await mysql.createConnection({ host: server.hostname, port: Number(server.port), user: decodeURIComponent(server.username) || "root", password: decodeURIComponent(server.password) });
  const name = `reylo_structured_${process.pid}`;
  await admin.query(`drop database if exists ${name}`);
  await admin.query(`create database ${name}`);
  const url = new URL(MYSQL!);
  url.pathname = `/${name}`;
  const db = await mysql.createConnection({ host: server.hostname, port: Number(server.port), user: decodeURIComponent(server.username) || "root", password: decodeURIComponent(server.password), database: name });
  const root = project("mysql2");
  try {
    await db.query("create table events (id bigint primary key, ts datetime(6) not null, status varchar(32) not null default 'new', amount decimal(12,2), meta json)");
    const insert = `insert into events :events::rows(id bigint not null, ts datetime(6) not null, status? varchar(32) not null, amount decimal(12,2), meta json('{"type": "object"}'))`;
    const a = await analysis(root, url.href, insert);
    const events = a.params[0]!.type;
    assert.ok(events.kind === "rows");
    assert.deepEqual(events.fields.map((f) => [f.name, f.type.kind === "scalar" ? f.type.name : f.type.kind]), [["id", "int8"], ["ts", "timestamp"], ["status", "text"], ["amount", "numeric"], ["meta", "json"]]);
    const q = await built(root, url.href, "insert_events", insert);
    await q.run(db, { events: [{ id: 9223372036854775807n, ts: "2026-10-20 10:00:00", amount: "1.50", meta: { a: 1 } }, { id: 1, ts: "2026-10-20 11:00:00", amount: null, meta: null }] });
    await assert.rejects(q.run(db, { events: [{ id: 2, ts: "2026-10-20 10:00:00", status: "x".repeat(33) }] }), /at most 32 characters/);
    const mismatch = "insert into events :e::rows(id bigint not null, ts datetime not null)";
    const found = (await analysis(root, url.href, mismatch)).diagnostics.filter((x) => x.code === 21);
    assert.deepEqual(found.map((x) => [x.message, mismatch.slice(x.start, x.start + x.length)]), [["events.ts is datetime(6), and the column list says datetime: a column list writes each column's own type, so that the checks the run time makes are the column's.", "datetime"]]);
    assert.deepEqual((await analysis(root, url.href, "insert into events :e::rows(id bigint not null, ts datetime(6) not null)")).diagnostics, []);
    // A typed JSON parameter: MySQL receives `cast(? as json)`.
    const typed = await built(root, url.href, "typed_events", `select j.id from json_table(:ids::json('{"type": "array", "items": {"type": "integer", "minimum": 1}}'), '$[*]' columns (id bigint path '$' error on error)) as j order by j.id`);
    assert.deepEqual(plain(await typed.run(db, { ids: [3, 1] })), [{ id: 1 }, { id: 3 }]);
    await assert.rejects(typed.run(db, { ids: [0] }), /Parameter "ids" at \/0 must be at least 1/);
    const [rows] = await db.query("select cast(id as char) as id, status, amount, meta from events order by id");
    assert.deepEqual(plain(rows), [
      { id: "1", status: "new", amount: null, meta: null },
      { id: "9223372036854775807", status: "new", amount: "1.50", meta: { a: 1 } },
    ]);
  } finally {
    await closeAll();
    await db.end();
    await admin.query(`drop database if exists ${name}`);
    await admin.end();
  }
});

test("an imported query carries its input's and its output's JSON Schema, a structured parameter's keys included", async () => {
  const root = project();
  const db = new DatabaseSync(join(root, "app.db"));
  db.exec("create table events (id integer primary key, ts text not null, status text not null default 'new', amount real)");
  db.close();
  try {
    const q = await built(root, "file:./app.db", "insert_events", "insert into events :events::rows(id integer not null, ts text not null, status? text not null, amount real) returning id, status");
    assert.deepEqual(q.inputSchema, {
      $schema: "https://json-schema.org/draft/2020-12/schema",
      type: "object",
      properties: {
        events: {
          type: "array",
          items: {
            type: "object",
            properties: { id: { type: "integer", format: "int64" }, ts: { type: "string" }, status: { type: "string" }, amount: { anyOf: [{ type: "number", format: "double" }, { type: "null" }] } },
            required: ["id", "ts"],
            additionalProperties: false,
          },
        },
      },
      required: ["events"],
      additionalProperties: false,
    });
    assert.deepEqual(q.outputSchema, {
      $schema: "https://json-schema.org/draft/2020-12/schema",
      type: "array",
      items: { type: "object", properties: { id: { type: "integer", format: "int64" }, status: { type: "string" } }, required: ["id", "status"] },
    });
    // Standard Schema's converter gives the same.
    assert.deepEqual(q["~standard"].jsonSchema.input({ target: "draft-2020-12" }), q.inputSchema);
  } finally {
    await closeAll();
  }
});

test("SQLite: a typed JSON parameter is typed by its schema, takes its defaults, and is checked before the statement runs", async () => {
  const root = project();
  const db = new DatabaseSync(join(root, "app.db"));
  db.exec("create table todos (id integer primary key, title text not null, done integer not null default 0)");
  const url = "file:./app.db";
  const insert = `insert into todos (title, done)
select value ->> '$.title', value ->> '$.done' from json_each(:todos::json('{
  "type": "array",
  "maxItems": 2,
  "items": {
    "type": "object",
    "properties": { "title": { "type": "string", "minLength": 1 }, "done": { "type": "boolean", "default": false } },
    "required": ["title", "done"],
    "additionalProperties": false
  }
}')) returning id, title, done`;
  try {
    const a = await analysis(root, url, insert);
    const p = a.params[0]!;
    assert.deepEqual([p.name, p.type_name, p.type.kind], ["todos", "json", "json"]);
    // `done` has a default, so the caller may leave it out; the run time fills it.
    assert.deepEqual(p.type.kind === "json" && p.type.schema && (p.type.schema as { items: { required: string[] } }).items.required, ["title"]);
    const q = await built(root, url, "insert_todos", insert);
    assert.deepEqual(plain(await q.run(db, { todos: [{ title: "a" }, { title: "b", done: true }] })), [{ id: 1, title: "a", done: 0 }, { id: 2, title: "b", done: 1 }]);
    await assert.rejects(q.run(db, { todos: [{ title: "" }] }), /Parameter "todos" at \/0\/title must be at least 1 characters long/);
    await assert.rejects(q.run(db, { todos: [{ title: "a", extra: 1 }] }), /at \/0\/extra is a key the schema does not allow/);
    await assert.rejects(q.run(db, { todos: [{ title: "a" }, { title: "b" }, { title: "c" }] }), /must have at most 2 items/);
    // A parameter whose schema has a default at its root may be left out.
    const limited = await built(root, url, "limited", `select id from todos order by id limit :n::json('{ "type": "integer", "minimum": 1, "maximum": 200, "default": 1 }')`);
    assert.deepEqual(plain(await limited.run(db, {})), [{ id: 1 }]);
    assert.deepEqual(plain(await limited.run(db, { n: 2 })), [{ id: 1 }, { id: 2 }]);
    await assert.rejects(limited.run(db, { n: 0 }), /Parameter "n" must be at least 1/);
    assert.deepEqual(limited.inputSchema.required, []);
    // What the profile refuses is the type check's error at the schema, and fails the build.
    const bad = `select * from json_each(:p::json('{ "$ref": "#/$defs/x" }'))`;
    const d = (await analysis(root, url, bad)).diagnostics.find((x) => x.code === 20);
    assert.ok(d);
    assert.match(d.message, /breaks the schema's locality/);
    assert.equal(bad.slice(d.start, d.start + d.length), `'{ "$ref": "#/$defs/x" }'`);
  } finally {
    db.close();
    await closeAll();
  }
});

test("SQLite: a JSON member of a column list is checked against its schema, its nested defaults filled", async () => {
  const root = project();
  const db = new DatabaseSync(join(root, "app.db"));
  db.exec("create table events (id integer primary key, meta text)");
  try {
    const q = await built(root, "file:./app.db", "insert_events", `insert into events :events::rows(id integer not null, meta json('{"type": "object", "properties": {"source": {"type": "string", "default": "web"}, "n": {"type": "integer", "minimum": 0}}, "required": ["source"]}')) returning id, meta`);
    assert.deepEqual(plain(await q.run(db, { events: [{ id: 1, meta: {} }, { id: 2, meta: { source: "app", n: 3 } }] })), [{ id: 1, meta: `{"source":"web"}` }, { id: 2, meta: `{"source":"app","n":3}` }]);
    await assert.rejects(q.run(db, { events: [{ id: 3, meta: { n: -1 } }] }), /row 1: "meta" at \/n must be at least 0/);
  } finally {
    db.close();
    await closeAll();
  }
});
