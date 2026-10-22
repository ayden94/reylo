import { test } from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { Session } from "../src/analyze.ts";
import { schemaFiles, schemaText, statements } from "../src/schema-files.ts";

// `schema` in .reylo.yaml: the type check reads the schema from SQL files, loaded into PGlite or an
// in-memory SQLite, and connects to nothing. Each project here installs one engine, as a project
// does, and PGlite where it loads a Postgres schema.
const REPO = fileURLToPath(new URL("..", import.meta.url));
const CLI = join(REPO, "src", "cli.ts");
const DUMP = readFileSync(join(REPO, "tests/fixtures/pg-dump-18.sql"), "utf8");
const PG = process.env.REYLO_TEST_PG;
const DRIVERS = join(REPO, "lab/drivers/node_modules");
const pgLive = !PG ? "REYLO_TEST_PG names no server" : !existsSync(join(DRIVERS, "postgres")) ? "lab/drivers is not installed" : false;

function project(engines: string[], files: Record<string, string>, pglite = engines.includes("postgres")): string {
  // The CLI sees its working directory as the real path (/private/var on macOS).
  const root = realpathSync(mkdtempSync(join(tmpdir(), "reylo-schema-")));
  writeFileSync(join(root, "package.json"), "{}");
  mkdirSync(join(root, "node_modules/@reylosql"), { recursive: true });
  for (const db of engines) symlinkSync(join(REPO, "packages", db), join(root, "node_modules/@reylosql", db));
  if (pglite) {
    mkdirSync(join(root, "node_modules/@electric-sql"));
    symlinkSync(join(REPO, "node_modules/@electric-sql/pglite"), join(root, "node_modules/@electric-sql/pglite"));
  }
  for (const [path, text] of Object.entries(files)) {
    mkdirSync(join(root, path, ".."), { recursive: true });
    writeFileSync(join(root, path), text);
  }
  return root;
}

type Ok = Extract<Awaited<ReturnType<Session["analyze"]>>, { ok: true }>;

/** Each statement's analysis, in a session of its own with no DATABASE_URL anywhere. */
async function analyze(root: string, ...sqls: string[]) {
  const session = await Session.open(root, {});
  try {
    const out = [];
    for (const sql of sqls) out.push(await session.analyze(sql));
    return out;
  } finally {
    await session.close();
  }
}

function columns(outcome: Awaited<ReturnType<Session["analyze"]>> | undefined) {
  assert.ok(outcome?.ok, outcome && !outcome.ok ? outcome.message : "no outcome");
  return (outcome as Ok).analysis.columns.map((c) => `${c.name} ${c.type_name}${c.nullable ? " null" : ""}`);
}

const USER = "select id, email, kind, nick from users where id = :id";
const VIEW = "select title, email from user_posts";
const COUNT = "select public.post_count(:uid) as n";
const INSERT = "insert into posts (author_id, title, tags) values (:author, :title, :tags) returning id";

test("a pg_dump --schema-only types queries in PGlite with no database: its owners, grants, extension and settings load", async () => {
  const root = project(["postgres"], { ".reylo.yaml": "schema: db/schema.sql\n", "db/schema.sql": DUMP });
  const [user, view, count, insert] = await analyze(root, USER, VIEW, COUNT, INSERT);
  assert.deepEqual(columns(user), ["id int4", "email citext", "kind role_kind", "nick text null"]);
  assert.equal((user as Ok).analysis.cardinality, "maybeOne");
  assert.deepEqual((user as Ok).analysis.columns[2]!.type, { kind: "enum", name: "role_kind", labels: ["admin", "member"] });
  assert.equal((user as Ok).source, "database");
  assert.deepEqual(columns(view), ["title text", "email citext"]);
  assert.deepEqual(columns(count), ["n int8 null"]);
  assert.deepEqual((insert as Ok).analysis.params.map((p) => `${p.name} ${p.type_name}${p.nullable ? " null" : ""}`), ["author int4", "title text", "tags _text null"]);
  // A second session reads every statement from .reylo/, keyed by the files, and loads nothing.
  const again = await analyze(root, USER, VIEW);
  assert.deepEqual(again.map((o) => o.ok && o.source), ["cache", "cache"]);
  assert.deepEqual(again[0], { ...user, source: "cache" });
});

