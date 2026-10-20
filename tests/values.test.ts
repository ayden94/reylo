import { test } from "node:test";
import assert from "node:assert/strict";
import { DatabaseSync } from "node:sqlite";
import { execute, type QueryModule } from "../src/execute.ts";
import { renderType } from "../src/render.ts";
import { typeSchema } from "../src/schema.ts";
import { coerce, columnKind, DecodeError, kindType, scalarKind, textDecoder, timestamptz } from "../src/values.ts";
import type { EngineType } from "../src/engine-type.ts";

// The one table (src/values.ts): the kind of each SQL type, the TypeScript type the module declares
// for it, the JSON Schema of its JSON form, and the value the run time gives it, which must agree.

const scalar = (name: string): EngineType => ({ kind: "scalar", name });
/** The TypeScript type a value is, as the declared type names it. */
function typeOf(v: unknown): string {
  if (v === null) return "null";
  if (v instanceof Date) return "Date";
  if (v instanceof Uint8Array) return "Uint8Array";
  if (Array.isArray(v)) return `${typeOf(v.find((x) => x !== null))}[]`;
  return typeof v;
}

// A Postgres value's text, as the server writes it, for every type whose kind is not its text.
const TEXTS: Record<string, string> = {
  bool: "t", int2: "2", int4: "4", oid: "26", float4: "1.1", float8: "2.2", int8: "9007199254740993", numeric: "1.100000000000000001",
  money: "$1.50", date: "2026-10-20", timestamp: "2026-10-20 10:00:00.123456", timestamptz: "2026-10-20 10:00:00.123456+02",
  time: "10:00:00", timetz: "10:00:00+02", interval: "1 day", json: '{"a":1}', jsonb: "[1,2]", bytea: "\\x0102ff", uuid: "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11",
  text: "hi", point: "(1,2)", int4range: "[1,5)", xid: "7", void: "",
};

test("every Postgres type's decoded value is the type the module declares for it, and its JSON form is its schema's", () => {
  for (const [name, text] of Object.entries(TEXTS)) {
    const t = scalar(name);
    const kind = columnKind(t, "postgres");
    const value = textDecoder(kind)(text);
    const declared = renderType(t, false, "postgres");
    if (declared !== "unknown") assert.equal(typeOf(value), declared, name);
    // The array of it, NULL elements null.
    const array: EngineType = { kind: "array", of: t };
    const values = textDecoder(columnKind(array, "postgres"))(`{${JSON.stringify(text)},NULL}`) as unknown[];
    assert.equal(values[1], null, name);
    if (declared !== "unknown") assert.equal(`${typeOf(values[0])}[]`, renderType(array, false, "postgres"), name);
    // What JSON.stringify writes of the value has the schema's JSON type.
    const schema = typeSchema(t, "postgres") as { type?: string | string[] };
    if (schema.type !== undefined) {
      const json = JSON.parse(JSON.stringify(value instanceof Uint8Array ? Buffer.from(value).toString("base64") : value));
      const jsonType = typeof json === "number" ? (Number.isInteger(json) ? ["integer", "number"] : ["number"]) : [typeof json];
      assert.ok([schema.type].flat().some((t) => jsonType.includes(t)), `${name}: ${JSON.stringify(json)} against ${JSON.stringify(schema)}`);
    }
  }
});

