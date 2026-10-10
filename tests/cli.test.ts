import { test } from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, realpathSync, renameSync, writeFileSync } from "node:fs";
import { DatabaseSync } from "node:sqlite";
import { tmpdir } from "node:os";
import { join } from "node:path";

const CLI = join(import.meta.dirname, "..", "src", "cli.ts");

function reylo(cwd: string, ...args: string[]) {
  const r = spawnSync(process.execPath, ["--no-warnings", CLI, ...args], { cwd, encoding: "utf8", env: { PATH: process.env.PATH, DATABASE_URL: "file:./app.db" } });
  // The time varies from run to run; everything else is the readout.
  return { status: r.status, out: r.stdout.replace(/\(\d+\.\d s\)/, "(_ s)"), err: r.stderr };
}

test("reylo generate analyzes every .sql file of the project, and prints each file with a mark and every error in full", () => {
  // The CLI sees its working directory as the real path (/private/var on macOS).
  const root = realpathSync(mkdtempSync(join(tmpdir(), "reylo-cli-")));
  writeFileSync(join(root, "package.json"), "{}");
  const db = new DatabaseSync(join(root, "app.db"));
  db.exec("create table users (id integer primary key, name text not null)");
  db.close();
  for (const dir of ["src/queries", "migrations", "node_modules/dep", ".next", "generated"]) mkdirSync(join(root, dir), { recursive: true });
  writeFileSync(join(root, "src", "queries", "user.sql"), "select id, name from users where id = :id");
  writeFileSync(join(root, "src", "queries", "count.sql"), "select count(*) as n from users");
  writeFileSync(join(root, "src", "queries", "bad.sql"), "select id\nfrom nope");
  writeFileSync(join(root, "src", "queries", "typo.sql"), "selec id from users");
  // A migration is skipped: it is not one query.
  writeFileSync(join(root, "migrations", "0001.sql"), "create table posts (id integer primary key);\ncreate index posts_id on posts (id);");
  writeFileSync(join(root, "migrations", "0002.sql"), "alter table posts add column title text");
  // Dependencies, build output and what git ignores are not the project's queries.
  writeFileSync(join(root, "node_modules", "dep", "x.sql"), "select 1");
  writeFileSync(join(root, ".next", "y.sql"), "select 1");
  writeFileSync(join(root, "generated", "z.sql"), "select 1");
  writeFileSync(join(root, ".gitignore"), "generated/\n");
  assert.equal(spawnSync("git", ["init", "-q"], { cwd: root }).status, 0);

  const first = reylo(root, "generate");
  assert.equal(first.status, 1, first.err);
  assert.equal(first.out, [
    `sqlite at ${join(root, "app.db")} (DATABASE_URL in the environment)`,
    `  src/queries/bad.sql    ✗`,
    `  src/queries/count.sql  ✓`,
    `  src/queries/typo.sql   ✗`,
    `  src/queries/user.sql   ✓`,
    ``,
    // SQLite names no place for this error, so no line of the statement is shown.
    `src/queries/bad.sql:1:1: no such table: nope`,
    ``,
    // A misspelt keyword is an error, not a statement that is not a query.
    `src/queries/typo.sql:1:1: near "selec": syntax error`,
    `  1 | selec id from users`,
    `    | ^^^^^`,
    ``,
    `Skipped 2 files that are not one query (a migration?): migrations/0001.sql, migrations/0002.sql`,
    ``,
    `✗ 2 of 4 queries failed (_ s)`,
    ``,
  ].join("\n"));

  // .reylo.yaml narrows the files; a file named on the command line is analyzed as named.
  writeFileSync(join(root, ".reylo.yaml"), `cli:\n  include: ["src/**/*.sql"]\n  exclude: ["**/bad.sql", "**/typo.sql"]\n`);
  const configured = reylo(root, "generate");
  assert.equal(configured.status, 0, configured.err);
  assert.match(configured.out, /^  src\/queries\/count\.sql  ✓\n  src\/queries\/user\.sql   ✓\n\n✓ 2 queries typed \(_ s\)\n$/m);
  assert.equal(reylo(root, "generate", "src/queries/bad.sql").status, 1);

  // Without the database the cache still answers, and --force cannot.
  renameSync(join(root, "app.db"), join(root, "gone.db"));
  const offline = reylo(root, "generate", "src/queries/user.sql");
  assert.equal(offline.status, 0, offline.err);
  assert.match(offline.out, /^Cannot open the database .*app\.db \(DATABASE_URL in the environment\): .*; the files are read from \.reylo\/ as they are\.$/m);
  assert.match(offline.out, /^  src\/queries\/user\.sql  ✓$/m);
  const forced = reylo(root, "generate", "--force", "src/queries/user.sql");
  assert.equal(forced.status, 1);
  assert.match(forced.out, /^src\/queries\/user\.sql: --force needs the database, which cannot be reached$/m);

  assert.equal(reylo(root, "generate", "--watch").status, 2);
  assert.equal(reylo(root, "generate", "package.json").status, 2);
  assert.equal(reylo(root, "frobnicate").status, 2);
});
