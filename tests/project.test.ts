import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { engine } from "../src/engine.ts";
import { findProjectFile, resolveProject } from "../src/project.ts";

// Any engine reads the project file; the repository's own install has every one.
const ROOT = fileURLToPath(new URL("..", import.meta.url));

// A project tree: `files` maps a path under a fresh directory to its text.
function tree(files: Record<string, string>): string {
  const root = mkdtempSync(join(tmpdir(), "reylo-project-"));
  for (const [path, text] of Object.entries(files)) {
    mkdirSync(join(root, path, ".."), { recursive: true });
    writeFileSync(join(root, path), text);
  }
  return root;
}

test("without a project file, DATABASE_URL comes from the environment, then .env.local, then .env", async () => {
  const e = await engine("postgres", ROOT);
  const root = tree({ "app/package.json": "{}", "app/.env": "DATABASE_URL=postgres://from-env-file/db\n", "app/.env.local": "DATABASE_URL=postgres://from-local/db\n" });
  const dir = join(root, "app");
  assert.equal(findProjectFile(dir, {}), undefined);
  const local = resolveProject(e, dir, {});
  assert.deepEqual([local.url, local.urlSource, local.dialect, local.error], ["postgres://from-local/db", "DATABASE_URL in .env.local", "postgres", undefined]);
  const set = resolveProject(e, dir, { DATABASE_URL: "file:./x.db" });
  assert.deepEqual([set.url, set.urlSource, set.dialect], ["file:./x.db", "DATABASE_URL in the environment", "sqlite"]);
});

test("the project file sits beside package.json, and its url, env files and strict apply", async () => {
  const e = await engine("postgres", ROOT);
  const root = tree({
    "package.json": "{}",
    ".reylo.yaml": "url: postgres://pooler/app\ndirect_url: mysql://root:${DB_PASSWORD}@127.0.0.1:3306/app\nenv: [secrets.env]\nstrict: true\napi:\n  routes: api\n",
    "secrets.env": "DB_PASSWORD=hunter2\n",
    "src/db/tsconfig.json": "{}",
  });
  // The tsconfig sits below the package; the project file is the package's.
  const r = resolveProject(e, join(root, "src/db"), {});
  assert.equal(r.file, join(root, ".reylo.yaml"));
  assert.equal(r.root, root);
  // The type check takes the direct connection, and the dialect is its scheme's.
  assert.equal(r.url, "mysql://root:hunter2@127.0.0.1:3306/app");
  assert.equal(r.urlSource, '"direct_url" in ../../.reylo.yaml, DB_PASSWORD in ../../secrets.env');
  assert.equal(r.dialect, "mysql");
  assert.equal(r.strict, true);
  assert.equal(r.error, undefined);
  assert.ok(r.watched.includes(join(root, "secrets.env")));
  // A package of its own below does not see the file above it.
  const nested = tree({ "package.json": "{}", ".reylo.yaml": "url: file:./a.db\n", "packages/web/package.json": "{}" });
  assert.equal(findProjectFile(join(nested, "packages/web"), {}), undefined);
  // REYLO_CONFIG names the file instead.
  const other = tree({ "elsewhere.yaml": "url: file:./dev.db\n" });
  assert.equal(resolveProject(e, join(root, "src/db"), { REYLO_CONFIG: join(other, "elsewhere.yaml") }).dialect, "sqlite");
});

test("a types section, removed on 2026-10-20, is refused where it stands, whatever it holds; package.json decides no type", async () => {
  const e = await engine("postgres", ROOT);
  for (const types of ["types:\n  dates: temporal\n", "types: {}\n", "types:\n  datez: Date\n"]) {
    const root = tree({ "package.json": JSON.stringify({ dependencies: { mariadb: "^3", "@planetscale/database": "^1" } }), ".reylo.yaml": `url: mysql://root@127.0.0.1:3306/app\n\n${types}` });
    const r = resolveProject(e, root, {});
    assert.match(r.error!, /^\.reylo\.yaml:3:1: types: the types section was removed on 2026-10-20\. Reylo now gives every value one type on every driver, whatever the driver's settings: .* Delete the section\.$/, types);
    assert.equal("repr" in r || "driver" in r, false);
    assert.ok(!r.watched.includes(join(root, "package.json")));
  }
  // A key that merely starts with `types` is the engine's to refuse, and a nested `types:` is another section's.
  const nested = tree({ "package.json": "{}", ".reylo.yaml": "url: postgres://x/db\napi:\n  types: whatever\n" });
  assert.equal(resolveProject(e, nested, {}).error, undefined);
});

test("a key the library does not know, and a variable nothing sets, are errors that say where", async () => {
  const e = await engine("postgres", ROOT);
  const typo = tree({ "package.json": "{}", ".reylo.yaml": "url: postgres://x/db\n\nstirct: true\n" });
  assert.match(resolveProject(e, typo, {}).error!, /^\.reylo\.yaml:3:1: unknown field `stirct`/);
  const unset = tree({ "package.json": "{}", ".reylo.yaml": "url: ${NOPE}\n" });
  const r = resolveProject(e, unset, {});
  assert.equal(r.url, undefined);
  assert.match(r.error!, /^\.reylo\.yaml: "url" names \$\{NOPE\}, which neither the environment nor/);
  // A scheme reylo does not read names no dialect.
  const odd = tree({ "package.json": "{}", ".reylo.yaml": "url: ./dev.db\n" });
  assert.match(resolveProject(e, odd, {}).error!, /^Cannot tell the database from the URL \("url" in \.reylo\.yaml\)/);
});
