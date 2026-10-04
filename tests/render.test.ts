import { test } from "node:test";
import assert from "node:assert/strict";
import type { EngineType } from "../src/engine.ts";
import { defaultRepr, renderParamType, renderType, type Repr } from "../src/render.ts";

const scalar = (name: string): EngineType => ({ kind: "scalar", name });
const field = (name: string, type: EngineType, nullable: boolean) => ({ name, type, nullable });

// A table's row type as the engine renders it once the catalog gave its columns.
const users: EngineType = {
  kind: "composite",
  name: "users",
  fields: [
    field("id", scalar("int8"), false),
    field("seen", scalar("timestamptz"), true),
    field("tags", { kind: "array", of: scalar("text") }, true),
    field("mood", { kind: "enum", name: "mood", labels: ["ok", "sad"] }, false),
  ],
};
// `json_build_object('author', u, 'posts', json_agg(p), 'n', count(*))`: a composite inside a document.
const doc: EngineType = {
  kind: "object",
  fields: [
    field("author", users, false),
    field("posts", { kind: "list", of: users, nullable_items: true }, true),
    field("n", scalar("int8"), false),
  ],
};

const reprs: Repr[] = [];
for (const dates of ["string", "Date", "temporal"] as const) for (const bigint of ["string", "number", "bigint"] as const) reprs.push({ ...defaultRepr, dates, bigint });

const inJson = `{ id: number; seen: string | null; tags: string[] | null; mood: "ok" | "sad" }`;

test("a composite is its row's text on the wire, under every representation", () => {
  for (const repr of reprs) {
    assert.equal(renderType(users, false, "postgres", repr), "string", JSON.stringify(repr));
    assert.equal(renderType({ kind: "array", of: users }, false, "postgres", repr), "string", JSON.stringify(repr));
    assert.equal(renderParamType(users, "postgres", repr), "string", JSON.stringify(repr));
  }
});

test("inside a JSON document a composite is an object of its JSON-encoded fields, whatever the representation", () => {
  for (const repr of reprs) {
    assert.equal(renderType(users, true, "postgres", repr), inJson, JSON.stringify(repr));
    // The renderer parenthesizes an element whose text holds a `|` anywhere, as it does for an object.
    assert.equal(renderType({ kind: "array", of: users }, true, "postgres", repr), `(${inJson})[]`, JSON.stringify(repr));
    // A constructor's fields are inside the document, at the top level too.
    const expected = `{ author: ${inJson}; posts: (${inJson} | null)[] | null; n: number }`;
    assert.equal(renderType(doc, false, "postgres", repr), expected, JSON.stringify(repr));
    assert.equal(renderType(doc, true, "postgres", repr), expected, JSON.stringify(repr));
  }
});

test("the representation options still apply beside a composite on the wire", () => {
  const row = (repr: Repr) => [scalar("int8"), scalar("timestamptz"), users].map((t) => renderType(t, false, "postgres", repr));
  assert.deepEqual(row({ ...defaultRepr, dates: "string", bigint: "string" }), ["string", "string", "string"]);
  assert.deepEqual(row({ ...defaultRepr, dates: "Date", bigint: "number" }), ["number", "Date", "string"]);
  assert.deepEqual(row({ ...defaultRepr, dates: "temporal", bigint: "bigint" }), ["bigint", "Temporal.Instant", "string"]);
});

test("a composite with no columns inside a document is an empty object", () => {
  assert.equal(renderType({ kind: "composite", name: "empty", fields: [] }, true), "Record<never, never>");
  assert.equal(renderType({ kind: "composite", name: "empty", fields: [] }, false), "string");
});

