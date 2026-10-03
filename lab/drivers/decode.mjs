// motherload's per-query decoding (src/decode.ts) against real drivers: pg's Client, Pool and a
// PoolClient in a transaction, pg-promise, Neon's Pool and Client through a local stand-in for
// its WebSocket proxy, and PGlite. Each driver runs the same statement through motherload's `run` and
// directly (the control, which shows the driver's own shapes), then checks that a parser the
// user registered still applies and that the user's own queries are unchanged.
// Needs the local Postgres (DECODE_URL, default postgres://localhost:5432/porg_decode, created
// and dropped here): nub decode.mjs
import net from "node:net";
import { createRequire } from "node:module";
import { isDeepStrictEqual, inspect } from "node:util";
import pg from "pg";
import pgPromise from "pg-promise";
import { PGlite } from "@electric-sql/pglite";
import { defineQuery } from "../../src/index.ts";

const require = createRequire(import.meta.url);
const server = process.env.DECODE_SERVER ?? "postgres://localhost:5432";
const URL = `${server}/porg_decode`;

const schema = `
  create type role as enum ('admin', 'member');
  create domain posint as int check (value > 0);
  create type pair as (a int, b text);`;
const sql = `select array['admin', 'member', null]::role[] as roles, array['a', 'b']::name[] as names,
  array[1, 2]::posint[] as posints, array[row(1, 'x'), row(2, 'y,z')]::pair[] as pairs,
  interval '1 year 2 mons 3 days 04:05:06.789' as span, array[interval '1 day', interval '2 hours'] as spans,
  array[1.5, 12345678901234567890.1]::numeric[] as amounts, 9007199254740993::int8 as big,
  timestamptz '2024-01-15 10:30:00.123+00' as at, :id::int as id`;
const q = defineQuery({ dialect: "postgres", sql, params: ["id"], cardinality: "one" });
const direct = sql.replace(":id", "$1");

// What motherload declares for each column (an enum array is a union array, interval its text, ...).
const declared = {
  roles: ["admin", "member", null],
  names: ["a", "b"],
  posints: [1, 2],
  pairs: '{"(1,x)","(2,\\"y,z\\")"}',
  span: "1 year 2 mons 3 days 04:05:06.789",
  spans: ["1 day", "02:00:00"],
  amounts: ["1.5", "12345678901234567890.1"],
  big: "9007199254740993",
  at: new Date("2024-01-15T10:30:00.123Z"),
  id: 1,
};

let failures = 0;
const show = (v) => inspect(v, { breakLength: Infinity, depth: 4 });
function check(label, ok, detail = "") {
  if (!ok) failures++;
  console.log(`${ok ? "ok  " : "FAIL"} ${label}${detail ? `: ${detail}` : ""}`);
}
function same(label, actual, expected) {
  const diff = Object.keys(expected).filter((k) => !isDeepStrictEqual(actual?.[k], expected[k]));
  check(label, diff.length === 0, diff.length ? diff.map((k) => `${k}=${show(actual?.[k])} (expected ${show(expected[k])})`).join("; ") : "");
}
// The control: the driver alone gives its own shapes for the same statement.
function control(label, row, { pglite = false } = {}) {
  const pgShapes = pglite
    ? typeof row.roles === "string" || (Array.isArray(row.posints) && row.posints[0] === "1") || Array.isArray(row.pairs)
    : typeof row.roles === "string" && typeof row.names === "string" && typeof row.posints === "string" && typeof row.span === "object" && typeof row.amounts[0] === "number";
  check(`${label} control (no motherload)`, pgShapes, `roles=${show(row.roles)} names=${show(row.names)} posints=${show(row.posints)} span=${show(row.span)} amounts=${show(row.amounts)}`);
}

