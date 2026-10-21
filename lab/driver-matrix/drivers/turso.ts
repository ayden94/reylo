// Turso's new engine, @tursodatabase/database, on a database file the runner seeded (--file):
//   --api promise  the Database connect() resolves to, the same inside BEGIN/COMMIT by exec(), and
//                  (0.7 and later) the Transaction db.transactionAsync(fn) hands its callback
//   --api compat   the synchronous Database of @tursodatabase/database/compat, and its
//                  db.transaction(fn), as better-sqlite3's
// 0.4 has no SAVEPOINT ("SAVEPOINT not supported yet"), so its shapes skip S7c, as the compat
// db.transaction(fn) does on every release. And 0.4 answers json_group_array and json_group_object
// over no rows with NULL, where SQLite and Turso 0.6 and later answer `[]` and `{}`, so it skips
// S10j, whose `none` is such an array: Reylo types it as never NULL, as SQLite has it.
import { args, callbackTx, manualTx, notRun, rawSavepoint, requireFrom, runJob, type Shape } from "../lib/harness.ts";

const a = args();
const compat = a.api === "compat";
const meta = { pkg: compat ? "@tursodatabase/database/compat" : "@tursodatabase/database", version: a.version!, target: "SQLite file (Turso's engine)" };

let turso: any;
try {
  turso = await import(requireFrom.resolve(compat ? `${a.pkg}/compat` : a.pkg!));
} catch (e) {
  notRun(meta, `cannot load ${a.pkg}: ${(e as Error).message.split("\n")[0]}`);
  process.exit(0);
}
const savepoints = !a.version!.startsWith("0.4.");
const noSavepoint = savepoints ? undefined : { S7c: "Turso 0.4 has no SAVEPOINT (\"SAVEPOINT not supported yet\")" };
const emptyAggregates = savepoints ? undefined : { S10j: "Turso 0.4 answers json_group_array over no rows with NULL, where SQLite and Turso 0.6 and later answer []" };

await runJob(meta, async () => {
  if (compat) {
    const db = new turso.Database(a.file!);
    const counter = db.prepare("select count(*) as n from scratch where tag = ?");
    const shapes: Shape[] = [
      { name: "compat Database", kind: "sync", scope: (fn) => fn(db) },
      // A transaction function called inside another fails, where better-sqlite3's runs as a savepoint.
      { name: "compat db.transaction(fn)", kind: "sync", syncTx: db, skip: { S7c: "the compat Database's db.transaction(fn) inside another fails (\"cannot start a transaction within a transaction\"), where better-sqlite3's runs as a savepoint" } },
    ];
    return { ...meta, dialect: "sqlite", count: async (tag) => Number(counter.get(tag).n), shapes, close: async () => db.close(), skip: emptyAggregates };
  }
  const db = await turso.connect(a.file!);
  const shapes: Shape[] = [
    { name: "Database (connect())", kind: "async", scope: (fn) => fn(db) },
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
        exec: savepoints ? (c, sql) => c.exec(sql) : undefined,
      }),
      skip: noSavepoint,
    },
  ];
  if (typeof db.transactionAsync === "function") {
    const begin = (fn: (tx: any) => Promise<unknown>) => db.transactionAsync(async (tx: any) => fn(tx))();
    shapes.push({
      name: "db.transactionAsync(fn) Transaction",
      kind: "async",
      inTransaction: true,
      scope: (fn) => begin(fn),
      tx: callbackTx(begin, rawSavepoint((tx, sql) => tx.exec(sql))),
    });
  }
  return {
    ...meta,
    dialect: "sqlite",
    count: async (tag) => Number((await db.prepare("select count(*) as n from scratch where tag = ?").get(tag)).n),
    shapes,
    close: async () => db.close(),
    skip: emptyAggregates,
  };
});
