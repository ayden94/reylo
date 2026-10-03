import { test } from "node:test";
import assert from "node:assert/strict";
import { EventEmitter } from "node:events";
import { execute, type QueryModule } from "../src/execute.ts";
import { parseArray } from "../src/decode.ts";

// A fake database and pg-family clients that decode as pg does: a Result asks its types table
// (the query's `types`, else the client's) for one parser per field and applies it to the
// field's text. The default parsers carry pg-types 2.2.0's names, which is what decode.ts reads
// (checked against lab/drivers/node_modules/.store/pg-types@2.2.0 and lab/drivers/decode.mjs).
type Parser = (text: string) => unknown;
function noParse(x: string) {
  return String(x);
}
const pgTypesDefaults: Record<number, Parser> = {
  20: function parseBigInteger(x) {
    return x;
  },
  23: function parseInteger(x) {
    return parseInt(x, 10);
  },
  1007: function parseIntegerArray(x) {
    return parseArray(x, Number);
  },
  1184: function parseDate(x) {
    return new Date(x);
  },
  1186: function PostgresInterval(x) {
    return { interval: x };
  },
  1187: function parseIntervalArray(x) {
    return parseArray(x, (s) => ({ interval: s }));
  },
  1231: function parseFloatArray(x) {
    return parseArray(x, parseFloat);
  },
};
const globalTypes = { getTypeParser: (oid: number) => pgTypesDefaults[oid] ?? noParse };

