// PGlite, in-process: the database, and db.transaction(tx => ...) (a savepoint by hand inside it,
// since its API has none).
import { args, callbackTx, notRun, rawSavepoint, requireFrom, runJob, seedSql } from "../lib/harness.ts";

const a = args();
const meta = { pkg: "@electric-sql/pglite", version: a.version!, target: "PGlite (in-process)" };

let PGlite: any;
try {
  PGlite = (await import(requireFrom.resolve(a.pkg!))).PGlite;
} catch (e) {
  notRun(meta, `cannot load ${a.pkg}: ${(e as Error).message}`);
  process.exit(0);
}

await runJob(meta, async () => {
  const db = new PGlite();
  await db.exec(seedSql("postgres"));
  const begin = (fn: (tx: any) => Promise<unknown>) => db.transaction(fn);
  return {
    ...meta,
    dialect: "postgres",
    count: async (tag) => (await db.query("select count(*)::int as n from driver_matrix.scratch where tag = $1", [tag])).rows[0].n,
    shapes: [
      { name: "PGlite", kind: "async", scope: (fn) => fn(db) },
      {
        name: "db.transaction(tx)",
        kind: "async",
        inTransaction: true,
        scope: (fn) => begin((tx) => fn(tx)),
        tx: callbackTx(
          begin,
          rawSavepoint((tx, sql) => tx.query(sql)),
        ),
      },
    ],
    close: () => db.close(),
  };
});