test("against a server with the same schema, every analysis is the server's", { skip: pgLive }, async () => {
  const postgres = createRequire(join(DRIVERS, "postgres", "package.json"))("postgres");
  const admin = postgres(PG, { max: 1, onnotice: () => {} });
  const name = `reylo_schema_files_${process.pid}`;
  await admin.unsafe(`drop database if exists ${name}`);
  await admin.unsafe(`create database ${name}`);
  const url = new URL(PG!);
  url.pathname = `/${name}`;
  const sql = postgres(url.href, { max: 1, onnotice: () => {} });
  try {
    // The roles the dump names exist on a server; psql would skip the \restrict lines.
    for (const role of ["app_owner", "app_reader"]) await admin.unsafe(`do $$ begin create role ${role}; exception when duplicate_object then null; end $$`);
    await sql.unsafe(schemaText(DUMP, "postgres")).simple();
    const live = project(["postgres"], {}, false);
    symlinkSync(join(DRIVERS, "postgres"), join(live, "node_modules/postgres"));
    writeFileSync(join(live, ".env"), `DATABASE_URL=${url.href}\n`);
    const fromFile = project(["postgres"], { ".reylo.yaml": "schema: schema.sql\n", "schema.sql": DUMP });
    const server = await analyze(live, USER, VIEW, COUNT, INSERT);
    const files = await analyze(fromFile, USER, VIEW, COUNT, INSERT);
    assert.deepEqual(files.map((o) => o.ok && o.analysis), server.map((o) => o.ok && o.analysis));
  } finally {
    await sql.end();
    await admin.unsafe(`drop database if exists ${name}`);
    await admin.end();
  }
});

test("a directory of migrations: in order by number, down migrations and down sections left out", async () => {
  const root = project(["postgres"], {
    ".reylo.yaml": "schema: [migrations/*.sql, db/*.sql]\n",
    // golang-migrate: one file each way, numbered without padding.
    "migrations/1_users.up.sql": "create table users (id int primary key, name text);",
    "migrations/1_users.down.sql": "drop table users;",
    "migrations/2_email.up.sql": "alter table users add column email text;",
    "migrations/10_email_required.up.sql": "alter table users alter column email set not null;",
    "migrations/10_email_required.down.sql": "alter table users alter column email drop not null;",
    // dbmate and goose: both ways in one file.
    "db/20261001_posts.sql": "-- migrate:up\ncreate table posts (id int primary key, author int not null references users(id));\n\n-- migrate:down\ndrop table posts;\n",
    "db/20261002_title.sql": "-- +goose Up\nalter table posts add column title text not null;\n-- +goose Down\nalter table posts drop column title;\n",
  });
  const found = schemaFiles(root, ["migrations/*.sql", "db/*.sql"]);
  assert.deepEqual(found.files.map((f) => f.slice(root.length + 1)), ["migrations/1_users.up.sql", "migrations/2_email.up.sql", "migrations/10_email_required.up.sql", "db/20261001_posts.sql", "db/20261002_title.sql"]);
  const [users, posts] = await analyze(root, "select id, name, email from users", "select p.title, u.email from posts p join users u on u.id = p.author");
  assert.deepEqual(columns(users), ["id int4", "name text null", "email text"]);
  assert.deepEqual(columns(posts), ["title text", "email text"]);
});

test("a SQLite .schema types queries in memory, and a change to the file is a new schema", async () => {
  const schema = "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, bio TEXT);\nCREATE TABLE sqlite_sequence(name,seq);\nCREATE INDEX users_name ON users (name);\n";
  const root = project(["sqlite"], { ".reylo.yaml": "schema: schema.sql\n", "schema.sql": schema });
  const session = await Session.open(root, {});
  try {
    assert.equal(session.project.dialect, "sqlite");
    const user = await session.analyze("select id, name, bio from users where id = :id");
    assert.deepEqual(columns(user), ["id integer", "name text", "bio text null"]);
    assert.equal((user as Ok).analysis.cardinality, "maybeOne");
    writeFileSync(join(root, "schema.sql"), schema.replace("bio TEXT", "bio TEXT NOT NULL"));
    assert.equal(await session.recheck(), true);
    assert.deepEqual(columns(await session.analyze("select id, name, bio from users where id = :id")), ["id integer", "name text", "bio text"]);
  } finally {
    await session.close();
  }
});

