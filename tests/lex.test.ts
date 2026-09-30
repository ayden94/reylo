import { test } from "node:test";
import assert from "node:assert/strict";
import { lexPlaceholders } from "../src/lex.ts";
import { defineQuery } from "../src/index.ts";

const names = (sql: string) => lexPlaceholders(sql).params;

test("a parameter's name drops its prefix, and a position is its number", () => {
  assert.deepEqual(names("select :a, @b, $c, :a"), ["a", "b", "c"]);
  assert.deepEqual(names("select $1, $2, $1"), ["1", "2"]);
  assert.deepEqual(names("select ?, ?"), ["1", "2"]);
  // A bare `?` is one past the largest number before it, as SQLite numbers it.
  assert.deepEqual(names("select ?5, ?, ?2"), ["2", "5", "6"]);
  // Positional parameters bind by number, whatever order the statement names them in.
  assert.deepEqual(names("select $2::text, $1::int"), ["1", "2"]);
});

// Without the mapper's cache the runtime rewrites the statement itself (the loader's fallback).
const capture = () => {
  const calls: { text: string; values?: unknown[] | undefined }[] = [];
  return { calls, client: { async query(text: string, values?: unknown[]) { calls.push({ text, values }); return { rows: [{}] }; } } };
};

test("run binds positional parameters by number", async () => {
  const { calls, client } = capture();
  const q = defineQuery<{ 1: number; 2: string }, {}, "one", "postgres">({ dialect: "postgres", sql: "select $2::text, $1::int", params: ["1", "2"], cardinality: "one" });
  await q.run(client, { 1: 7, 2: "z" });
  assert.deepEqual(calls, [{ text: "select $2::text, $1::int", values: [7, "z"] }]);
});

test("run without the parameters returns a function of them, and runs at once without any", async () => {
  const { calls, client } = capture();
  const q = defineQuery<{ id: number }, {}, "one", "postgres">({ dialect: "postgres", sql: "select :id::int", params: ["id"], cardinality: "one" });
  const get = q.run(client);
  assert.equal(calls.length, 0);
  await get({ id: 1 });
  const none = defineQuery<{}, {}, "one", "postgres">({ dialect: "postgres", sql: "select 1", params: [], cardinality: "one" });
  await none.run(client);
  assert.deepEqual(calls, [{ text: "select $1::int", values: [1] }, { text: "select 1", values: [] }]);
});

test("a hand-built query whose params miss a placeholder is refused", async () => {
  const { calls, client } = capture();
  const q = defineQuery<{ id: number }, {}, "one", "postgres">({ dialect: "postgres", sql: "select $1", params: ["id"], cardinality: "one" });
  await assert.rejects(q.run(client, { id: 1 }), /placeholder \$1 is parameter "1"/);
  assert.equal(calls.length, 0);
});
