// The synchronous SQLite drivers, on a database file the runner seeded (--file):
//   --lib bs3-13 (any better-sqlite3 alias; --from native-old for the source-built lines)
//   --lib node:sqlite   DatabaseSync, BEGIN/COMMIT by hand, and db.createTagStore()
//   --lib bun:sqlite    under Bun
// On these clients reylo's run() returns the value itself and throws synchronously, so it works
// inside better-sqlite3's and bun:sqlite's db.transaction(fn), whose function must not await.
import { createRequire } from "node:module";
import { args, here, manualTx, notRun, requireFrom, runJob, type Shape } from "../lib/harness.ts";

const a = args();
const lib = a.lib!;
const pkg = lib === "node:sqlite" ? "node:sqlite" : lib === "bun:sqlite" ? "bun:sqlite" : "better-sqlite3";
const version = lib === "node:sqlite" ? process.versions.node : lib === "bun:sqlite" ? ((globalThis as any).Bun?.version ?? "?") : a.version!;
const meta = { pkg, version, target: "SQLite file" };

let open: (file: string) => any;
try {
  if (lib === "node:sqlite") {
    const { DatabaseSync } = await import("node:sqlite");
    open = (file) => new DatabaseSync(file);
  } else if (lib === "bun:sqlite") {
    const { Database } = await import("bun:sqlite" as string);
    open = (file) => new Database(file);
  } else {
    const req = a.from ? createRequire(`${here}${a.from}/package.json`) : requireFrom;
    const Database = req(lib);
    open = (file) => new Database(file);
  }
} catch (e) {
  notRun(meta, `cannot load ${lib}: ${(e as Error).message.split("\n")[0]}`);
  process.exit(0);
}

await runJob(meta, async () => {
  const db = open(a.file!);
  const counter = db.prepare("select count(*) as n from scratch where tag = ?");
  const shapes: Shape[] = [{ name: lib === "node:sqlite" ? "DatabaseSync" : "Database", kind: "sync", scope: (fn) => fn(db) }];
  if (lib === "node:sqlite") {
    shapes.push({
      name: "DatabaseSync in BEGIN/COMMIT (exec)",
      kind: "sync",
      inTransaction: true,
      scope: async (fn) => {
        db.exec("BEGIN");
        try {
          await fn(db);
          db.exec("COMMIT");
        } catch (e) {
          db.exec("ROLLBACK");
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
    });
    if (typeof db.createTagStore === "function") {
      // The tagged-template store Node's docs show; not a prepare() client.
      const store = db.createTagStore();
      shapes.push({ name: "db.createTagStore()", kind: "async", scope: (fn) => fn(store), refused: /A tag store is not a client: pass its database, store\.db\./ });
      shapes.push({ name: "db.createTagStore().db", kind: "sync", scope: (fn) => fn(store.db) });
    }
  } else {
    shapes.push({ name: "db.transaction(fn)", kind: "sync", syncTx: db });
  }
  return {
    ...meta,
    dialect: "sqlite",
    count: async (tag) => counter.get(tag).n,
    shapes,
    close: async () => db.close(),
  };
});
