import { openPostgresDescriber } from "../../src/describe.ts";
import { mkdtempSync, writeFileSync, symlinkSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { writeSync } from "node:fs";
const log = (...a) => writeSync(2, a.join(" ") + "\n");
const URL = "postgres://localhost:5432/postgres";
const drivers = new URL("./node_modules", import.meta.url).pathname;
const q = `select u.oid, u.relname, (select coalesce(jsonb_agg(jsonb_build_object('a', 1)), '[]'::jsonb)) as j from pg_catalog.pg_class u where u.oid = $1 and u.relname = $2`;
for (const only of ["postgres", "pg"]) {
  const dir = mkdtempSync(`/tmp/dotsql-proj-${only}-`);
  writeFileSync(join(dir, "package.json"), "{}"); mkdirSync(join(dir, "node_modules"));
  symlinkSync(join(drivers, only), join(dir, "node_modules", only));
  if (only === "pg") for (const dep of ["pg-pool","pg-protocol","pg-types","pg-connection-string","pgpass","pg-cloudflare","postgres-array","postgres-bytea","postgres-date","postgres-interval","split2","xtend","pg-int8","pg-numeric","postgres-range","obuf","buffer-writer","packet-reader"]) { try { symlinkSync(join(drivers, dep), join(dir, "node_modules", dep)); } catch {} }
  log("opening", only, dir); const d = await openPostgresDescriber(dir, URL); log("opened", d.driver);
  const t0 = performance.now();
  log("describing"); const r = await d.describe(q); log("described");
  const t1 = performance.now();
  console.log(d.driver, JSON.stringify(r), `${(t1 - t0).toFixed(1)}ms`);
  console.log(d.driver, "error surface:", await d.describe("select nope from pg_class").then(() => "no error", e => `${e.code} ${e.message} @${e.position}`));
  console.log(d.driver, "catalog query:", JSON.stringify(await d.query("select attname, attnotnull from pg_attribute where attrelid = 'pg_class'::regclass and attnum in (1,2)")));
  await d.close();
}
