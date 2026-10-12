// Failing controls: clients that are deliberately wrong, so every row here must FAIL. They show
// the scenarios can fail at all (a green matrix with no failing control is no evidence).
//   --case pg      PGlite behind a client that binds each integer parameter plus one and a plain
//                  Uint8Array as its JSON text, as mariadb does without a Buffer; an unsafe()
//                  client that binds an array as its elements joined by commas, as Bun.sql does,
//                  running statements without their inputs, so reylo converts no array; an
//                  unsafe() client that binds every array as its array literal, a JSON
//                  parameter's too, as reylo did on Bun.sql at cf2bc64; and a
//                  transaction whose "rollback" commits and whose failure is swallowed
//   --case sqlite  a node:sqlite database declared asynchronous, so runSync must refuse it and does not;
//                  and a transaction(fn) whose statements answer through a promise, so runSync must throw
import { arrayLiteral } from "../../../src/execute.ts";
import { args, callbackTx, requireFrom, runJob, seedSql, type TxOps } from "../lib/harness.ts";

const a = args();

if (a.case === "sqlite") {
  const { DatabaseSync } = await import("node:sqlite");
  const meta = { pkg: "control", version: "sqlite", target: "node:sqlite, declared asynchronous" };
  await runJob(meta, async () => {
    const db = new DatabaseSync(":memory:");
    db.exec(seedSql("sqlite"));
    // better-sqlite3's transaction(fn), nesting as savepoints, over statements whose all() is async.
    let depth = 0;
    const promised = {
      prepare: (sql: string) => {
        const statement = db.prepare(sql);
        return { all: async (...params: any[]) => statement.all(...params) };
      },
      transaction: (fn: () => unknown) => () => {
        const name = `sp${depth}`;
        db.exec(depth ? `SAVEPOINT ${name}` : "BEGIN");
        depth++;
        try {
          const result = fn();
          depth--;
          db.exec(depth ? `RELEASE ${name}` : "COMMIT");
          return result;
        } catch (e) {
          depth--;
          db.exec(depth ? `ROLLBACK TO ${name}; RELEASE ${name}` : "ROLLBACK");
          throw e;
        }
      },
    };
    return {
      ...meta,
      dialect: "sqlite",
      count: async (tag) => (db.prepare("select count(*) as n from scratch where tag = ?").get(tag) as any).n,
      shapes: [
        { name: "sync client declared async", kind: "async", scope: (fn) => fn(db) },
        { name: "db.transaction(fn) whose statements answer through a promise", kind: "sync", syncTx: promised },
      ],
    };
  });
} else {
  const { PGlite } = await import(requireFrom.resolve("pglite-05"));
  const meta = { pkg: "control", version: "pg", target: "PGlite behind a corrupting wrapper" };
  await runJob(meta, async () => {
    const db = new PGlite();
    await db.exec(seedSql("postgres"));
    const corrupt = { query: (text: string, values?: unknown[]) => db.query(text, values?.map((v) => (typeof v === "number" ? v + 1 : v instanceof Uint8Array && !Buffer.isBuffer(v) ? JSON.stringify(v) : v))) };
    const unsafe = (bind: (v: unknown) => unknown) => ({ unsafe: async (text: string, values?: unknown[]) => (await db.query(text, values?.map(bind))).rows });
    const joined = unsafe((v) => (Array.isArray(v) ? v.join(",") : v));
    const literal = unsafe((v) => (Array.isArray(v) ? arrayLiteral(v) : v));
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
        { name: "parameters corrupted: an integer plus one, a Uint8Array as JSON", kind: "async", scope: (fn) => fn(corrupt) },
        { name: "unsafe() binding an array as a,b, statements without inputs", kind: "async", untyped: true, scope: (fn) => fn(joined) },
        { name: "unsafe() binding every array as its array literal, a JSON parameter's too", kind: "async", scope: (fn) => fn(literal) },
        { name: "rollback that commits", kind: "async", tx: broken },
      ],
      close: () => db.close(),
    };
  });
}
