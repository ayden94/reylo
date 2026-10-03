import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { engine } from "../src/engine.ts";
import { findProjectFile, resolveProject } from "../src/project.ts";

// A project tree: `files` maps a path under a fresh directory to its text.
function tree(files: Record<string, string>): string {
  const root = mkdtempSync(join(tmpdir(), "motherload-project-"));
  for (const [path, text] of Object.entries(files)) {
    mkdirSync(join(root, path, ".."), { recursive: true });
    writeFileSync(join(root, path), text);
  }
  return root;
}

test("without a project file, DATABASE_URL comes from the environment, then .env.local, then .env", async () => {
  const e = await engine();
  const root = tree({ "app/package.json": "{}", "app/.env": "DATABASE_URL=postgres://from-env-file/db\n", "app/.env.local": "DATABASE_URL=postgres://from-local/db\n" });
  const dir = join(root, "app");
  assert.equal(findProjectFile(dir, {}), undefined);
  const local = resolveProject(e, dir, {});
  assert.deepEqual([local.url, local.urlSource, local.dialect, local.error], ["postgres://from-local/db", "DATABASE_URL in .env.local", "postgres", undefined]);
  const set = resolveProject(e, dir, { DATABASE_URL: "file:./x.db" });
  assert.deepEqual([set.url, set.urlSource, set.dialect], ["file:./x.db", "DATABASE_URL in the environment", "sqlite"]);
});

test("the project file sits beside package.json, and its url, env files, strict and types apply", async () => {
  const e = await engine();
  const root = tree({
    "package.json": "{}",
    ".motherload.yaml": "url: postgres://pooler/app\ndirect_url: mysql://root:${DB_PASSWORD}@127.0.0.1:3306/app\nenv: [secrets.env]\nstrict: true\ntypes:\n  dates: temporal\napi:\n  routes: api\n",
    "secrets.env": "DB_PASSWORD=hunter2\n",
    "src/db/tsconfig.json": "{}",
  });
  // The tsconfig sits below the package; the project file is the package's.
  const r = resolveProject(e, join(root, "src/db"), {});
  assert.equal(r.file, join(root, ".motherload.yaml"));
  assert.equal(r.root, root);
  // The type check takes the direct connection, and the dialect is its scheme's.
  assert.equal(r.url, "mysql://root:hunter2@127.0.0.1:3306/app");
  assert.equal(r.urlSource, '"direct_url" in ../../.motherload.yaml, DB_PASSWORD in ../../secrets.env');
  assert.equal(r.dialect, "mysql");
  assert.equal(r.strict, true);
  // `dates` is the file's; the rest are MySQL's defaults, mysql2's, since package.json names no driver.
  assert.equal(r.driver, undefined);
  assert.deepEqual(r.repr, { dates: "temporal", bigint: "number", decimal: "string", json: "parsed", binary: "Uint8Array" });
  assert.equal(r.error, undefined);
  assert.ok(r.watched.includes(join(root, "secrets.env")));
  // A package of its own below does not see the file above it.
  const nested = tree({ "package.json": "{}", ".motherload.yaml": "url: file:./a.db\n", "packages/web/package.json": "{}" });
  assert.equal(findProjectFile(join(nested, "packages/web"), {}), undefined);
  // MOTHERLOAD_CONFIG names the file instead.
  const other = tree({ "elsewhere.yaml": "url: file:./dev.db\n" });
  assert.equal(resolveProject(e, join(root, "src/db"), { MOTHERLOAD_CONFIG: join(other, "elsewhere.yaml") }).dialect, "sqlite");
});

test("the types start from the driver package.json names, and the project file's keys override them", async () => {
  const e = await engine();
  const project = (deps: object, yaml = "") => {
    const root = tree({ "package.json": JSON.stringify(deps), ".motherload.yaml": `url: mysql://root@127.0.0.1:3306/app\n${yaml}` });
    return resolveProject(e, root, {});
  };
  // mariadb returns a BigInt; the file's `bigint` wins over the driver's.
  assert.deepEqual([project({ dependencies: { mariadb: "^3" } }).driver, project({ dependencies: { mariadb: "^3" } }).repr.bigint], ["mariadb", "bigint"]);
  // A Buffer is a Uint8Array, which needs no @types/node; a project that has them asks for Buffer.
  assert.equal(project({ dependencies: { mariadb: "^3" } }).repr.binary, "Uint8Array");
  assert.equal(project({ dependencies: { mariadb: "^3" } }, "types:\n  binary: Buffer\n").repr.binary, "Buffer");
  assert.equal(project({ dependencies: { mariadb: "^3" } }, "types:\n  bigint: string\n").repr.bigint, "string");
  assert.deepEqual(project({ dependencies: { mariadb: "^3" } }, "types:\n  decimal: number\n  json: string\n").repr, { dates: "Date", bigint: "bigint", decimal: "number", json: "string", binary: "Uint8Array" });
  // PlanetScale's HTTP driver returns dates and 64-bit integers as text; a dev dependency counts.
  assert.deepEqual(project({ devDependencies: { "@planetscale/database": "^1" } }).repr, { dates: "string", bigint: "string", decimal: "string", json: "parsed", binary: "Uint8Array" });
  // With several, the first motherload lists is taken: mysql2 before mariadb.
  assert.equal(project({ dependencies: { mariadb: "^3", mysql2: "^3" } }).driver, "mysql2");
  // A driver of another dialect is not this project's.
  assert.equal(project({ dependencies: { pg: "^8" } }).driver, undefined);
  const sqlite = tree({ "package.json": JSON.stringify({ dependencies: { "@libsql/client": "^0.15" } }), ".motherload.yaml": "url: file:./app.db\n" });
  assert.equal(resolveProject(e, sqlite, {}).repr.binary, "ArrayBuffer");
  assert.ok(resolveProject(e, sqlite, {}).watched.includes(join(sqlite, "package.json")));
});

test("a key the library does not know, and a variable nothing sets, are errors that say where", async () => {
  const e = await engine();
  const typo = tree({ "package.json": "{}", ".motherload.yaml": "url: postgres://x/db\n\ntypes:\n  datez: Date\n" });
  assert.match(resolveProject(e, typo, {}).error!, /^\.motherload\.yaml:4:3: types: unknown field `datez`, expected one of `dates`, `bigint`, `decimal`, `json`, `binary`/);
  const unset = tree({ "package.json": "{}", ".motherload.yaml": "url: ${NOPE}\n" });
  const r = resolveProject(e, unset, {});
  assert.equal(r.url, undefined);
  assert.match(r.error!, /^\.motherload\.yaml: "url" names \$\{NOPE\}, which neither the environment nor/);
  // A scheme motherload does not read names no dialect.
  const odd = tree({ "package.json": "{}", ".motherload.yaml": "url: ./dev.db\n" });
  assert.match(resolveProject(e, odd, {}).error!, /^Cannot tell the database from the URL \("url" in \.motherload\.yaml\)/);
});
