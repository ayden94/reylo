import { test } from "node:test";
import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { DatabaseSync } from "node:sqlite";
import { Session } from "../src/analyze.ts";
import { sqlModule } from "../src/loader.ts";

// Reylo's one representation per SQL type (src/values.ts), on the real drivers, each set up by the
// app against it: the module a build step writes, run on each, must give the same rows, and each
// app's own query keeps the app's settings (the failing control). Postgres and MySQL run on
// throwaway servers (REYLO_TEST_PG, REYLO_TEST_MYSQL), each test in a database of its own; the
// drivers are lab/drivers'.
const PG = process.env.REYLO_TEST_PG;
const MYSQL = process.env.REYLO_TEST_MYSQL;
const DRIVERS = fileURLToPath(new URL("../lab/drivers/node_modules", import.meta.url));
const INDEX = pathToFileURL(fileURLToPath(new URL("../src/index.ts", import.meta.url))).href;
const drivers = existsSync(join(DRIVERS, "pg")) || "lab/drivers is not installed";
const pgLive = !PG ? "REYLO_TEST_PG names no server" : drivers === true ? false : drivers;
const mysqlLive = !MYSQL ? "REYLO_TEST_MYSQL names no server" : drivers === true ? false : drivers;
const require = createRequire(join(DRIVERS, "pg", "package.json"));

function project(...linked: string[]): string {
  const root = mkdtempSync(join(tmpdir(), "reylo-representation-"));
  writeFileSync(join(root, "package.json"), "{}");
  mkdirSync(join(root, "node_modules"));
  for (const driver of linked) symlinkSync(join(DRIVERS, driver), join(root, "node_modules", driver));
  return root;
}

/** The module a build step writes for `sql`, imported. */
async function built(root: string, url: string, name: string, sql: string) {
  const file = join(root, `${name}.sql`);
  writeFileSync(file, sql);
  writeFileSync(join(root, ".env"), `DATABASE_URL=${url}\n`);
  try {
    const { code } = await sqlModule(file, sql);
    const out = join(root, `${name}.mjs`);
    writeFileSync(out, code.replace(`from "reylo"`, `from ${JSON.stringify(INDEX)}`));
    return (await import(pathToFileURL(out).href)).default;
  } finally {
    for (const session of Session.opened()) await session.close();
  }
}

/** Rows compared by value: a driver's own row prototype and a Buffer's are not Reylo's to set. No value Reylo returns is a Date. */
const plain = (v: unknown): unknown => {
  if (v instanceof Date) throw new assert.AssertionError({ message: `a Date in Reylo's rows: ${v.toISOString()}` });
  return v instanceof Uint8Array ? { bytes: [...v] } : Array.isArray(v) ? v.map(plain) : v !== null && typeof v === "object" ? Object.fromEntries(Object.entries(v).map(([k, x]) => [k, plain(x)])) : v;
};

const PG_SCHEMA = `create type mood as enum ('ok', 'sad');
create domain posint as int check (value > 0);
create type pair as (a int, b text);
create table t (
  id int primary key, created_by int8, b bool, i2 int2, f4 float4, f8 float8, n numeric, d date, ts timestamp(6), tstz timestamptz, tm time, iv interval,
  j json, jb jsonb, by bytea, e mood, u uuid, p pair, r int4range,
  a_i4 int4[], a_i8 int8[], a_n numeric[], a_tx text[], a_e mood[], a_dom posint[], a_p pair[], a_ts timestamp[], a_tstz timestamptz[], a_by bytea[], a_iv interval[]
);
insert into t values (1, 9007199254740993, true, 2, 1.5, 2.2, 12345678901234567890.123456789, '2026-10-20', '2026-10-20 10:00:00.123456', '2026-10-20 10:00:00.123456+02', '10:00:00',
  '1 year 2 mons 3 days 04:05:06.789', '{"a":1}', '{"big":1}', '\\x0102ff', 'sad', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', '(1,"x,y")', '[1,5)',
  '{1,NULL,3}', '{9007199254740993}', '{1.5,12345678901234567890.1}', '{a,NULL,"NULL","b,c"}', '{ok,sad}', '{1,2}', array['(1,x)'::pair], '{"2026-10-20 10:00:00.5"}', '{"2026-10-20 10:00:00+00"}', '{"\\\\x01"}', '{"1 day"}'),
  (2, null, null, null, null, null, null, 'infinity', '0044-03-15 12:00:00 BC', '0001-01-01 00:00:00+00', null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null)`;