const admin = new pg.Client(`${server}/postgres`);
await admin.connect();
await admin.query("drop database if exists porg_decode");
await admin.query("create database porg_decode");
try {
  const setup = new pg.Client(URL);
  await setup.connect();
  await setup.query(schema);
  await setup.end();

  // pg Client: decoded, the user's own next query unchanged, a client parser and a global parser kept.
  {
    const client = new pg.Client(URL);
    await client.connect();
    control("pg Client", (await client.query(direct, [1])).rows[0]);
    same("pg Client through run", await q.run(client, { id: 1 }), declared);
    same("pg Client through run, parameters as an array", await q.run(client, [1]), declared);
    control("pg Client, the user's own query after motherload's", (await client.query(direct, [1])).rows[0]);
    client.setTypeParser(1184, (x) => `client:${x}`);
    client.setTypeParser(1186, function userInterval(x) {
      return `user:${x}`;
    });
    pg.types.setTypeParser(20, BigInt);
    const row = await q.run(client, { id: 1 });
    check("pg Client: a client timestamptz parser applies to motherload's query", row.at === "client:2024-01-15 02:30:00.123-08" || String(row.at).startsWith("client:"), show(row.at));
    check("pg Client: a global int8 parser (BigInt) applies to motherload's query", row.big === 9007199254740993n, show(row.big));
    check("pg Client: the user's interval parser wins over motherload's text, for interval and interval[]", row.span === "user:1 year 2 mons 3 days 04:05:06.789" && row.spans[0] === "user:1 day", `${show(row.span)} ${show(row.spans)}`);
    pg.types.setTypeParser(20, pg.types.getTypeParser(25));
    // A type created after motherload read the catalog: decoded after one lookup of its oid.
    await client.query("create type late as enum ('x', 'y')");
    const late = defineQuery({ dialect: "postgres", sql: "select array['x', 'y']::late[] as lates, 'x'::late as one", params: [], cardinality: "one" });
    same("pg Client: an enum created after the catalog was read", await late.run(client), { lates: ["x", "y"], one: "x" });
    check("pg Client control: that enum array through the client alone", (await client.query("select array['x', 'y']::late[] as lates")).rows[0].lates === "{x,y}");
    await client.end();
  }

  // pg Pool without a listener (pool.query with types), with a connect listener (a checked-out client), and a PoolClient in a transaction.
  {
    const pool = new pg.Pool({ connectionString: URL, max: 2 });
    control("pg Pool", (await pool.query(direct, [1])).rows[0]);
    same("pg Pool through run", await q.run(pool, { id: 1 }), declared);
    control("pg Pool, the user's own query after motherload's", (await pool.query(direct, [1])).rows[0]);
    const listened = new pg.Pool({ connectionString: URL, max: 2 });
    listened.on("connect", (c) => c.setTypeParser(1184, (x) => `connect:${x}`));
    const row = await q.run(listened, { id: 1 });
    same("pg Pool with a connect listener through run", row, { ...declared, at: row.at });
    check("pg Pool with a connect listener: the listener's parser applies to motherload's query", String(row.at).startsWith("connect:"), show(row.at));
    check("pg Pool with a connect listener: the checked-out client went back", listened.idleCount === listened.totalCount, `idle ${listened.idleCount} of ${listened.totalCount}`);
    // As pool.query does, a failing statement releases the client with its error, which ends it.
    const failing = defineQuery({ dialect: "postgres", sql: "select 1 / 0 as x", params: [], cardinality: "one" });
    const error = await failing.run(listened).then(() => null, (e) => e);
    check("pg Pool with a connect listener: the driver's error comes through, and the client is ended", error?.code === "22012" && listened.totalCount === 0, `${error?.message}; ${listened.totalCount} clients`);
    const tx = await pool.connect();
    await tx.query("begin");
    await tx.query("create type in_tx as enum ('p', 'q')");
    same("pg PoolClient inside a transaction through run", await q.run(tx, { id: 1 }), declared);
    const inTx = defineQuery({ dialect: "postgres", sql: "select array['p']::in_tx[] as v", params: [], cardinality: "one" });
    same("pg PoolClient: an enum created inside the transaction", await inTx.run(tx), { v: ["p"] });
    await tx.query("rollback");
    control("pg PoolClient, the user's own query after motherload's", (await tx.query(direct, [1])).rows[0]);
    tx.release();
    await pool.end();
    await listened.end();
  }

  // pg-promise: the database, a task and a transaction.
  {
    const pgp = pgPromise();
    const db = pgp(URL);
    control("pg-promise", await db.one(direct, [1]).catch(() => db.one({ text: direct, values: [1] })));
    same("pg-promise database through run", await q.run(db, { id: 1 }), declared);
    pgp.pg.types.setTypeParser(1184, (x) => `pgp-global:${x}`);
    const row = await db.tx((t) => q.run(t, { id: 1 }));
    same("pg-promise transaction through run", row, { ...declared, at: row.at });
    check("pg-promise: a global parser (pgp.pg.types) applies to motherload's query", String(row.at).startsWith("pgp-global:"), show(row.at));
    pgp.pg.types.setTypeParser(1184, pg.types.getTypeParser(1082));
    control("pg-promise, the user's own query after motherload's", await db.one({ text: direct, values: [1] }));
    pgp.end();
  }

  // Neon's Pool and Client, through a local stand-in for Neon's WebSocket proxy (as the 2026-10-02 study ran it).
  {
    const { WebSocketServer } = require("./node_modules/.store/ws@8.22.0/node_modules/ws");
    const wss = new WebSocketServer({ port: 18433 });
    wss.on("connection", (ws) => {
      const sock = net.connect(5432, "localhost");
      ws.on("message", (d) => sock.write(d));
      sock.on("data", (d) => ws.send(d));
      ws.on("close", () => sock.end());
      sock.on("close", () => ws.close());
      sock.on("error", () => ws.close());
    });
    const neon = await import("@neondatabase/serverless");
    neon.neonConfig.wsProxy = (host, port) => `localhost:18433/v2?address=${host}:${port}`;
    neon.neonConfig.useSecureWebSocket = false;
    neon.neonConfig.pipelineConnect = false;
    neon.neonConfig.webSocketConstructor = globalThis.WebSocket;
    const neonUrl = `postgres://${process.env.USER}@localhost:5432/porg_decode`;
    const pool = new neon.Pool({ connectionString: neonUrl });
    control("Neon Pool", (await pool.query(direct, [1])).rows[0]);
    same("Neon Pool through run", await q.run(pool, { id: 1 }), declared);
    neon.types.setTypeParser(1184, (x) => `neon-global:${x}`);
    const row = await q.run(pool, { id: 1 });
    check("Neon Pool: Neon's global parser (its own pg-types copy) applies to motherload's query", String(row.at).startsWith("neon-global:"), show(row.at));
    neon.types.setTypeParser(1184, pg.types.getTypeParser(1082));
    control("Neon Pool, the user's own query after motherload's", (await pool.query(direct, [1])).rows[0]);
    const client = new neon.Client({ connectionString: neonUrl });
    await client.connect();
    same("Neon Client through run", await q.run(client, { id: 1 }), declared);
    await client.end();
    await pool.end();
    wss.close();
  }

  // PGlite: types created after the instance started stay text in its own decoding.
  {
    const lite = await PGlite.create();
    await lite.exec(schema);
    control("PGlite", (await lite.query(direct, [1])).rows[0], { pglite: true });
    const row = await q.run(lite, { id: 1 });
    // PGlite's int8 is a number within 2^53 and a BigInt above (motherload declares `number | bigint` for it).
    same("PGlite through run", row, { ...declared, big: 9007199254740993n });
    control("PGlite, the user's own query after motherload's", (await lite.query(direct, [1])).rows[0], { pglite: true });
    // After refreshArrayTypes() PGlite parses the arrays itself, a domain's elements and a composite's as strings.
    await lite.refreshArrayTypes();
    const refreshed = (await lite.query(direct, [1])).rows[0];
    check("PGlite refreshed control (no motherload)", isDeepStrictEqual(refreshed.posints, ["1", "2"]) && Array.isArray(refreshed.pairs), `posints=${show(refreshed.posints)} pairs=${show(refreshed.pairs)}`);
    same("PGlite refreshed through run", await q.run(lite, { id: 1 }), { ...declared, big: 9007199254740993n });
    await lite.close();
    const withParsers = await PGlite.create({ parsers: { 1184: (x) => `lite:${x}` } });
    await withParsers.exec(schema);
    const own = await q.run(withParsers, { id: 1 });
    check("PGlite: an instance parser applies to motherload's query", String(own.at).startsWith("lite:"), show(own.at));
    same("PGlite with instance parsers through run", own, { ...declared, big: 9007199254740993n, at: own.at });
    await withParsers.close();
  }
} finally {
  await admin.query("drop database if exists porg_decode with (force)");
  await admin.end();
}
console.log(failures ? `${failures} failed` : "all passed");
process.exitCode = failures ? 1 : 0;