test("a schema that does not load names the file, the line and the error", async () => {
  // The server gives no position for a missing relation: the statement that failed is found.
  const missing = project(["postgres"], { ".reylo.yaml": "schema: [a.sql, b.sql]\n", "a.sql": "create table users (id int primary key);\n", "b.sql": "-- posts\n\ncreate table posts (\n  author int references authors(id)\n);\n" });
  const [a] = await analyze(missing, "select 1");
  assert.deepEqual(a, { ok: false, code: 5, schema: true, message: `Cannot load the schema: b.sql:3:1: relation "authors" does not exist` });
  const syntax = project(["postgres"], { ".reylo.yaml": "schema: schema.sql\n", "schema.sql": "create table users (\n  id int primry key\n);\n" });
  assert.equal(((await analyze(syntax, "select 1"))[0] as { message: string }).message, `Cannot load the schema: schema.sql:2:10: syntax error at or near "primry"`);
  const extension = project(["postgres"], { ".reylo.yaml": "schema: schema.sql\n", "schema.sql": "create extension postgis;\n" });
  assert.match(((await analyze(extension, "select 1"))[0] as { message: string }).message, /^Cannot load the schema: schema\.sql:1:1: extension "postgis" is not available \(PGlite, which Reylo loads the schema into, has no postgis/);
  const sqlite = project(["sqlite"], { ".reylo.yaml": "schema: schema.sql\n", "schema.sql": "create table users (id integer primary key);\ncreate tabel posts (id integer);\n" });
  assert.deepEqual((await analyze(sqlite, "select 1"))[0], { ok: false, code: 5, schema: true, message: `Cannot load the schema: schema.sql: near "tabel": syntax error` });
});

test("schema needs files that exist, PGlite for Postgres, and a database it can load", async () => {
  const message = async (root: string, env: Record<string, string> = {}) => {
    const session = await Session.open(root, env);
    try {
      const o = await session.analyze("select 1");
      return o.ok ? "ok" : o.message;
    } finally {
      await session.close();
    }
  };
  assert.equal(await message(project(["sqlite"], { ".reylo.yaml": "schema: db/schema.sql\n" })), `"schema" in .reylo.yaml names db/schema.sql, which does not exist`);
  assert.equal(await message(project(["sqlite"], { ".reylo.yaml": "schema: migrations/*.sql\n" })), `"schema" in .reylo.yaml names migrations/*.sql, which matches no file`);
  assert.equal(await message(project(["postgres"], { ".reylo.yaml": "schema: schema.sql\n", "schema.sql": "create table t (id int);" }, false)), `"schema" in .reylo.yaml loads a Postgres schema into PGlite, which is not installed: npm install -D @electric-sql/pglite`);
  // The URL names the database where one is set; the type check does not connect to it.
  const mysql = project(["mysql", "postgres"], { ".reylo.yaml": "schema: schema.sql\n", "schema.sql": "create table t (id int);" });
  assert.match(await message(mysql, { DATABASE_URL: "mysql://root@127.0.0.1:1/app" }), /^\.reylo\.yaml: "schema" loads a Postgres or SQLite schema, and MySQL needs its server/);
  assert.equal(await message(mysql, { DATABASE_URL: "postgres://nowhere.invalid:1/app" }), "ok");
  assert.match(await message(mysql), /^\.reylo\.yaml: "schema" names a schema of which database\? @reylosql\/postgres, @reylosql\/mysql are installed, so set DATABASE_URL or direct_url/);
});

test("reylo generate names the schema files, and analyzes the queries alone", () => {
  const root = project(["sqlite"], {
    ".reylo.yaml": "schema: db/schema.sql\n",
    "db/schema.sql": "create table users (id integer primary key, name text not null);\ncreate index users_name on users (name);\n",
    "src/user.sql": "select id, name from users where id = :id",
    "src/bad.sql": "select nope from users",
  });
  const run = () => {
    const r = spawnSync(process.execPath, [CLI, "generate"], { cwd: root, encoding: "utf8", env: { PATH: process.env.PATH } });
    return { status: r.status, out: r.stdout.replace(/\(\d+\.\d s\)/, "(_ s)"), err: r.stderr };
  };
  const first = run();
  assert.equal(first.err, "");
  assert.equal(first.out, [`sqlite from db/schema.sql ("schema" in .reylo.yaml)`, `  src/bad.sql   ✗`, `  src/user.sql  ✓`, ``, `src/bad.sql:1:8: no such column: nope`, `  1 | select nope from users`, `    |        ^^^^`, ``, `✗ 1 of 2 queries failed (_ s)`, ``].join("\n"));
  // A schema that does not load is said once, not under every query.
  writeFileSync(join(root, "db/schema.sql"), "create tabel users (id integer);\n");
  assert.equal(run().out, [`sqlite from db/schema.sql ("schema" in .reylo.yaml)`, `  src/bad.sql   ✗`, `  src/user.sql  ✗`, ``, `Cannot load the schema: db/schema.sql: near "tabel": syntax error`, ``, `✗ 2 of 2 queries failed (_ s)`, ``].join("\n"));
});

test("a script splits into statements at semicolons outside strings, comments, dollar quotes and begin atomic bodies", () => {
  const script = [
    "create table a (s text default 'x;y', \"q;\" int); -- c;",
    "/* outer /* inner; */ ; */ create function f() returns text language plpgsql as $body$ begin return 'a;b'; end $body$;",
    "select E'it\\'s;', $$;$$, $tag$ $$; $tag$;",
    "create function g(x int) returns int language sql begin atomic select case when x > 0 then 1 else 2 end; select x; end;",
    "select 1",
  ].join("\n");
  const parts = statements(script).map(({ start, end }) => script.slice(start, end).trim());
  assert.deepEqual(parts.map((p) => p.split(/\s+/).slice(0, 3).join(" ")), ["create table a", "-- c; /*", "select E'it\\'s;', $$;$$,", "create function g(x", "select 1"]);
  assert.ok(parts[3]!.endsWith("select x; end;"));
  // What the tools write that no SQL loader reads: pg_dump's \restrict lines, SQLite's own table.
  assert.equal(schemaText("\\restrict abc\nselect 1;\n\\unrestrict abc\n", "postgres"), "\nselect 1;\n\n");
  assert.equal(schemaText("CREATE TABLE t (id INTEGER PRIMARY KEY AUTOINCREMENT);\nCREATE TABLE sqlite_sequence(name,seq);\n", "sqlite"), "CREATE TABLE t (id INTEGER PRIMARY KEY AUTOINCREMENT);\n\n");
});
