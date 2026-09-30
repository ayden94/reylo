import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { engine } from "../src/engine.ts";
import { findProjectFile, resolveProject } from "../src/project.ts";

// A project tree: `files` maps a path under a fresh directory to its text.
function tree(files: Record<string, string>): string {
  const root = mkdtempSync(join(tmpdir(), "porg-project-"));
  for (const [path, text] of Object.entries(files)) {
    mkdirSync(join(root, path, ".."), { recursive: true });
    writeFileSync(join(root, path), text);
  }
  return root;
}

test("without a project file, DATABASE_URL comes from the environment, then .env.local, then .env", async () => {
  const e = await engine();
  const root = tree({ "app/.env": "DATABASE_URL=postgres://from-env-file/db\n", "app/.env.local": "DATABASE_URL=postgres://from-local/db\n" });
  const dir = join(root, "app");
  assert.equal(findProjectFile(dir, {}), undefined);
  const local = resolveProject(e, dir, {});
  assert.deepEqual([local.url, local.urlSource, local.dialect, local.error], ["postgres://from-local/db", "DATABASE_URL in .env.local", "postgres", undefined]);
  const set = resolveProject(e, dir, { DATABASE_URL: "file:./x.db" });
  assert.deepEqual([set.url, set.urlSource, set.dialect], ["file:./x.db", "DATABASE_URL in the environment", "sqlite"]);
});

test("the project file is found walking up, and its url, env files and types apply", async () => {
  const e = await engine();
  const root = tree({
    ".porg.toml": 'url = "mysql://root:${DB_PASSWORD}@127.0.0.1:3306/app"\nenv_files = ["secrets.env"]\n[types]\ndates = "temporal"\n[api]\nroutes = "api"\n',
    "secrets.env": "DB_PASSWORD=hunter2\n",
    "packages/web/tsconfig.json": "{}",
  });
  const r = resolveProject(e, join(root, "packages/web"), {});
  assert.equal(r.file, join(root, ".porg.toml"));
  assert.equal(r.root, root);
  assert.equal(r.url, "mysql://root:hunter2@127.0.0.1:3306/app");
  assert.equal(r.urlSource, '"url" in ../../.porg.toml, DB_PASSWORD in ../../secrets.env');
  assert.equal(r.dialect, "mysql");
  assert.deepEqual(r.repr, { dates: "temporal", bigint: "string" });
  assert.equal(r.error, undefined);
  assert.ok(r.watched.includes(join(root, "secrets.env")));
  // PORG_CONFIG names the file instead of the walk.
  const other = tree({ "elsewhere.toml": 'dialect = "sqlite"\n' });
  assert.equal(resolveProject(e, join(root, "packages/web"), { PORG_CONFIG: join(other, "elsewhere.toml") }).dialect, "sqlite");
});

test("a key the library does not know, and a variable nothing sets, are errors that say where", async () => {
  const e = await engine();
  const typo = tree({ ".porg.toml": 'dialect = "postgres"\n\n[types]\ndatez = "Date"\n' });
  assert.match(resolveProject(e, typo, {}).error!, /^\.porg\.toml:4:1: unknown field `datez`, expected `dates` or `bigint`/);
  const unset = tree({ ".porg.toml": 'url = "${NOPE}"\n' });
  const r = resolveProject(e, unset, {});
  assert.equal(r.url, undefined);
  assert.match(r.error!, /^\.porg\.toml: "url" names \$\{NOPE\}, which neither the environment nor/);
});
