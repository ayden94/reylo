import { test } from "node:test";
import assert from "node:assert/strict";
import { convertRows, convertValue } from "../src/convert.ts";

const T = (globalThis as { Temporal?: { Instant: { prototype: object }; PlainDateTime: { prototype: object }; PlainDate: { prototype: object } } }).Temporal;
const local = new Date(2026, 0, 2, 3, 4, 5, 678); // a zone-less value as pg and mysql2 build it
const utcMidnight = new Date(Date.UTC(2026, 0, 2)); // a `date` as postgres, PGlite and Bun.sql build it
const localMidnight = new Date(2026, 0, 2); // a `date` as pg, mysql2 and mariadb build it
const s = { dates: "string", bigint: "string" } as const;

test("dates: string gives ISO 8601 text; a zone-less value keeps its wall clock", () => {
  assert.equal(convertValue(local, "timestamptz", s), local.toISOString());
  assert.equal(convertValue(local, "timestamp", s), "2026-01-02T03:04:05.678");
  assert.equal(convertValue(utcMidnight, "date", s), "2026-01-02");
  assert.equal(convertValue(localMidnight, "date", s), "2026-01-02");
  assert.deepEqual(convertValue([local, null], "timestamptz[]", s), [local.toISOString(), null]);
});

test("dates: Date leaves the driver's value alone; a string from a configured driver stays", () => {
  const d = { dates: "Date", bigint: "string" } as const;
  assert.equal(convertValue(local, "timestamptz", d), local);
  assert.equal(convertValue("2026-01-02 03:04:05+00", "timestamptz", s), "2026-01-02 03:04:05+00");
  assert.equal(convertValue(null, "timestamp", s), null);
});

test("dates: temporal builds the Temporal type of the Postgres type", { skip: !T && "no global Temporal" }, () => {
  const t = { dates: "temporal", bigint: "string" } as const;
  const instant = convertValue(local, "timestamptz", t) as { toString(): string };
  assert.ok(Object.getPrototypeOf(instant) === T!.Instant.prototype);
  assert.equal(instant.toString(), local.toISOString().replace(".678Z", ".678Z"));
  assert.equal(String(convertValue(local, "timestamp", t)), "2026-01-02T03:04:05.678");
  assert.equal(String(convertValue(utcMidnight, "date", t)), "2026-01-02");
  assert.equal(String(convertValue("2026-01-02 03:04:05+00", "timestamptz", t)), "2026-01-02T03:04:05Z");
  assert.equal(String(convertValue("2026-01-02 03:04:05.5", "timestamp", t)), "2026-01-02T03:04:05.5");
  assert.equal(String(convertValue("2026-01-02", "date", t)), "2026-01-02");
});

test("bigint: each representation from each driver value", () => {
  assert.equal(convertValue("9007199254740993", "int8", s), "9007199254740993");
  assert.equal(convertValue(9007199254740993n, "int8", s), "9007199254740993");
  assert.equal(convertValue(42, "int8", s), "42");
  const n = { dates: "string", bigint: "number" } as const;
  assert.equal(convertValue("42", "int8", n), 42);
  assert.equal(convertValue(42n, "int8", n), 42);
  const b = { dates: "string", bigint: "bigint" } as const;
  assert.equal(convertValue("9007199254740993", "int8", b), 9007199254740993n);
  assert.equal(convertValue(42, "int8", b), 42n);
  assert.deepEqual(convertValue(["1", "2"], "int8[]", b), [1n, 2n]);
});

test("convertRows touches only the named columns, in place", () => {
  const rows = [{ id: 1, at: local, big: "7", other: local }, null];
  convertRows(rows, { at: "timestamptz", big: "int8", missing: "date" }, { dates: "string", bigint: "number" });
  assert.deepEqual(rows[0], { id: 1, at: local.toISOString(), big: 7, other: local });
});
