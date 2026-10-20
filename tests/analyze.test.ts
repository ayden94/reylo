import { test } from "node:test";
import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, renameSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { DatabaseSync } from "node:sqlite";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { Session } from "../src/analyze.ts";
import { sqlModule } from "../src/loader.ts";

// A throwaway Postgres server for the tests that need one, never a database anyone uses: each such
// test makes a database of its own there and drops it after. The driver is the labs', never the
// package's own.
const PG = process.env.REYLO_TEST_PG;
const DRIVERS = fileURLToPath(new URL("../lab/drivers/node_modules", import.meta.url));
const live = !PG ? "REYLO_TEST_PG names no server" : !existsSync(join(DRIVERS, "postgres")) ? "lab/drivers is not installed" : false;

// A SQLite project: a package, its database, and DATABASE_URL naming it.
function project(): { root: string; env: Record<string, string> } {
  const root = mkdtempSync(join(tmpdir(), "reylo-analyze-"));
  writeFileSync(join(root, "package.json"), "{}");
  const db = new DatabaseSync(join(root, "app.db"));
  db.exec("create table users (id integer primary key, name text not null)");
  db.close();
  return { root, env: { DATABASE_URL: "file:./app.db" } };
}

// Each session stands for a process of its own: the type check, a build, reylo generate.
async function analyze(root: string, env: Record<string, string>, sql: string, options: { force?: boolean } = {}) {
  const session = await Session.open(root, env);
  try {
    return await session.analyze(sql, options);
  } finally {
    await session.close();
  }
}

const SELECT = "select id, name from users where id = :id";

test("a statement is analyzed against the database once, and read from .reylo/ by every process after", async () => {
  const { root, env } = project();
  const first = await analyze(root, env, SELECT);
  assert.ok(first.ok);
  assert.equal(first.source, "database");
  assert.equal(first.analysis.cardinality, "maybeOne");
  assert.ok(first.entry.startsWith(join(root, ".reylo", "queries")));
  assert.ok(existsSync(first.entry));
  // The directory keeps itself out of git.
  assert.match(readFileSync(join(root, ".reylo", ".gitignore"), "utf8"), /^\*$/m);
  const second = await analyze(root, env, SELECT);
  assert.ok(second.ok);
  assert.deepEqual([second.source, second.entry, second.analysis], ["cache", first.entry, first.analysis]);
});

test("a change to the schema makes the entry stale, and the statement is analyzed again", async () => {
  const { root, env } = project();
  const before = await analyze(root, env, "select * from users");
  assert.ok(before.ok);
  assert.deepEqual(before.analysis.columns.map((c) => c.name), ["id", "name"]);
  const db = new DatabaseSync(join(root, "app.db"));
  db.exec("alter table users add column email text");
  db.close();
  const after = await analyze(root, env, "select * from users");
  assert.ok(after.ok);
  assert.equal(after.source, "database");
  assert.deepEqual(after.analysis.columns.map((c) => c.name), ["id", "name", "email"]);
  assert.equal(after.entry, before.entry);
});

test("without the database, .reylo/ answers as it is, and a statement it lacks is an error that names reylo generate", async () => {
  const { root, env } = project();
  assert.ok((await analyze(root, env, SELECT)).ok);
  renameSync(join(root, "app.db"), join(root, "gone.db"));
  const cached = await analyze(root, env, SELECT);
  assert.ok(cached.ok);
  assert.equal(cached.source, "cache");
  const missing = await analyze(root, env, "select name from users");
  assert.ok(!missing.ok);
  assert.equal(missing.code, 5);
  assert.match(missing.message, /^Cannot open the database .*app\.db .*\.reylo has no analysis of this statement, so run reylo generate where the database can be reached\.$/);
  // --force asks the database even for a statement .reylo/ holds.
  const forced = await analyze(root, env, SELECT, { force: true });
  assert.ok(!forced.ok);
});

test("the module a build step emits carries the analysis and reads nothing at run time", async () => {
  const { root, env } = project();
  const file = join(root, "user.sql");
  writeFileSync(file, SELECT);
  // The build steps share one session per project; this one finds the database through .env.
  writeFileSync(join(root, ".env"), `DATABASE_URL=${env.DATABASE_URL}\n`);
  try {
    const { code, entry } = await sqlModule(file, SELECT);
    assert.ok(existsSync(entry));
    assert.equal(code, [
      `import { defineQuery } from "reylo";`,
      `export const sql = ${JSON.stringify(SELECT)};`,
      `export const params = ["id"];`,
      `export default defineQuery({ dialect: "sqlite", sql, params, cardinality: "maybeOne", text: "select id, name from users where id = ?", order: ["id"], inputs: {"id":{"type":{"kind":"scalar","name":"integer"}}}, jsonSchema: {"input":{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object","properties":{"id":{"type":"integer","format":"int64"}},"required":["id"],"additionalProperties":false},"output":{"$schema":"https://json-schema.org/draft/2020-12/schema","anyOf":[{"type":"object","properties":{"id":{"type":"integer","format":"int64"},"name":{"type":"string"}},"required":["id","name"]},{"type":"null"}]}} });`,
      ``,
    ].join("\n"));
    // A build (a process of its own) that has neither the database nor the entry fails instead of guessing.
    for (const session of Session.opened()) await session.close();
    renameSync(join(root, "app.db"), join(root, "gone.db"));
    await assert.rejects(sqlModule(file, "select name from users"), /user\.sql: Cannot open the database .*run reylo generate/);
  } finally {
    for (const session of Session.opened()) await session.close();
  }
});