test("the representation, value by value", () => {
  const d = (name: string, text: string) => textDecoder(scalarKind(name, "postgres"))(text);
  assert.equal(d("int8", "9007199254740993"), "9007199254740993");
  assert.equal(d("numeric", "12345678901234567890.123456789"), "12345678901234567890.123456789");
  assert.equal(d("date", "2026-10-20"), "2026-10-20");
  assert.equal(d("timestamp", "2026-10-20 10:00:00.123456"), "2026-10-20T10:00:00.123456");
  assert.equal(d("timestamp", "0044-03-15 12:00:00 BC"), "0044-03-15T12:00:00 BC");
  assert.equal(d("timestamp", "infinity"), "infinity");
  assert.equal(d("interval", "1 year 2 mons"), "1 year 2 mons");
  assert.deepEqual(d("bytea", "\\x0102ff"), Buffer.from([1, 2, 255]));
  assert.deepEqual([...(d("bytea", "\\001\\\\x") as Uint8Array)], [1, 92, 120], "the escape format too");
  assert.deepEqual(d("jsonb", '{"big":9007199254740993}'), { big: 9007199254740992 });
  assert.equal(d("bool", "f"), false);
  assert.ok(Number.isNaN(d("float8", "NaN") as number));
  assert.equal(d("float8", "-Infinity"), -Infinity);
});

test("timestamptz: Reylo's parser, which new Date(text) gets wrong for BC, year 1 and offsets with seconds", () => {
  for (const [text, iso] of [
    ["2026-10-20 03:03:00.123456-07", "2026-10-20T10:03:00.123Z"],
    ["0001-01-01 00:00:00+00", "0001-01-01T00:00:00.000Z"],
    ["1883-11-18 12:00:00-07:52:58", "1883-11-18T19:52:58.000Z"],
    ["0044-03-15 12:00:00+00 BC", "-000043-03-15T12:00:00.000Z"],
    ["12345-01-01 00:00:00+05:30", "+012344-12-31T18:30:00.000Z"],
  ] as const) assert.equal(timestamptz(text).toISOString(), iso, text);
  assert.ok(Number.isNaN(timestamptz("infinity").getTime()));
  // The failing control: what postgres.js and PGlite use.
  assert.equal(new Date("0001-01-01 00:00:00+00").getUTCFullYear(), 2001);
});

test("SQLite: the kinds, and a NUMERIC column of either", () => {
  assert.deepEqual(["integer", "real", "text", "blob", "numeric", "bool", "mystery"].map((n) => scalarKind(n, "sqlite")), ["int", "float", "text", "bytes", "int|float|text", "bool", "unknown"]);
  assert.deepEqual(["int", "int|float|text", "unknown", "bytes"].map(kindType), ["number", "number | string", "unknown", "Uint8Array"]);
});

test("coerce: a value a driver decoded, given its kind; an integer a number cannot hold is refused, naming the column", () => {
  assert.equal(coerce("int8", 9007199254740993n, "c"), "9007199254740993");
  assert.equal(coerce("date", new Date("2026-10-20T00:00:00Z"), "c"), "2026-10-20");
  assert.equal(coerce("timestamp", new Date("2026-10-20T10:00:00.500Z"), "c"), "2026-10-20T10:00:00.5");
  assert.deepEqual(coerce("int[]", Int32Array.of(1, 2), "c"), [1, 2]);
  assert.deepEqual(coerce("text[]", "{a,NULL}", "c"), ["a", null]);
  assert.equal(coerce("text", ["x", "z"], "c"), "x,z", "mariadb's SET");
  assert.deepEqual(coerce("bytes", true, "c"), Uint8Array.of(1), "Bun.sql's and mariadb's BIT(1)");
  assert.deepEqual(coerce("bytes", [1, 2], "c"), Uint8Array.of(1, 2), "D1's BLOB");
  assert.deepEqual(coerce("bytes", Uint8Array.of(3).buffer, "c"), Uint8Array.of(3), "libSQL's BLOB");
  assert.equal(coerce("bool", 1n, "c"), true);
  assert.deepEqual(coerce("json", '{"a":1}', "c"), { a: 1 });
  assert.equal(coerce("int", 9007199254740991n, "c"), 9007199254740991);
  assert.throws(() => coerce("int", 9007199254740993n, "big_id"), (e: Error) => e instanceof DecodeError && /^Column "big_id" holds 9007199254740993, an integer past ±\(2\^53 - 1\)/.test(e.message));
  assert.throws(() => coerce("int", 9007199254740992, "big_id"), /Column "big_id" holds an integer past ±\(2\^53 - 1\), which the driver already rounded to 9007199254740992/);
  // A NUMERIC may hold a REAL past 2^53, which is no rounded integer.
  assert.equal(coerce("int|float|text", 1e20, "c"), 1e20);
  assert.equal(coerce("int8|int", 1n, "c"), "1", "a MySQL comparison, a BIGINT");
});

