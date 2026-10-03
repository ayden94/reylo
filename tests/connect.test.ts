import { test } from "node:test";
import assert from "node:assert/strict";
import { existsSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { createServer, type AddressInfo } from "node:net";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { reason, Unreachable } from "../src/engine.ts";
import { forPostgresJs, openPostgresDescriber } from "../src/describe.ts";
import { openMysqlDescriber } from "../src/mysql-describe.ts";

// The drivers are the labs' dependencies, never the package's own.
const lab = (name: string) => fileURLToPath(new URL(`../lab/${name}`, import.meta.url));
const installed = (dir: string, pkg: string) => existsSync(`${dir}/node_modules/${pkg}`) || `${dir} has no ${pkg} installed`;
const unreachable = (r: PromiseSettledResult<unknown>, pattern: RegExp) => {
  assert.equal(r.status, "rejected");
  const e = (r as PromiseRejectedResult).reason;
  assert.ok(e instanceof Unreachable, `expected Unreachable, got ${e}`);
  assert.match(e.message, pattern);
};

test("reason reads a refused connection, an AggregateError with no message of its own", () => {
  const refused = Object.assign(new AggregateError([new Error("connect ECONNREFUSED ::1:1"), new Error("connect ECONNREFUSED 127.0.0.1:1")], ""), { code: "ECONNREFUSED" });
  assert.equal(reason(refused), "connect ECONNREFUSED ::1:1; connect ECONNREFUSED 127.0.0.1:1");
  assert.equal(reason(Object.assign(new Error(""), { code: "ETIMEDOUT" })), "ETIMEDOUT");
});

test("postgres.js: a refused connection fails every call as Unreachable", { skip: installed(lab("basic"), "postgres") !== true && "lab/basic is not installed" }, async () => {
  const d = await openPostgresDescriber(lab("basic"), "postgres://localhost:1/x");
  const results = await Promise.allSettled([d.values("select 1"), d.describe("select 2"), d.query("select 3")]);
  for (const r of results) unreachable(r, /ECONNREFUSED/);
  await d.close();
});

test("postgres.js: calls queued behind a server that never answers fail together at the first timeout", { skip: installed(lab("basic"), "postgres") !== true && "lab/basic is not installed", timeout: 30_000 }, async () => {
  // A server that accepts the socket and never says anything: the connection times out, and
  // postgres.js would connect again for each queued call, one timeout after another.
  const silent = createServer(() => {});
  await new Promise<void>((resolve) => silent.listen(0, "127.0.0.1", resolve));
  try {
    const d = await openPostgresDescriber(lab("basic"), `postgres://127.0.0.1:${(silent.address() as AddressInfo).port}/x`);
    const t0 = Date.now();
    const results = await Promise.allSettled([d.values("select 1"), d.values("select 2"), d.values("select 3")]);
    for (const r of results) unreachable(r, /CONNECT_TIMEOUT/);
    assert.ok(Date.now() - t0 < 15_000, `took ${Date.now() - t0} ms`);
    await d.close();
  } finally {
    silent.close();
  }
});

test("mysql2: a refused connection is Unreachable", { skip: installed(lab("mysql"), "mysql2") !== true && "lab/mysql is not installed" }, async () => {
  const [r] = await Promise.allSettled([openMysqlDescriber(lab("mysql"), "mysql://root@localhost:1/x")]);
  unreachable(r!, /ECONNREFUSED/);
});

test("a project with no driver for its dialect is told which ones the type check reads", async () => {
  const dir = mkdtempSync(join(tmpdir(), "motherload-no-driver-"));
  try {
    writeFileSync(join(dir, "package.json"), '{ "name": "no-driver", "private": true }\n');
    await assert.rejects(openPostgresDescriber(dir, "postgres://localhost:1/x"), /install "postgres" or "pg"/);
    await assert.rejects(openMysqlDescriber(dir, "mysql://root@localhost:1/x"), /install "mysql2"/);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("postgres.js gets no channel_binding parameter, and one that requires binding gets a verified certificate", () => {
  assert.deepEqual(forPostgresJs("postgresql://u:p%40ss@ep-x-pooler.aws.neon.tech/neondb?sslmode=require&channel_binding=require"), { url: "postgresql://u:p%40ss@ep-x-pooler.aws.neon.tech/neondb?sslmode=require", ssl: "verify-full" });
  assert.deepEqual(forPostgresJs("postgres://localhost/db?channel_binding=prefer"), { url: "postgres://localhost/db" });
  assert.deepEqual(forPostgresJs("postgres://localhost/db?channel_binding=disable&application_name=x"), { url: "postgres://localhost/db?application_name=x" });
  assert.deepEqual(forPostgresJs("postgres://localhost/db?sslmode=disable"), { url: "postgres://localhost/db?sslmode=disable" });
  assert.deepEqual(forPostgresJs("postgres://localhost/db"), { url: "postgres://localhost/db" });
});
