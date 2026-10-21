import { test } from "node:test";
import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { DatabaseSync } from "node:sqlite";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { Session } from "../src/analyze.ts";
import { differences, type Fresh } from "../src/fresh.ts";
import { sqlModule } from "../src/loader.ts";

// The schema freshness check (src/fresh.ts, engine/core/src/fresh.rs; docs/structured-params.md §6, the
// maintainer, 2026-10-20: any change fails). Postgres and MySQL run on throwaway servers named by
// REYLO_TEST_PG and REYLO_TEST_MYSQL, each test in a database of its own.
const PG = process.env.REYLO_TEST_PG;
const MYSQL = process.env.REYLO_TEST_MYSQL;
const DRIVERS = fileURLToPath(new URL("../lab/drivers/node_modules", import.meta.url));
const INDEX = pathToFileURL(fileURLToPath(new URL("../src/index.ts", import.meta.url))).href;
const pgLive = !PG ? "REYLO_TEST_PG names no server" : !existsSync(join(DRIVERS, "postgres")) ? "lab/drivers is not installed" : false;
const mysqlLive = !MYSQL ? "REYLO_TEST_MYSQL names no server" : !existsSync(join(DRIVERS, "mysql2")) ? "lab/drivers is not installed" : false;

function project(driver?: string): string {
  const root = mkdtempSync(join(tmpdir(), "reylo-fresh-"));
  writeFileSync(join(root, "package.json"), "{}");
  if (driver) {
    mkdirSync(join(root, "node_modules"));
    symlinkSync(join(DRIVERS, driver), join(root, "node_modules", driver));
  }
  return root;
}

/** A module built for `sql` and imported: each call gives a query object of its own, never checked yet. */
async function built(root: string, url: string, name: string, sql: string) {
  const file = join(root, `${name}.sql`);
  writeFileSync(file, sql);
  writeFileSync(join(root, ".env"), `DATABASE_URL=${url}\n`);
  const { code } = await sqlModule(file, sql);
  const out = join(root, `${name}.mjs`);
  writeFileSync(out, code.replace(`from "reylo"`, `from ${JSON.stringify(INDEX)}`));
  return (await import(`${pathToFileURL(out).href}?${Math.random()}`)).default;
}

const closeAll = async () => {
  for (const session of Session.opened()) await session.close();
};

test("a difference is a change: a column gone, retyped, made nullable or given a default, and a column an insert must now fill; rows of another shape check nothing", () => {
  const fresh: Fresh = {
    query: "…",
    rows: [
      ["users", "id", "integer", "true", "true"],
      ["users", "email", "text", "true", "false"],
    ],
    inserts: ["users"],
  };
  assert.deepEqual(differences(fresh, [["users", "id", "integer", "true", "true"], ["users", "email", "text", "true", "false"], ["users", "bio", "text", "false", "false"]]), []);
  assert.deepEqual(differences(fresh, [{ fresh_table: "users", fresh_column: "id", fresh_type: "integer", fresh_not_null: true, fresh_default: true }]), ["users.email was text not null and no longer exists"]);
  assert.deepEqual(differences(fresh, [["users", "id", "bigint", "true", "true"], ["users", "email", "text", "false", "false"]]), ["users.id was integer not null with a default and is now bigint not null with a default", "users.email was text not null and is now text"]);
  assert.deepEqual(differences(fresh, [...fresh.rows, ["users", "tenant", "integer", "true", "false"]]), ["users.tenant is now integer not null, which the insert does not write"]);
  // A client that answers every query with a row of its own, as lab/nextjs's edge stub does, is no catalog.
  assert.equal(differences(fresh, [{ id: 1, name: "stub", email: null, role: "member", big: "7" }]), undefined);
  assert.equal(differences(fresh, [["users", "id", "integer"]]), undefined);
});

