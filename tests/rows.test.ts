import { test } from "node:test";
import assert from "node:assert/strict";
import type { EngineType } from "../src/engine.ts";
import { defaultRepr, renderParamType } from "../src/render.ts";
import { prepareRows, variant, type RowsType } from "../src/rows.ts";

// What the engine gives for `:events::rows(id int8 not null, ts timestamptz not null, status? text
// not null, amount numeric(12,2), tags text[], meta jsonb('{"type": "object", …}'))`.
const scalar = (name: string): EngineType => ({ kind: "scalar", name });
const events: RowsType = {
  kind: "rows",
  one: false,
  fields: [
    { name: "id", type: scalar("int8"), written: "int8", nullable: false, optional: false },
    { name: "ts", type: scalar("timestamptz"), written: "timestamptz", nullable: false, optional: false },
    { name: "status", type: scalar("text"), written: "text", nullable: false, optional: true },
    { name: "amount", type: scalar("numeric"), written: "numeric(12,2)", nullable: true, optional: true },
    { name: "tags", type: { kind: "array", of: scalar("text") }, written: "text[]", nullable: true, optional: true },
    { name: "meta", type: { kind: "json", schema: { type: "object", properties: { source: { type: "string" } }, required: ["source"], additionalProperties: true } }, written: "jsonb", nullable: true, optional: true },
  ],
};

test("the caller's type is the column list's: each key as its parameter type takes it, ? where it may be left out, | null where it may be null", () => {
  assert.equal(
    renderParamType(events, "postgres", defaultRepr),
    "readonly { id: number | bigint | string; ts: Date | string; status?: string; amount?: string | null; tags?: string[] | null; meta?: { source: string; [key: string]: unknown } | null }[]",
  );
  assert.equal(renderParamType({ ...events, one: true, fields: events.fields.slice(0, 1) }, "postgres", defaultRepr), "{ id: number | bigint | string }");
});

test("a value is checked against its column list, and written as the JSON the statement reads", () => {
  const row = { id: 9007199254740993n, ts: new Date("2026-10-20T10:00:00Z"), status: "done", amount: "1.50", tags: ["a"], meta: { source: "x" } };
  // Postgres reads a 64-bit integer from its digits exactly, and the run time binds the JSON value.
  assert.deepEqual(prepareRows("events", events, [row], "postgres", true).value, [{ id: "9007199254740993", ts: "2026-10-20T10:00:00.000Z", status: "done", amount: "1.50", tags: ["a"], meta: { source: "x" } }]);
  // MySQL and SQLite take the JSON text, with the integer a bare number.
  assert.equal(prepareRows("events", events, [row], "mysql", true).value, `[{"id":9007199254740993,"ts":"2026-10-20T10:00:00.000Z","status":"done","amount":"1.50","tags":["a"],"meta":{"source":"x"}}]`);
  const refused: [unknown, RegExp][] = [
    [{ ts: new Date() }, /row 1: "id" is int8 not null, and the row leaves it out/],
    [{ id: 1, ts: new Date(), status: null }, /"status" is text not null, and the row gives null; leave the key out for the column's default/],
    [{ id: 1, ts: new Date(), stauts: "x" }, /the column list names no key "stauts"/],
    [{ id: 2 ** 53, ts: new Date() }, /past 2\^53 - 1/],
    [{ id: 2n ** 63n, ts: new Date() }, /from -9223372036854775808 to 9223372036854775807/],
    [{ id: 1.5, ts: new Date() }, /"id" is int8, an integer; got 1.5/],
    [{ id: 1, ts: "2026-10-20 10:00" }, /names its zone/],
    [{ id: 1, ts: new Date(), amount: "1.234" }, /keeps 2 digits after the point, so 1.234 would be rounded/],
    [{ id: 1, ts: new Date(), amount: "12345678901" }, /at most 10 digits before the point/],
    [{ id: 1, ts: new Date(), tags: ["a", 1] }, /"tags" element 2 is text, a string; got 1/],
    [{ id: 1, ts: new Date(), meta: { n: 1n } }, /holds a JSON value/],
  ];
  for (const [value, message] of refused) assert.throws(() => prepareRows("events", events, [value], "postgres", true), message, JSON.stringify(value, (_, v) => (typeof v === "bigint" ? `${v}n` : v)));
  assert.throws(() => prepareRows("events", events, { id: 1 }, "postgres", true), /pass an array of objects, one per row; got an object/);
  assert.throws(() => prepareRows("user", { ...events, one: true }, [], "postgres", true), /each row is an object/);
});

test("the rows an insert writes carry the same keys, and its lists name only those", () => {
  const a = { id: 1, ts: new Date() };
  assert.throws(() => prepareRows("events", events, [a, { ...a, status: "x" }], "postgres", true), /row 2 gives "status", which row 1 leaves out/);
  // Elsewhere a key left out reads as NULL, so the rows may differ.
  assert.doesNotThrow(() => prepareRows("events", { ...events, fields: events.fields.filter((f) => f.name !== "status") }, [a, { ...a, amount: 1 }], "postgres", false));
  const { present } = prepareRows("events", events, [a, a], "postgres", true);
  const text = `insert into events ("id", "ts", "status", "amount", "tags", "meta") SELECT e."id", e."ts", e."status", e."amount", e."tags", e."meta" FROM x`;
  const entries = (prefix: string) => ["id", "ts", "status", "amount", "tags", "meta"].map((n) => `${prefix}"${n}"`);
  const list = (entries: string[]) => {
    const start = text.indexOf(entries.join(", "));
    return { start, end: start + entries.join(", ").length, entries };
  };
  const structured = { name: "events", lists: [list(entries("")), list(entries("e."))] };
  assert.equal(variant(text, structured, events.fields, present), `insert into events ("id", "ts") SELECT e."id", e."ts" FROM x`);
  assert.throws(() => prepareRows("p", { kind: "rows", one: true, fields: [{ ...events.fields[2]! }] }, {}, "postgres", true), /every key is left out/);
});
