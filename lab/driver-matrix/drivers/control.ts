// Failing controls: clients that are deliberately wrong, so every row here must FAIL. They show
// the scenarios can fail at all (a green matrix with no failing control is no evidence).
//   --case pg      PGlite behind a client that binds each integer parameter plus one, and a
//                  transaction whose "rollback" commits and whose failure is swallowed
//   --case sqlite  a node:sqlite database declared asynchronous, so the return-discipline check must fire
import { args, callbackTx, requireFrom, runJob, seedSql, type TxOps } from "../lib/harness.ts";

const a = args();

if (a.case === "sqlite") {
  const { DatabaseSync } = await import("node:sqlite");
  const meta = { pkg: "control", version: "sqlite", target: "node:sqlite, declared asynchronous" };
  await runJob(meta, async () => {
    const db = new DatabaseSync(":memory:");
    db.exec(seedSql("sqlite"));
    return {
      ...meta,
      dialect: "sqlite",
      count: async (tag) => (db.prepare("select count(*) as n from scratch where tag = ?").get(tag) as any).n,
      shapes: [{ name: "sync client declared async", kind: "async", scope: (fn) => fn(db) }],
    };
  });
} else {
  const { PGlite } = await import(requireFrom.resolve("pglite-05"));
  const meta = { pkg: "control", version: "pg", target: "PGlite behind a corrupting wrapper" };
  await runJob(meta, async () => {
    const db = new PGlite();
    await db.exec(seedSql("postgres"));
    const corrupt = { query: (text: string, values?: unknown[]) => db.query(text, values?.map((v) => (typeof v === "number" ? v + 1 : v))) };
    const good = callbackTx((fn) => db.transaction(fn));
    const broken: TxOps = {
      commit: good.commit,
      // "Rolls back" by committing.
      rollback: (body) => db.transaction((tx: any) => body(tx)),
      // Runs both writes and never rolls the savepoint back.
      savepoint: (outer, inner) =>
        db.transaction(async (tx: any) => {
          await outer(tx);
          await inner(tx);
        }),
      // Swallows the error, so the transaction commits the first write and nothing rejects.
      failing: (body) => db.transaction((tx: any) => body(tx).catch(() => {})),
    };
    return {
      ...meta,
      dialect: "postgres",
      count: async (tag) => (await db.query("select count(*)::int as n from driver_matrix.scratch where tag = $1", [tag])).rows[0].n,
      shapes: [
        { name: "integer parameters bound plus one", kind: "async", scope: (fn) => fn(corrupt) },
        { name: "rollback that commits", kind: "async", tx: broken },
      ],
      close: () => db.close(),
    };
  });
}