test("SQLite: a query whose columns changed fails, naming them; one whose table gained a column it does not name runs", async () => {
  const root = project();
  const db = new DatabaseSync(join(root, "app.db"));
  db.exec("create table users (id integer primary key, email text not null, name text)");
  db.exec("insert into users (email, name) values ('a@x', 'Ada')");
  const url = "file:./app.db";
  try {
    const read = await built(root, url, "read", "select id, email from users order by id");
    assert.deepEqual((await read.run(db)).map((r: object) => ({ ...r })), [{ id: 1, email: "a@x" }]);
    // A column the statement does not name: no fact, so the query runs on.
    db.exec("alter table users add column bio text");
    const read2 = await built(root, url, "read2", "select id, email from users order by id");
    db.exec("alter table users add column age integer");
    assert.equal((await read2.run(db)).length, 1);
    // A column it names, renamed: the call fails before the statement runs, synchronously too.
    const read3 = await built(root, url, "read3", "select id, email from users order by id");
    db.exec("alter table users rename column email to mail");
    await assert.rejects(read3.run(db), /The database changed after read3\.sql was built: users\.email was TEXT not null and no longer exists\. Build it again/);
    const read4 = await built(root, url, "read4", "select id, name from users order by id");
    db.exec("alter table users rename column name to full_name");
    assert.throws(() => read4.runSync(db), /users\.name was TEXT and no longer exists/);
    // An insert's table that gains a column it must fill.
    const insert = await built(root, url, "insert", "insert into users (mail) values (:mail)");
    db.exec("alter table users add column tenant integer not null default 0");
    await insert.run(db, { mail: "b@x" });
  } finally {
    db.close();
    await closeAll();
  }
});

test("Postgres: a retyped column fails the call, a new nullable one does not, and a new required one fails an insert", { skip: pgLive }, async () => {
  const require = createRequire(join(DRIVERS, "postgres", "package.json"));
  const postgres = require("postgres");
  const admin = postgres(PG, { max: 1, onnotice: () => {} });
  const name = `reylo_fresh_${process.pid}`;
  await admin.unsafe(`drop database if exists ${name}`);
  await admin.unsafe(`create database ${name}`);
  const url = new URL(PG!);
  url.pathname = `/${name}`;
  const sql = postgres(url.href, { max: 1, onnotice: () => {} });
  const root = project("postgres");
  try {
    await sql.unsafe(`create table "Users" (id int8 generated always as identity primary key, email text not null, name text)`);
    const insert = await built(root, url.href, "insert_user", `insert into "Users" (email) values (:email) returning id`);
    await insert.run(sql, { email: "a@x" });
    const read = await built(root, url.href, "read_users", `select id, email from "Users" order by id`);
    await sql.unsafe(`alter table "Users" add column bio text`);
    assert.equal((await read.run(sql)).length, 1);
    const read2 = await built(root, url.href, "read_users2", `select id, email from "Users" order by id`);
    await sql.unsafe(`alter table "Users" alter column email type varchar(100)`);
    await assert.rejects(read2.run(sql), /Users\.email was text not null and is now character varying\(100\) not null/);
    const insert2 = await built(root, url.href, "insert_user2", `insert into "Users" (email) values (:email) returning id`);
    // Rows exist, so the column takes a default to be added, which then goes.
    await sql.unsafe(`alter table "Users" add column tenant int not null default 0; alter table "Users" alter column tenant drop default`);
    await assert.rejects(insert2.run(sql, { email: "b@x" }), /Users\.tenant is now integer not null, which the insert does not write/);
  } finally {
    await closeAll();
    await sql.end();
    await admin.unsafe(`drop database if exists ${name}`);
    await admin.end();
  }
});

test("MySQL: a retyped column fails the call, and a new nullable one does not", { skip: mysqlLive }, async () => {
  const require = createRequire(join(DRIVERS, "mysql2", "package.json"));
  const mysql = require("mysql2/promise");
  const server = new URL(MYSQL!);
  const connect = (database?: string) => mysql.createConnection({ host: server.hostname, port: Number(server.port), user: decodeURIComponent(server.username) || "root", password: decodeURIComponent(server.password), database });
  const admin = await connect();
  const name = `reylo_fresh_${process.pid}`;
  await admin.query(`drop database if exists ${name}`);
  await admin.query(`create database ${name}`);
  const url = new URL(MYSQL!);
  url.pathname = `/${name}`;
  const db = await connect(name);
  const root = project("mysql2");
  try {
    await db.query("create table users (id bigint auto_increment primary key, email varchar(100) not null)");
    await db.query("insert into users (email) values ('a@x')");
    const read = await built(root, url.href, "read_users", "select id, email from users order by id");
    await db.query("alter table users add column bio text");
    assert.equal((await read.run(db)).length, 1);
    const read2 = await built(root, url.href, "read_users2", "select id, email from users order by id");
    await db.query("alter table users modify email varchar(200) not null");
    await assert.rejects(read2.run(db), /users\.email was varchar\(100\) not null and is now varchar\(200\) not null/);
  } finally {
    await closeAll();
    await db.end();
    await admin.query(`drop database if exists ${name}`);
    await admin.end();
  }
});