test("a parameter written twice is an error at each later use, which the type check places and a build fails on; a with reads the value twice", async () => {
  const { root, env } = project();
  writeFileSync(join(root, ".env"), `DATABASE_URL=${env.DATABASE_URL}\n`);
  const twice = "select id, name from users\nwhere name like :q or name = :q or id = :q";
  try {
    const session = await Session.for(root);
    const outcome = await session.analyze(twice);
    assert.ok(outcome.ok);
    // Byte offsets, as the content mapper hands them to the type check: the second and third :q.
    const second = twice.indexOf(":q or id");
    assert.deepEqual(outcome.analysis.diagnostics.map((d) => [d.code, d.start, d.length]), [[3, second, 2], [3, twice.lastIndexOf(":q"), 2]]);
    assert.equal(outcome.analysis.diagnostics[0]!.message, "The parameter :q appears more than once; use each parameter once, and select it in a with to read it twice, as in: with input as (select :q as q) select … from input");
    const file = join(root, "search.sql");
    writeFileSync(file, twice);
    await assert.rejects(sqlModule(file, twice), /search\.sql: The parameter :q appears more than once/);
    // SQLite's own forms and its numbered placeholder are names too; a bare ? never repeats.
    for (const sql of ["select id from users where name = @q or name = @q", "select id from users where id = ?1 or id = ?1", "select id from users where id = ? or id = ?1"]) {
      const refused = await session.analyze(sql);
      assert.ok(refused.ok);
      assert.deepEqual(refused.analysis.diagnostics.map((d) => d.code), [3], sql);
    }
    const bare = await session.analyze("select id from users where id = ? or id = ?");
    assert.ok(bare.ok);
    assert.deepEqual(bare.analysis.diagnostics, []);
    const once = "with input as (select cast(:q as text) as q) select id, name from users, input where input.q is null or name like input.q or name = input.q";
    const read = await session.analyze(once);
    assert.ok(read.ok);
    assert.deepEqual(read.analysis.diagnostics, []);
    assert.deepEqual(read.analysis.params.map((p) => [p.name, p.nullable]), [["q", true]]);
    writeFileSync(file, once);
    const { code } = await sqlModule(file, once);
    assert.match(code, /export const params = \["q"\];/);
  } finally {
    for (const session of Session.opened()) await session.close();
  }
});

test("the Postgres fingerprint runs on the server, holds while nothing changes, and moves with each catalog the analysis reads", { skip: live }, async () => {
  const postgres = createRequire(join(DRIVERS, "postgres", "package.json"))("postgres");
  const admin = postgres(PG, { max: 1, onnotice: () => {} });
  const name = `reylo_fingerprint_${process.pid}`;
  await admin.unsafe(`drop database if exists ${name}`);
  await admin.unsafe(`create database ${name}`);
  const url = new URL(PG!);
  url.pathname = `/${name}`;
  const db = postgres(url.href, { max: 1, onnotice: () => {} });
  const root = mkdtempSync(join(tmpdir(), "reylo-fingerprint-"));
  writeFileSync(join(root, "package.json"), "{}");
  mkdirSync(join(root, "node_modules"));
  symlinkSync(join(DRIVERS, "postgres"), join(root, "node_modules", "postgres"));
  // A session per call, as a process of its own, so that none answers from a fingerprint it took.
  const fingerprint = async () => {
    const session = await Session.open(root, { DATABASE_URL: url.href });
    try {
      return (await session.schema()).value;
    } finally {
      await session.close();
    }
  };
  try {
    await db.unsafe(`create table a (x int, s text); create table b (x int, s text);
      create function skip() returns trigger language plpgsql as 'begin return null; end'`);
    // A query the server refuses gives "unverified <pid> <time>", a value no entry ever matches.
    const first = await fingerprint();
    assert.match(first ?? "", /^[0-9a-f]{16}$/);
    assert.equal(await fingerprint(), first);
    // Each change touches only a catalog the row count or the trust check reads: rows of b are read
    // by a scan of a, a BEFORE ROW trigger can skip an INSERT's row and a rule can replace it, a
    // key holds only under the index's collation, and a user's operator is not the built-in one.
    let last = first;
    for (const change of [
      "alter table b inherit a",
      "create trigger skip before insert on a for each row execute function skip()",
      "alter table a disable trigger skip",
      "create rule nothing as on insert to b do instead nothing",
      `alter table a alter column s type text collate "C"`,
      "create operator === (leftarg = int, rightarg = int, function = int4eq)",
    ]) {
      await db.unsafe(change);
      const next = await fingerprint();
      assert.notEqual(next, last, change);
      assert.equal(await fingerprint(), next, change);
      last = next;
    }
  } finally {
    await db.end();
    await admin.unsafe(`drop database if exists ${name}`);
    await admin.end();
  }
});