test("a SQLite INTEGER follows the bigint setting, as an int8 does elsewhere; dates changes no SQLite type", () => {
  const sqlite = (bigint: Repr["bigint"]): Repr => ({ ...defaultRepr, dates: "string", bigint });
  assert.equal(renderType(scalar("integer"), false, "sqlite", sqlite("number")), "number");
  assert.equal(renderType(scalar("integer"), false, "sqlite", sqlite("bigint")), "bigint");
  assert.equal(renderType(scalar("integer"), false, "sqlite", sqlite("string")), "string");
  // A NUMERIC column holds whichever storage class the row stored.
  assert.equal(renderType(scalar("numeric"), false, "sqlite", sqlite("number")), "number | string");
  assert.equal(renderType(scalar("numeric"), false, "sqlite", sqlite("bigint")), "bigint | number | string");
  assert.equal(renderType(scalar("real"), false, "sqlite", sqlite("bigint")), "number");
  assert.equal(renderType(scalar("text"), false, "sqlite", { ...defaultRepr, dates: "Date", bigint: "number" }), "string");
  // The driver binds a number or a BigInt whatever it returns.
  assert.equal(renderParamType(scalar("integer"), "sqlite", sqlite("number")), "number");
  assert.equal(renderParamType(scalar("integer"), "sqlite", sqlite("bigint")), "number | bigint");
});

test("decimal, json and binary follow the driver's representation; parameters take what every driver binds", () => {
  const repr = (r: Partial<Repr>): Repr => ({ ...defaultRepr, ...r });
  assert.equal(renderType(scalar("numeric"), false, "postgres", repr({})), "string");
  assert.equal(renderType(scalar("numeric"), false, "mysql", repr({ decimal: "number" })), "number");
  assert.equal(renderParamType(scalar("numeric"), "postgres", repr({ decimal: "number" })), "number | string");
  assert.equal(renderType(scalar("bytea"), false, "postgres", repr({ binary: "Buffer" })), "Buffer");
  assert.equal(renderType(scalar("blob"), false, "sqlite", repr({ binary: "ArrayBuffer" })), "ArrayBuffer");
  assert.equal(renderType({ kind: "array", of: scalar("bytea") }, false, "postgres", repr({ binary: "number[]" })), "number[][]");
  assert.equal(renderParamType(scalar("blob"), "sqlite", repr({ binary: "number[]" })), "Uint8Array");
  // A JSON value, built by the statement or read from a column, is the text when the driver returns it so.
  const built: EngineType = { kind: "object", fields: [{ name: "id", type: scalar("int4"), nullable: false }] };
  assert.equal(renderType(built, false, "mysql", repr({})), "{ id: number }");
  assert.equal(renderType(built, false, "mysql", repr({ json: "string" })), "string");
  assert.equal(renderType({ kind: "json" }, false, "postgres", repr({ json: "string" })), "string");
  assert.equal(renderParamType({ kind: "json" }, "postgres", repr({ json: "string" })), "unknown");
  // Inside a JSON value the database wrote the bytes and the number already.
  const inner: EngineType = { kind: "object", fields: [{ name: "b", type: scalar("bytea"), nullable: false }, { name: "n", type: scalar("numeric"), nullable: false }] };
  assert.equal(renderType(inner, false, "postgres", repr({ binary: "Buffer", decimal: "string" })), "{ b: string; n: number }");
});

test("an array's elements are nullable only where the statement builds it from a value that may be NULL", () => {
  // `array_agg(u.email)` over a LEFT JOIN is `{NULL}` for a team with no users; a table's array
  // column, which the statement does not build, is `T[]` as the maintainer decided on 2026-10-03.
  const texts = (nullable_items?: boolean): EngineType => (nullable_items === undefined ? { kind: "array", of: scalar("text") } : { kind: "array", of: scalar("text"), nullable_items });
  assert.equal(renderType(texts()), "string[]");
  assert.equal(renderType(texts(false)), "string[]");
  assert.equal(renderType(texts(true)), "(string | null)[]");
  assert.equal(renderType(texts(true), true), "(string | null)[]");
  assert.equal(renderType({ kind: "array", of: { kind: "enum", name: "mood", labels: ["ok", "sad"] }, nullable_items: true }), `("ok" | "sad" | null)[]`);
  // A parameter's elements are the caller's to pass.
  assert.equal(renderParamType(texts()), "string[]");
});