const PG_SELECT = "select * from t order by id";
const PG_ROWS = [
  {
    id: 1, created_by: 9007199254740993n, b: true, i2: 2, f4: 1.5, f8: 2.2, n: "12345678901234567890.123456789", d: "2026-10-20", ts: "2026-10-20T10:00:00.123456",
    tstz: "2026-10-20T08:00:00.123456Z", tm: "10:00:00", iv: "1 year 2 mons 3 days 04:05:06.789", j: { a: 1 }, jb: { big: 1 }, by: { bytes: [1, 2, 255] }, e: "sad",
    u: "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11", p: '(1,"x,y")', r: "[1,5)", a_i4: [1, null, 3], a_i8: [9007199254740993n], a_n: ["1.5", "12345678901234567890.1"],
    a_tx: ["a", null, "NULL", "b,c"], a_e: ["ok", "sad"], a_dom: [1, 2], a_p: '{"(1,x)"}', a_ts: ["2026-10-20T10:00:00.5"], a_tstz: ["2026-10-20T10:00:00Z"], a_by: [{ bytes: [1] }], a_iv: ["1 day"],
  },
  {
    id: 2, created_by: null, b: null, i2: null, f4: null, f8: null, n: null, d: "infinity", ts: "0044-03-15T12:00:00 BC", tstz: "0001-01-01T00:00:00Z", tm: null, iv: null,
    j: null, jb: null, by: null, e: null, u: null, p: null, r: null, a_i4: null, a_i8: null, a_n: null, a_tx: null, a_e: null, a_dom: null, a_p: null, a_ts: null, a_tstz: null, a_by: null, a_iv: null,
  },
];

