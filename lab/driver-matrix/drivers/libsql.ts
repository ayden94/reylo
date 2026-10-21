// @libsql/client on a local database file (file: URL, no server): the Client and an interactive
// client.transaction("write") (a savepoint by hand inside it).
import { args, notRun, requireFrom, runJob, type TxOps } from "../lib/harness.ts";

const a = args();
const meta = { pkg: "@libsql/client", version: a.version!, target: "SQLite file (file: URL)" };

let libsql: any;
try {
  libsql = requireFrom(a.pkg!);
} catch (e) {
  notRun(meta, `cannot load ${a.pkg}: ${(e as Error).message.split("\n")[0]}`);
  process.exit(0);
}

await runJob(meta, async () => {
  const client = libsql.createClient({ url: `file:${a.file}` });
  const inTx = async (body: (tx: any) => Promise<unknown>, end: "commit" | "rollback") => {
    const tx = await client.transaction("write");
    try {
      await body(tx);
      await tx[end]();
    } catch (e) {
      await tx.rollback().catch(() => {});
      throw e;
    } finally {
      tx.close();
    }
  };
  const tx: TxOps = {
    commit: (body) => inTx(body, "commit"),
    rollback: (body) => inTx(body, "rollback"),
    savepoint: (outer, inner) =>
      inTx(async (t) => {
        await outer(t);
        await t.execute("SAVEPOINT dm_sp");
        await inner(t);
        await t.execute("ROLLBACK TO SAVEPOINT dm_sp");
      }, "commit"),
    failing: (body) => inTx(body, "commit"),
  };
  return {
    ...meta,
    dialect: "sqlite",
    count: async (tag) => Number((await client.execute({ sql: "select count(*) as n from scratch where tag = ?", args: [tag] })).rows[0].n),
    shapes: [
      { name: "Client", kind: "async", scope: (fn) => fn(client) },
      { name: 'client.transaction("write")', kind: "async", inTransaction: true, scope: (fn) => inTx(fn, "commit"), tx },
    ],
    close: async () => client.close(),
  };
});
