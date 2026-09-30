import { test } from "node:test";
import assert from "node:assert/strict";
import type { EngineType } from "../src/engine.ts";
import { renderParamType, renderType, type Repr } from "../src/render.ts";

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
for (const dates of ["string", "Date", "temporal"] as const) for (const bigint of ["string", "number", "bigint"] as const) reprs.push({ dates, bigint });

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
  assert.deepEqual(row({ dates: "string", bigint: "string" }), ["string", "string", "string"]);
  assert.deepEqual(row({ dates: "Date", bigint: "number" }), ["number", "Date", "string"]);
  assert.deepEqual(row({ dates: "temporal", bigint: "bigint" }), ["bigint", "Temporal.Instant", "string"]);
});

test("a composite with no columns inside a document is an empty object", () => {
  assert.equal(renderType({ kind: "composite", name: "empty", fields: [] }, true), "Record<never, never>");
  assert.equal(renderType({ kind: "composite", name: "empty", fields: [] }, false), "string");
});