test("Postgres: one row on every driver, however the app set its driver up; the app's own queries keep its settings", { skip: pgLive, timeout: 120_000 }, async () => {
  const pg = require("pg");
  const postgres = require("postgres");
  const pgPromise = require("pg-promise");
  const { PGlite } = require("@electric-sql/pglite");
  const admin = new pg.Client(PG);
  await admin.connect();
  const name = `reylo_representation_${process.pid}`;
  await admin.query(`drop database if exists ${name}`);
  await admin.query(`create database ${name}`);
  const url = new URL(PG!);
  url.pathname = `/${name}`;
  const root = project("pg", "postgres");
  const parsers = { 20: pg.types.getTypeParser(20), 1700: pg.types.getTypeParser(1700) };
  try {
    const setup = new pg.Client(url.href);
    await setup.connect();
    await setup.query(PG_SCHEMA);
    await setup.end();
    const q = await built(root, url.href, "all_types", PG_SELECT);
    // pg, with a global int8 parser that returns a string and numeric to a float, a pool table and a connect listener.
    pg.types.setTypeParser(20, (x: string) => `${x}`);
    pg.types.setTypeParser(1700, parseFloat);
    const client = new pg.Client(url.href);
    await client.connect();
    assert.deepEqual(plain(await q.run(client)), PG_ROWS, "pg Client with a global int8 parser");
    const control = (await client.query("select created_by, tstz from t where id = 1")).rows[0];
    assert.equal(control.created_by, "9007199254740993", "the control: the app's own query, an int8 its text");
    assert.ok(control.tstz instanceof Date, "the control: pg's own timestamptz, a Date");
    await client.end();
    const pool = new pg.Pool({ connectionString: url.href, max: 2, types: { getTypeParser: (oid: number, f?: string) => (oid === 1184 ? (x: string) => `app:${x}` : pg.types.getTypeParser(oid, f)) } });
    pool.on("connect", (c: any) => c.setTypeParser(1082, (x: string) => `connect:${x}`));
    assert.deepEqual(plain(await q.run(pool)), PG_ROWS, "pg Pool");
    const tx = await pool.connect();
    await tx.query("begin");
    assert.deepEqual(plain(await q.run(tx)), PG_ROWS, "pg PoolClient in a transaction");
    await tx.query("rollback");
    tx.release();
    await pool.end();
    // binary: true, which pg 8 reads wrong, is refused.
    const binary = new pg.Client({ connectionString: url.href, binary: true });
    await binary.connect();
    await assert.rejects(q.run(binary), /binary: true/);
    await binary.end();
    // pg-promise: the database and a transaction; its receive event is refused.
    const pgp = pgPromise();
    const db = pgp(url.href);
    assert.deepEqual(plain(await q.run(db)), PG_ROWS, "pg-promise database");
    assert.deepEqual(plain(await db.tx((t: unknown) => q.run(t))), PG_ROWS, "pg-promise transaction");
    pgp.end();
    const camel = pgPromise({ receive: ({ data }: { data: Record<string, unknown>[] }) => data.forEach((r) => ((r.createdBy = r.created_by), delete r.created_by)) });
    const cdb = camel(url.href);
    await assert.rejects(q.run(cdb), /receive event rewrites every result/);
    camel.end();
    // postgres.js with BigInt and camelCase: Reylo's rows keep created_by, and its values.
    const sql = postgres(url.href, { types: { bigint: postgres.BigInt }, transform: postgres.camel, onnotice: () => {} });
    assert.deepEqual(plain(await q.run(sql)), PG_ROWS, "postgres.js with transform camel and types bigint");
    assert.deepEqual(plain(await sql.begin((t: unknown) => q.run(t))), PG_ROWS, "postgres.js transaction");
    const [own] = await sql`select created_by from t where id = 1`;
    assert.deepEqual(own, { createdBy: 9007199254740993n }, "the control: the app's own query");
    await sql.end();
    const rowTransform = postgres(url.href, { transform: { row: (r: unknown) => ({ wrapped: r }) }, onnotice: () => {} });
    await assert.rejects(q.run(rowTransform), /transform\.row rewrites every row/);
    await rowTransform.end();
    // PGlite, with the same schema and the app's own parsers on the instance.
    const lite = await PGlite.create({ parsers: { 20: (x: string) => Number(x), 1184: (x: string) => new Date(x) } });
    await lite.exec(PG_SCHEMA);
    assert.deepEqual(plain(await q.run(lite)), PG_ROWS, "PGlite");
    assert.deepEqual(plain(await q.run(lite)), PG_ROWS, "PGlite, a later run");
    assert.deepEqual(plain(await lite.transaction((t: unknown) => q.run(t))), PG_ROWS, "PGlite transaction");
    const liteControl = (await lite.query("select created_by, tstz from t where id = 1")).rows[0];
    assert.deepEqual([typeof liteControl.created_by, liteControl.tstz instanceof Date], ["number", true], "the control: the app's own query");
    await lite.close();
  } finally {
    pg.types.setTypeParser(20, parsers[20]);
    pg.types.setTypeParser(1700, parsers[1700]);
    await admin.query(`drop database if exists ${name} with (force)`);
    await admin.end();
  }
});