test("SQLite: Reylo's own statement per database and text, with integers read as BigInts whatever the database's setting, and the app's statements unchanged", () => {
  const db = new DatabaseSync(":memory:", { readBigInts: true, returnArrays: true });
  db.exec("create table t (id integer primary key, n integer, r real, b blob, doc text, flag integer, x); insert into t values (1, 5, 1.5, x'0102', '{\"a\":1}', 1, 7), (2, 9007199254740993, null, null, null, 0, null)");
  const q: QueryModule<"sqlite"> = { dialect: "sqlite", sql: "select id, n, r, b, doc, flag, x from t where id = :id", params: ["id"], columns: [["id", "int"], ["n", "int"], ["r", "float"], ["b", "bytes"], ["doc", "json"], ["flag", "bool"], ["x", "unknown"]] };
  const [row] = execute(db, q, { id: 1 }) as Record<string, unknown>[];
  assert.deepEqual({ ...row }, { id: 1, n: 5, r: 1.5, b: Uint8Array.of(1, 2), doc: { a: 1 }, flag: true, x: 7 });
  // The failing control: the app's own statement keeps its database's settings.
  assert.deepEqual(db.prepare("select n from t where id = 1").get(), [5n]);
  assert.throws(() => execute(db, q, { id: 2 }), (e: Error) => e instanceof DecodeError && e instanceof TypeError && /^Column "n" holds 9007199254740993/.test(e.message));
  // A query defined by hand: integers exact, nothing else known.
  const [bare] = execute(db, { dialect: "sqlite", sql: "select n, doc from t where id = 1", params: [] }) as Record<string, unknown>[];
  assert.deepEqual({ ...bare }, { n: 5, doc: '{"a":1}' });
});

test("SQLite: the asynchronous drivers' rows normalized: libSQL's BigInt, string and ArrayBuffer, D1's array, a rounded integer refused", async () => {
  const q: QueryModule<"sqlite"> = { dialect: "sqlite", sql: "select n, b, doc from t", params: [], columns: [["n", "int"], ["b", "bytes"], ["doc", "json"]] };
  const libsql = (rows: unknown[]) => ({ execute: async () => ({ rows }), executeMultiple: async () => {} });
  assert.deepEqual(await execute(libsql([{ n: 5n, b: Uint8Array.of(1).buffer, doc: "[1]" }]), q), [{ n: 5, b: Uint8Array.of(1), doc: [1] }]);
  assert.deepEqual(await execute(libsql([{ n: "5", b: null, doc: null }]), q), [{ n: 5, b: null, doc: null }], "intMode: string");
  const d1 = { batch: async () => [], prepare: () => ({ first: async () => null, bind: () => ({ all: async () => ({ results: [{ n: 9007199254740992, b: [1, 2], doc: "{}" }] }) }) }) };
  await assert.rejects(execute(d1, q) as Promise<unknown>, /Column "n" holds an integer past .* already rounded/);
  const ok = { ...d1, prepare: () => ({ first: async () => null, bind: () => ({ all: async () => ({ results: [{ n: 1, b: [1, 2], doc: "{}" }] }) }) }) };
  assert.deepEqual(await execute(ok, q), [{ n: 1, b: Uint8Array.of(1, 2), doc: {} }]);
});

