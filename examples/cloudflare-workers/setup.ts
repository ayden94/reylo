// Creates the local D1 database from migrations/, starting over: the tables of
// 0001_users_and_posts.sql, with their rows, in the file .reylo.yaml names.
import { rmSync } from "node:fs";
import { spawnSync } from "node:child_process";

rmSync(".wrangler/state/v3/d1", { recursive: true, force: true });
const run = spawnSync("wrangler d1 migrations apply DB --local", { shell: true, encoding: "utf8" });
if (run.status !== 0) {
  process.stderr.write(run.stdout + run.stderr);
  process.exit(run.status ?? 1);
}
console.log("created users and posts");