const MYSQL_SCHEMA = [
  "create table t (id int primary key, a datetime(3), a0 datetime, c date, d bigint unsigned, d2 bigint, e decimal(30,18), f json, g bit(1), h bit(8), i set('x','y','z'), j enum('a','b'), k tinyint(1), l year, n double, o varbinary(4), q time(2), s text)",
  "insert into t values (1, '2026-03-08 09:31:00.000', '2026-03-08 09:31:00', '2026-03-08', 18446744073709551615, -9007199254740993, 1.100000000000000001, '{\"k\": [1, 2]}', b'1', b'10000001', 'x,z', 'b', 1, 2026, 2.2, x'0102ff', '10:00:00.5', 'hi'), (2, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null, null)",
  // A TINYINT(1) holds any TINYINT: the failing control of a boolean column.
  "create table odd (id int primary key, ok boolean)",
  "insert into odd values (1, 2)",
];
const MYSQL_ROWS = [
  { id: 1, a: "2026-03-08T09:31:00.000", a0: "2026-03-08T09:31:00", c: "2026-03-08", d: 18446744073709551615n, d2: -9007199254740993n, e: "1.100000000000000001", f: { k: [1, 2] }, g: true, h: { bytes: [129] }, i: "x,z", j: "b", k: true, l: 2026, n: 2.2, o: { bytes: [1, 2, 255] }, q: "10:00:00.50", s: "hi", n2: 2n, first: true },
  { id: 2, a: null, a0: null, c: null, d: null, d2: null, e: null, f: null, g: null, h: null, i: null, j: null, k: null, l: null, n: null, o: null, q: null, s: null, n2: 2n, first: false },
];

test("MySQL: one row on mysql2 and mariadb, however the app set them up; mysql2's decimalNumbers gives Reylo the exact DECIMAL", { skip: mysqlLive, timeout: 120_000 }, async () => {
  const mysql = require("mysql2/promise");
  const mariadb = require("mariadb");
  const admin = await mysql.createConnection(MYSQL);
  const name = `reylo_representation_${process.pid}`;
  await admin.query(`drop database if exists ${name}`);
  await admin.query(`create database ${name}`);
  const url = new URL(MYSQL!);
  url.pathname = `/${name}`;
  const root = project("mysql2");
  try {
    const setup = await mysql.createConnection(url.href);
    for (const statement of MYSQL_SCHEMA) await setup.query(statement);
    await setup.end();
    const q = await built(root, url.href, "all_types", "select t.*, count(*) over () as n2, t.id = 1 as first from t order by id");
    const odd = await built(root, url.href, "odd", "select ok from odd");
    const refused = /Column "ok" is a boolean, stored as 0 or 1, and holds 2/;
    const hostile = mysql.createPool({ uri: url.href, decimalNumbers: true, jsonStrings: true, rowsAsArray: true, supportBigNumbers: false, dateStrings: ["DATE"], typeCast: (f: { type: string }, next: () => unknown) => (f.type === "TINY" ? next() === 1 : next()) });
    assert.deepEqual(plain(await q.run(hostile)), MYSQL_ROWS, "mysql2 pool with decimalNumbers, jsonStrings, rowsAsArray, supportBigNumbers off and a typeCast");
    const [[own]] = await hostile.query("select e, d2 from t where id = 1");
    assert.deepEqual(own, [1.1, -9007199254740992], "the control: the app's own query, lossy");
    await assert.rejects(odd.run(hostile), refused);
    assert.deepEqual((await hostile.query("select ok from odd"))[0], [[false]], "the control: the app's own typeCast reads the 2 as false");
    const conn = await hostile.getConnection();
    await conn.beginTransaction();
    assert.deepEqual(plain(await q.run(conn)), MYSQL_ROWS, "mysql2 PoolConnection in a transaction");
    await conn.rollback();
    conn.release();
    await hostile.end();
    const pool = mariadb.createPool({ host: url.hostname, port: Number(url.port), user: url.username || "root", password: decodeURIComponent(url.password), database: name, bigIntAsNumber: true, decimalAsNumber: true, jsonStrings: true, rowsAsArray: true, metaAsArray: true, connectionLimit: 2 });
    assert.deepEqual(plain(await q.run(pool)), MYSQL_ROWS, "mariadb pool with bigIntAsNumber, decimalAsNumber, jsonStrings, rowsAsArray and metaAsArray");
    await assert.rejects(odd.run(pool), refused);
    await pool.end();
  } finally {
    await admin.query(`drop database if exists ${name}`);
    await admin.end();
  }
});