test("mysql2: per-query options and Reylo's typeCast, never decimalNumbers; a DECIMAL and JSON as their text, a date's dropped fraction and its T, TIME's digits", async () => {
  let sent: Record<string, any> = {};
  const field = (type: string, text: string | null) => ({ type, string: () => text, buffer: () => (text === null ? null : Buffer.from(text)) });
  const pool = {
    query: async () => [[], []],
    // mysql2 calls the typeCast once per value with the field and its own reading, and resolves to [rows, fields].
    execute: async (options: Record<string, any>) => {
      sent = options;
      const cast = options.typeCast as (f: unknown, next: () => unknown) => unknown;
      const row = {
        amount: cast(field("NEWDECIMAL", "1.100000000000000001"), () => 1.1),
        doc: cast(field("JSON", '{"big":9007199254740993}'), () => ({ big: "9007199254740993" })),
        at: cast(field("DATETIME", null), () => "2026-03-08 09:31:00"),
        at6: cast(field("DATETIME", null), () => "2026-03-08 09:31:00.123456"),
        latest: cast(field("DATETIME", null), () => "2026-03-08 09:31:00"),
        t: cast(field("TIME", null), () => "10:00:00.5"),
        geo: cast(field("GEOMETRY", "wkb"), () => ({ x: 1, y: 2 })),
        big: cast(field("LONGLONG", null), () => "9007199254740993"),
      };
      const fields = [["at", 12, 3], ["at6", 12, 6], ["latest", 12, 0], ["t", 11, 2]].map(([name, columnType, decimals]) => ({ name, columnType, decimals }));
      return [[row], fields];
    },
  };
  const [row] = (await execute(pool as never, { dialect: "mysql", sql: "select 1", params: [] })) as Record<string, unknown>[];
  assert.deepEqual(row, { amount: "1.100000000000000001", doc: { big: 9007199254740992 }, at: "2026-03-08T09:31:00.000", at6: "2026-03-08T09:31:00.123456", latest: "2026-03-08T09:31:00", t: "10:00:00.50", geo: Buffer.from("wkb"), big: "9007199254740993" });
  assert.equal("decimalNumbers" in sent, false, "a per-query decimalNumbers poisons mysql2's parser cache for every pool");
  assert.equal("jsonStrings" in sent, false);
});

test("mariadb: every option per query, metaAsArray off, a SET joined and a DATETIME's T; a GEOMETRY read as bytes only where a column may hold one", async () => {
  let sent: Record<string, any> = {};
  const meta = [{ name: () => "tags", type: "SET", isSet: () => true }, { name: () => "at", type: "DATETIME", isSet: () => false }, { name: () => "id", type: "INT", isSet: () => false }];
  const conn = { batch: async () => ({}), query: async (options: Record<string, any>) => ((sent = options), Object.assign([{ tags: ["x", "z"], at: "2026-03-08 09:31:00.000", id: 1 }], { meta })) };
  assert.deepEqual([...((await execute(conn as never, { dialect: "mysql", sql: "select 1", params: [] })) as unknown[])], [{ tags: "x,z", at: "2026-03-08T09:31:00.000", id: 1 }]);
  assert.equal(sent.metaAsArray, false);
  assert.equal(sent.bigIntAsNumber, false);
  assert.equal(sent.typeCast, undefined, "an app's typeCast is switched off");
  await execute(conn as never, { dialect: "mysql", sql: "select 1", params: [], columns: [["shape", "unknown"]] });
  assert.equal(typeof sent.typeCast, "function");
});

