// The sqlite3 package's callback Database, and the sqlite package's promise wrapper over it
// (--wrapper sqlite), on a database file the runner seeded (--file).
//   sqlite3: the Database, and BEGIN/COMMIT inside db.serialize(), where motherload's statement has to
//            queue in order with the db.run("BEGIN") and db.run("COMMIT") around it.
//   sqlite:  the Database from open({ filename, driver }), and BEGIN/COMMIT by exec().
import { args, manualTx, notRun, requireFrom, runJob, type Body, type TxOps } from "../lib/harness.ts";

const a = args();
const wrapper = a.wrapper === "sqlite";
const meta = wrapper ? { pkg: "sqlite", version: a.version!, target: `SQLite file (over sqlite3 ${a.driverVersion})` } : { pkg: "sqlite3", version: a.version!, target: "SQLite file" };

let sqlite3: any;
let sqlite: any;
try {
  sqlite3 = requireFrom(a.pkg!);
  if (wrapper) sqlite = requireFrom("sqlite");
} catch (e) {
  notRun(meta, `cannot load: ${(e as Error).message.split("\n")[0]}`);
  process.exit(0);
}

await runJob(meta, async () => {
  if (wrapper) {
    const db = await sqlite.open({ filename: a.file!, driver: sqlite3.Database });
    return {
      ...meta,
      dialect: "sqlite",
      count: async (tag) => (await db.get("select count(*) as n from scratch where tag = ?", [tag])).n,
      shapes: [
        { name: "Database (open())", kind: "async", scope: (fn) => fn(db) },
        {
          name: "Database in BEGIN/COMMIT (exec)",
          kind: "async",
          inTransaction: true,
          scope: async (fn) => {
            await db.exec("BEGIN");
            try {
              await fn(db);
              await db.exec("COMMIT");
            } catch (e) {
              await db.exec("ROLLBACK");
              throw e;
            }
          },
          tx: manualTx({
            open: async () => ({ client: db }),
            begin: (c) => c.exec("BEGIN"),
            commit: (c) => c.exec("COMMIT"),
            rollback: (c) => c.exec("ROLLBACK"),
            exec: (c, sql) => c.exec(sql),
          }),
        },
      ],
      close: () => db.close(),
    };
  }
  const db: any = await new Promise((resolve, reject) => {
    const d = new sqlite3.Database(a.file!, (e: unknown) => (e ? reject(e) : resolve(d)));
  });
  const run = (sql: string, params: unknown[] = []) => new Promise<void>((resolve, reject) => db.run(sql, params, (e: unknown) => (e ? reject(e) : resolve())));
  // Every statement of the transaction is queued inside one db.serialize() callback; motherload's
  // run() issues its statement synchronously, so it lands between BEGIN and COMMIT.
  const serialized = (steps: (string | Body)[]) =>
    new Promise<void>((resolve, reject) => {
      const pending: Promise<unknown>[] = [];
      db.serialize(() => {
        for (const step of steps) {
          if (typeof step === "string") pending.push(run(step));
          else pending.push(step(db));
        }
      });
      Promise.all(pending).then(() => resolve(), reject);
    });
  const tx: TxOps = {
    commit: (body) => serialized(["BEGIN", body, "COMMIT"]),
    rollback: (body) => serialized(["BEGIN", body, "ROLLBACK"]),
    savepoint: (outer, inner) => serialized(["BEGIN", outer, "SAVEPOINT dm_sp", inner, "ROLLBACK TO SAVEPOINT dm_sp", "COMMIT"]),
    // The asynchronous form: await each step and roll back on the error.
    failing: async (body) => {
      await run("BEGIN");
      try {
        await body(db);
        await run("COMMIT");
      } catch (e) {
        await run("ROLLBACK");
        throw e;
      }
    },
  };
  return {
    ...meta,
    dialect: "sqlite",
    count: (tag) => new Promise((resolve, reject) => db.get("select count(*) as n from scratch where tag = ?", [tag], (e: unknown, row: any) => (e ? reject(e) : resolve(row.n)))),
    shapes: [
      { name: "Database", kind: "async", scope: (fn) => fn(db) },
      {
        name: "Database in BEGIN/COMMIT (serialize)",
        kind: "async",
        inTransaction: true,
        scope: async (fn) => {
          await run("BEGIN");
          try {
            await fn(db);
            await run("COMMIT");
          } catch (e) {
            await run("ROLLBACK");
            throw e;
          }
        },
        tx,
      },
    ],
    close: () => new Promise((r) => db.close(r)),
  };
});