test("SQLite: one row on every driver, whatever its integer setting; an integer past 2^53 and a BOOLEAN column's 2 an error naming its column", { skip: drivers === true ? false : drivers, timeout: 60_000 }, async () => {
  const Database = require("better-sqlite3");
  const { createClient } = require("@libsql/client");
  const sqlite3 = require("sqlite3");
  const { open } = require("sqlite");
  const root = project();
  const file = join(root, "app.db");
  const setup = new DatabaseSync(file);
  setup.exec(`create table t (id integer primary key, n integer not null, r real, s text, b blob, flag boolean, x, z numeric);
    insert into t values (1, 5, 1.5, 'hi', x'0102ff', 1, 7, 2.5), (2, 6, null, null, null, 0, x'01', 'abc');
    create table big (id integer primary key, v integer); insert into big values (1, 9007199254740993);
    create table odd (id integer primary key, ok boolean); insert into odd values (1, 2)`);
  setup.close();
  // A column declared BOOLEAN and a comparison are booleans.
  const q = await built(root, "file:./app.db", "all_types", "select *, n > 5 as more from t order by id");
  const big = await built(root, "file:./app.db", "big", "select id, v from big");
  const odd = await built(root, "file:./app.db", "odd", "select ok from odd");
  // libSQL's intMode "string" hands the 2 over as its digits.
  const refused = /Column "ok" is a boolean, stored as 0 or 1, and holds "?2"?, which is neither/;
  const rows = [
    { id: 1, n: 5, r: 1.5, s: "hi", b: { bytes: [1, 2, 255] }, flag: true, x: 7, z: 2.5, more: false },
    { id: 2, n: 6, r: null, s: null, b: null, flag: false, x: { bytes: [1] }, z: "abc", more: true },
  ];
  const bigError = /Column "v" holds .*9007199254740992|Column "v" holds 9007199254740993/;
  // better-sqlite3 with every integer a BigInt: Reylo's statement reads numbers, the app's keep its setting.
  const db = new Database(file);
  db.defaultSafeIntegers(true);
  assert.deepEqual(plain(q.runSync(db)), rows, "better-sqlite3 with defaultSafeIntegers(true)");
  assert.deepEqual(plain(db.transaction(() => q.runSync(db))()), rows, "better-sqlite3 in db.transaction");
  assert.equal(typeof db.prepare("select n from t").get().n, "bigint", "the control: the app's own statement");
  assert.throws(() => big.runSync(db), bigError);
  assert.throws(() => odd.runSync(db), refused);
  assert.equal(db.prepare("select ok from odd").get().ok, 2n, "the control: SQLite lets the 2 in");
  db.close();
  const node = new DatabaseSync(file, { readBigInts: true });
  assert.deepEqual(plain(await q.run(node)), rows, "node:sqlite with readBigInts");
  await assert.rejects(big.run(node), bigError);
  await assert.rejects(odd.run(node), refused);
  node.close();
  for (const intMode of ["number", "bigint", "string"]) {
    const client = createClient({ url: `file:${file}`, intMode });
    // Under intMode "string" an integer in a column with no type is its digits (wiki/driver-decoding.md).
    if (intMode !== "string") assert.deepEqual(plain(await q.run(client)), rows, `libSQL with intMode ${intMode}`);
    await assert.rejects(odd.run(client), refused);
    client.close();
  }
  const s3 = new sqlite3.Database(file);
  assert.deepEqual(plain(await q.run(s3)), rows, "sqlite3");
  await assert.rejects(big.run(s3), /Column "v" holds an integer past .* already rounded to 9007199254740992/);
  await assert.rejects(odd.run(s3), refused);
  await new Promise((resolve) => s3.close(resolve));
  const wrapper = await open({ filename: file, driver: sqlite3.Database });
  assert.deepEqual(plain(await q.run(wrapper)), rows, "sqlite");
  await wrapper.close();
});