test("PlanetScale: Reylo's cast per query, the driver's own default with a DATETIME's T, over the app's", async () => {
  let cast: (field: { type: string; charset?: number }, value: string | null) => unknown = () => null;
  const conn = { execute: async (_: string, __?: unknown, options?: { cast: typeof cast }) => ((cast = options!.cast), { rows: [], statement: "" }) };
  await execute(conn, { dialect: "mysql", sql: "select 1", params: [] });
  const bytes = (s: string) => String.fromCharCode(...Buffer.from(s, "utf8"));
  assert.deepEqual(
    [cast({ type: "INT64" }, "9007199254740993"), cast({ type: "DATETIME" }, "2026-03-08 09:31:00.000"), cast({ type: "DECIMAL" }, "1.10"), cast({ type: "JSON" }, bytes('{"é":1}')), cast({ type: "INT32" }, "7"), cast({ type: "FLOAT64" }, "2.2"), cast({ type: "VARBINARY", charset: 63 }, "\u0001ÿ"), cast({ type: "VARCHAR", charset: 255 }, bytes("é")), cast({ type: "INT64" }, null)],
    ["9007199254740993", "2026-03-08T09:31:00.000", "1.10", { é: 1 }, 7, 2.2, Uint8Array.of(1, 255), "é", null],
  );
});

test("mariadb's callback API, whose rows carry no meta, and mariadb 2, whose columns have no isSet(): the module's kinds join a SET and give a DATETIME its T", async () => {
  const conn = {
    batch: () => ({}),
    query: (_: unknown, __: unknown, done: (e: Error | null, rows?: unknown, meta?: unknown) => void) => (setImmediate(() => done(null, [{ tags: ["x", "z"], at: "2026-03-08 09:31:00.000", id: 1 }], [{ name: () => "tags", type: "SET" }])), { on() {} }),
  };
  const q: QueryModule<"mysql"> = { dialect: "mysql", sql: "select 1", params: [], columns: [["tags", "text"], ["at", "timestamp"], ["id", "int"]] };
  assert.deepEqual(await execute(conn as never, q), [{ tags: "x,z", at: "2026-03-08T09:31:00.000", id: 1 }]);
  // A query defined by hand: the callback's own meta.
  assert.deepEqual(await execute(conn as never, { ...q, columns: undefined }), [{ tags: "x,z", at: "2026-03-08 09:31:00.000", id: 1 }]);
});

test("runSync refuses a client whose prepared statement answers through a promise, which run awaits", async () => {
  const db = { prepare: () => ({ all: async () => [{ n: 1 }] }) };
  const q: QueryModule<"sqlite"> = { dialect: "sqlite", sql: "select 1 as n", params: [], columns: [["n", "int"]] };
  assert.deepEqual(await execute(db as never, q), [{ n: 1 }]);
  const { executeSync } = await import("../src/execute.ts");
  assert.throws(() => executeSync(db as never, { ...q, sql: "select 2 as n" }), /answered with a promise/);
});

test("mysql2: Reylo's typeCast only where it is needed, since mysql2 calls it for every value: a JSON or unknown column, unknown columns, or the app's typeCast, decimalNumbers or jsonStrings", async () => {
  const sent: unknown[] = [];
  const pool = (connectionConfig: Record<string, unknown> | undefined) => ({ pool: connectionConfig && { config: { connectionConfig } }, query: async () => [[], []], execute: async (options: { typeCast: unknown }) => (sent.push(typeof options.typeCast === "function" ? "cast" : options.typeCast), [[], []]) });
  const q = (columns?: [string, string][]): QueryModule<"mysql"> => ({ dialect: "mysql", sql: "select 1", params: [], ...(columns ? { columns } : {}) });
  await execute(pool({}) as never, q([["n", "int8"], ["amount", "decimal"], ["at", "timestamp"]]));
  await execute(pool({}) as never, q([["doc", "json"]]));
  await execute(pool({}) as never, q([["shape", "unknown"]]));
  await execute(pool({}) as never, q());
  await execute(pool({ decimalNumbers: true }) as never, q([["amount", "decimal"]]));
  await execute(pool({ jsonStrings: true }) as never, q([["n", "int"]]));
  await execute(pool({ typeCast: () => null }) as never, q([["n", "int"]]));
  await execute(pool(undefined) as never, q([["n", "int"]]));
  assert.deepEqual(sent, [true, "cast", "cast", "cast", "cast", "cast", "cast", "cast"]);
});