// The catalog: [array oid, element oid, base oid, text (a composite element), delimiter].
const builtin: [number, number, number, boolean, string][] = [
  [1003, 19, 19, false, ","],
  [1007, 23, 23, false, ","],
  [1187, 1186, 1186, false, ","],
  [1231, 1700, 1700, false, ","],
  [1020, 603, 603, false, ";"],
];
const ROLE_ARR = 70000, ROLE = 70001, POSINT_ARR = 70010, POSINT = 70011, PAIR_ARR = 70020, PAIR = 70021, LATE_ARR = 70030, LATE = 70031;
function database() {
  const db = {
    types: [...builtin, [ROLE_ARR, ROLE, ROLE, false, ","], [POSINT_ARR, POSINT, 23, false, ","], [PAIR_ARR, PAIR, PAIR, true, ","]] as typeof builtin,
    catalogQueries: [] as string[],
    late() {
      db.types.push([LATE_ARR, LATE, LATE, false, ","]);
    },
    // What the server answers: fields with their oids and text values, or catalog rows.
    answer(text: string): { fields: { name: string; dataTypeID: number }[]; raw: (string | null)[][] } | Record<string, unknown>[] {
      if (text.includes("pg_catalog.pg_type")) {
        db.catalogQueries.push(text);
        const wanted = /any\('\{([\d,]+)\}'/.exec(text)?.[1]?.split(",").map(Number);
        return db.types
          .filter(([array, elem]) => (wanted ? wanted.includes(array) || wanted.includes(elem) : true))
          .map(([array, elem, base, isText, delim]) => ({ array, elem, base, text: isText, delim }));
      }
      if (text === "select 1") return [{ "?column?": 1 }];
      const columns: [string, number, string | null][] = [
        ["roles", ROLE_ARR, "{admin,member,NULL}"],
        ["names", 1003, "{a,b}"],
        ["posints", POSINT_ARR, "{1,2}"],
        ["pairs", PAIR_ARR, '{"(1,x)"}'],
        ["span", 1186, "1 year 2 mons"],
        ["spans", 1187, '{"1 day","02:00:00"}'],
        ["amounts", 1231, "{1.5,12345678901234567890.1}"],
        ["ints", 1007, "{1,2}"],
        ["big", 20, "9007199254740993"],
        ["at", 1184, "2024-01-15 10:30:00+00"],
        ["boxes", 1020, "{(1,1),(0,0);(2,2),(1,1)}"],
        ["nothing", ROLE_ARR, null],
      ];
      if (text.includes("late")) columns.push(["lates", LATE_ARR, "{x,y}"], ["late", LATE, "x"]);
      return { fields: columns.map(([name, dataTypeID]) => ({ name, dataTypeID })), raw: [columns.map(([, , v]) => v)] };
    },
  };
  return db;
}
type Db = ReturnType<typeof database>;

/** pg's Client: getTypeParser over its own setTypeParser overrides and the global table. */
function pgClient(db: Db) {
  const overrides: Record<number, Parser> = {};
  const client = Object.assign(new EventEmitter(), {
    calls: [] as unknown[][],
    setTypeParser: (oid: number, fn: Parser) => void (overrides[oid] = fn),
    getTypeParser: (oid: number, _format?: string) => overrides[oid] ?? globalTypes.getTypeParser(oid),
    async query(config: string | { text: string; values?: unknown[]; types?: typeof globalTypes }, values?: unknown[]) {
      client.calls.push([config, values]);
      const text = typeof config === "string" ? config : config.text;
      const types = (typeof config === "object" && config.types) || client;
      const answer = db.answer(text);
      if (Array.isArray(answer)) return { rows: answer, fields: [] };
      const parsers = answer.fields.map((f) => types.getTypeParser(f.dataTypeID, "text"));
      return { fields: answer.fields, rows: answer.raw.map((raw) => Object.fromEntries(answer.fields.map((f, i) => [f.name, raw[i] === null ? null : parsers[i]!(raw[i]!)]))) };
    },
  });
  return client;
}

const probe: QueryModule<"postgres"> = { dialect: "postgres", sql: "select * from probe where id = :id", params: ["id"] };
const lateProbe: QueryModule<"postgres"> = { dialect: "postgres", sql: "select *, late from probe where id = :id", params: ["id"] };

// What motherload declares for each column.
const declared = {
  roles: ["admin", "member", null],
  names: ["a", "b"],
  posints: [1, 2],
  pairs: '{"(1,x)"}',
  span: "1 year 2 mons",
  spans: ["1 day", "02:00:00"],
  amounts: ["1.5", "12345678901234567890.1"],
  ints: [1, 2],
  big: "9007199254740993",
  at: new Date("2024-01-15T10:30:00Z"),
  boxes: ["(1,1),(0,0)", "(2,2),(1,1)"],
  nothing: null,
};

test("parseArray: quoting, escapes, NULL, nesting, bounds and box's delimiter", () => {
  assert.deepEqual(parseArray("{}", String), []);
  assert.deepEqual(parseArray('{a,"b,c","NULL",NULL,"",\"q\\"x\\\\y\"}', String), ["a", "b,c", "NULL", null, "", 'q"x\\y']);
  assert.deepEqual(parseArray("{{1,2},{3,NULL}}", Number), [[1, 2], [3, null]]);
  assert.deepEqual(parseArray("[0:1]={7,8}", Number), [7, 8]);
  assert.deepEqual(parseArray('{"(1,x)","(2,\\"y,z\\")"}', String), ["(1,x)", '(2,"y,z")']);
  assert.deepEqual(parseArray("{(1,1),(0,0);(2,2),(1,1)}", String, ";"), ["(1,1),(0,0)", "(2,2),(1,1)"]);
});

test("pg Client: motherload's per-query types decode what pg-types leaves as text or objects; the client's next query is unchanged", async () => {
  const db = database();
  const client = pgClient(db);
  const control = (await client.query("select * from probe")).rows[0]!;
  assert.equal(control.roles, "{admin,member,NULL}", "control: pg leaves an enum array as its text");
  assert.deepEqual(control.span, { interval: "1 year 2 mons" }, "control: pg's interval is an object");
  assert.deepEqual(control.amounts, [1.5, 12345678901234567000], "control: pg's numeric[] is floats");
  const [row] = (await execute(client, probe, { id: 1 })) as Record<string, unknown>[];
  assert.deepEqual(row, declared);
  assert.equal(db.catalogQueries.length, 1);
  await execute(client, probe, { id: 1 });
  assert.equal(db.catalogQueries.length, 1, "the catalog is read once per client");
  assert.deepEqual((await client.query("select * from probe")).rows[0], control, "the user's own queries are unchanged");
  const sent = client.calls.at(-2)![0] as { text: string; values: unknown[]; types: unknown };
  assert.deepEqual([sent.text, sent.values], ["select * from probe where id = $1", [1]]);
});

test("pg Client: a parser the user registered still wins, for the oids motherload takes over and for the others", async () => {
  const db = database();
  const client = pgClient(db);
  client.setTypeParser(1184, (x) => `text:${x}`);
  client.setTypeParser(20, BigInt as unknown as Parser);
  client.setTypeParser(1186, function userInterval(x) {
    return `mine:${x}`;
  });
  client.setTypeParser(ROLE, (x) => x.toUpperCase());
  const [row] = (await execute(client, probe, { id: 1 })) as Record<string, unknown>[];
  assert.equal(row!.at, "text:2024-01-15 10:30:00+00");
  assert.equal(row!.big, 9007199254740993n);
  assert.equal(row!.span, "mine:1 year 2 mons");
  assert.deepEqual(row!.spans, ["mine:1 day", "mine:02:00:00"], "interval[]'s elements use the user's interval parser");
  assert.deepEqual(row!.roles, ["ADMIN", "MEMBER", null], "an enum array's elements use the user's parser for the enum");
  client.setTypeParser(ROLE_ARR, () => "theirs");
  assert.equal(((await execute(client, probe, { id: 1 })) as Record<string, unknown>[])[0]!.roles, "theirs");
});

test("pg Client: a type created after the catalog was read comes back decoded, after one lookup of that oid", async () => {
  const db = database();
  const client = pgClient(db);
  await execute(client, probe, { id: 1 });
  db.late();
  const [row] = (await execute(client, lateProbe, { id: 1 })) as Record<string, unknown>[];
  assert.deepEqual([row!.lates, row!.late], [["x", "y"], "x"]);
  assert.equal(db.catalogQueries.length, 2);
  assert.match(db.catalogQueries[1]!, /any\('\{70030,70031\}'::pg_catalog\.oid\[\]\)/);
  await execute(client, lateProbe, { id: 1 });
  assert.equal(db.catalogQueries.length, 2, "a learned oid is not looked up again");
});

/** pg's Pool: no getTypeParser; query() runs on a client it checks out. */
function pgPool(db: Db) {
  const clients: ReturnType<typeof pgClient>[] = [];
  const released: unknown[] = [];
  class Client {
    options: unknown;
    constructor(options: unknown) {
      this.options = options;
    }
    getTypeParser(oid: number) {
      return globalTypes.getTypeParser(oid);
    }
  }
  const pool = Object.assign(new EventEmitter(), {
    options: { max: 2 },
    Client,
    clients,
    released,
    async connect() {
      const c = pgClient(db);
      pool.emit("connect", c);
      clients.push(c);
      return Object.assign(c, { release: (e?: unknown) => void released.push(e ?? "ok") });
    },
    async query(config: any, values?: unknown[]) {
      const c = await pool.connect();
      try {
        return await c.query(config, values);
      } finally {
        c.release();
      }
    },
  });
  return pool;
}

test("pg Pool without a connect listener: pool.query with types, falling back to a client built from pool.Client", async () => {
  const db = database();
  const pool = pgPool(db);
  const [row] = (await execute(pool, probe, { id: 1 })) as Record<string, unknown>[];
  assert.deepEqual(row, declared);
  assert.ok((pool.clients[1]!.calls[0]![0] as { types?: unknown }).types, "the statement went through pool.query with types");
});

test("pg Pool with a connect listener: a checked-out client, whose own parsers apply, released once; an error releases it with the error", async () => {
  const db = database();
  const pool = pgPool(db);
  pool.on("connect", (c: ReturnType<typeof pgClient>) => c.setTypeParser(1184, (x) => `conn:${x}`));
  const [row] = (await execute(pool, probe, { id: 1 })) as Record<string, unknown>[];
  assert.equal(row!.at, "conn:2024-01-15 10:30:00+00");
  assert.deepEqual(row!.roles, ["admin", "member", null]);
  assert.equal(pool.clients.length, 1, "the catalog and the statement ran on the one checked-out client");
  assert.deepEqual(pool.released, ["ok"]);
  const failure = new Error("boom");
  pool.connect = async () => Object.assign(pgClient(db), { query: async () => Promise.reject(failure), release: (e?: unknown) => void pool.released.push(e ?? "ok") });
  await assert.rejects(() => execute(pool, probe, { id: 1 }) as Promise<unknown>, (e) => e === failure);
  assert.equal(pool.released.at(-1), failure);
});

test("pg-promise: { text, values, types }, falling back to the table of the connection its catalog query ran on", async () => {
  const db = database();
  const conn = pgClient(db);
  conn.setTypeParser(1184, (x) => `pgp:${x}`);
  const calls: unknown[] = [];
  // pg-promise resolves query() to the rows and result() to pg's Result, whose _types is the connection's table.
  const pgp = {
    manyOrNone: async () => [],
    query: async (q: any): Promise<any> => (calls.push(q), (await conn.query(q)).rows),
    result: async (q: string) => Object.assign(await conn.query(q), { _types: { getTypeParser: conn.getTypeParser } }),
  };
  const [row] = (await execute(pgp, probe, { id: 1 })) as Record<string, unknown>[];
  assert.deepEqual(row, { ...declared, at: "pgp:2024-01-15 10:30:00+00" });
  assert.deepEqual(Object.keys(calls[0] as object), ["text", "values", "types"]);
  // Without result() (or a Result without _types) the query is sent as before, undecoded.
  const bare = { manyOrNone: async () => [], query: async (q: any): Promise<any> => (calls.push(q), (await conn.query(q)).rows) };
  assert.equal(((await execute(bare, probe, { id: 1 })) as Record<string, unknown>[])[0]!.roles, "{admin,member,NULL}");
  assert.deepEqual(calls.at(-1), { text: "select * from probe where id = $1", values: [1] });
});

test("PGlite: per-query parsers for the arrays its instance leaves as text, merged over the instance's", async () => {
  const db = database();
  // The instance's parsers: built-in scalars, and an array parser for each array type at start
  // that applies the element's parser (none for a domain or a composite, so their elements stay text).
  const instance: Record<number, (x: string, oid?: number) => unknown> = { 23: (x) => Number(x), 1184: (x) => new Date(x), 1007: (x) => parseArray(x, Number) };
  for (const [array, elem] of [[1003, 19], [POSINT_ARR, POSINT], [PAIR_ARR, PAIR], [1231, 1700], [1187, 1186]] as const) instance[array] = (x) => parseArray(x, (s) => (instance[elem] ? instance[elem]!(s) : s));
  const lite = {
    parsers: instance,
    refreshArrayTypes: async () => {},
    seen: [] as unknown[],
    async query(text: string, values?: unknown[], options?: { parsers?: Record<number, (x: string, oid: number) => unknown> }) {
      lite.seen.push(options);
      const answer = db.answer(text);
      if (Array.isArray(answer)) return { rows: answer, fields: [] };
      const parsers = { ...instance, ...options?.parsers };
      return { fields: answer.fields, rows: answer.raw.map((raw) => Object.fromEntries(answer.fields.map((f, i) => [f.name, raw[i] === null ? null : parsers[f.dataTypeID] ? parsers[f.dataTypeID]!(raw[i]!, f.dataTypeID) : raw[i]]))) };
    },
  };
  const control = (await lite.query("select * from probe")).rows[0]!;
  assert.deepEqual([control.roles, control.posints, control.pairs], ["{admin,member,NULL}", ["1", "2"], ["(1,x)"]], "control: PGlite's own decoding");
  const [row] = (await execute(lite as never, probe, { id: 1 })) as Record<string, unknown>[];
  assert.deepEqual([row!.roles, row!.posints, row!.pairs, row!.names, row!.ints], [["admin", "member", null], [1, 2], '{"(1,x)"}', ["a", "b"], [1, 2]]);
  const parsers = (lite.seen.at(-1) as { parsers: Record<number, unknown> }).parsers;
  assert.deepEqual(Object.keys(parsers).map(Number).sort(), [1020, PAIR_ARR, POSINT_ARR, ROLE_ARR].sort(), "only the arrays the instance leaves as text are passed");
  db.late();
  const [late] = (await execute(lite as never, lateProbe, { id: 1 })) as Record<string, unknown>[];
  assert.deepEqual(late!.lates, ["x", "y"], "a type created after the catalog was read is decoded after one lookup");
});

test("Neon's neon() and a PGlite transaction are called as before, without a decoder", async () => {
  const calls: unknown[][] = [];
  const neon = Object.assign(() => Promise.reject(new Error("not a template")), { query: async (...args: unknown[]) => (calls.push(args), [{ a: 1 }]), transaction: () => {}, unsafe: () => ({}) });
  const tx = { query: async (...args: unknown[]) => (calls.push(args), { rows: [{ a: 1 }], fields: [] }), sql: () => {}, exec: () => {}, rollback: () => {}, listen: () => {}, closed: false };
  assert.deepEqual(await execute(neon as never, probe, { id: 1 }), [{ a: 1 }]);
  assert.deepEqual(await execute(tx as never, probe, { id: 1 }), [{ a: 1 }]);
  assert.deepEqual(calls, [["select * from probe where id = $1", [1]], ["select * from probe where id = $1", [1]]]);
});
