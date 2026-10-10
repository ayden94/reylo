// mysql2 against a MySQL container (--port, --target): the promise Pool, Connection and
// PoolConnection, a PoolConnection inside beginTransaction(), and the callback Pool, Connection
// and PoolConnection passed without .promise().
import { args, lazy, manualTx, notRun, requireFrom, runJob } from "../lib/harness.ts";

const a = args();
const meta = { pkg: "mysql2", version: a.version!, target: a.target! };
const config = { host: "127.0.0.1", port: Number(a.port), user: "root", password: "porg", database: "driver_matrix" };

let mysql: any;
let mysqlPromise: any;
try {
  mysql = requireFrom(a.pkg!);
  mysqlPromise = requireFrom(`${a.pkg}/promise`);
} catch (e) {
  notRun(meta, `cannot load ${a.pkg}: ${(e as Error).message}`);
  process.exit(0);
}

await runJob(meta, async () => {
  const pool = mysqlPromise.createPool({ ...config, connectionLimit: 10 });
  const conn = lazy(() => mysqlPromise.createConnection(config));
  const cbPool = mysql.createPool({ ...config, connectionLimit: 5 });
  const cbConn = lazy(() => mysql.createConnection(config));
  const checkout = async () => {
    const c = await pool.getConnection();
    return { client: c, release: () => c.release() };
  };
  return {
    ...meta,
    dialect: "mysql",
    count: async (tag) => (await pool.query("select count(*) as n from scratch where tag = ?", [tag]))[0][0].n,
    shapes: [
      { name: "promise Pool", kind: "async", scope: (fn) => fn(pool) },
      { name: "promise Connection", kind: "async", scope: async (fn) => fn(await conn()) },
      {
        name: "promise PoolConnection (getConnection())",
        kind: "async",
        scope: async (fn) => {
          const { client, release } = await checkout();
          try {
            await fn(client);
          } finally {
            release();
          }
        },
      },
      {
        name: "promise PoolConnection in beginTransaction()",
        kind: "async",
        inTransaction: true,
        scope: async (fn) => {
          const { client, release } = await checkout();
          try {
            await client.beginTransaction();
            await fn(client);
            await client.commit();
          } catch (e) {
            await client.rollback().catch(() => {});
            throw e;
          } finally {
            release();
          }
        },
        tx: manualTx({
          open: checkout,
          begin: (c) => c.beginTransaction(),
          commit: (c) => c.commit(),
          rollback: (c) => c.rollback(),
          exec: (c, sql) => c.query(sql),
        }),
      },
      { name: "callback Pool (no .promise())", kind: "async", scope: (fn) => fn(cbPool) },
      { name: "callback Connection (no .promise())", kind: "async", scope: async (fn) => fn(await cbConn()) },
      {
        name: "callback PoolConnection (no .promise())",
        kind: "async",
        scope: async (fn) => {
          const c: any = await new Promise((resolve, reject) => cbPool.getConnection((e: unknown, c: unknown) => (e ? reject(e) : resolve(c))));
          try {
            await fn(c);
          } finally {
            c.release();
          }
        },
      },
    ],
    close: async () => {
      await pool.end();
      await (await conn()).end();
      await new Promise((r) => cbPool.end(r));
      (await cbConn()).end();
    },
  };
});
