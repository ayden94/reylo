import { test } from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, realpathSync, renameSync, writeFileSync } from "node:fs";
import { DatabaseSync } from "node:sqlite";
import { tmpdir } from "node:os";
import { join } from "node:path";

const CLI = join(import.meta.dirname, "..", "src", "cli.ts");

function porg(cwd: string, ...args: string[]) {
  const r = spawnSync(process.execPath, ["--no-warnings", CLI, ...args], { cwd, encoding: "utf8", env: { PATH: process.env.PATH, DATABASE_URL: "file:./app.db" } });
  // The timing varies from run to run; everything else is the readout.
  return { status: r.status, out: r.stdout.replace(/ +\d+ ms$/gm, " _ ms").replace(/\(\d+\.\d s\)/, "(_ s)"), err: r.stderr };
}

test("porg generate analyzes the .sql files the sources import, and prints one line per file", () => {
  // The CLI sees its working directory as the real path (/private/var on macOS).
  const root = realpathSync(mkdtempSync(join(tmpdir(), "porg-cli-")));
  writeFileSync(join(root, "package.json"), "{}");
  const db = new DatabaseSync(join(root, "app.db"));
  db.exec("create table users (id integer primary key, name text not null)");
  db.close();
  mkdirSync(join(root, "src", "queries"), { recursive: true });
  mkdirSync(join(root, "node_modules", "dep"), { recursive: true });
  writeFileSync(join(root, "src", "queries", "user.sql"), "select id, name from users where id = :id");
  writeFileSync(join(root, "src", "queries", "count.sql"), "select count(*) as n from users");
  writeFileSync(join(root, "src", "queries", "bad.sql"), "select id\nfrom nope");
  writeFileSync(join(root, "src", "main.ts"), [`import user from "./queries/user.sql";`, `const count = await import("./queries/count.sql");`, `import bad from "./queries/bad.sql";`, `import gone from "./queries/gone.sql";`, `import aliased from "@/queries/user.sql";`].join("\n"));
  // Dependencies are not the project's sources.
  writeFileSync(join(root, "node_modules", "dep", "index.js"), `import x from "./x.sql";`);

  const first = porg(root, "generate");
  assert.equal(first.status, 1, first.err);
  assert.equal(first.out, [
    `.: sqlite at ${join(root, "app.db")} (DATABASE_URL in the environment)`,
    `  error     src/queries/bad.sql    src/queries/bad.sql:1:1: no such table: nope`,
    `  analyzed  src/queries/count.sql  one       - _ ms`,
    `  error     src/queries/gone.sql   no such file; imported by src/main.ts`,
    `  analyzed  src/queries/user.sql   maybeOne  id _ ms`,
    `Skipped 1 import that is not a relative path: "@/queries/user.sql" in src/main.ts.`,
    `4 files in 1 project: 2 analyzed, 2 errors (_ s)`,
    ``,
  ].join("\n"));

  // Named files are analyzed as named; the second run reads .porg/.
  const second = porg(root, "generate", "src/queries/user.sql", "src/queries/count.sql");
  assert.equal(second.status, 0, second.err);
  assert.match(second.out, /^  cached    src\/queries\/count\.sql  one       - _ ms$/m);
  assert.match(second.out, /^2 files in 1 project: 2 cached \(_ s\)$/m);

  // Without the database the cache still answers, and --force cannot.
  renameSync(join(root, "app.db"), join(root, "gone.db"));
  const offline = porg(root, "generate", "src/queries/user.sql");
  assert.equal(offline.status, 0, offline.err);
  assert.match(offline.out, /^  Cannot open the database .*app\.db \(DATABASE_URL in the environment\): .*; the files are read from \.porg\/ as they are\.$/m);
  assert.match(offline.out, /^  cached    src\/queries\/user\.sql  maybeOne  id _ ms$/m);
  const forced = porg(root, "generate", "--force", "src/queries/user.sql");
  assert.equal(forced.status, 1);
  assert.match(forced.out, /^  error     src\/queries\/user\.sql  --force needs the database$/m);

  assert.equal(porg(root, "generate", "--watch").status, 2);
  assert.equal(porg(root, "frobnicate").status, 2);
});
