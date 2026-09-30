// pg-promise against the shared local Postgres: the db object, a task, a tx, and a nested tx
// (pg-promise runs a tx inside a tx as a savepoint).
import { args, callbackTx, notRun, requireFrom, runJob } from "../lib/harness.ts";

const a = args();
const meta = { pkg: "pg-promise", version: a.version!, target: "Postgres 14 (local)" };
const url = process.env.DM_PG_URL ?? "postgres://localhost:5432/postgres";

let pgPromise: any;
try {
  pgPromise = requireFrom(a.pkg!);
} catch (e) {
  notRun(meta, `cannot load ${a.pkg}: ${(e as Error).message}`);
  process.exit(0);
}

await runJob(meta, async () => {
  const pgp = pgPromise();
  const db = pgp({ connectionString: url, max: 10 });
  const begin = (fn: (t: any) => Promise<unknown>) => db.tx(fn);
  return {
    ...meta,
    dialect: "postgres",
    count: async (tag) => (await db.one("select count(*)::int as n from driver_matrix.scratch where tag = $1", [tag])).n,
    shapes: [
      { name: "db", kind: "async", scope: (fn) => fn(db) },
      { name: "db.task(t)", kind: "async", scope: (fn) => db.task((t: any) => fn(t)) },
      {
        name: "db.tx(t)",
        kind: "async",
        inTransaction: true,
        scope: (fn) => begin((t) => fn(t)),
        tx: callbackTx(begin, (t, fn) => t.tx(fn)),
      },
      {
        name: "t.tx(t2) (savepoint)",
        kind: "async",
        inTransaction: true,
        scope: (fn) => begin((t) => t.tx((t2: any) => fn(t2))),
        tx: callbackTx(
          (fn) => begin((t) => t.tx(fn)),
          (t2, fn) => t2.tx(fn),
        ),
      },
    ],
    close: () => pgp.end(),
  };
});
